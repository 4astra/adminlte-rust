use tide::{Request, Result};
use crate::models::admin_dashboard::AdminDashboard;


pub async fn admin(_req: Request<()>) -> Result {
    Ok(AdminDashboard::new("AdminLTE 3 | Rust").into())
}