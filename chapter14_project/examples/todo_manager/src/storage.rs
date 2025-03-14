use crate::todo::Todo;
use serde_json;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("找不到文件")]
    FileNotFound,
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    #[error("序列化错误: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("无效的ID: {0}")]
    InvalidId(usize),
}

pub struct TodoStorage {
    todos: Vec<Todo>,
    file_path: PathBuf,
}

impl TodoStorage {
    /// 创建一个新的待办事项存储
    pub fn new<P: AsRef<Path>>(file_path: P) -> Self {
        TodoStorage {
            todos: Vec::new(),
            file_path: file_path.as_ref().to_path_buf(),
        }
    }

    /// 从文件加载待办事项
    pub fn load(&mut self) -> Result<(), StorageError> {
        let file = File::open(&self.file_path);
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                self.todos = serde_json::from_str(&contents)?;
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(StorageError::FileNotFound)
            }
            Err(e) => Err(StorageError::IoError(e)),
        }
    }

    /// 保存待办事项到文件
    pub fn save(&self) -> Result<(), StorageError> {
        let contents = serde_json::to_string_pretty(&self.todos)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// 添加一个新的待办事项
    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    /// 获取所有待办事项
    pub fn todos(&self) -> &[Todo] {
        &self.todos
    }

    /// 将待办事项标记为已完成
    pub fn mark_done(&mut self, id: usize) -> Result<(), StorageError> {
        if id >= self.todos.len() {
            return Err(StorageError::InvalidId(id));
        }
        self.todos[id].mark_as_done();
        Ok(())
    }

    /// 删除一个待办事项
    pub fn remove(&mut self, id: usize) -> Result<(), StorageError> {
        if id >= self.todos.len() {
            return Err(StorageError::InvalidId(id));
        }
        self.todos.remove(id);
        Ok(())
    }
}