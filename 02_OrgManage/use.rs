mod nation {
    pub mod government {
        pub fn govern() {
            println!("Government is working...");
        }
    }
}

use crate::nation::government::govern;

fn main() {
    govern();
}