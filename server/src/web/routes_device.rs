use crate::proto::add_device_request::AddDeviceRequestProto;
use crate::proto::basic_response::BasicResponseProto;
use crate::proto::device::DeviceProto;
use crate::proto::device::DevicesProto;
use crate::{ctx::Ctx, error::MyResult};
use crate::{model::controller::ModelController, protobuf::MyProtobuf};
use axum::{
    extract::{Path, State},
    routing::{delete, get},
    Router,
};
use axum_extra::protobuf::Protobuf;
use tracing::debug;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/devices", get(get_devices).post(add_device))
        .route("/device/:id", delete(delete_device).patch(patch_device))
        .with_state(mc)
}

async fn add_device(
    State(mc): State<ModelController>,
    ctx: Ctx,
    MyProtobuf(payload): MyProtobuf<AddDeviceRequestProto>,
) -> MyResult<Protobuf<BasicResponseProto>> {
    debug!(">>> {:<12} - add_device", "HANDLER");

    mc.add_device(ctx.user_id(), payload.0).await?;

    Ok(Protobuf(BasicResponseProto { success: true }))
}

async fn patch_device(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(device_id): Path<i32>,
    MyProtobuf(payload): MyProtobuf<AddDeviceRequestProto>,
) -> MyResult<Protobuf<BasicResponseProto>> {
    debug!(">>> {:<12} - patch_device", "HANDLER");
    let user_id = ctx.user_id();

    mc.does_user_own_device(user_id, device_id).await?;

    mc.patch_device(device_id, payload.0).await?;

    Ok(Protobuf(BasicResponseProto { success: true }))
}

async fn delete_device(
    State(mc): State<ModelController>,
    Path(device_id): Path<i32>,
    ctx: Ctx,
) -> MyResult<Protobuf<BasicResponseProto>> {
    debug!(">>> {:<12} - delete_device", "HANDLER");
    let user_id = ctx.user_id();

    mc.does_user_own_device(user_id, device_id).await?;

    mc.delete_device(device_id).await?;

    Ok(Protobuf(BasicResponseProto { success: true }))
}

async fn get_devices(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> MyResult<Protobuf<DevicesProto>> {
    debug!(">>> {:<12} - get_devices", "HANDLER");

    let devices_modell = mc.get_devices(ctx.user_id()).await?;

    let devices_proto = DevicesProto {
        devices: devices_modell.iter().map(DeviceProto::from).collect(),
    };

    Ok(Protobuf(devices_proto))
}
