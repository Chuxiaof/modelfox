use anyhow::Result;
use pinwheel::prelude::*;
use tangram_app_core::App;
use tangram_app_ui::topbar::{Topbar, TopbarAvatar};

pub struct AppLayoutInfo {
	pub topbar_avatar: Option<TopbarAvatar>,
}

#[derive(children, new)]
pub struct AppLayout {
	pub info: AppLayoutInfo,
	#[new(default)]
	pub children: Vec<Node>,
}

impl Component for AppLayout {
	fn into_node(self) -> Node {
		let topbar_avatar = self.info.topbar_avatar.map(|topbar_avatar| TopbarAvatar {
			avatar_url: topbar_avatar.avatar_url,
		});
		div()
			.class("app-layout-topbar-grid")
			.child(Topbar::new().topbar_avatar(topbar_avatar))
			.child(div().class("app-layout").child(self.children))
			.into_node()
	}
}

pub async fn app_layout_info(app: &App) -> Result<AppLayoutInfo> {
	let topbar_avatar = if app.options().auth_enabled() {
		Some(TopbarAvatar { avatar_url: None })
	} else {
		None
	};
	Ok(AppLayoutInfo { topbar_avatar })
}
