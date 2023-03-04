# file_transferer

Program written in rust to transfer a file to other PC using a valid TCP connection.

## Installation

Run the following command, you'll need [rust](https://www.rust-lang.org/) installed.

```bash
cargo build --release
```

or

```bash
cargo build
```

It's only one file so rustc works too.

```bash
rustc src/main.rs
```

Windows binaries (64bits) may be provided in the future.

## Usage

Output of file_transferer help:

```bash
file_transferer help

Usage: file_transferer [SUBCOMMAND] [OPTIONS]
Subcommands:
    send <ip> <file>       send <file> to a valid connection
                           <ip> must be a valid IPV4 or IPV6 with the port included
                           <file> must be a valid path to an existing file
                           example: 'file_transferer send 54.230.78.52:69690 file_name'
    receive <ip> <file>    receive a file from a known <ip>
                           <ip> must be a valid IPV4 or IPV6 with the port included
                           <file> must be a valid name for the received file, you can choose whatever
                           example: 'file_transferer receive 54.230.78.52:69690 file_name'
    help                   prints this
```

The PC sending the file needs to be able to provide a valid IP and a valid TCP connection. More information at [wikipedia](https://en.wikipedia.org/wiki/Transmission_Control_Protocol). Then you run something like the following to get the server runnning:

```bash
file_transferer send 11.22.33.44:24865 path_to_file
```

The PC receiving the file doesn't need much. You can choose the file path you want, but be careful, if there's a file already named like that it will get removed. The IP must be the same one. Just run the following command:

```bash
file_transferer receive 11.22.33.44:24865 path_to_file
```

## Things that will probably change

* Multi-threading.
* Buffered read/write?
* Maybe not removing file if exists.
* Maybe you could send the file name? But then we go back to the 3rd.
* Suggestions are always welcome! Feel free to message me anywhere you find me.
