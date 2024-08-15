use std::{
    net::SocketAddr,
    time::Duration,
};
use tokio_modbus::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();

    tokio::select! {
        _ = client_context(socket_addr) => println!("Exiting"),
    }

    Ok(())
}


async fn client_context(socket_addr: SocketAddr) {
    tokio::join!(
        async {
            // Give the server some time for starting up
            tokio::time::sleep(Duration::from_secs(1)).await;

            println!("CLIENT: Connecting client...");
            let mut ctx = tcp::connect(socket_addr).await.unwrap();

            println!("CLIENT: Reading 2 input registers...");
            let response = ctx.read_input_registers(0x00, 2).await.unwrap();
            println!("CLIENT: The result is '{response:?}'");
            assert_eq!(response, [1234, 5678]);

            println!("CLIENT: Writing 2 holding registers...");
            ctx.write_multiple_registers(0x01, &[7777, 8888])
                .await
                .unwrap();

            // Read back a block including the two registers we wrote.
            println!("CLIENT: Reading 4 holding registers...");
            let response = ctx.read_holding_registers(0x00, 4).await.unwrap();
            println!("CLIENT: The result is '{response:?}'");
            assert_eq!(response, [10, 7777, 8888, 40]);

            // Now we try to read with an invalid register address.
            // This should return a Modbus exception response with the code
            // IllegalDataAddress.
            println!("CLIENT: Reading nonexisting holding register address... (should return IllegalDataAddress)");
            let response = ctx.read_holding_registers(0x100, 1).await;
            println!("CLIENT: The result is '{response:?}'");
            assert!(response.is_err());
            // TODO: How can Modbus client identify Modbus exception responses? E.g. here we expect IllegalDataAddress
            // Question here: https://github.com/slowtec/tokio-modbus/issues/169

            println!("CLIENT: Done.")
        },
        tokio::time::sleep(Duration::from_secs(5))
    );
}