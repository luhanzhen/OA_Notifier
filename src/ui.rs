use std::cell::RefCell;
use std::sync::mpsc::Sender;
// use std::os::windows::process::CommandExt;
// use std::process::Command;
use crate::html::get_content;
use crate::item::Item;
use fltk::app::redraw;
use fltk::enums::{Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::image::{JpegImage, PngImage};
use fltk::menu::MenuBar;
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_table::SmartTable;
use webbrowser;

/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/21
 * @time: 13:34
 * @this_file_name:ui
 */

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
                .with_size(win.width(), win.height() - 10)
                .with_pos(0, 10);

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
    vector: &RefCell<Vec<Item>>,
    sender_keywords: Sender<String>,
) {
    menubar.add_choice("关于  |搜索  |过滤  ");
    let windx = menubar.width() + wind.x_root();
    let windy = menubar.height() + wind.y_root();
    let vv = RefCell::clone(vector);
    let mut tt = table.clone();
    let mut last_keywords = String::from(""); //记住上次设置的关键字
    menubar.set_callback(move |c| {
        if let Some(choice) = c.choice() {
            match choice.as_str() {
                "过滤  " => {
                    dialog::message_title("OA Notifier 过滤");
                    match dialog::input_default("输入要过滤的内容，用空格隔开:", last_keywords.as_str()) {
                        Some(code) => {
                            // println!("{}", code);
                            last_keywords = code.clone(); //更新关键字
                            sender_keywords.send(code).unwrap();


                            // 发送刚才输入的关键字
                        }
                        None => {}
                    }
                }
                "关于  " => {
                    dialog::message_title("OA Notifier 关于");
                    dialog::message(windx, windy, "使用本软件即同意以下内容:
                                    本软件当前版本号为1.3.0。
                                    本软件用于自动提醒吉大OA更新内容。
                                    双击表格点开信息，右击表格打开网页。
                                    支持搜索，但是搜索不能查找内容，原因是考虑到搜索的快速响应。
                                    支持过滤关键信息，过滤可以是多个关键字，关键字之间必须用空格隔开。
                                    无关键字的情况下，会通知全部信息。
                                    内容页支持图片显示，附件下载目前尚未支持，双击内容页或者图片也可以打开网页。
                                    为了防止误触，只能点击菜单栏退出才能关闭程序。
                                    本软件每隔十分钟爬取oa网站前三页的内容。
                                    本软件承诺保护用户隐私，不收集任何信息。
                                    本软件著作权及其解释权归作者镇路晗所有。
                                    项目源代码及最新版在网站[https://github.com/luhanzhen/OA_Notifier]上。
                                    有好的建议或者其它需求建议可以给我留言。
                                    个人用户享有使用权，作者不对使用者造成的后果负责。
                                    本软件仅供个人使用，不可用于商业盈利目的或者非法目的。
                                    请主动遵守国家法律法规和学校的有关规定，非法或者违规行为造成的法律责任和后果自负。");
                }
                "搜索  " => {
                    for i in 0..tt.rows()
                    {
                        tt.set_cell_value(i, 4, "");
                    }
                    dialog::message_title("OA Notifier 搜索");
                    match dialog::input_default("输入要查找的内容:", "") {
                        Some(code) => {
                            if !code.is_empty() {
                                // let code = String::from("吉林");
                                // println!("Finding...{}", code);
                                tt.set_selection(-1, -1, -1, -1);
                                for i in 0..vv.borrow().len()
                                {
                                    let t = &vv.borrow()[i];
                                    if t.title.contains(&code) || t.source.contains(&code) || t.time.contains(&code)
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

    table.set_col_width(0, (table.width() as f32 * 0.67) as i32);
    table.set_col_width(1, (table.width() as f32 * 0.18) as i32);
    table.set_col_width(2, table.width() - table.col_width(0) - table.col_width(1));
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
        tt.set_col_width(2, tt.width() - tt.col_width(0) - tt.col_width(1));
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
    draw::set_draw_color(Color::Black);
    draw::set_font(enums::Font::TimesBold, 16);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
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
    draw::set_draw_color(Color::Gray0);
    if is_top {
        draw::set_font(enums::Font::TimesBold, 15);
    } else {
        draw::set_font(enums::Font::Times, 14);
    }
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
