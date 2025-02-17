use std::rc::Rc;

use relm4::{adw, adw::prelude::*, component, gtk, prelude::*};

const TITLE: &str = "漢tracker";

use crate::db::Db;

use super::input as input_screen;
use super::result as result_screen;
use super::testing as testing_screen;

#[derive(Debug)]
pub struct Ht {
	view_stack: Rc<adw::ViewStack>,
	input_screen: AsyncController<input_screen::InputScreen>,
	testing_screen: AsyncController<testing_screen::TestingScreen>,
	result_screen: AsyncController<result_screen::ResultScreen>,
	db: Db,
}

#[component(pub, async)]
impl SimpleAsyncComponent for Ht {
	type Init = Db;
	type Input = NextScreen;
	type Output = ();

	view! {
		#[root]
		adw::Window::builder()
			.title(TITLE)
			.default_width(320)
			.default_height(240)
			.mnemonics_visible(false)
			.deletable(true)
			.resizable(true)
			.build() {
			#[wrap(Some)]
			set_content = &adw::ToolbarView {
				// TODO: make it disableable, i.e. when (features == "gnome")
				add_top_bar = &adw::HeaderBar::builder().build(),
				#[wrap(Some)]
				set_content = &adw::Clamp {
					set_css_classes: &["m-8"],
					set_overflow: gtk::Overflow::Visible,
					set_orientation: gtk::Orientation::Horizontal,
					set_valign: gtk::Align::Fill,
					set_halign: gtk::Align::Fill,
					set_unit: adw::LengthUnit::Px,
					set_maximum_size: 1920,
					set_tightening_threshold: 1000,

					#[local_ref]
					view_stack -> adw::ViewStack {
						set_hhomogeneous: false,
						add = model.input_screen.widget(),
						add = model.testing_screen.widget(),
						add = model.result_screen.widget(),
					},
				},
			},
		}
	}

	async fn init(
		db: Self::Init,
		widgets: Self::Root,
		sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let view_stack = Rc::new(adw::ViewStack::new());

		let input_screen = input_screen::InputScreen::builder()
			.launch(db.clone())
			.forward(sender.input_sender(), move |msg| match msg {
				input_screen::OutputMessage::ResumeTest => NextScreen::ResumeTest,
				input_screen::OutputMessage::NewTest => NextScreen::NewTest,
			});

		let testing_screen = testing_screen::TestingScreen::builder()
			.launch(db.clone())
			.forward(sender.input_sender(), |msg| match msg {
				testing_screen::OutputMessage::Finish(chars) => NextScreen::Results(chars),
			});

		let result_screen = result_screen::ResultScreen::builder().launch(()).forward(
			sender.input_sender(),
			|msg| match msg {
				result_screen::OutputMessage::StartOver => NextScreen::Input,
				result_screen::OutputMessage::Exit => NextScreen::Exit,
			},
		);

		let model = Self {
			view_stack,
			input_screen,
			testing_screen,
			result_screen,
			db,
		};

		let view_stack = &*model.view_stack;

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update(&mut self, message: Self::Input, _sender: AsyncComponentSender<Self>) {
		match message {
			NextScreen::Input => {
				self.db.set_test_progress(0).await.expect("failed");
				self.db.reset().await.expect("failed");
				let input_screen_widget = self.input_screen.widget();
				self.view_stack.set_visible_child(input_screen_widget);
			}
			NextScreen::NewTest => {
				let testing_screen_widget = self.testing_screen.widget();
				self.db.create_test_from_raw_text().await.expect("failed");
				let test = self
					.db
					.get_previous_test()
					.await
					.expect("failed")
					.expect("should exist");
				self.testing_screen
					.sender()
					.send(testing_screen::Message::StartTest(test))
					.expect("Shouldn't fail");
				self.view_stack.set_visible_child(testing_screen_widget);
			}
			NextScreen::ResumeTest => {
				let testing_screen_widget = self.testing_screen.widget();

				let test = self
					.db
					.get_previous_test()
					.await
					.expect("Query failed")
					.expect("No previous test");

				self.testing_screen
					.sender()
					.send(testing_screen::Message::StartTest(test))
					.expect("Shouldn't fail");
				self.view_stack.set_visible_child(testing_screen_widget);
			}
			NextScreen::Results(chars) => {
				let result_screen_widget = self.result_screen.widget();
				let text_to_test = self.db.get_text().await.expect("query failed");
				self.result_screen
					.sender()
					.send(result_screen::Message::ShowResults(text_to_test, chars))
					.expect("Shouldn't fail");
				self.view_stack.set_visible_child(result_screen_widget);
			}
			NextScreen::Exit => {
				self.db.clone().set_test_progress(0).await.expect("failed");
				relm4::main_adw_application().quit();
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum NextScreen {
	Input,
	NewTest,
	ResumeTest,
	Results(Vec<char>),
	Exit,
}
