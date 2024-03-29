use crate::html::{get_content, get_table, get_update, is_reachable};
use crate::item::{Item, VERSION};
use fltk::app::redraw;
use fltk::enums::{Color, Event, FrameType};
use fltk::frame::Frame;
use fltk::image::{IcoImage, JpegImage, PngImage};
use fltk::menu::MenuBar;
use fltk::window::DoubleWindow;
use fltk::{prelude::*, *};
use fltk_table::SmartTable;
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::sync::mpsc::Sender;
use webbrowser;

/**
 * <p> @project_name: OANotifier </p>
 * <p> @user_name: luhanzhen </p>
 * <p>@date: 2023/5/21</p>
 * <p>@time: 13:34</p>
 * <p>@this_file_name:ui</p>
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
                        let mess = if third == usize::MAX || four == usize::MAX {
                            "有最新版，你要更新吗？".to_string()
                        } else {
                            let update_log = &new_version[(third + 9)..four];
                            format!("有最新版，你要更新吗？{}", update_log)
                        };
                        dialog::message_title("OA Notifier 更新");
                        if let Some(choice) =
                            dialog::choice2(x, y, mess.as_str(), "算了", "在浏览器中下载", "")
                        {
                            if choice == 1 {
                                webbrowser::open(str).unwrap();
                                dialog::message_title("OA Notifier 更新");
                                dialog::message(x, y, "下载最新版，删除现在的软件即可。");
                            }
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

pub fn get_dialog(init_width: i32, init_height: i32) -> DoubleWindow {
    let mut win = window::Window::default()
        .with_size(init_width, init_height)
        .with_label("OA Notifier [loading....]")
        .center_screen();
    let mut icon: Option<IcoImage> = None;
    if fs::metadata("./icon.ico").is_ok() {
        icon = Some(IcoImage::load(Path::new("icon.ico")).unwrap());
    }

    // let btn = button::Button::new(0, 0, 160, 40, "正在获取OA内容。。。");
    // btn.center_of(&win);
    // btn.set_color(Color::White);

    win.set_icon(icon);
    win.set_color(Color::White);
    win.end();
    // win.platform_show();
    win
}

fn show_content(url: &str, title: &String, width: i32, height: i32) {
    if let Some((content, images)) = get_content(url) {
        let mut buf = text::TextBuffer::default();
        for e in content.iter() {
            buf.append(e);
        }
        buf.line_start(0);

        let mut win = window::Window::default()
            .with_size(width, height)
            .with_pos(width / 12, height / 12)
            .with_label(title);
        win.set_color(Color::from_rgb(246, 251, 255));
        win.set_label_color(Color::from_rgb(0, 32, 96));
        win.set_label_font(enums::Font::HelveticaBold);
        win.set_label_size(16);
        win.make_resizable(true);
        let mut icon: Option<IcoImage> = None;
        if fs::metadata("./icon.ico").is_ok() {
            icon = Some(IcoImage::load(Path::new("icon.ico")).unwrap());
        }
        win.set_icon(icon.clone());
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
        txt.set_text_color(Color::from_rgb(0, 32, 96));
        txt.set_text_font(enums::Font::HelveticaBold);

        txt.set_text_size(15);
        txt.wrap_mode(text::WrapMode::AtBounds, 0);

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
                    win_tmp.set_icon(icon.clone());
                    win_tmp.end();
                    win_tmp.show();
                    //确保双击图片可以用浏览器打开网页
                    let uurl = url.to_owned();
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
        let uurl = url.to_owned();
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
}

fn forced_refresh(table: &mut SmartTable) {
    let mut now: RefCell<Vec<Item>> = RefCell::new(vec![]);

    if !is_reachable("oa.jlu.edu.cn:80") {
        return;
    }
    match get_table(&mut now, 20) {
        //看看前两页有没有更新
        Some(_) => {}
        None => {
            return;
        }
    }
    let _ = (0..now.borrow().len()).map(|i| {
        if now.borrow()[i].is_top {
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
    });
    table.redraw();
    drop(now);
    println!("success!!");
}

pub fn add_menu(
    wind: &mut DoubleWindow,
    menubar: &mut MenuBar,
    table: &mut SmartTable,
    sender_keywords: Sender<String>,
    width: i32,
    height: i32,
) {
    menubar.add_choice("OA主页  |搜索  |过滤  |刷新  ");
    menubar.set_text_font(enums::Font::TimesBold);

    // menubar.set_color(Color::from_rgb(246, 251, 255));
    menubar.set_color(Color::White);
    let windx = width / 2 + wind.height() / 10;
    let windy = height / 2 + wind.width() / 10;
    let mut tt = table.clone();
    let mut last_keywords = String::from(""); //记住上次设置的关键字
    menubar.set_callback(move |c| {
        if let Some(choice) = c.choice() {
            match choice.as_str() {
                "刷新  " => {
                    forced_refresh(&mut tt);
                }
                "OA主页  " => {
                    webbrowser::open("https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?channelId=179577").unwrap();
                }
                "过滤  " => {
                    dialog::message_title("OA Notifier 过滤");
                    if let Some(code) =  dialog::input(
                        windx,
                        windy,
                        "输入要过滤的内容，用空格隔开:",
                        last_keywords.as_str(),
                    ) {

                            // println!("{}", code);
                            last_keywords = code.clone(); //更新关键字
                            sender_keywords.send(code).unwrap();
                            // 发送刚才输入的关键字

                    }
                }
                "搜索  " => {
                    for i in 0..tt.rows() {
                        tt.set_cell_value(i, 4, "");
                    }
                    dialog::message_title("OA Notifier 搜索");
                    if let  Some(code) =  dialog::input(windx, windy, "输入要查找的内容:", "") {

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
                                        tt.set_cell_value(i, 4, "f");
                                        tt.redraw();
                                    }
                                }
                            }
                    }
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn add_table(
    table: &mut SmartTable,
    wind: &mut DoubleWindow,
    vector: &mut RefCell<Vec<Item>>,
    width: i32,
    height: i32,
) {
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

    table.set_col_width(0, (table.width() as f32 * 0.69) as i32);
    table.set_col_width(1, (table.width() as f32 * 0.22) as i32);
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
                let str = format!("{} : {}", tt.cell_value(ress, 0), tt.cell_value(ress, 1));
                show_content(
                    &tt.cell_value(ress, 3),
                    &str,
                    (width as f32 * 0.8) as i32,
                    (height as f32 * 0.8) as i32,
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
                    // https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?channelId=179577&orgname=%E4%BA%BA%E5%8A%9B%E8%B5%84%E6%BA%90%E5%A4%84
                    let coll = tr.callback_col();
                    if coll == 1
                    {
                        let url = format!("https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?channelId=179577&orgname={}", &tt.cell_value(ress, 1));
                        webbrowser::open(url.as_str()).unwrap()
                    } else {
                        webbrowser::open(&tt.cell_value(ress, 3)).unwrap()
                    }
                }
            }
            true
        }
        _ => false,
    });

    for i in 0..vector.borrow().len() {
        if vector.borrow()[i].is_top {
            table.set_cell_value(i as i32, 0, &format!("[置顶]{}", &vector.borrow()[i].title));
        } else {
            table.set_cell_value(i as i32, 0, &vector.borrow()[i].title);
        }
        table.set_cell_value(i as i32, 1, &vector.borrow()[i].source);
        table.set_cell_value(i as i32, 2, &vector.borrow()[i].time);
        table.set_cell_value(i as i32, 3, &vector.borrow()[i].href);
    }
    let mut tt = table.clone();

    table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::ColHeader => {
            if col < 3 {
                draw_header(tt.col_header_value(col).as_str(), x, y, w, h, col);
            }
        } // Column titles
        table::TableContext::Cell => {
            draw_data(
                tt.cell_value(row, col).as_str(),
                (x, y, w, h),
                row,
                col,
                t.is_selected(row, col),
                tt.cell_value(row, 0).contains("置顶"),
                tt.cell_value(row, 4).contains('f'),
            );
        }
        // Data in cells
        _ => (),
    });

    let mut tt = table.clone();
    wind.draw(move |_| {
        tt.set_col_width(0, (tt.width() as f32 * 0.69) as i32);
        tt.set_col_width(1, (tt.width() as f32 * 0.22) as i32);
        tt.set_col_width(
            2,
            tt.width() - tt.col_width(0) - tt.col_width(1) - tt.scrollbar().width() - 3,
        );
        tt.set_col_width(3, 0);
        tt.set_col_width(4, 0);
        // tt.set_col_header_height((tt.height() as f32 * 0.04) as i32);
    });
}

pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32, col: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::from_rgb(230, 240, 255));
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::from_rgb(0, 32, 96));
    draw::set_font(enums::Font::TimesBold, 16);
    if col != 0 {
        draw::draw_text2(txt, x, y, w, h, enums::Align::Left);
    } else {
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    }
    // draw::set_draw_color(Color::from_hex(0x3D5A80));
    // draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
pub fn draw_data(
    txt: &str,
    (x, y, w, h): (i32, i32, i32, i32),
    row: i32,
    col: i32,
    selected: bool,
    is_top: bool,
    is_found: bool,
) {
    if col > 2 {
        return;
    }
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_rgb(201, 227, 251));
    } else if row % 2 == 1 {
        draw::set_draw_color(Color::from_rgb(240, 248, 255));
    } else {
        draw::set_draw_color(Color::from_rgb(255, 255, 255));
    }

    draw::draw_rectf(x, y, w, h);

    if is_top && col == 0 {
        draw::set_font(enums::Font::TimesBold, 15);
    } else {
        draw::set_font(enums::Font::Times, 14);
    }
    if col == 2 {
        if txt.contains("今天") {
            draw::set_draw_color(Color::from_hex(0x2A9D8F));
        } else {
            draw::set_draw_color(Color::from_hex(0x6D6875));
        }
        draw::set_font(enums::Font::TimesBold, 14);
    } else if col == 1 {
        draw::set_draw_color(Color::from_rgb(39, 118, 197));
    } else {
        draw::set_draw_color(Color::from_rgb(0, 32, 96));
    }
    if is_found {
        draw::set_draw_color(Color::Red);
    }
    // draw::set_draw_color()
    if col != 0 {
        draw::draw_text2(txt, x, y, w, h, enums::Align::Left);
    } else {
        draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    }

    // draw::set_draw_color(Color::from_hex(0x3D5A80));
    // draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
