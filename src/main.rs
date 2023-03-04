//! Program to send/receive a file through a valid TCP connection.
//!
//! Use, change, do whatever you want with this.
//!
//! Usable but expect improvements in the future.
//!
//! Suggestion are always welcome!
//!
//! Original author: aleferu - <https://github.com/aleferu>

use std::env::args;
use std::io::{Write, Read, BufReader};
use std::fs;
use std::process::exit;
use std::net::{TcpListener, TcpStream};
use std::error::Error;

// Socket size, change this to your liking
const SOCKET_SIZE: usize = 64usize * 1024usize;

// Calls exit_with_error_msg with a formatted error message
fn handle_error(message: &str, err: impl Error)  {
    exit_with_error_msg(&format!("{message}: {err}"));
}

// Used to print the given message to stderr and exit with code 1
fn exit_with_error_msg(message: &str) {
    eprintln!("{message}");
    exit(1);
}

// Used to receive the file
fn receive_file(ip: &str, file: &str) {
    fs::File::create(file).map_err(|err| {
        handle_error(&format!("Error trying to create the file '{file}'"), err)
    }).unwrap();
    let mut stream = TcpStream::connect(ip).map_err(|err| {
        handle_error(&format!("Error trying to connect to {ip}"), err);
    }).unwrap();
    let mut file_ref = fs::OpenOptions::new().append(true).open(file).map_err(|err| {
        handle_error(&format!("Error trying to write to file '{file}'"), err);
    }).unwrap();
    let mut bytes = [0; SOCKET_SIZE];
    println!("\nDownloading file, please wait...\n");
    let mut sum = 0;
    let mut file_size_as_bytes = [0u8; 64/8];
    // First read to receive file size
    if 8 != stream.read(&mut file_size_as_bytes).map_err(|err| {
                handle_error(&format!("Error trying to read from {ip}"), err);
            }).unwrap() {
        exit_with_error_msg("Error trying to receive file size");
    }
    let file_size = {
        let mut sum = 0u64;
        for byte in file_size_as_bytes.iter() {
            sum = (sum << 8) + (byte & 0xFF) as u64;
        }
        sum
    };
    println!("File size: {file_size} bytes");

    loop { // Actual receiving of the file
        let bytes_read = stream.read(&mut bytes).map_err(|err| {
            handle_error(&format!("Error trying to read from {ip}"), err);
        }).unwrap();
        sum += bytes_read;
        let sum_bytes = sum.to_be_bytes();
        file_ref.write_all(&bytes[..bytes_read]).map_err(|err| {
            handle_error(&format!("Error trying to write to file '{file}'"), err);
        }).unwrap(); // I don't think this is buffered, idk if u even can! Will investigate
        let current_size: u64 = {
            let mut byte_sum: u64 = 0u64;
            for byte in sum_bytes.iter() {
                byte_sum = (byte_sum << 8) + (byte & 0xFF) as u64;
            }
            byte_sum
        };
        let percentage = (file_size - current_size) as f64 / file_size as f64 * 100f64;
        println!("{percentage} % downloaded", percentage = ((100f64 - percentage) * 100f64).round() / 100f64);
        if file_size == current_size {
            break;
        }
    }
    stream.write(&[1]).unwrap(); // Used for syncing with server

    println!("File downloaded. Thank you for using file_transferer.");
    exit(0);// Success! This line will change
}

