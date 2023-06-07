#![windows_subsystem = "windows"]

use fltk::app::Screen;
use fltk::image::IcoImage;
use fltk::{app, window::Window};
use fltk::{prelude::*, *};
use fltk_table::{SmartTable, TableOpts};
use notify_rust::Notification;
use std::cell::{Ref, RefCell};
use std::collections::HashSet;
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
use ui::*;

/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/10
 * @time: 13:40
 * @this_file_name:main
 */

fn main() {
    let screens = Screen::all_screens();

    let init_width: i32 = (screens[0].w() as f32 * 0.618) as i32;
    let init_height: i32 = (screens[0].h() as f32 * 0.618) as i32;

    let mut vector: RefCell<Vec<Item>> = RefCell::new(vec![]);

    match get_html(&mut vector) {
        Some(_) => {}
        None => {
            if vector.borrow().is_empty() {
                for _ in 0..90 {
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

    let app = app::App::default().with_scheme(app::Scheme::Gleam);

    let mut wind = Window::default()
        .with_size(init_width, init_height)
        .with_label("OA Notifier");

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

    add_menu(
        &mut wind,
        &mut menubar,
        &mut table,
        &vector,
        sender_keywords
    );

    add_table(&mut table, &mut wind, &mut vector);

    wind.end();
    wind.show();

    drop(vector);

    wind.set_callback(move |_win| {
        if app::event() == enums::Event::Close {
            // _win.visible();

            // } else if app::event() == enums::Event::Show
            // {
            //     _win.set_size(init_width, init_height);
        }
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

            match get_html(&mut now) {
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
                            println!("新通知：{}", curr[i].title)
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
                for k in keys.iter() {
                    println!("keywords: {:?}", k);
                }
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
                    println!("{}", title.title);
                    match Notification::new()
                        .appname("OA Notifier⚠️⚠️⚠️")
                        .subtitle(title.source.as_str())
                        .body(title.title.as_str())
                        .show()
                    {
                        Ok(_) => {
                            println!("Notification successfully");
                        }
                        Err(_) => println!("Notification error"),
                    }
                }
            }
            // 更新表
            for i in 0..now.borrow().len() {
                if now.borrow()[i as usize].is_top {
                    table.set_label_font(enums::Font::Helvetica);
                    table.set_cell_value(
                        i as i32,
                        0,
                        &format!("[置顶]{}", &now.borrow()[i as usize].title),
                    );
                } else {
                    table.set_label_font(enums::Font::Times);
                    table.set_cell_value(i as i32, 0, &now.borrow()[i as usize].title);
                }
                table.set_cell_value(i as i32, 1, &now.borrow()[i as usize].source);
                table.set_cell_value(i as i32, 2, &now.borrow()[i as usize].time);
                table.set_cell_value(i as i32, 3, &now.borrow()[i as usize].href);
            }
            table.redraw();
            drop(now);
        })
    };

    app.run().unwrap();
    drop(_guard);
}
