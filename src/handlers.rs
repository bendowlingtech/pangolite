use axum::Json;
use crate::models::CreateLog;
pub async fn receive_logs(Json(payload): Json<CreateLog>) {

}