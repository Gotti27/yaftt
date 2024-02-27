use std::{io, process};
use bluer::{agent::{Agent, AgentHandle, ReqResult, RequestPasskey}, AdapterEvent, Result, Session};
use async_trait::async_trait;
use futures::{pin_mut, StreamExt};
use colored::Colorize;

use core::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

// Abstract view of a Bluetooth Low Energy authenticator
#[async_trait]
pub trait BLEAuthenticator {
    /// Scan for available Bluetooth devices.
    async fn scan(&self) -> Result<()>;

    /// Bluetooth pairing with another device.
    async fn pair(&self) -> Result<()>;
}

// Bluetooth Low Energy authenticator for Linux system
struct LinuxBLEAuthenticator {
    #[allow(unused)]
    agent_handle: AgentHandle,
    session: Session,
}

#[async_trait]
impl BLEAuthenticator for LinuxBLEAuthenticator {
    async fn scan(&self) -> Result<()>{
        println!("Start scanning for devices...");
        println!("\nctrl-q to stop scanning");

        let adapter = self.session.default_adapter().await?;
        adapter.set_powered(true).await?; //TODO fix it doesn't work properly

        let discover = adapter.discover_devices_with_changes().await?;
        pin_mut!(discover);

        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr)?;
                        
                    if let Some(device_name) = device.name().await? {
                        println!("{} {addr} {device_name}", "Device added".green());
                    }
                }
                _ => (),
            }

            enable_raw_mode()?;
            if poll(Duration::from_millis(500))? == true {
                match read()? {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::CONTROL, 
                        kind: KeyEventKind::Press, state: _ }) => {disable_raw_mode()?; break;},
                    _ => disable_raw_mode()?
                }
            } else {
                disable_raw_mode()?;
            }
            
        }

        println!("...end scanning.");
        Ok(())
    }

    async fn pair(&self) -> Result<()>{
        Ok(())
    }
}

async fn request_passkey(_req: RequestPasskey) -> ReqResult<u32> {
    print!("Insert passkey: ");
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let x: u32 = input_line.trim().parse().expect("Input not an integer");
    
    Ok(x)
}

impl LinuxBLEAuthenticator { 
    async fn new() -> LinuxBLEAuthenticator {
        let session_result = Session::new().await;

        let session = match session_result {
            Ok(session) => session,
            Err(_) => {println!("An error occurred when creating a session, exiting"); process::exit(1)},
        };

        // TODO check if unudes fields can be filled with Default::default()
        let agent = Agent {
            request_passkey: Some(Box::new(|req| Box::pin(request_passkey(req)))),
            request_default: false,
            request_pin_code: None,
            display_pin_code: None,
            display_passkey: None,
            request_confirmation: None,
            request_authorization: None,
            authorize_service: None,
            _non_exhaustive: (),
        };
        
        let agent_handle = match session.register_agent(agent).await {
            Ok(agent_handle) => agent_handle,
            Err(_) => {println!("An error occurred when registering the agent, exiting"); process::exit(1)},
        };
        LinuxBLEAuthenticator{session, agent_handle}
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