use crate::page::{Page, PrecisionRecallPoint};
use anyhow::{bail, Result};
use pinwheel::prelude::*;
use std::sync::Arc;
use tangram_app_context::Context;
use tangram_app_core::{
	error::{bad_request, not_found, redirect_to_login, service_unavailable},
	model::get_model_bytes,
	path_components,
	user::{authorize_user, authorize_user_for_model},
};
use tangram_app_layouts::model_layout::{model_layout_info, ModelNavItem};
use tangram_id::Id;

pub async fn get(request: &mut http::Request<hyper::Body>) -> Result<http::Response<hyper::Body>> {
	let context = Arc::clone(request.extensions().get::<Arc<Context>>().unwrap());
	let app_state = &context.app.state;
	let model_id = if let ["repos", _, "models", model_id, "training_metrics", "precision_recall"] =
		path_components(request).as_slice()
	{
		model_id.to_owned()
	} else {
		bail!("unexpected path");
	};
	let mut db = match app_state.database_pool.begin().await {
		Ok(db) => db,
		Err(_) => return Ok(service_unavailable()),
	};
	let user = match authorize_user(request, &mut db, app_state.options.auth_enabled()).await? {
		Ok(user) => user,
		Err(_) => return Ok(redirect_to_login()),
	};
	let model_id: Id = match model_id.parse() {
		Ok(model_id) => model_id,
		Err(_) => return Ok(bad_request()),
	};
	if !authorize_user_for_model(&mut db, &user, model_id).await? {
		return Ok(not_found());
	}
	let bytes = get_model_bytes(&app_state.storage, model_id).await?;
	let model = tangram_model::from_bytes(&bytes)?;
	let model = match model.inner() {
		tangram_model::ModelInnerReader::BinaryClassifier(binary_classifier) => {
			binary_classifier.read()
		}
		_ => return Ok(bad_request()),
	};
	let test_metrics = model.test_metrics();
	let precision_recall_curve_series = test_metrics
		.thresholds()
		.iter()
		.filter_map(
			|class_metrics| match (class_metrics.precision(), class_metrics.recall()) {
				(Some(precision), Some(recall)) => Some(PrecisionRecallPoint {
					precision,
					recall,
					threshold: class_metrics.threshold(),
				}),
				_ => None,
			},
		)
		.collect();
	let model_layout_info =
		model_layout_info(&mut db, app_state, model_id, ModelNavItem::TrainingMetrics).await?;
	let page = Page {
		class: model.positive_class().to_owned(),
		precision_recall_curve_series,
		id: model_id.to_string(),
		model_layout_info,
	};
	let html = html(page);
	let response = http::Response::builder()
		.status(http::StatusCode::OK)
		.body(hyper::Body::from(html))
		.unwrap();
	Ok(response)
}
