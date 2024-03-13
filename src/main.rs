use std::fs;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "A simple tool to convert Huawei switch configuration to H3C.")]
struct Cli {
    /// Input file path
    #[structopt(short = "i", long = "input", parse(from_os_str), help = "Input file path")]
    input: Option<std::path::PathBuf>,

    /// Output file path
    #[structopt(short = "o", long = "output", parse(from_os_str), help = "Output file path")]
    output: Option<std::path::PathBuf>,
}

struct HuaweiConfig {
    interface: String,
    description: String,
    ip_address: String,
    subnet_mask: String,
    vlans: Vec<String>,
}

impl HuaweiConfig {
    fn from_str(config_str: &str) -> Self {
        let mut interface = String::new();
        let mut description = String::new();
        let mut ip_address = String::new();
        let mut subnet_mask = String::new();
        let mut vlans = Vec::new();

        for line in config_str.lines() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            match parts.get(0) {
                Some(&"interface") => interface = parts[1].to_string(),
                Some(&"description") => description = parts[1..].join(" "),
                Some(&"ip") if parts.get(1) == Some(&"address") => {
                    ip_address = parts[2].to_string();
                    subnet_mask = parts[3].to_string();
                }
                Some(&"vlan") if parts.get(1) == Some(&"batch") => {
                    vlans.extend(parts[2..].iter().map(|s| s.to_string())); // 修改此行
                }
                _ => {}
            }
        }

        HuaweiConfig {
            interface,
            description,
            ip_address,
            subnet_mask,
            vlans,
        }
    }

    fn to_h3c_config(&self) -> String {
        let mut h3c_config = format!(
            "interface {} \ndescription {}\nip address {} {}\n",
            self.interface, self.description, self.ip_address, self.subnet_mask
        );

        for vlan in &self.vlans {
            h3c_config.push_str(&format!("vlan {}\n", vlan));
        }

        h3c_config
    }
}

fn main() {
    let args = Cli::from_args();

    // 如果没有传入参数，则输出帮助信息
    if args.input.is_none() || args.output.is_none() {
        if let Err(err) = Cli::clap().print_long_help() {
            eprintln!("Error: {:?}", err);
        }
        return;
    }

    let input_path = args.input.unwrap();
    let output_path = args.output.unwrap();

    let result = process_config(&input_path, &output_path);

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }
}

fn process_config(input_path: &std::path::PathBuf, output_path: &std::path::PathBuf) -> io::Result<()> {
    // 读取输入文件
    let huawei_config_str = fs::read_to_string(input_path)?;

    // 解析华为配置
    let huawei_config = HuaweiConfig::from_str(&huawei_config_str);

    // 生成新华三配置
    let h3c_config = huawei_config.to_h3c_config();

    // 输出新华三配置到指定文件
    fs::write(output_path, h3c_config)?;

    Ok(())
}
