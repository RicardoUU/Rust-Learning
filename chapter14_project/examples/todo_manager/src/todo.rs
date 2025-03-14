use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// 表示一个待办事项
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    /// 待办事项的标题
    title: String,
    /// 待办事项的详细描述
    description: Option<String>,
    /// 待办事项是否已完成
    completed: bool,
    /// 待办事项的创建时间
    created_at: DateTime<Local>,
}

impl Todo {
    /// 创建一个新的待办事项
    pub fn new(title: String, description: Option<String>) -> Self {
        Todo {
            title,
            description,
            completed: false,
            created_at: Local::now(),
        }
    }

    /// 获取待办事项的标题
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 获取待办事项的描述
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// 检查待办事项是否已完成
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// 获取待办事项的创建时间
    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    /// 将待办事项标记为已完成
    pub fn mark_as_done(&mut self) {
        self.completed = true;
    }

    /// 将待办事项标记为未完成
    pub fn mark_as_undone(&mut self) {
        self.completed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let todo = Todo::new("测试待办事项".to_string(), Some("这是一个测试".to_string()));
        assert_eq!(todo.title(), "测试待办事项");
        assert_eq!(todo.description(), Some(&"这是一个测试".to_string()));
        assert_eq!(todo.is_completed(), false);
    }

    #[test]
    fn test_mark_as_done() {
        let mut todo = Todo::new("测试待办事项".to_string(), None);
        assert_eq!(todo.is_completed(), false);
        
        todo.mark_as_done();
        assert_eq!(todo.is_completed(), true);
        
        todo.mark_as_undone();
        assert_eq!(todo.is_completed(), false);
    }
}