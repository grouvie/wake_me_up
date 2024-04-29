use crate::database::data::models::DeviceDB;
use crate::proto::add_device_request::AddDeviceRequestProto;
use crate::proto::device::DeviceProto;

pub(crate) struct DeviceModel {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) mac_address: String,
}
pub(crate) struct AddDeviceModel {
    pub(crate) name: String,
    pub(crate) mac_address: String,
}

impl From<&DeviceModel> for DeviceProto {
    fn from(value: &DeviceModel) -> Self {
        DeviceProto {
            device_id: value.id,
            name: value.name.clone(),
            mac_address: value.mac_address.clone(),
        }
    }
}

impl From<&DeviceDB> for DeviceModel {
    fn from(value: &DeviceDB) -> Self {
        DeviceModel {
            id: value.id,
            name: value.name.clone(),
            mac_address: value.mac_address.clone(),
        }
    }
}

impl From<&DeviceProto> for DeviceModel {
    fn from(value: &DeviceProto) -> Self {
        DeviceModel {
            id: value.device_id,
            name: value.name.clone(),
            mac_address: value.mac_address.clone(),
        }
    }
}

impl From<&AddDeviceRequestProto> for AddDeviceModel {
    fn from(value: &AddDeviceRequestProto) -> Self {
        AddDeviceModel {
            name: value.name.clone(),
            mac_address: value.mac_address.clone(),
        }
    }
}
