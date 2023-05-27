use std::cell::RefCell;
// use std::os::windows::process::CommandExt;
// use std::process::Command;
use webbrowser;
use fltk::{prelude::*, *};
use fltk::app::redraw;
use fltk::enums::{Event};
use fltk::menu::MenuBar;
use fltk::window::DoubleWindow;
use fltk_table::{SmartTable};
use crate::item::Item;

// use crate::item::VECTOR;
/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/21
 * @time: 13:34
 * @this_file_name:ui
 */

pub const INIT_WIDTH: i32 = 1200;
pub const INIT_HEIGHT: i32 = 600;

pub fn add_menu(wind: &mut DoubleWindow, menubar: &mut MenuBar, table: &mut SmartTable, vector: &RefCell<Vec<Item>>) {
    menubar.add_choice("关于  |查找  |退出  ");
    let windx = menubar.width() + wind.x_root();
    let windy = menubar.height() + wind.y_root();
    let vv = RefCell::clone(vector);
    let mut tt = table.clone();
    menubar.set_callback(move |c| {
        if let Some(choice) = c.choice() {
            match choice.as_str() {
                "关于  " => {
                    dialog::message_title("OA About");
                    dialog::message(windx, windy, "使用本软件即同意以下内容:
                                    本软件用于自动提醒吉大OA更新内容。
                                    双击点开信息，支持搜索。
                                    本软件当前版本号为1.1.0。
                                    本软件每隔十分钟爬取oa网站前三页的内容。
                                    本软件承诺保护用户隐私，不收集任何信息。
                                    本软件著作权及其解释权归作者镇路晗所有。
                                    个人用户享有使用权，作者不对使用者造成的后果负责。
                                    本软件仅供个人使用，不得随意传播，不可用于商业盈利目的或者非法目的。
                                    请主动遵守国家法律法规和学校的有关规定，非法或者违规行为造成的后果和法律责任自负。");
                }
                "查找  " => {
                    for i in 0..tt.rows()
                    {
                        tt.set_cell_value(i, 4, "");
                    }

                    let code = dialog::input_default("输入要查找的内容:", "").unwrap();
                    // let code = String::from("吉林");
                    println!("Finding...{}", code);
                    tt.set_selection(-1, -1, -1, -1);
                    for i in 0..vv.borrow().len()
                    {
                        let t = &vv.borrow()[i];
                        if t.title.contains(&code) || t.source.contains(&code) || t.time.contains(&code)
                        {
                            println!("Found == {}", i);
                            tt.set_cell_value(i as i32, 4, "f");
                            tt.redraw();
                        }
                    }
                }
                "退出  " => {
                    println!("Quitting");
                    app::quit();
                }
                _ => unreachable!(),
            }
        }
    });
}

pub fn add_table(table: &mut SmartTable, wind: &mut DoubleWindow, vector: &mut RefCell<Vec<Item>>)
{
    table.set_row_resize(true);
    table.set_col_resize(true);
    table.set_col_header(true);
    table.set_row_header(false);


    table.set_col_width(0, (table.width() as f32 * 0.67) as i32);
    table.set_col_width(1, (table.width() as f32 * 0.18) as i32);
    table.set_col_width(2, table.width() - table.col_width(0) - table.col_width(1));
    table.set_col_width(3, 0);
    table.set_col_width(4, 0);

    table.set_col_header_height((table.height() as f32 * 0.045) as i32);


    wind.make_resizable(true);

    let titles = ["标题", "部门", "时间", "链接"];

    table.set_col_header_value(0, titles[0]);
    table.set_col_header_value(1, titles[1]);
    table.set_col_header_value(2, titles[2]);
    table.set_col_header_value(3, titles[3]);
    let mut tt = table.clone();
    table.handle(move |tr, event| match event {
        Event::Released => {
            if app::event_clicks_num() == 1
            {
                let ress = tr.callback_row();
                // Command::new("cmd.exe").creation_flags(0x08000000).arg("/c").arg("start").arg(&tt.cell_value(ress, 3)).status().expect("Command");
                webbrowser::open(&tt.cell_value(ress, 3)).unwrap();
            }
            if app::event_clicks_num() == 0
            {
                for i in 0..tt.rows()
                {
                    tt.set_cell_value(i, 4, "");
                }
                redraw();
            }

            true
        }
        _ => false,
    });

    for i in 0..vector.borrow().len()
    {
        if vector.borrow()[i as usize].is_top {
            table.set_cell_value(i as i32, 0, &format!("[置顶]{}", &vector.borrow()[i as usize].title));
        } else {
            table.set_cell_value(i as i32, 0, &vector.borrow()[i as usize].title);
        }
        table.set_cell_value(i as i32, 1, &vector.borrow()[i as usize].source);
        table.set_cell_value(i as i32, 2, &vector.borrow()[i as usize].time);
        table.set_cell_value(i as i32, 3, &vector.borrow()[i as usize].href);
    }
    let mut tt = table.clone();

    table.draw_cell(move |t, ctx, row, col, x, y, w, h|
        match ctx {
            table::TableContext::ColHeader => {
                if col < 3
                {
                    draw_header(tt.col_header_value(col).as_str(), x, y, w, h);
                }
            }// Column titles
            table::TableContext::Cell => {
                draw_data(tt.cell_value(row, col).as_str(), x, y, w, h, t.is_selected(row, col), tt.cell_value(row, 0).contains("置顶"), tt.cell_value(row, 4).contains("f"));
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
    });
}

//
//
pub fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox, x, y, w, h, enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::TimesBold, 16);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}


// The selected flag sets the color of the cell to a grayish color, otherwise white
pub fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool, is_top: bool, is_found: bool) {
    draw::push_clip(x, y, w, h);
    if selected || is_found {
        draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
    } else {
        draw::set_draw_color(enums::Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    if is_top {
        draw::set_font(enums::Font::TimesBold, 15);
    } else {
        draw::set_font(enums::Font::Times, 14);
    }
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
