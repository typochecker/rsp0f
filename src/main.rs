use clap::Parser;
use colored::*;
use pcap::Device;
use rsp0f::Cli;

fn main() {
    println!("{}", format!("entry main {}", "#".repeat(25)).blue());
    let args = Cli::parse();
    println!("{}", format!("all args: {:#?}", args).yellow());
    if args.laani_flag {
        println!("{}", "list all available network interface: ".yellow());
        // 展示所有可用设备接口
        for device in Device::list().unwrap() {
            println!("Device: {:?}", device);
        }
    }

    println!("{}", format!("exit program {}", "#".repeat(25)).blue());
}
