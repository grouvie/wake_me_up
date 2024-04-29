pub mod headers;
pub mod login;
pub mod websocket;
// Include the proto modules, which are generated from protos/*.proto.
// It is important to maintain the same structure as in the proto.
pub mod proto {
    pub mod login_request {
        include!(concat!(env!("OUT_DIR"), "/proto.login_request.rs"));
    }
    pub mod login_response {
        include!(concat!(env!("OUT_DIR"), "/proto.login_response.rs"));
    }
    pub mod basic_response {
        include!(concat!(env!("OUT_DIR"), "/proto.basic_response.rs"));
    }
    pub mod device {
        include!(concat!(env!("OUT_DIR"), "/proto.device.rs"));
    }
    pub mod wake_up {
        include!(concat!(env!("OUT_DIR"), "/proto.wake_up.rs"));
    }
}
