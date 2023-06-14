#![windows_subsystem = "windows"]

use fltk::app::Screen;
use fltk::image::IcoImage;
use fltk::{app, window::Window};
use fltk::{prelude::*, *};
use fltk_table::{SmartTable, TableOpts};
use notify_rust::Notification;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;

use fltk::enums::Color;
use std::fs;
use std::path::Path;
use std::sync::mpsc;

extern crate chrono;
extern crate timer;

pub mod html;
pub mod item;
pub mod ui;

use html::*;
use item::Item;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{icon::Icon, TrayEvent, TrayIconBuilder};
use ui::*;

extern crate single_instance;

use crate::item::VERSION;
use single_instance::SingleInstance;

/**
 * <p>@project_name: OANotifier
 * <p/>
 * <p>@user_name: luhanzhen
 * <p/>
 * <p>@date: 2023/5/10
 * <p/>
 * <p>@time: 13:40
 * <p/>
 * <p>@this_file_name:main
 */

fn main() {
    let instance = SingleInstance::new("OANotifier").unwrap();
    if !instance.is_single() {
        return;
    }

    // is_reachable("github.com:80");
    // is_reachable("59.72.109.14:7788");
    // is_reachable("192.168.1.100:7788");
    if !is_reachable("oa.jlu.edu.cn:80") {
        return;
    }
    let screens = Screen::all_screens();

    let init_width: i32 = (screens[0].w() as f32 * 0.618) as i32;
    let init_height: i32 = (screens[0].h() as f32 * 0.618) as i32;

    let mut vector: RefCell<Vec<Item>> = RefCell::new(vec![]);

    match get_table(&mut vector) {
        Some(_) => {}
        None => {
            if vector.borrow().is_empty() {
                for _ in 0..10 {
                    let item = Item {
                        title: String::from("不能访问OA，网络不可用"),
                        time: String::from("。"),
                        source: String::from("。"),
                        href: String::from("。"),
                        is_top: false,
                    };
                    vector.borrow_mut().push(item);
                }
            }
        }
    };

    let app = app::App::default().with_scheme(app::Scheme::Oxy);

    let mut wind = Window::default()
        .with_size(init_width, init_height)
        .with_label("OA Notifier");
    wind.set_color(Color::from_rgb(246, 251, 255));
    wind.set_selection_color(Color::from_rgb(246, 251, 255));
    wind.set_label_color(Color::from_rgb(246, 251, 255));

    if fs::metadata("./icon.ico").is_ok() {
        let icon: IcoImage = IcoImage::load(&Path::new("icon.ico")).unwrap();
        wind.set_icon(Some(icon));
    }
    let mut menubar = menu::MenuBar::new(0, 0, init_width, 25, "");
    let mut table = SmartTable::default()
        .with_size(wind.width() - 2, wind.height() - 25)
        .with_pos(0, 25)
        .with_opts(TableOpts {
            rows: vector.borrow().len() as i32,
            cols: 5,
            editable: true,
            ..Default::default()
        });

    let (sender_keywords, receiver_keywords) = mpsc::channel();

    add_menu(&mut wind, &mut menubar, &mut table, sender_keywords);

    add_table(&mut table, &mut wind, &mut vector);

    wind.end();
    wind.show();

    drop(vector);

    wind.set_callback(move |_win| {
        if app::event() == enums::Event::Close {
            dialog::message_title("退出确认?");
            match dialog::choice2(
                _win.x() + _win.width() / 3 * 2,
                _win.y() + 10,
                "确定要退出吗？",
                "点错了",
                "隐藏",
                "确实",
            ) {
                Some(choice) => {
                    // println!("full screen!!!{choice}");
                    if choice == 2 {
                        app::quit();
                    } else if choice == 1 {
                        _win.platform_hide();
                    }
                }
                None => {}
            }
        }

        // if _ == enums::Event::Resize {
        //     println!("full screen!!!");
        // }
    });

    let timer = timer::Timer::new();
    let mut keywords = String::from("");

    let _guard = {
        // let count = count.clone();

        timer.schedule_repeating(chrono::Duration::seconds(600), move || {
            let mut now: RefCell<Vec<Item>> = RefCell::new(vec![]);
            match receiver_keywords.try_recv() {
                Ok(keyword) => {
                    keywords = keyword;
                }
                Err(_) => {}
            }

            match get_table(&mut now) {
                Some(_) => {}
                None => {
                    return;
                }
            }
            // if now.borrow().len() != table.rows() as usize {
            //     return;
            // }
            let changed = |table: &mut SmartTable, curr: &Ref<Vec<Item>>| -> Vec<Item> {
                let mut new_items = vec![];
                if !curr.is_empty() {
                    let mut set: HashSet<String> = HashSet::new();
                    for i in 0..table.rows() {
                        let other = table.cell_value(i as i32, 0).replace("[置顶]", "");
                        set.insert(other);
                    }
                    for i in 0..curr.len() {
                        if !set.contains(curr[i].title.as_str()) {
                            new_items.push(curr[i].clone());
                            // println!("新通知：{}", curr[i].title)
                        }
                    }
                }
                return new_items;
            };

            let filter = |new_items: Vec<Item>, keyword: String| -> Vec<Item> {
                let keys: Vec<&str> = keyword.split_whitespace().collect();
                if keys.is_empty()
                //没有关键字 就什么都不做。
                {
                    return new_items;
                }
                // for k in keys.iter() {
                //     println!("keywords: {:?}", k);
                // }
                let mut filtered: Vec<Item> = vec![];
                for item in new_items {
                    let content: Vec<String>;
                    match get_content(item.href.as_str()) {
                        Some((con, _)) => {
                            content = con;
                        }
                        None => {
                            content = Vec::new();
                        }
                    }
                    let mut found = false;
                    for key in keys.iter() {
                        if item.title.as_str().contains(key)
                            || item.source.contains(key)
                            || item.time.contains(key)
                        {
                            found = true;
                        }
                        if found {
                            break;
                        }
                        for line in content.iter() {
                            if line.contains(key) {
                                found = true;
                                break;
                            }
                        }
                        if found {
                            break;
                        }
                    }
                    if found {
                        filtered.push(item.clone());
                    }
                }
                return filtered;
            };
            let new_items = changed(&mut table, &now.borrow());
            let filtered = filter(new_items, keywords.clone());

            if !filtered.is_empty() {
                // println!("改变了");
                for title in filtered {
                    match Notification::new()
                        .appname("OA Notifier")
                        .subtitle(title.source.as_str())
                        .body(title.title.as_str())
                        .auto_icon()
                        .show()
                    {
                        Ok(_) => {
                            // println!("Notification successfully");
                        }
                        Err(_) => {
                            // println!("Notification error");
                        }
                    }
                }
            }
            // 更新表
            for i in 0..now.borrow().len() {
                if now.borrow()[i as usize].is_top {
                    table.set_label_font(enums::Font::Helvetica);
                    table.set_cell_value(i as i32, 0, &format!("[置顶]{}", &now.borrow()[i].title));
                } else {
                    table.set_label_font(enums::Font::Times);
                    table.set_cell_value(i as i32, 0, &now.borrow()[i].title);
                }
                table.set_cell_value(i as i32, 1, &now.borrow()[i].source);
                table.set_cell_value(i as i32, 2, &now.borrow()[i].time);
                table.set_cell_value(i as i32, 3, &now.borrow()[i].href);
                table.set_cell_value(i as i32, 4, "");
            }
            table.redraw();
            drop(now);
        })
    };

    // 设置托盘
    let tray_menu = Menu::new();
    let quit_i = MenuItem::new("退出", true, None);
    let about_i = MenuItem::new("关于", true, None);
    let update_i = MenuItem::new("检查更新", true, None);
    tray_menu.append_items(&[&update_i, &about_i, &quit_i]);
    let mut tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("OA Notifier")
        .build()
        .unwrap();
    if fs::metadata("./icon.ico").is_ok() {
        let icon = Icon::from_path("./icon.ico", None).unwrap();
        tray.set_icon(Some(icon)).unwrap();
    }
    while app.wait() {
        // 处理托盘事件
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == quit_i.id() {
                app.quit();
            }
            if event.id == update_i.id() {
                if !wind.visible() {
                    wind.platform_show();
                }
                check_update(init_width / 2, init_height / 2)
            }
            if event.id == about_i.id() {
                if !wind.visible() {
                    wind.platform_show();
                }
                dialog::message_title("OA Notifier 关于");
                let message = format!("使用本软件即同意以下内容:
                                    本软件当前版本号为{}。
                                    本软件用于自动提醒吉林大学OA更新内容。
                                    双击表格点开信息，右击表格在浏览器中打开网页。
                                    支持搜索，但是搜索不能查找内容，原因是考虑到搜索的快速响应。
                                    支持过滤关键信息，过滤可以是多个关键字，关键字之间必须用空格隔开。
                                    无关键字的情况下，会通知全部信息。
                                    内容页支持图片显示，附件下载目前尚未支持，双击内容页或者图片也可以在浏览器中打开网页。
                                    支持以托盘方式在桌面右下角显示。
                                    支持检查是否存在最新版。
                                    本软件每隔十分钟爬取oa网站前若干页的内容。
                                    本软件承诺保护用户隐私，不收集任何信息。
                                    本软件著作权及其解释权归作者镇路晗所有。
                                    项目源代码及最新版在网站[https://github.com/luhanzhen/OA_Notifier]上。
                                    有好的建议或者其它需求可以给我留言。
                                    个人用户享有使用权，作者不对使用者造成的后果负责。
                                    本软件仅供个人使用，不可用于商业盈利目的或者非法目的。
                                    请主动遵守国家法律法规和学校的有关规定，非法或者违规行为造成的法律责任和后果自负。", VERSION);
                dialog::message(init_width / 2, init_height / 2, message.as_str());
            }
        }

        if let Ok(_) = TrayEvent::receiver().try_recv() {
            wind.platform_show();
        }
    }

    app.run().unwrap();
    drop(_guard);
}
