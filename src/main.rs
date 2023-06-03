#![windows_subsystem = "windows"]

use fltk::app::Screen;
use fltk::image::IcoImage;
use fltk::{app, window::Window};
use fltk::{prelude::*, *};
use fltk_table::{SmartTable, TableOpts};
use notify_rust::Notification;
use std::cell::{Ref, RefCell};
use std::fs;
use std::path::Path;

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
    // println!("{} - {}", screens[0].w(), screens[0].h());
    // println!("{} - {}", screens[1].w(), screens[1].h());

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

    add_menu(&mut wind, &mut menubar, &mut table, &vector);

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
    let _guard = {
        // let count = count.clone();
        timer.schedule_repeating(chrono::Duration::seconds(600), move || {
            // println!("hello world");
            let mut now: RefCell<Vec<Item>> = RefCell::new(vec![]);
            match get_html(&mut now) {
                Some(_) => {}
                None => {
                    return;
                }
            }
            if now.borrow().len() != table.rows() as usize {
                return;
            }
            let changed = |table: &mut SmartTable, curr: &Ref<Vec<Item>>| -> String {
                let mut title = String::from("");
                if curr.is_empty() {
                    return title;
                } else {
                    for i in 0..curr.len() {
                        let other = table.cell_value(i as i32, 0).replace("[置顶]", "");
                        if !curr[i].title.eq(&other) {
                            title = curr[i].title.clone();
                            return title;
                        }
                    }
                    return title;
                }
            };
            let title = changed(&mut table, &now.borrow());
            if !title.is_empty() {
                // println!("改变了");
                match Notification::new()
                    .appname("OA Notifier")
                    .subtitle("OA 更新")
                    .body(title.as_str())
                    .icon("firefox")
                    .show()
                {
                    Ok(_) => println!("Notification successfully"),
                    Err(_) => println!("Notification error"),
                }
            }
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
