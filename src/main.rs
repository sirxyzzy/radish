use argh::FromArgs;
use anyhow::{Result};
use std::result::Result::Ok;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use modbus_rtu::{Function, Master, Request};

#[derive(FromArgs)]
/// Servo driver (uses MODBUS-RTU)
struct AppArgs {
    /// whether to be verbose
    #[argh(switch, short = 'v')]
    verbose: bool,

    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum MySubCommandEnum {
    Parse(PrettyPrintCommand),
    Scan(ScanCommand),
    Connect(ConnectCommand)
}

#[derive(FromArgs)]
/// Pretty print log file as decoded MODBUS messages.
#[argh(subcommand, name = "parse")]
struct PrettyPrintCommand {
    /// log file to parse
    #[argh(option, short = 'l')]
    logfile: String, 
}

#[derive(FromArgs)]
/// Scan for available ports and exit.
#[argh(subcommand, name = "scan")]
struct ScanCommand {
}

#[derive(FromArgs)]
/// Open a port and send modbus messages to it
#[argh(subcommand, name = "connect")]
struct ConnectCommand {
        /// port to connect to
    #[argh(option, short = 'p')]
    port: String,
}

fn main() -> Result<()> {
    let args: AppArgs = argh::from_env();

    let verbose = args.verbose;

    match &args.nested {
        MySubCommandEnum::Parse(cmd) => {
            if verbose {
                println!("Parse mode enabled, parsing log file: {}", cmd.logfile);
            }

            let lines = read_lines(&cmd.logfile)?;
            for line in lines.map_while(Result::ok) {
                if !line.is_empty() {
                    println!("> {}", line);
                    if line.starts_with("Send:") || line.starts_with("Recv:") {
                        for byte_str in line[5..].split_whitespace() {
                            let rr: std::result::Result<u8, std::num::ParseIntError> = u8::from_str_radix(byte_str, 16);
                            match rr {
                                Ok(b) => { print!("{b:02X} ") }
                                Err(e) => { eprintln!("Error parsing byte '{}': {e}", byte_str) }
                            }
                        }
                        println!();
                    }   
                }
            }
        }

        MySubCommandEnum::Scan(_cmd) => {
            if verbose {
                println!("Scan mode enabled, exiting after listing ports.");
            }
            let ports = serialport::available_ports().expect("No ports found!");
            if ports.is_empty() {
                println!("No ports found");
            } else {
                for p in ports {
                    println!("{}", p.port_name);
                }
            }
        }
        MySubCommandEnum::Connect(cmd) => {
            if verbose {
                println!("Using port: {}", cmd.port);
            }

            let mut master = Master::new_rs485(&cmd.port, 19_200)?;

            let func = Function::ReadHoldingRegisters { starting_address: 0x0000, quantity: 2 };
            let request = Request::new(0x01, &func, std::time::Duration::from_millis(200));

            let response = master.send(&request)?;
            println!("response: {response:?}");
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
