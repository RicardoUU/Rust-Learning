use clap::{App, AppSettings, Arg, SubCommand};

/// 解析命令行参数
pub fn build_cli() -> App<'static, 'static> {
    App::new("待办事项管理器")
        .version("1.0")
        .author("Rust学习者")
        .about("一个简单的命令行待办事项管理器")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add")
                .about("添加一个新的待办事项")
                .arg(
                    Arg::with_name("title")
                        .help("待办事项的标题")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("description")
                        .help("待办事项的详细描述")
                        .index(2),
                )
                .arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .help("待办事项的优先级 (low, medium, high)")
                        .takes_value(true)
                        .possible_values(&["low", "medium", "high"])
                )
                .arg(
                    Arg::with_name("due")
                        .short("d")
                        .long("due")
                        .help("待办事项的截止日期 (YYYY-MM-DD)")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tags")
                        .help("待办事项的标签，用逗号分隔")
                        .takes_value(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("列出所有待办事项")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("显示所有待办事项，包括已完成的")
                )
                .arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .help("按优先级筛选 (low, medium, high)")
                        .takes_value(true)
                        .possible_values(&["low", "medium", "high"])
                )
                .arg(
                    Arg::with_name("tag")
                        .short("t")
                        .long("tag")
                        .help("按标签筛选")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("due")
                        .short("d")
                        .long("due")
                        .help("显示有截止日期的待办事项")
                )
                .arg(
                    Arg::with_name("overdue")
                        .short("o")
                        .long("overdue")
                        .help("显示已过期的待办事项")
                )
                .arg(
                    Arg::with_name("sort")
                        .short("s")
                        .long("sort")
                        .help("排序方式 (priority, date, title)")
                        .takes_value(true)
                        .possible_values(&["priority", "date", "title"])
                ),
        )
        .subcommand(
            SubCommand::with_name("done")
                .about("将待办事项标记为已完成")
                .arg(
                    Arg::with_name("id")
                        .help("待办事项的ID")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("undone")
                .about("将待办事项标记为未完成")
                .arg(
                    Arg::with_name("id")
                        .help("待办事项的ID")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("删除一个待办事项")
                .arg(
                    Arg::with_name("id")
                        .help("待办事项的ID")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("编辑待办事项")
                .arg(
                    Arg::with_name("id")
                        .help("待办事项的ID")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("title")
                        .short("t")
                        .long("title")
                        .help("新的标题")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("description")
                        .short("d")
                        .long("description")
                        .help("新的描述")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("priority")
                        .short("p")
                        .long("priority")
                        .help("新的优先级 (low, medium, high)")
                        .takes_value(true)
                        .possible_values(&["low", "medium", "high"])
                )
                .arg(
                    Arg::with_name("due")
                        .long("due")
                        .help("新的截止日期 (YYYY-MM-DD)")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("add-tag")
                        .long("add-tag")
                        .help("添加标签")
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("remove-tag")
                        .long("remove-tag")
                        .help("移除标签")
                        .takes_value(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("导出待办事项")
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .help("导出格式 (json, csv, html)")
                        .takes_value(true)
                        .possible_values(&["json", "csv", "html"])
                        .default_value("json")
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .help("输出文件路径")
                        .takes_value(true)
                        .required(true)
                ),
        )
}