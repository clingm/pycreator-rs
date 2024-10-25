#[allow(dead_code)]
use std::process::Command;

fn get_conda_base_path() -> Result<String, String> {
    let output = Command::new("conda")
        .arg("info")
        .arg("--base")
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?; // 捕获命令执行错误

    if output.status.success() {
        let conda_base_path = String::from_utf8_lossy(&output.stdout);
        Ok(conda_base_path.trim().to_string()) // 成功时返回路径
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed: {}", error_message)) // 失败时返回错误信息
    }
}

pub struct CondaVenv {
    base_path: String,
    name: String,
}

impl CondaVenv {
    pub fn new(name: &str) -> Self {
        CondaVenv {
            base_path: get_conda_base_path().unwrap(),
            name: name.to_string(),
        }
    }

    pub fn base_path(&self) -> String {
        self.base_path.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn check_env(&self) -> Result<(), String> {
        let output = Command::new("conda")
            .arg("env")
            .arg("list")
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            let env_list = String::from_utf8_lossy(&output.stdout);
            // println!("Available environments:\n{}", env_list); // 打印可用环境列表
            if env_list.contains(&self.name) {
                Ok(())
            } else {
                Err(format!("Environment {} not found", self.name))
            }
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(format!("Command failed: {}", error_message))
        }
    }
}
