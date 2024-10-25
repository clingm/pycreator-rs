use pycreator::config::{write_config, LSPConfig};
use pycreator::venv::CondaVenv;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

static HELLO_WORLD: &str = r#"#此文件为自动创建

if __name__ == "__main__":
    print("Hello, world!")
"#;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pycreator <project_name> <venv_name>");
        std::process::exit(1);
    }

    // 第一个参数是程序名称，第二个是项目名称
    let project_name = &args[1];
    let venv_name = &args[2];
    println!("Creating project: {}", project_name);

    // 获取 conda 环境路径并检查环境
    let venv = CondaVenv::new(venv_name);
    if let Err(e) = venv.check_env() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    println!("Environment check passed.");

    // 配置LSP服务器
    let mut lsp_config = LSPConfig::new();
    println!("LSP server: {}", lsp_config.lsp_server());
    println!("Config file name: {}", lsp_config.config_file_name());

    // 添加配置项
    // 配置 Python 语言环境
    lsp_config.add_config_item("venvPath", &venv.base_path());
    lsp_config.add_config_item("venv", &venv.name());

    // 接受命令行参数
    // Todo: 解析命令行参数并添加配置项

    // 创建项目目录
    let project_dir = format!("./{}", project_name);
    let src_dir = format!("{}/src", project_dir);
    match fs::create_dir_all(&src_dir) {
        Ok(_) => println!("Directory created successfully: {}", src_dir),
        Err(e) => eprintln!("Failed to create directory: {}", e),
    }

    // 写入配置文件
    let config_file = format!("{}/{}", project_dir, lsp_config.config_file_name());
    let mut file = File::create(&config_file).unwrap();
    write_config(&mut file, &lsp_config).unwrap();

    // 创建 main.py 写入 hello world
    let main_file = format!("{}/main.py", src_dir);
    let mut file = File::create(&main_file).unwrap();
    file.write_all(HELLO_WORLD.as_bytes()).unwrap();

    // 项目创建成功
    println!("Project created successfully: {}", project_dir);
}
