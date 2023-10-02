# Rust IP info

Ip info in console written in rust

# Usage

`ipinfo` is very easy to use

```
ipinfo <ip (optional)>
```

If you provide an IP it will get information for that IP, If you don't provide an IP it will use your IP.

# Example
```
ipinfo -> returns information about your IP

ipinfo 127.0.0.1 -> returns an error due to 127.0.0.1 being a loopback address

ipinfo 592.910.239.723 -> returns all null values
```

# Installation Guide

To install `ipinfo` first run
```
git clone https://github.com/Minejerik/rust-ipinfo.git
```
This clones the repo to your local disk, now run
```
cargo install --path rust-ipinfo/
```
This installs all the dependencies and builds the `ipinfo` tool.
Once you have completed both of these commands, just run `ipinfo` in the terminal to use the tool.
