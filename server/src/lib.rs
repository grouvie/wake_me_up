#![feature(lazy_cell)]

pub mod ctx;
pub mod database;
pub mod error;
pub mod log;
pub mod migrations;
pub mod model;
pub mod protobuf;
pub mod web;

// Include the proto modules, which are generated from protos/*.proto.
// It is important to maintain the same structure as in the proto.
pub mod proto {
    pub mod user {
        include!(concat!(env!("OUT_DIR"), "/proto.user.rs"));
    }
    pub mod basic_response {
        include!(concat!(env!("OUT_DIR"), "/proto.basic_response.rs"));
    }
    pub mod login_request {
        include!(concat!(env!("OUT_DIR"), "/proto.login_request.rs"));
    }
    pub mod login_response {
        include!(concat!(env!("OUT_DIR"), "/proto.login_response.rs"));
    }
    pub mod logout_response {
        include!(concat!(env!("OUT_DIR"), "/proto.logout_response.rs"));
    }
    pub mod add_device_request {
        include!(concat!(env!("OUT_DIR"), "/proto.add_device_request.rs"));
    }
    pub mod device {
        include!(concat!(env!("OUT_DIR"), "/proto.device.rs"));
    }
    pub mod wake_up {
        include!(concat!(env!("OUT_DIR"), "/proto.wake_up.rs"));
    }
}
