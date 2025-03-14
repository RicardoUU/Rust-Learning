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
                ),
        )
        .subcommand(SubCommand::with_name("list").about("列出所有待办事项"))
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
            SubCommand::with_name("remove")
                .about("删除一个待办事项")
                .arg(
                    Arg::with_name("id")
                        .help("待办事项的ID")
                        .required(true)
                        .index(1),
                ),
        )
}