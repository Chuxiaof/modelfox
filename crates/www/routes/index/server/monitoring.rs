use pinwheel::prelude::*;
use modelfox_ui as ui;

pub struct Monitoring;

impl Component for Monitoring {
	fn into_node(self) -> Node {
		let elixir = ui::doc!(
			r#"
				# Log the prediction.
				ModelFox.log_prediction(model, %ModelFox.LogPredictionArgs{
					identifier: id,
					options: predict_options,
					input: input,
					output: output,
				})

				# Later on, if we get an official diagnosis for the patient, log the true value.
				ModelFox.log_true_value(model, %ModelFox.LogTrueValueArgs{
					identifier: id,
					true_value: "Positive",
				})
			"#
		)
		.into();
		let go = ui::doc!(
			r#"
				// Log the prediction.
				err = model.LogPrediction(modelfox.LogPredictionArgs{
					Identifier: id,
					Input:      input,
					Options:    predictOptions,
					Output:     output,
				})
				if err != nil {
					log.Fatal(err)
				}

				// Later on, if we get an official diagnosis for the patient, log the true value.
				err = model.LogTrueValue(modelfox.LogTrueValueArgs{
					Identifier: id,
					TrueValue:  "Positive",
				})
				if err != nil {
					log.Fatal(err)
				}
			"#
		)
		.into();
		let javascript = ui::doc!(
			r#"
				// Log the prediction.
				model.logPrediction({
					identifier: id,
					input,
					options,
					output,
				})

				// Later on, if we get an official diagnosis for the patient, log the true value.
				model.logTrueValue({
					identifier: id,
					trueValue: "Positive",
				})
			"#
		)
		.into();
		let php = ui::doc!(
			r#"
				// Log the predicton
				$model->log_prediction(
					id,
					$input,
					$output,
					$options,
				);

				// Later on, if we get an official diagnosis for the patient, log the true value. Make sure to match the `identifier`.
				$model->log_true_value(
					id,
					'Positive',
				);
			"#
		).into();
		let python = ui::doc!(
			r#"
				# Log the prediction.
				model.log_prediction(
						identifier=id,
						input=input,
						output=output,
						options=predict_options,
				)

				# Later on, if we get an official diagnosis for the patient, log the true value.
				model.log_true_value(
						identifier=id,
						true_value="Positive",
				)
			"#
		)
		.into();
		let ruby = ui::doc!(
			r#"
				# Log the prediction.
				model.log_prediction(
					identifier: id,
					input: input,
					output: output,
					options: options
				)

				# Later on, if we get an official diagnosis for the patient, log the true value.
				model.log_true_value(
					identifier: id,
					true_value: 'Positive'
				)
			"#
		)
		.into();
		let rust = ui::doc!(
			r#"
				// Log the prediction.
				model.log_prediction(modelfox::LogPredictionArgs {
					identifier: id,
					input,
					options: Some(options),
					output,
				})?;

				// Later on, if we get an official diagnosis for the patient, log the true value.
				model.log_true_value(modelfox::LogTrueValueArgs {
					identifier: id,
					true_value: "Positive".into(),
				})?;
			"#
		)
		.into();
		let code_for_language = ui::highlight_code_for_language(ui::CodeForLanguage {
			elixir,
			go,
			javascript,
			php,
			python,
			ruby,
			rust,
		});
		let title = div()
			.class("index-step-title")
			.child("Monitor your models in production.");
		let p1 = div().class("index-step-text").child("Once your model is deployed, make sure that it performs as well in production as it did in training.");
		let p2 = div()
			.class("index-step-text")
			.child("Opt in to logging by calling ")
			.child(ui::InlineCode::new("logPrediction"))
			.child(".");
		let p3 = div()
			.class("index-step-text")
			.child("Later on, if you find out the true value for a prediction, call ")
			.child(ui::InlineCode::new("logTrueValue"))
			.child(".");
		let left = div()
			.child(title)
			.child(p1)
			.child(br())
			.child(p2)
			.child(br())
			.child(p3);
		let right =
			ui::Window::new().child(ui::CodeSelect::new(code_for_language).line_numbers(true));
		div()
			.class("index-step")
			.child(left)
			.child(right)
			.into_node()
	}
}
