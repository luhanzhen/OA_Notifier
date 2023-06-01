/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/21
 * @time: 15:54
 * @this_file_name:Item
 */

#[derive(Debug, Clone)]
pub struct Item {
    pub title: String,
    pub time: String,
    pub source: String,
    pub href: String,
    pub is_top: bool,
}

// pub static  mut VECTOR: Vec<Item> = vec![];

// impl Initialize for Item {
//     fn initialize(&mut self, title: String, time: String, source: String, href: String)
//     {
//         self.title = title;
//         self.time = time;
//         self.source = source;
//         self.href = href;
//     }
// }

// impl Clone for Item {
//     fn clone(&mut self) -> Self {
//         self.title = title.clone();
//         self.time = time.clone();
//         self.source = source.clone();
//         self.href = href.clone();
//     }
// }
