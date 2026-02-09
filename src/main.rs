use modbus_rtu::{Function, Master, Request};
use argh::FromArgs;

#[derive(FromArgs)]
/// Jump program
struct AppArgs {
    /// whether to be verbose
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// port to use
    #[argh(option)]
    port: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world, scanning ports!");

    let ports = serialport::available_ports().expect("No ports found!");
    if ports.is_empty() {
        println!("No ports found");
    } else {
        for p in ports {
            println!("{}", p.port_name);
        }
    }

    let args: AppArgs = argh::from_env();
    println!("Verbose: {}, Port: {}", args.verbose, args.port);

    let mut master = Master::new_rs485(&args.port, 19_200)?;

    let func = Function::ReadHoldingRegisters { starting_address: 0x0000, quantity: 2 };
    let request = Request::new(0x01, &func, std::time::Duration::from_millis(200));

    let response = master.send(&request)?;
    println!("response: {response:?}");
    Ok(())
}
