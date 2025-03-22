use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// 待办事项的优先级
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "低"),
            Priority::Medium => write!(f, "中"),
            Priority::High => write!(f, "高"),
        }
    }
}

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
    /// 待办事项的优先级
    priority: Priority,
    /// 待办事项的截止日期
    due_date: Option<DateTime<Local>>,
    /// 待办事项的标签
    tags: Vec<String>,
}


impl Todo {
    /// 创建一个新的待办事项
    pub fn new(title: String, description: Option<String>) -> Self {
        Todo {
            title,
            description,
            completed: false,
            created_at: Local::now(),
            priority: Priority::default(),
            due_date: None,
            tags: Vec::new(),
        }
    }
    
    /// 创建一个带有优先级的新待办事项
    pub fn with_priority(title: String, description: Option<String>, priority: Priority) -> Self {
        let mut todo = Self::new(title, description);
        todo.priority = priority;
        todo
    }
    
    /// 创建一个带有截止日期的新待办事项
    pub fn with_due_date(title: String, description: Option<String>, due_date: DateTime<Local>) -> Self {
        let mut todo = Self::new(title, description);
        todo.due_date = Some(due_date);
        todo
    }
    
    /// 创建一个带有标签的新待办事项
    pub fn with_tags(title: String, description: Option<String>, tags: Vec<String>) -> Self {
        let mut todo = Self::new(title, description);
        todo.tags = tags;
        todo
    }

    /// 获取待办事项的标题
    pub fn title(&self) -> &str {
        &self.title
    }
    
    /// 设置待办事项的标题
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// 获取待办事项的描述
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    
    /// 设置待办事项的描述
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    /// 检查待办事项是否已完成
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// 获取待办事项的创建时间
    pub fn created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }
    
    /// 获取待办事项的优先级
    pub fn priority(&self) -> Priority {
        self.priority
    }
    
    /// 设置待办事项的优先级
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
    
    /// 获取待办事项的截止日期
    pub fn due_date(&self) -> Option<&DateTime<Local>> {
        self.due_date.as_ref()
    }
    
    /// 设置待办事项的截止日期
    pub fn set_due_date(&mut self, due_date: Option<DateTime<Local>>) {
        self.due_date = due_date;
    }
    
    /// 获取待办事项的标签
    pub fn tags(&self) -> &[String] {
        &self.tags
    }
    
    /// 添加标签到待办事项
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
    
    /// 从待办事项中移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
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
        assert_eq!(todo.priority(), Priority::Medium);
        assert!(todo.due_date().is_none());
        assert!(todo.tags().is_empty());
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
    
    #[test]
    fn test_priority() {
        let mut todo = Todo::new("测试待办事项".to_string(), None);
        assert_eq!(todo.priority(), Priority::Medium);
        
        todo.set_priority(Priority::High);
        assert_eq!(todo.priority(), Priority::High);
    }
    
    #[test]
    fn test_tags() {
        let mut todo = Todo::new("测试待办事项".to_string(), None);
        assert!(todo.tags().is_empty());
        
        todo.add_tag("工作".to_string());
        todo.add_tag("重要".to_string());
        assert_eq!(todo.tags().len(), 2);
        assert!(todo.tags().contains(&"工作".to_string()));
        
        todo.remove_tag("工作");
        assert_eq!(todo.tags().len(), 1);
        assert!(!todo.tags().contains(&"工作".to_string()));
    }
}