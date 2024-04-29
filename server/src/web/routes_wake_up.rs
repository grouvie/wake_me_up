use std::time::Duration;

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use axum_extra::protobuf::Protobuf;
use tokio::{sync::mpsc, time::sleep};

use crate::{
    ctx::Ctx, error::MyResult, model::controller::ModelController,
    proto::basic_response::BasicResponseProto,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/wake_up/:id", get(wake_up))
        .with_state(mc)
}

async fn wake_up(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(device_id): Path<i32>,
) -> MyResult<Protobuf<BasicResponseProto>> {
    let user_id = ctx.user_id();

    mc.does_user_own_device(user_id, device_id).await?;

    let (sender, mut receiver) = mpsc::channel(1);

    mc.wake_up_device(user_id, device_id, sender).await?;

    let result = tokio::select! {
        maybe_result = receiver.recv() => {
            if let Some(success) = maybe_result {
                println!("{success}");
                BasicResponseProto { success }
            } else {
                BasicResponseProto { success: false }
            }
        }
        _ = sleep(Duration::from_secs(5)) => {
            BasicResponseProto { success: false }
        }
    };

    Ok(Protobuf(result))
}
