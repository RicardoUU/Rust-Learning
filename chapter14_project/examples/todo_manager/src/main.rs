mod cli;
mod storage;
mod todo;

use std::env;
use std::path::PathBuf;
use std::process;

use cli::build_cli;
use storage::{StorageError, TodoStorage};
use todo::Todo;

fn main() {
    // 解析命令行参数
    let matches = build_cli().get_matches();

    // 获取用户主目录，用于存储待办事项文件
    let home_dir = env::var("HOME").unwrap_or_else(|_| {
        eprintln!("无法获取用户主目录");
        process::exit(1);
    });

    // 创建待办事项存储文件路径
    let todo_file = PathBuf::from(home_dir).join(".todo.json");
    
    // 创建待办事项存储
    let mut storage = TodoStorage::new(todo_file);
    
    // 尝试从文件加载待办事项
    match storage.load() {
        Ok(_) => {},
        Err(StorageError::FileNotFound) => {
            println!("未找到待办事项文件，将创建新文件。");
        },
        Err(e) => {
            eprintln!("加载待办事项时出错: {}", e);
            process::exit(1);
        }
    }

    // 处理子命令
    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let title = sub_matches.value_of("title").unwrap().to_string();
            let description = sub_matches.value_of("description").map(|s| s.to_string());
            
            let todo = Todo::new(title, description);
            storage.add(todo);
            
            if let Err(e) = storage.save() {
                eprintln!("保存待办事项时出错: {}", e);
                process::exit(1);
            }
            
            println!("待办事项已添加。");
        },
        ("list", _) => {
            let todos = storage.todos();
            
            if todos.is_empty() {
                println!("没有待办事项。");
                return;
            }
            
            println!("待办事项列表：");
            for (i, todo) in todos.iter().enumerate() {
                let status = if todo.is_completed() { "[✓]" } else { "[ ]" };
                println!("{} {}. {}", status, i, todo.title());
                
                if let Some(desc) = todo.description() {
                    println!("   描述: {}", desc);
                }
                
                println!("   创建时间: {}", todo.created_at().format("%Y-%m-%d %H:%M:%S"));
            }
        },
        ("done", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").unwrap().parse::<usize>().unwrap_or_else(|_| {
                eprintln!("无效的ID");
                process::exit(1);
            });
            
            match storage.mark_done(id) {
                Ok(_) => {
                    if let Err(e) = storage.save() {
                        eprintln!("保存待办事项时出错: {}", e);
                        process::exit(1);
                    }
                    println!("待办事项已标记为完成。");
                },
                Err(e) => {
                    eprintln!("标记待办事项时出错: {}", e);
                    process::exit(1);
                }
            }
        },
        ("remove", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").unwrap().parse::<usize>().unwrap_or_else(|_| {
                eprintln!("无效的ID");
                process::exit(1);
            });
            
            match storage.remove(id) {
                Ok(_) => {
                    if let Err(e) = storage.save() {
                        eprintln!("保存待办事项时出错: {}", e);
                        process::exit(1);
                    }
                    println!("待办事项已删除。");
                },
                Err(e) => {
                    eprintln!("删除待办事项时出错: {}", e);
                    process::exit(1);
                }
            }
        },
        _ => unreachable!()
    }
}