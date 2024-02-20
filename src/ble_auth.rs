use std::process;
use bluer::{Session, AdapterEvent, Result};
use async_trait::async_trait;
use futures::{pin_mut, StreamExt};
use colored::Colorize;

// Abstract view of a Bluetooth Low Energy authenticator
#[async_trait]
pub trait BLEAuthenticator {
    async fn scan(&self) -> Result<()>;
    async fn pair(&self);
}

// Bluetooth Low Energy authenticator for Linux system
struct LinuxBLEAuthenticator {
    session: Session,
}

#[async_trait]
impl BLEAuthenticator for LinuxBLEAuthenticator {
    async fn scan(&self) -> Result<()>{
        let adapter = self.session.default_adapter().await?;

        adapter.set_powered(true).await?;

        let discover = adapter.discover_devices().await?;
        pin_mut!(discover);

        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    println!("{} {addr}", "Device added".green());
                }
                AdapterEvent::DeviceRemoved(addr) => {
                    println!("{} {addr}", "Device removed".red());
                }
                _ => (),
            }
        }
        Ok(())
    }

    async fn pair(&self) {
        todo!()
    }
}

impl LinuxBLEAuthenticator { 
    async fn new() -> LinuxBLEAuthenticator {
        let session_result = Session::new().await;

        let session = match session_result {
            Ok(session) => session,
            Err(_) => {println!("An error occurred, exiting"); process::exit(1)},
        };
        LinuxBLEAuthenticator{session}
    }
}



// Factory to crete a Bluetooth Low Energy authenticator
pub struct BLEAuthenticatorFactory {

}

impl BLEAuthenticatorFactory {
    pub fn new() -> BLEAuthenticatorFactory{
        BLEAuthenticatorFactory{}
    }

    pub async fn create(&self) -> Box<dyn BLEAuthenticator> {
        Box::new(LinuxBLEAuthenticator::new().await)
    }
}