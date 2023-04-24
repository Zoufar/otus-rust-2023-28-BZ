#![allow(dead_code)]

struct Socket {
    description: String,
    state: bool,
}
impl Socket {
    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn change_state(&mut self) {
        self.state = !self.get_state();
    }

    pub fn current_power_consumption(&self) -> f32 {
        todo!()
    }
}

struct Thermometer {
    upper_limit: i32,
    lower_limit: i32,
}

impl Thermometer {
    pub fn current_temperature(&self) -> f32 {
        todo!()
    }
}

fn main() {
    let mut socket = Socket {
        description: "socket1".to_owned(),
        state: false,
    };
    println!(
        "Hello {} in state {}",
        socket.get_description(),
        socket.get_state()
    );
    socket.change_state();
    println!("changed state to {}!", socket.get_state());
}
