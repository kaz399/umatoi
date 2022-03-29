use log::{debug, error};
use std::error::Error;
use std::time::Duration;
use thiserror::Error;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};

pub async fn scan_example(
    filter: ScanFilter,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;

    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(filter.clone())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("No devices are found...");
        } else {
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                println!(
                    "Peripheral {:?} is connected {:?}",
                    local_name, is_connected
                );
                if !is_connected {
                    println!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                println!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected, &local_name
                );
                peripheral.discover_services().await?;
                println!("Discover peripheral {:?} services...", &local_name);
                for service in peripheral.services() {
                    println!(
                        "Service UUID {}, primary: {}",
                        service.uuid, service.primary
                    );
                    for characteristic in service.characteristics {
                        println!("  {:?}", characteristic);
                    }
                }
                if is_connected {
                    println!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Error disconnecting from BLE peripheral");
                }
            }
        }
    }
    Ok(())
}

#[derive(Error, Debug, PartialEq)]
pub enum ScannerError {
    #[error("bluetooth adapter is not found")]
    AdapterNotFound,
    #[error("internal error of scanner.rs")]
    FoundBug,
}

pub async fn scan(
    filter: ScanFilter,
    wait: Duration,
) -> Result<Vec<Peripheral>, Box<dyn Error + Send + Sync + 'static>> {
    let mut peripheral_list: Vec<Peripheral> = Vec::new();
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;

    if adapter_list.is_empty() {
        error!("No Bluetooth adapters found");
        return Err(Box::new(ScannerError::AdapterNotFound));
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter.start_scan(filter.clone()).await?;
        time::sleep(wait).await;
        peripheral_list.extend(adapter.peripherals().await?);
    }
    for (index, peripheral) in peripheral_list.iter().enumerate() {
        debug!("{} {:?}", index, peripheral);
    }
    debug!("total {} peripherals found", peripheral_list.len());
    Ok(peripheral_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _setup() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn scanner_scan_cube() {
        _setup();
        let _scan_result = scan(ScanFilter::default(), Duration::from_secs(2)).await;
    }
}
