use std::error::Error;

use bluest::Adapter;
use futures_lite::StreamExt;
use tracing::info;
use tracing::metadata::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    let mut unconnected_devices = adapter.discover_devices(&[]).await?;

    while let Some(uncon) = unconnected_devices.next().await {
        let device = uncon.unwrap();
        info!("connecting to {}....", device.name()?);
        adapter.connect_device(&device).await?;
        info!("connected to {}", device.name()?);

        let services = device.services().await?;
        for service in services {
            info!("  {:?}", service);
            info!("service UUID => {}", service.uuid().to_string());

            let characteristics = service.characteristics().await?;

            for characteristic in characteristics {
                info!("    {:?}", characteristic);
                info!("characteristic UUID => {}", characteristic.uuid().to_string());
                let props = characteristic.properties().await?;
                info!("      props: {:?}", props);
                if props.read {
                    info!("      value: {:?}", characteristic.read().await);
                }
                if props.write_without_response {
                    info!("      max_write_len: {:?}", characteristic.max_write_len());
                }

                let descriptors = characteristic.descriptors().await?;
                for descriptor in descriptors {
                    info!("      {:?}: {:?}", descriptor, descriptor.read().await);
                }
            }
        }

        adapter.disconnect_device(&device).await?;
        info!("disconnected!");
    }

    info!("done");

    Ok(())
}
