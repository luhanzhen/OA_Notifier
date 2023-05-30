#![windows_subsystem = "windows"]

use std::cell::{Ref, RefCell};
use std::{fs};
use std::path::Path;
use fltk::{prelude::*, *};
use fltk::image::{IcoImage};
use fltk_table::{SmartTable, TableOpts};
use notify_rust::Notification;
use fltk::{app, window::Window};
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



// fn main() {
//
//
//
//
//     // let app = app::App::default();
//     // let mut win = window::Window::default().with_size(900, 300);
//     // win.make_resizable(true);
//     // let mut b = browser::MultiBrowser::new(10, 10, 900 - 20, 300 - 20, "");
//     // let widths = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50];
//     // b.set_column_widths(widths);
//     // b.set_column_char('\t');
//     // // 现在在我们的`add()`方法中可以使用'\t'来制表符
//     // b.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
//     // b.add("root\t2888\t0.0\t0.0\t1352\t0\ttty3\tSW\tAug15\t0:00\t@b@f/sbin/mingetty tty3");
//     // b.add("erco\t2889\t0.0\t13.0\t221352\t0\ttty3\tR\tAug15\t1:34\t@b@f/usr/local/bin/render a35 0004");
//     // b.add("uucp\t2892\t0.0\t0.0\t1352\t0\tttyS0\tSW\tAug15\t0:00\t@b@f/sbin/agetty -h 19200 ttyS0 vt100");
//     // b.add("root\t13115\t0.0\t0.0\t1352\t0\ttty2\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty2");
//     // b.add(
//     //     "root\t13464\t0.0\t0.0\t1352\t0\ttty1\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty1 --noclear",
//     // );
//     // win.end();
//     // win.show();
//     // app.run().unwrap();
// }


fn main() {

    let mut vector: RefCell<Vec<Item>> = RefCell::new(vec![]);

    get_html(&mut vector);
    if vector.borrow().is_empty() {
        for _ in 0..90 {
            let item = Item { title: String::from("不能访问OA，网络不可用"), time: String::from("。"), source: String::from("。"), href: String::from("。"), is_top: false };
            vector.borrow_mut().push(item);
        }
    }

    let app = app::App::default().with_scheme(app::Scheme::Gleam);


    let mut wind = Window::default().with_size(INIT_WIDTH, INIT_HEIGHT).with_label("OA Notifier");

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

    drop(vector);
    let timer = timer::Timer::new();
    let _guard = {
        // let count = count.clone();
        timer.schedule_repeating(chrono::Duration::seconds(600), move || {
            // println!("hello world");
            let mut now: RefCell<Vec<Item>> = RefCell::new(vec![]);
            get_html(&mut now);
            if now.borrow().len() != table.rows() as usize
            {
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
            if  !title.is_empty(){
                // println!("改变了");

                Notification::new().appname("OA Notifier").subtitle("OA 更新")
                    .body(title.as_str())
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
            drop(now);
        })
    };

    println!("This code has been executed after 3 seconds");


    if fs::metadata("./icon.ico").is_ok() {
        let icon: IcoImage = IcoImage::load(&Path::new("icon.ico")).unwrap();
        wind.set_icon(Some(icon));
    }

    wind.set_callback(move |_| {
        if app::event() == enums::Event::Close {}
    });


    app.run().unwrap();
    drop(_guard);
}
