#![windows_subsystem = "windows"]

use std::cell::{Ref, RefCell};
use std::{fs};
use std::path::Path;
use fltk::{prelude::*, *};
use fltk::image::IcoImage;
use fltk_table::{SmartTable, TableOpts};
use notify_rust::Notification;
extern crate timer;
extern crate chrono;

pub mod ui;
pub mod html;
pub mod item;

use ui::*;
use html::*;
use item::Item;


/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/10
 * @time: 13:40
 * @this_file_name:main
 */


fn main() {
    let mut vector: RefCell<Vec<Item>> = RefCell::new(vec![]);

    get_html(&mut vector);

    let app = app::App::default().with_scheme(app::Scheme::Gleam);


    let mut wind = window::Window::default().with_size(INIT_WIDTH, INIT_HEIGHT).with_label("OA Notifier");

    let mut menubar = menu::MenuBar::new(0, 0, INIT_WIDTH, 25, "");
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


    let timer = timer::Timer::new();
    let _guard = {
        // let count = count.clone();
        timer.schedule_repeating(chrono::Duration::seconds(600), move || {
            // println!("hello world");
            let mut now: RefCell<Vec<Item>> = RefCell::new(vec![]);
            get_html(&mut now);
            let changed = |table: &mut SmartTable, curr: &Ref<Vec<Item>>| -> bool {
                if curr.is_empty() {
                    return false;
                } else {
                    for i in 0..curr.len() {
                        let other = table.cell_value(i as i32, 0).replace("[置顶]", "");
                        if !curr[i].title.eq(&other) {
                            return true;
                        }
                    }
                    return false;
                }
            };
            if changed(&mut table, &now.borrow()) {
                println!("改变了");
                Notification::new().appname("OA Notifier").subtitle("OA 更新")
                    .body("OA有新的消息.")
                    .icon("firefox")
                    .show().unwrap();
            }
            for i in 0..now.borrow().len()
            {
                if now.borrow()[i as usize].is_top {
                    table.set_label_font(enums::Font::Helvetica);
                    table.set_cell_value(i as i32, 0, &format!("[置顶]{}", &now.borrow()[i as usize].title));
                } else {
                    table.set_label_font(enums::Font::Times);
                    table.set_cell_value(i as i32, 0, &now.borrow()[i as usize].title);
                }
                table.set_cell_value(i as i32, 1, &now.borrow()[i as usize].source);
                table.set_cell_value(i as i32, 2, &now.borrow()[i as usize].time);
                table.set_cell_value(i as i32, 3, &now.borrow()[i as usize].href);
            }
            table.redraw();
        })
    };

    println!("This code has been executed after 3 seconds");


    if fs::metadata("./icon.ico").is_ok() {
        let icon: IcoImage = IcoImage::load(&Path::new("icon.ico")).unwrap();
        wind.set_icon(Some(icon));
    }

    wind.set_callback(move |_| {
        if app::event() == enums::Event::Close {
        }
    });


    app.run().unwrap();
    drop(_guard);
}
