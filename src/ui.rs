use crate::html::{get_content, get_update};
use crate::item::{Item, VERSION};
use fltk::app::redraw;
use fltk::enums::{Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::image::{JpegImage, PngImage};
use fltk::menu::MenuBar;
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_table::SmartTable;
use std::cell::RefCell;
use std::sync::mpsc::Sender;
use webbrowser;

/**
 * <p> @project_name: OANotifier <p/>
 * <p> @user_name: luhanzhen <p/>
 * <p>@date: 2023/5/21<p/>
 * <p>@time: 13:34<p/>
 * <p>@this_file_name:ui<p/>
 */

pub fn check_update(x: i32, y: i32) {
    // dialog::message_title("OA Notifier 更新");
    // dialog::message_set_hotspot(false);
    // dialog::message(x, y, "正在检查....");

    match get_update() {
        Some(new_version) => match new_version.find("@@first@@") {
            Some(first) => match new_version.find("@@second@@") {
                Some(second) => {
                    let str = &new_version[(first + 9)..second];
                    // println!("{new_version}:  {:?}", str);
                    if !str.contains(VERSION) {
                        // 存在最新版。
                        // dialog::message(x, y, "已经是最新版。");
                        let third = match new_version.find("@@third@@") {
                            Some(x) => x,
                            None => usize::MAX,
                        };
                        let four = match new_version.find("@@four@@") {
                            Some(x) => x,
                            None => usize::MAX,
                        };
                        let mess = if third != usize::MAX && four != usize::MAX {
                            format!("有最新版，你要更新吗？")
                        } else {
                            let update_log = &new_version[(third + 9)..four];
                            format!("有最新版，你要更新吗？{}", update_log)
                        };
                        dialog::message_title("OA Notifier 更新");
                        match dialog::choice2(x, y, mess.as_str(), "算了", "在浏览器中下载", "")
                        {
                            Some(choice) => {
                                if choice == 1 {
                                    webbrowser::open(str).unwrap();
                                    dialog::message_title("OA Notifier 更新");
                                    dialog::message(x, y, "下载最新版，删除现在的软件即可。");
                                }
                            }
                            None => {}
                        }
                    } else {
                        dialog::message_title("OA Notifier 更新");
                        dialog::message(x, y, "当前版本已经是最新版。");
                    }
                }
                None => {
                    dialog::message_title("OA Notifier 更新");
                    dialog::message(x, y, "网络好像有点问题。");
                }
            },
            None => {
                dialog::message_title("OA Notifier 更新");
                dialog::message(x, y, "网络好像有点问题。");
            }
        },
        None => {
            dialog::message_title("OA Notifier 更新");
            dialog::message(x, y, "网络好像有点问题。");
        }
    }
}

fn show_content(url: &String, title: &String, width: i32, height: i32) {
    match get_content(url) {
        Some((content, images)) => {
            let mut buf = text::TextBuffer::default();
            for e in content.iter() {
                buf.append(e);
            }
            buf.line_start(0);

            let mut win = window::Window::default()
                .with_size(width, height)
                .with_label(title);
            let mut txt = text::TextDisplay::default()
                .with_size(win.width(), win.height() - 3)
                .with_pos(0, 3);

            //为了下载附件，可能有多个附件
            // let mut btn = Frame::default()
            //     .with_label("Click")
            //     .below_of(&txt, 0)
            //     .with_size(100, 30);
            // btn.set_color(Color::from_rgb(246, 251, 255));
            //
            // btn.handle(move |tr, event| match event {
            //     Event::Released => {
            //         if app::event_clicks_num() == 0 {
            //             println!("btn1 is checked");
            //         }
            //         true
            //     }
            //     _ => false,
            // });

            txt.set_buffer(buf);
            txt.set_color(Color::from_rgb(246, 251, 255));
            win.set_color(Color::from_rgb(246, 251, 255));

            txt.wrap_mode(text::WrapMode::AtBounds, 0);
            win.make_resizable(true);
            win.end();
            win.show();
            let mut vector_win_tmp = vec![];
            if !images.is_empty() {
                for (i_size, imgs) in images.iter().enumerate() {
                    let bit_imges = reqwest::blocking::get(imgs)
                        .unwrap()
                        .bytes()
                        .unwrap()
                        .to_vec();
                    let images_exist;
                    let title_tmp = format!("{}-img[{}]", title, i_size + 1);
                    let mut win_tmp = window::Window::default()
                        .with_size(
                            (width as f32 * 0.618) as i32,
                            (height as f32 * 0.618) as i32,
                        )
                        .with_label(title_tmp.as_str());

                    let mut frame = Frame::default().with_pos(0, 0);
                    frame.set_frame(FrameType::EngravedBox);

                    win_tmp.make_resizable(true);

                    if imgs.contains("png") {
                        let mut image = PngImage::from_data(bit_imges.as_slice()).unwrap();

                        let scala = image.width() as f32 / image.height() as f32;
                        let w: i32;
                        let h: i32;
                        if win_tmp.width() < win_tmp.height() {
                            h = win_tmp.height();
                            w = (h as f32 * scala) as i32;
                        } else {
                            w = win_tmp.width();
                            h = (w as f32 / scala) as i32;
                        }
                        // println!("{w},{h}");
                        frame.set_size(w, h);
                        win_tmp.set_size(w, h);
                        let win_tmp_tmp = win_tmp.clone();
                        frame.draw(move |f| {
                            f.set_size(win_tmp_tmp.width(), win_tmp_tmp.height());
                            image.scale(f.w(), f.h(), true, true);
                            image.draw(f.x(), f.y(), f.w(), f.h());
                        });

                        images_exist = true;
                    } else if imgs.contains("jpg") {
                        let mut image = JpegImage::from_data(bit_imges.as_slice()).unwrap();
                        let scala = image.width() as f32 / image.height() as f32;
                        let w;
                        let h;
                        if win_tmp.width() < win_tmp.height() {
                            h = win_tmp.height();
                            w = (h as f32 * scala) as i32;
                        } else {
                            w = win_tmp.width();
                            h = (w as f32 / scala) as i32;
                        }
                        // println!("{w},{h}");
                        frame.set_size(w, h);
                        win_tmp.set_size(w, h);
                        let win_tmp_tmp = win_tmp.clone();
                        frame.draw(move |f| {
                            f.set_size(win_tmp_tmp.width(), win_tmp_tmp.height());
                            image.scale(f.w(), f.h(), true, true);
                            image.draw(f.x(), f.y(), f.w(), f.h());
                        });

                        images_exist = true;
                    } else {
                        images_exist = false;
                    }
                    drop(bit_imges);
                    if images_exist {
                        win_tmp.end();
                        win_tmp.show();
                        //确保双击图片可以用浏览器打开网页
                        let uurl = url.clone();
                        win_tmp.handle(move |_, event| match event {
                            Event::Released => {
                                if app::event_clicks_num() == 1 {
                                    webbrowser::open(&uurl).unwrap()
                                }
                                true
                            }
                            _ => false,
                        });
                        vector_win_tmp.push(win_tmp.clone());
                    }
                }
            }

            // 确保关闭窗口可以让图片窗口跟着关闭
            let mut vector_win_tmp_tmp = vector_win_tmp.clone();
            win.set_callback(move |_win| {
                if app::event() == Event::Close {
                    // println!("Close Close!!!!!");
                    for w in &mut vector_win_tmp_tmp {
                        w.clear();
                        w.hide();
                    }
                    _win.clear();
                    _win.platform_hide();
                }
            });
            win.handle(move |_win, event| match event {
                Event::Hide => {
                    // println!("Hide Hide!!!!!");
                    for w in &mut vector_win_tmp {
                        w.hide();
                    }
                    true
                }
                Event::Show => {
                    // println!("Show Show!!!!!");
                    for w in &mut vector_win_tmp {
                        w.show();
                    }
                    true
                }

                _ => {
                    // println!("!!!!!{:#}",event);
                    false
                }
            });

            //确保双击文字可以用浏览器打开网页
            let uurl = url.clone();
            txt.handle(move |_, event| match event {
                Event::Released => {
                    if app::event_clicks_num() == 1 {
                        webbrowser::open(&uurl).unwrap()
                    }
                    true
                }
                _ => false,
            });
        }
        None => {}
    }
}

pub fn add_menu(
    wind: &mut DoubleWindow,
    menubar: &mut MenuBar,
    table: &mut SmartTable,
    sender_keywords: Sender<String>,
) {
    menubar.add_choice("搜索  |过滤  ");
    menubar.set_text_font(enums::Font::TimesBold);

    menubar.set_color(Color::from_rgb(246, 251, 255));
    let windx = wind.x_root();
    let windy = wind.y_root();
    let mut tt = table.clone();
    let mut last_keywords = String::from(""); //记住上次设置的关键字
    menubar.set_callback(move |c| {
        if let Some(choice) = c.choice() {
            match choice.as_str() {
                "过滤  " => {
                    dialog::message_title("OA Notifier 过滤");
                    match dialog::input(
                        c.x() + windx,
                        c.y() + windy,
                        "输入要过滤的内容，用空格隔开:",
                        last_keywords.as_str(),
                    ) {
                        Some(code) => {
                            // println!("{}", code);
                            last_keywords = code.clone(); //更新关键字
                            sender_keywords.send(code).unwrap();
                            // 发送刚才输入的关键字
                        }
                        None => {}
                    }
                }
                "搜索  " => {
                    for i in 0..tt.rows() {
                        tt.set_cell_value(i, 4, "");
                    }
                    dialog::message_title("OA Notifier 搜索");
                    match dialog::input(c.x() + windx, c.y() + windy, "输入要查找的内容:", "")
                    {
                        Some(code) => {
                            if !code.is_empty() {
                                // let code = String::from("吉林");
                                // println!("Finding...{}", code);
                                tt.set_selection(-1, -1, -1, -1);
                                for i in 0..tt.rows() {
                                    if tt.cell_value(i, 0).contains(&code)
                                        || tt.cell_value(i, 1).contains(&code)
                                        || tt.cell_value(i, 2).contains(&code)
                                    {
                                        // println!("Found == {}", i);
                                        tt.set_cell_value(i as i32, 4, "f");
                                        tt.redraw();
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn add_table(table: &mut SmartTable, wind: &mut DoubleWindow, vector: &mut RefCell<Vec<Item>>) {
    table.set_row_resize(true);
    table.set_col_resize(true);
    table.set_col_header(true);
    table.set_row_header(false);
    // table.set_color(Color::from_rgb(246, 251, 255));
    // table.set_selection_color(Color::from_rgb(246, 251, 255));
    table.scrollbar().set_color(Color::from_rgb(246, 251, 255));
    table.hscrollbar().set_color(Color::from_rgb(246, 251, 255));
    table
        .scrollbar()
        .set_selection_color(Color::from_hex(0xD8D8D8));
    table
        .hscrollbar()
        .set_selection_color(Color::from_hex(0xD8D8D8));

    table.set_col_width(0, (table.width() as f32 * 0.67) as i32);
    table.set_col_width(1, (table.width() as f32 * 0.18) as i32);
    table.set_col_width(
        2,
        table.width() - table.col_width(0) - table.col_width(1) - table.scrollbar().width() - 3,
    );
    table.set_col_width(3, 0);
    table.set_col_width(4, 0);

    table.set_col_header_height((table.height() as f32 * 0.038) as i32);

    wind.make_resizable(true);

    let titles = ["标题", "部门", "时间", "链接"];

    table.set_col_header_value(0, titles[0]);
    table.set_col_header_value(1, titles[1]);
    table.set_col_header_value(2, titles[2]);
    table.set_col_header_value(3, titles[3]);
    let mut tt = table.clone();
    table.handle(move |tr, event| match event {
        Event::Released => {
            if app::event_clicks_num() == 1 && app::event_mouse_button() == app::MouseButton::Left {
                let ress = tr.callback_row();
                // Command::new("cmd.exe").creation_flags(0x08000000).arg("/c").arg("start").arg(&tt.cell_value(ress, 3)).status().expect("Command");
                // webbrowser::open(&tt.cell_value(ress, 3)).unwrap();
                let str = format!("{}：{}", tt.cell_value(ress, 0), tt.cell_value(ress, 1));
                show_content(
                    &tt.cell_value(ress, 3),
                    &str,
                    (tr.width() as f32 * 0.618) as i32,
                    (tr.height() as f32 * 0.618) as i32,
                );
            }
            if app::event_clicks_num() == 0 {
                for i in 0..tt.rows() {
                    tt.set_cell_value(i, 4, "");
                }
                redraw();
                if app::event_mouse_button() == app::MouseButton::Right {
                    // println!("click right Mouse button");
                    let ress = tr.callback_row();
                    webbrowser::open(&tt.cell_value(ress, 3)).unwrap()
                }
            }

            true
        }
        _ => false,
    });

    for i in 0..vector.borrow().len() {
        if vector.borrow()[i as usize].is_top {
            table.set_cell_value(
                i as i32,
                0,
                &format!("[置顶]{}", &vector.borrow()[i as usize].title),
            );
        } else {
            table.set_cell_value(i as i32, 0, &vector.borrow()[i as usize].title);
        }
        table.set_cell_value(i as i32, 1, &vector.borrow()[i as usize].source);
        table.set_cell_value(i as i32, 2, &vector.borrow()[i as usize].time);
        table.set_cell_value(i as i32, 3, &vector.borrow()[i as usize].href);
    }
    let mut tt = table.clone();

    table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::ColHeader => {
            if col < 3 {
                draw_header(tt.col_header_value(col).as_str(), x, y, w, h);
            }
        } // Column titles
        table::TableContext::Cell => {
            draw_data(
                tt.cell_value(row, col).as_str(),
                x,
                y,
                w,
                h,
                t.is_selected(row, col),
                tt.cell_value(row, 0).contains("置顶"),
                tt.cell_value(row, 4).contains("f"),
            );
        }
        // Data in cells
        _ => (),
    });

    let mut tt = table.clone();
    wind.draw(move |_| {
        tt.set_col_width(0, (tt.width() as f32 * 0.67) as i32);
        tt.set_col_width(1, (tt.width() as f32 * 0.18) as i32);
        tt.set_col_width(
            2,
            tt.width() - tt.col_width(0) - tt.col_width(1) - tt.scrollbar().width() - 3,
        );
        tt.set_col_width(3, 0);
        tt.set_col_width(4, 0);
        // tt.set_col_header_height((tt.height() as f32 * 0.04) as i32);
    });
}

//
//
pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::from_rgb(246, 251, 255));
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Black);
    draw::set_font(enums::Font::TimesBold, 16);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::set_draw_color(Color::from_hex(0x3D5A80));
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
pub fn draw_data(
    txt: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    selected: bool,
    is_top: bool,
    is_found: bool,
) {
    draw::push_clip(x, y, w, h);
    if selected || is_found {
        draw::set_draw_color(Color::from_rgb(201, 227, 251));
    } else {
        draw::set_draw_color(Color::from_rgb(246, 251, 255));
    }
    draw::draw_rectf(x, y, w, h);

    if is_top {
        draw::set_font(enums::Font::TimesBold, 15);
    } else {
        draw::set_font(enums::Font::Times, 14);
    }
    if txt.contains("今天") {
        draw::set_draw_color(Color::from_rgb(0, 128, 0));
    } else {
        draw::set_draw_color(Color::Black);
    }
    // draw::set_draw_color()
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::set_draw_color(Color::from_hex(0x3D5A80));
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
