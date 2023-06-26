/**
 * @project_name: OANotifier
 * @user_name: luhanzhen
 * @date: 2023/5/21
 * @time: 15:54
 * @this_file_name:Item
 */

#[derive(Clone)]
pub struct Item {
    pub title: String,  //新消息的标题
    pub time: String,   //新消息的时间
    pub source: String, //新消息的来源
    pub href: String,   //链接
    pub is_top: bool,   //是否是置顶的内容
}

pub const VERSION: &'static str = "1.5.4";