// Used to send the file
fn send_file(mut socket: TcpStream, file: &str) {
    let file_pointer = fs::File::open(file).map_err(|err| {
        handle_error(&format!("Error trying to open file '{file}'"), err);
    }).unwrap();
    let total_file_size = file_pointer.metadata().map_err(|err|{
        handle_error(&format!("Error trying to get size of file '{file}'"), err);
    }).unwrap().len();
    println!("File size: {total_file_size} bytes");
    let mut total_sent: u64 = 0;
    let mut buffer = [0u8; SOCKET_SIZE];
    let mut buf_reader = BufReader::new(file_pointer);
    let mut bytes_read = buf_reader.read(&mut buffer).map_err(|err| {
        handle_error(&format!("Error trying to read from file '{file}'"), err);
    }).unwrap();
    let mut byte_pointer = 0;
    let mut bytes_left = &buffer[byte_pointer..bytes_read];
    //First write to send file size
    if 8 != socket.write(&total_file_size.to_be_bytes()).map_err(|err| {
            handle_error("Error trying to write in socket", err);
        }).unwrap() {
        exit_with_error_msg("Error trying to send file size");
    }

    while bytes_left.len() > 0 { // Actual sending of the file
        let bytes_written = socket.write(&mut bytes_left).map_err(|err| {
            handle_error("Error trying to write in socket", err);
        }).unwrap();
        byte_pointer += bytes_written;
        total_sent += bytes_written as u64;
        if byte_pointer == bytes_read {
            byte_pointer = 0;
            bytes_read = buf_reader.read(&mut buffer).map_err(|err| {
                handle_error(&format!("Error trying to read from file '{file}'"), err);
            }).unwrap();
            let percentage_left = (total_file_size - total_sent) as f64 / total_file_size as f64 * 100f64;
            println!("{percentage_left} % left", 
                percentage_left = (percentage_left * 100f64).round() / 100f64);
        }
        bytes_left = &buffer[byte_pointer..bytes_read];
    }
    socket.read(&mut [1]).unwrap(); // Used for syncing with client

    println!("\nFile sent!\n");
    exit(0); // Success! This line will change
}

// Function in charge of binding the given IP to a TCP listener
// and send the file to an incoming client calling send_file 
fn start_server(ip: &str, file: &str) {
    let listener = TcpListener::bind(ip).map_err(|err| {
        handle_error(&format!("Error found trying to bound to '{ip}'").as_str(), err);
    }).unwrap();
    println!("Waiting a connection...");
    loop { // Currently this loop does nothing until multi-threading is implemented
        match listener.accept() {
            Ok((socket, addr)) => { 
                println!("Trying to send {file} to {addr}...");
                send_file(socket, file);
                break;
            }
            Err(err) => { 
                handle_error("Error trying to establish connection", err);
            }
        }
    }
}

// Basic help command, nothing special
fn print_help() {
    println!("Usage: file_transferer [SUBCOMMAND] [OPTIONS]");
    println!("Subcommands:");
    println!("    send <ip> <file>       send <file> to a valid connection");
    println!("                           <ip> must be a valid IPV4 or IPV6 with the port included");
    println!("                           <file> must be a valid path to an existing file");
    println!("                           example: 'file_transferer send 54.230.78.52:69690 file_name'");
    println!("    receive <ip> <file>    receive a file from a known <ip>");
    println!("                           <ip> must be a valid IPV4 or IPV6 with the port included");
    println!("                           <file> must be a valid name for the received file, you can choose whatever");
    println!("                           example: 'file_transferer receive 54.230.78.52:69690 file_name'");
    println!("    help                   prints this");
}

// Basically a subcommand manager
fn main() {
    let args: Vec<String> = args().collect();
    match args.get(1).ok_or_else(|| {
        exit_with_error_msg("Please provide a subcommand. See 'file_transferer help' for help.")
    }).unwrap().as_str() {
        "send" => { 
            if args.len() < 4 {
                exit_with_error_msg("Not enough arguments provided. See 'file_transferer help' for help.")
            }
            start_server(args.get(2).unwrap(), args.get(3).unwrap());
        },
        "receive" => { 
            if args.len() < 4 {
                exit_with_error_msg("Not enough arguments provided. See 'file_transferer help' for help.")
            }
            receive_file(args.get(2).unwrap(), args.get(3).unwrap());
        },
        "help" => { print_help(); }
            _ => { 
            exit_with_error_msg("Please, provide a valid subcommand. See the list at 'file_transferer help'.")
        }
    }
}

// TODO:
//
// - Multi-threading
// - Buffered read/write?
// - Maybe not removing file if exists
// - Maybe you could send the file name? But then we go back to ^
// - Idk