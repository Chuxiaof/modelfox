use modelfox_ui as ui;
use pinwheel::prelude::*;
use std::borrow::Cow;

pub struct Train;

impl Component for Train {
	fn into_node(self) -> Node {
		let title = div()
			.class("index-step-title")
			.child("Train a model on the command line.");
		let p1 = div()
			.class("index-step-text")
			.child("Train a machine learning model by running ")
			.child(ui::InlineCode::new("modelfox train"))
			.child(" with the path to a CSV file and the name of the column you want to predict.");
		let p2 = div()
			.class("index-step-text")
			.child("The CLI automatically transforms your data into features, trains a number of linear and gradient boosted decision tree models to predict the target column, and writes the best model to a ")
			.child(ui::InlineCode::new(".modelfox"))
			.child(" file. If you want more control, you can provide a config file.");
		let left = div().child(title).child(p1).child(br()).child(p2);
		let right = ui::Window::new()
			.child(ui::Code::new().code(Cow::Borrowed(include_str!("./train.txt"))));
		div()
			.class("index-step")
			.child(left)
			.child(right)
			.into_node()
	}
}
