mod cli;
mod storage;
mod todo;

use std::env;
use std::path::PathBuf;
use std::process;
use chrono::TimeZone;

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
            
            // 解析优先级
            let priority = match sub_matches.value_of("priority") {
                Some("low") => todo::Priority::Low,
                Some("medium") => todo::Priority::Medium,
                Some("high") => todo::Priority::High,
                _ => todo::Priority::default(),
            };
            
            // 解析截止日期
            let due_date = if let Some(date_str) = sub_matches.value_of("due") {
                match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                    Ok(date) => {
                        let datetime = date.and_hms_opt(23, 59, 59).unwrap();
                        Some(chrono::Local::now().timezone().from_local_datetime(&datetime).unwrap())
                    },
                    Err(_) => {
                        eprintln!("无效的日期格式，应为YYYY-MM-DD");
                        process::exit(1);
                    }
                }
            } else {
                None
            };
            
            // 解析标签
            let tags = if let Some(tags_str) = sub_matches.value_of("tags") {
                tags_str.split(',').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };
            
            // 创建待办事项
            let mut todo = Todo::new(title, description);
            todo.set_priority(priority);
            if let Some(due) = due_date {
                todo.set_due_date(Some(due));
            }
            for tag in tags {
                todo.add_tag(tag);
            }
            
            storage.add(todo);
            
            if let Err(e) = storage.save() {
                eprintln!("保存待办事项时出错: {}", e);
                process::exit(1);
            }
            
            println!("待办事项已添加。");
        },
        ("list", Some(sub_matches)) => {
            let mut todos: Vec<&Todo> = storage.todos().iter().collect();
            
            // 筛选未完成的待办事项（除非指定--all参数）
            if !sub_matches.is_present("all") {
                todos.retain(|todo| !todo.is_completed());
            }
            
            // 按优先级筛选
            if let Some(priority_str) = sub_matches.value_of("priority") {
                let priority = match priority_str {
                    "low" => todo::Priority::Low,
                    "medium" => todo::Priority::Medium,
                    "high" => todo::Priority::High,
                    _ => todo::Priority::Medium,
                };
                todos.retain(|todo| todo.priority() == priority);
            }
            
            // 按标签筛选
            if let Some(tag) = sub_matches.value_of("tag") {
                todos.retain(|todo| todo.tags().iter().any(|t| t == tag));
            }
            
            // 筛选有截止日期的待办事项
            if sub_matches.is_present("due") {
                todos.retain(|todo| todo.due_date().is_some());
            }
            
            // 筛选已过期的待办事项
            if sub_matches.is_present("overdue") {
                let now = chrono::Local::now();
                todos.retain(|todo| {
                    if let Some(due) = todo.due_date() {
                        due < &now && !todo.is_completed()
                    } else {
                        false
                    }
                });
            }
            
            // 排序
            if let Some(sort_by) = sub_matches.value_of("sort") {
                match sort_by {
                    "priority" => {
                        todos.sort_by(|a, b| b.priority().cmp(&a.priority()));
                    },
                    "date" => {
                        todos.sort_by(|a, b| {
                            match (a.due_date(), b.due_date()) {
                                (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                                (Some(_), None) => std::cmp::Ordering::Less,
                                (None, Some(_)) => std::cmp::Ordering::Greater,
                                (None, None) => a.created_at().cmp(b.created_at()),
                            }
                        });
                    },
                    "title" => {
                        todos.sort_by(|a, b| a.title().cmp(b.title()));
                    },
                    _ => {}
                }
            }
            
            if todos.is_empty() {
                println!("没有符合条件的待办事项。");
                return;
            }
            
            println!("待办事项列表：");
            for (i, todo) in todos.iter().enumerate() {
                let status = if todo.is_completed() { "[✓]" } else { "[ ]" };
                let priority_str = match todo.priority() {
                    todo::Priority::Low => "[低]  ",
                    todo::Priority::Medium => "[中]  ",
                    todo::Priority::High => "[高]  ",
                };
                println!("{} {} {}. {}", status, priority_str, i, todo.title());
                
                if let Some(desc) = todo.description() {
                    println!("   描述: {}", desc);
                }
                
                println!("   创建时间: {}", todo.created_at().format("%Y-%m-%d %H:%M:%S"));
                
                if let Some(due) = todo.due_date() {
                    let now = chrono::Local::now();
                    let status = if due < &now && !todo.is_completed() {
                        "已过期"
                    } else {
                        "未过期"
                    };
                    println!("   截止日期: {} ({})", due.format("%Y-%m-%d %H:%M:%S"), status);
                }
                
                if !todo.tags().is_empty() {
                    println!("   标签: {}", todo.tags().join(", "));
                }
                
                println!("");
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
        ("undone", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").unwrap().parse::<usize>().unwrap_or_else(|_| {
                eprintln!("无效的ID");
                process::exit(1);
            });
            
            match storage.mark_undone(id) {
                Ok(_) => {
                    if let Err(e) = storage.save() {
                        eprintln!("保存待办事项时出错: {}", e);
                        process::exit(1);
                    }
                    println!("待办事项已标记为未完成。");
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
        ("edit", Some(sub_matches)) => {
            let id = sub_matches.value_of("id").unwrap().parse::<usize>().unwrap_or_else(|_| {
                eprintln!("无效的ID");
                process::exit(1);
            });
            
            // 获取待办事项的可变引用
            let todo = match storage.get_todo_mut(id) {
                Ok(todo) => todo,
                Err(e) => {
                    eprintln!("获取待办事项时出错: {}", e);
                    process::exit(1);
                }
            };
            
            // 更新标题
            if let Some(title) = sub_matches.value_of("title") {
                todo.set_title(title.to_string());
            }
            
            // 更新描述
            if let Some(desc) = sub_matches.value_of("description") {
                todo.set_description(Some(desc.to_string()));
            }
            
            // 更新优先级
            if let Some(priority_str) = sub_matches.value_of("priority") {
                let priority = match priority_str {
                    "low" => todo::Priority::Low,
                    "medium" => todo::Priority::Medium,
                    "high" => todo::Priority::High,
                    _ => todo::Priority::Medium,
                };
                todo.set_priority(priority);
            }
            
            // 更新截止日期
            if let Some(date_str) = sub_matches.value_of("due") {
                if date_str.to_lowercase() == "none" {
                    todo.set_due_date(None);
                } else {
                    match chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                        Ok(date) => {
                            let datetime = date.and_hms_opt(23, 59, 59).unwrap();
                            todo.set_due_date(Some(chrono::Local::now().timezone().from_local_datetime(&datetime).unwrap()));
                        },
                        Err(_) => {
                            eprintln!("无效的日期格式，应为YYYY-MM-DD");
                            process::exit(1);
                        }
                    }
                }
            }
            
            // 添加标签
            if let Some(tag) = sub_matches.value_of("add-tag") {
                todo.add_tag(tag.to_string());
            }
            
            // 移除标签
            if let Some(tag) = sub_matches.value_of("remove-tag") {
                todo.remove_tag(tag);
            }
            
            // 保存更改
            if let Err(e) = storage.save() {
                eprintln!("保存待办事项时出错: {}", e);
                process::exit(1);
            }
            
            println!("待办事项已更新。");
        },
        ("export", Some(sub_matches)) => {
            let format = sub_matches.value_of("format").unwrap_or("json");
            let output_path = sub_matches.value_of("output").unwrap();
            
            let todos = storage.todos();
            if todos.is_empty() {
                println!("没有待办事项可导出。");
                return;
            }
            
            match format {
                "json" => {
                    // 导出为JSON格式
                    let json = serde_json::to_string_pretty(&todos).unwrap_or_else(|e| {
                        eprintln!("序列化待办事项时出错: {}", e);
                        process::exit(1);
                    });
                    
                    std::fs::write(output_path, json).unwrap_or_else(|e| {
                        eprintln!("写入文件时出错: {}", e);
                        process::exit(1);
                    });
                },
                "csv" => {
                    // 导出为CSV格式
                    let mut csv_content = String::from("ID,标题,描述,优先级,状态,创建时间,截止日期,标签\n");
                    
                    for (i, todo) in todos.iter().enumerate() {
                        let status = if todo.is_completed() { "已完成" } else { "未完成" };
                        let priority = match todo.priority() {
                            todo::Priority::Low => "低",
                            todo::Priority::Medium => "中",
                            todo::Priority::High => "高",
                        };
                        let description = todo.description().map_or("", |s| s);
                        let due_date = todo.due_date().map_or("".to_string(), |d| d.format("%Y-%m-%d").to_string());
                        let tags = todo.tags().join(",");
                        
                        csv_content.push_str(&format!("{},\"{}\",\"{}\",{},{},{},{},\"{}\"\n", 
                            i, 
                            todo.title().replace("\"", "\\\""), 
                            description.replace("\"", "\\\""), 
                            priority, 
                            status,
                            todo.created_at().format("%Y-%m-%d"),
                            due_date,
                            tags.replace("\"", "\\\"")
                        ));
                    }
                    
                    std::fs::write(output_path, csv_content).unwrap_or_else(|e| {
                        eprintln!("写入文件时出错: {}", e);
                        process::exit(1);
                    });
                },
                "html" => {
                    // 导出为HTML格式
                    let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>待办事项列表</title>\n<style>\n");
                    html.push_str("body { font-family: Arial, sans-serif; margin: 20px; }\n");
                    html.push_str("h1 { color: #333; }\n");
                    html.push_str("table { border-collapse: collapse; width: 100%; }\n");
                    html.push_str("th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }\n");
                    html.push_str("th { background-color: #f2f2f2; }\n");
                    html.push_str(".high { color: red; }\n");
                    html.push_str(".medium { color: orange; }\n");
                    html.push_str(".low { color: green; }\n");
                    html.push_str(".completed { text-decoration: line-through; }\n");
                    html.push_str(".tag { display: inline-block; background-color: #eee; padding: 2px 5px; margin: 2px; border-radius: 3px; }\n");
                    html.push_str("</style>\n</head>\n<body>\n");
                    html.push_str("<h1>待办事项列表</h1>\n");
                    html.push_str("<table>\n");
                    html.push_str("<tr><th>ID</th><th>标题</th><th>描述</th><th>优先级</th><th>状态</th><th>创建时间</th><th>截止日期</th><th>标签</th></tr>\n");
                    
                    for (i, todo) in todos.iter().enumerate() {
                        let status = if todo.is_completed() { "已完成" } else { "未完成" };
                        let priority_class = match todo.priority() {
                            todo::Priority::Low => "low",
                            todo::Priority::Medium => "medium",
                            todo::Priority::High => "high",
                        };
                        let priority_text = match todo.priority() {
                            todo::Priority::Low => "低",
                            todo::Priority::Medium => "中",
                            todo::Priority::High => "高",
                        };
                        let title_class = if todo.is_completed() { "completed" } else { "" };
                        let description = todo.description().map_or("", |s| s);
                        let due_date = todo.due_date().map_or("".to_string(), |d| d.format("%Y-%m-%d").to_string());
                        
                        let tags_html = if todo.tags().is_empty() {
                            "".to_string()
                        } else {
                            todo.tags().iter()
                                .map(|tag| format!("<span class=\"tag\">{}</span>", tag))
                                .collect::<Vec<_>>()
                                .join(" ")
                        };
                        
                        html.push_str(&format!("<tr>\n"));
                        html.push_str(&format!("<td>{}</td>\n", i));
                        html.push_str(&format!("<td class=\"{}\">{}</td>\n", title_class, todo.title()));
                        html.push_str(&format!("<td>{}</td>\n", description));
                        html.push_str(&format!("<td class=\"{}\">{}</td>\n", priority_class, priority_text));
                        html.push_str(&format!("<td>{}</td>\n", status));
                        html.push_str(&format!("<td>{}</td>\n", todo.created_at().format("%Y-%m-%d %H:%M:%S")));
                        html.push_str(&format!("<td>{}</td>\n", due_date));
                        html.push_str(&format!("<td>{}</td>\n", tags_html));
                        html.push_str(&format!("</tr>\n"));
                    }
                    
                    html.push_str("</table>\n");
                    html.push_str("</body>\n</html>");
                    
                    std::fs::write(output_path, html).unwrap_or_else(|e| {
                        eprintln!("写入文件时出错: {}", e);
                        process::exit(1);
                    });
                },
                _ => {
                    eprintln!("不支持的导出格式: {}", format);
                    process::exit(1);
                }
            }
            
            println!("待办事项已导出到 {}", output_path);
        },
        ("html", Some(sub_matches)) => {
            // HTML导出逻辑已在上面实现
        },
        _ => unreachable!()
    }
}