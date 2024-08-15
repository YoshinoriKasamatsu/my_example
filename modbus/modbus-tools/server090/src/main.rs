mod myservice;

use std::{
    collections::HashMap,
    net::SocketAddr,
};

use tokio::net::TcpListener;

use tokio_modbus::server::tcp::{accept_tcp_connection, Server};
use crate::myservice::ExampleService;

/// Helper function implementing reading registers from a HashMap.
fn register_read(
    registers: &HashMap<u16, u16>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, std::io::Error> {
    let mut response_values = vec![0; cnt.into()];
    for i in 0..cnt {
        let reg_addr = addr + i;
        if let Some(r) = registers.get(&reg_addr) {
            response_values[i as usize] = *r;
        } else {
            // TODO: Return a Modbus Exception response `IllegalDataAddress` https://github.com/slowtec/tokio-modbus/issues/165
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                format!("no register at address {reg_addr}"),
            ));
        }
    }

    Ok(response_values)
}

/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.
fn register_write(
    registers: &mut HashMap<u16, u16>,
    addr: u16,
    values: &[u16],
) -> Result<(), std::io::Error> {
    for (i, value) in values.iter().enumerate() {
        let reg_addr = addr + i as u16;
        if let Some(r) = registers.get_mut(&reg_addr) {
            *r = *value;
        } else {
            // TODO: Return a Modbus Exception response `IllegalDataAddress` https://github.com/slowtec/tokio-modbus/issues/165
            println!("SERVER: Exception::IllegalDataAddress");
            return Err(std::io::Error::new(
                std::io::ErrorKind::AddrNotAvailable,
                format!("no register at address {reg_addr}"),
            ));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let socket_addr = "0.0.0.0:5502".parse().unwrap();

    tokio::select! {
        _ = server_context(socket_addr) => unreachable!(),
    }
}

async fn server_context(socket_addr: SocketAddr) -> anyhow::Result<()> {
    println!("Starting up server on {socket_addr}");
    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);
    let new_service = |_socket_addr| Ok(Some(ExampleService::new()));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        eprintln!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
