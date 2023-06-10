use crate::item::Item;
use std::cell::RefCell;
use std::sync::mpsc;
use std::thread;

/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/21
 * @time: 13:40
 * @this_file_name:html
 */

pub fn get_content(url: &str) -> Option<(Vec<String>, Vec<String>)> {
    match reqwest::blocking::get(url) {
        Ok(webpage) => {
            let response = webpage.text().unwrap();
            if response.is_empty() {
                return None;
            }
            let pre = "https://oa.jlu.edu.cn/";

            let document = scraper::Html::parse_document(&response);

            let content_selector1 =
                scraper::Selector::parse(r#"div[class="content_font fontsize immmge"]"#).unwrap();
            let content_selector2 =
                scraper::Selector::parse(r#"div[class="content_font"]"#).unwrap();

            let img_selector = scraper::Selector::parse("img").unwrap();

            let element = match document.select(&content_selector1).next() {
                Some(ele) => ele,
                None => document.select(&content_selector2).next().unwrap(),
            };
            // let element = document.select(&content_selector2).next().unwrap();

            let mut doc = element.inner_html();
            // println!("{}", doc);
            doc = doc.replace("<br>", "\n");
            doc = doc.replace("</p>", "</p>\n");
            // doc = doc.replace("<span", "  <span");
            let s = String::from('\u{2002}');
            doc = doc.replace(&s, "  ");

            let sub_document = scraper::Html::parse_document(&doc);
            let mut imges = vec![];

            for e in sub_document.select(&img_selector) {
                let img = format!("{}{}", pre, e.value().attr("src").unwrap());
                // println!("{}", img);
                imges.push(img);
            }
            let mut strings = vec![];
            for e in sub_document.tree {
                if e.is_text() {
                    let text = e.as_text().unwrap().text.to_string();
                    // text = format!("  {}", text);
                    strings.push(text);
                }
            }

            return Some((strings, imges));
        }
        Err(_) => {}
    };
    return None;
}

fn get_title_page(url: String) -> Option<Box<Vec<Item>>> {
    // let pre = String::from("https://oa.jlu.edu.cn/defaultroot/");
    let pre = "https://oa.jlu.edu.cn/defaultroot/";
    let mut vec: Box<Vec<Item>> = Box::new(Vec::new());
    match reqwest::blocking::get(url) {
        Ok(webpage) => {
            let response = webpage.text().unwrap();
            if response.is_empty() {
                return None;
            }
            // let response= fs::read_to_string(".\\test.html").unwrap();
            let document = scraper::Html::parse_document(&response);
            let title_selector = scraper::Selector::parse(r#"DIV[class="li rel"]"#).unwrap();
            let a_font14_selector = scraper::Selector::parse("a.font14").unwrap();
            let a_column_selector = scraper::Selector::parse("a.column").unwrap();
            let span_time_selector = scraper::Selector::parse("span.time").unwrap();

            for element in document.select(&title_selector) {
                // element.select(&a_font14_selector);
                let first_element = element.select(&a_font14_selector).next().unwrap();
                let mut title = first_element.inner_html().replace("&nbsp;", "");
                let mut is_top = false;
                if title.contains("red") {
                    is_top = true;
                }
                title = title.replace("<font class=\"red\">[置顶]</font>", "");

                let href = format!("{}{}", pre, first_element.value().attr("href").unwrap());
                let source = element
                    .select(&a_column_selector)
                    .next()
                    .unwrap()
                    .inner_html();
                let time = element
                    .select(&span_time_selector)
                    .next()
                    .unwrap()
                    .inner_html()
                    .replace("&nbsp;", "");
                // println!("{}: {}:{}:{}:{}", is_top,title, href, source,time);
                let a = Item {
                    title,
                    time,
                    source,
                    href,
                    is_top,
                };
                vec.push(a);
            }
            return Some(vec);
        }
        Err(_) => {
            return None;
        }
    }
}

pub fn get_table(vector: &mut RefCell<Vec<Item>>) -> Option<&mut RefCell<Vec<Item>>> {
    const PAGES: i32 = 10;
    let mut url = vec![];
    for i in 0..PAGES {
        let u = format!("https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?1=1&channelId=179577&startPage={}", i + 1);
        url.push(u);
    }
    let mut tx = vec![];
    let mut rx = vec![];
    for _ in 0..url.len() {
        let (tx1, rx1) = mpsc::channel();
        tx.push(tx1);
        rx.push(rx1);
    }

    let mut thread = vec![];
    for i in 0..url.len() {
        let tt = tx[i].clone();
        let uu = url[i].clone();
        let t = thread::spawn(move || match get_title_page(uu) {
            Some(vec) => {
                tt.send(vec).unwrap();
            }
            None => {
                return;
            }
        });
        thread.push(t);
    }
    for th in thread {
        th.join().unwrap();
    }

    vector.borrow_mut().clear();
    for r in rx.iter() {
        let v = r.recv().unwrap();
        if v.len() != 30 {
            return None;
        }
        for e in v.iter() {
            let ee = e.clone();
            vector.borrow_mut().push(ee);
        }
        drop(v);
    }
    Some(vector)
}
