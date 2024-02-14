// Abstract view of a Bluetooth Low Energy authenticator
pub trait BLEAuthenticator {
    fn scan(&self);
    fn pair(&self);
}

// Bluetooth Low Energy authenticator for Linux system
struct LinuxBLEAuthenticator { }

impl BLEAuthenticator for LinuxBLEAuthenticator {
    fn scan(&self) {
        todo!()
    }

    fn pair(&self) {
        todo!()
    }
}

impl LinuxBLEAuthenticator { 
    fn new() -> LinuxBLEAuthenticator {
        LinuxBLEAuthenticator { }
    }
}



// Factory to crete a Bluetooth Low Energy authenticator
pub struct BLEAuthenticatorFactory {

}

impl BLEAuthenticatorFactory {
    pub fn create(&self) -> Box<dyn BLEAuthenticator> {
        Box::new(LinuxBLEAuthenticator::new())
    }
}