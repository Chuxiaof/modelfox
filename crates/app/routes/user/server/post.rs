use anyhow::Result;
use modelfox_app_context::Context;
use modelfox_app_core::{
	error::{bad_request, not_found, service_unavailable, unauthorized},
	user::{authorize_normal_user, NormalUser},
};
use std::{borrow::BorrowMut, sync::Arc};

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "action")]
enum Action {
	#[serde(rename = "logout")]
	Logout,
}

pub async fn post(request: &mut http::Request<hyper::Body>) -> Result<http::Response<hyper::Body>> {
	let context = Arc::clone(request.extensions().get::<Arc<Context>>().unwrap());
	let app = &context.app;
	if !app.options().auth_enabled() {
		return Ok(not_found());
	}
	let mut db = match app.begin_transaction().await {
		Ok(db) => db,
		Err(_) => return Ok(service_unavailable()),
	};
	let user = match authorize_normal_user(request, &mut db).await? {
		Ok(user) => user,
		Err(_) => return Ok(unauthorized()),
	};
	let data = match hyper::body::to_bytes(request.body_mut()).await {
		Ok(data) => data,
		Err(_) => return Ok(bad_request()),
	};
	let action: Action = match serde_urlencoded::from_bytes(&data) {
		Ok(action) => action,
		Err(_) => return Ok(bad_request()),
	};
	let response = match action {
		Action::Logout => logout(&user, &mut db).await?,
	};
	app.commit_transaction(db).await?;
	Ok(response)
}

async fn logout(
	user: &NormalUser,
	txn: &mut sqlx::Transaction<'_, sqlx::Any>,
) -> Result<http::Response<hyper::Body>> {
	sqlx::query(
		"
			delete from tokens where token = $1
		",
	)
	.bind(&user.token)
	.execute(txn.borrow_mut())
	.await?;
	let response = http::Response::builder()
		.status(http::StatusCode::SEE_OTHER)
		.header(http::header::LOCATION, "/login")
		.header(http::header::SET_COOKIE, "auth=; Path=/; Max-Age=0")
		.body(hyper::Body::empty())
		.unwrap();
	Ok(response)
}
