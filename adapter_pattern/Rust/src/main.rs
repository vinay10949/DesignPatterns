struct Client;

struct Windows;

struct Mac;

trait Computer {
    fn insert_into_lightning_port(&self);
}

impl Client {
    fn insert_lightning_connector_into_computer(&self,com: &dyn Computer) {
        println!("Client inserts Lightning connector into computer.");
        com.insert_into_lightning_port();
    }
}

impl Computer for Mac {
    fn insert_into_lightning_port(&self) {
        println!("Lightning connector is plugged into mac machine.");
    }
}

impl Windows {
    pub fn insert_into_usb_port(&self) {
        println!("USB connector is plugged into windows machine.");
    }
}

struct WindowsAdaptor<'a> {
    pub windows_machine: &'a Windows,
}

impl <'a> Computer for WindowsAdaptor<'a> {
    fn insert_into_lightning_port(&self) {
        println!("Adapter converts Lightning signal to USB.");
        self.windows_machine.insert_into_usb_port();
    }
}

fn main() {
    let Client = &Client {};
    let Mac = &Mac {};

    Client.insert_lightning_connector_into_computer(&Mac);

    let windows_machine = &Windows {};
    let windows_machine_adapter = &WindowsAdaptor {
        windows_machine
    };

    Client.insert_lightning_connector_into_computer(windows_machine_adapter)
}
