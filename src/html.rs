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
            let s = String::from('\u{2002}');
            doc = doc.replace(&s, " ");

            let sub_document = scraper::Html::parse_document(&doc);
            let mut imges = vec![];

            for e in sub_document.select(&img_selector) {
                let img = format!("{}{}", pre, e.value().attr("src").unwrap());
                println!("{}", img);
                imges.push(img);
            }
            let mut strings = vec![];
            for e in sub_document.tree {
                if e.is_text() {
                    let text = e.as_text().unwrap().text.to_string();
                    // let ele = e.as_element().unwrap();
                    // println!("{:#}:{}",e.as_element().unwrap(), text);

                    // text = text.replace("\n", "");
                    // text = text.replace("\t", "");
                    // if !text.is_empty() {
                    // println!("{:#}", text);
                    strings.push(text);
                    // }
                }
            }

            return Some((strings, imges));
        }
        Err(_) => {}
    };
    return None;
}

fn get_info(url: &str, vec: &mut Box<Vec<Item>>) {
    // let pre = String::from("https://oa.jlu.edu.cn/defaultroot/");
    let pre = "https://oa.jlu.edu.cn/defaultroot/";
    match reqwest::blocking::get(url) {
        Ok(webpage) => {
            let response = webpage.text().unwrap();
            if response.is_empty() {
                return;
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
                let mut title = first_element.inner_html().replace("&nbsp;&nbsp;", "");
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
                    .replace("&nbsp;&nbsp;", "");
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
        }
        Err(_) => {}
    }
}

pub fn get_html(vector: &mut RefCell<Vec<Item>>) {
    let url1 = "https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?1=1&channelId=179577&startPage=1";
    let url2 = "https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?1=1&channelId=179577&startPage=2";
    let url3 = "https://oa.jlu.edu.cn/defaultroot/PortalInformation!jldxList.action?1=1&channelId=179577&startPage=3";

    let mut vec_1: Box<Vec<Item>> = Box::new(vec![]);
    let mut vec_2: Box<Vec<Item>> = Box::new(vec![]);
    let mut vec_3: Box<Vec<Item>> = Box::new(vec![]);
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let (tx3, rx3) = mpsc::channel();

    let t1 = thread::spawn(move || {
        get_info(url1, &mut vec_1);
        tx1.send(vec_1).unwrap();
    });
    let t2 = thread::spawn(move || {
        get_info(url2, &mut vec_2);
        tx2.send(vec_2).unwrap();
    });
    let t3 = thread::spawn(move || {
        get_info(url3, &mut vec_3);
        tx3.send(vec_3).unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    vec_1 = rx1.recv().unwrap();
    vec_2 = rx2.recv().unwrap();
    vec_3 = rx3.recv().unwrap();

    if vec_1.len() != 30 && vec_2.len() != 30 && vec_3.len() != 30 {
        return;
    }

    vector.borrow_mut().clear();

    for e in vec_1.iter() {
        // println!("{:#?}", e);
        let ee = e.clone();
        vector.borrow_mut().push(ee);
    }
    for e in vec_2.iter() {
        // println!("{:#?}", e);
        let ee = e.clone();
        vector.borrow_mut().push(ee);
    }
    for e in vec_3.iter() {
        // println!("{:#?}", e);
        let ee = e.clone();
        vector.borrow_mut().push(ee);
    }
}
