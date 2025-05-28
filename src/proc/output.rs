use super::cli::CliArgs;
use super::process::ProcessInfo;
use std::fs::File;
use std::io::Write;

pub async fn handle_output(processes: &[ProcessInfo], args: &CliArgs) {
    if let Some(path) = &args.output {
        match File::create(path) {
            Ok(mut file) => {
                let json = serde_json::to_string_pretty(processes).unwrap();
                if let Err(e) = file.write_all(json.as_bytes()) {
                    eprintln!("Erro ao escrever no arquivo: {}", e);
                } else {
                    println!("Exportado com sucesso para: {}", path);
                }
            }
            Err(e) => {
                eprintln!("Erro ao criar arquivo: {}", e);
            }
        }
    } else {
        for p in processes {
            println!(
                "[PID: {}] {} | CPU: {:.2}% | Mem: {} KB",
                p.pid, p.name, p.cpu_usage, p.memory
            );
        }   
    }
}

