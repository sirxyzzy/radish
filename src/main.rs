use argh::FromArgs;
use anyhow::{Result,Ok};

use modbus_rtu::{Function, Master, Request};

#[derive(FromArgs)]
/// Servo driver (uses MODBUS-RTU)
struct AppArgs {
    /// whether to be verbose
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// perform a scan of available ports and exit
    #[argh(switch, short = 's')]
    scan: bool,

    /// port to use
    #[argh(option, short = 'p', default = "String::from(\"/dev/ttyUSB0\")")]
    port: String,
}

fn main() -> Result<()> {
    let args: AppArgs = argh::from_env();

    let verbose = args.verbose;

    if args.scan {
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
        return Ok(());
    }

    let mut master = Master::new_rs485(&args.port, 19_200)?;

    let func = Function::ReadHoldingRegisters { starting_address: 0x0000, quantity: 2 };
    let request = Request::new(0x01, &func, std::time::Duration::from_millis(200));

    let response = master.send(&request)?;
    println!("response: {response:?}");
    Ok(())
}
