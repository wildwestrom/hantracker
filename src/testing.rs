use relm4::adw::prelude::*;
use relm4::component;
use relm4::gtk;
use relm4::prelude::*;

use crate::db::Db;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Test {
	pub(crate) char: char,
	pub(crate) recalled: Recalled,
}

#[derive(Debug)]
pub struct TestingScreen {
	tests: Vec<Test>,
	current_test: usize,
	db: Db,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Recalled {
	Unknown,
	Known,
}

#[component(pub, async)]
impl SimpleAsyncComponent for TestingScreen {
	type Init = Db;
	type Input = Message;
	type Output = OutputMessage;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			gtk::Label {
				set_css_classes: &["heading"],
				set_label: "Test your knowledge",
			},
			gtk::Label {
				set_css_classes: &["mb-2"],
				#[watch]
				set_label: &format!("Do you know the meaning of this character?\n{} of {}",
					model.current_test + 1,
					model.tests.len()
				),
			},
			gtk::Box {
				set_homogeneous: true,

				gtk::Revealer {
					#[watch]
					set_reveal_child: model.current_test > 0,
					set_transition_type: gtk::RevealerTransitionType::SwingLeft,

					gtk::Frame {
						set_margin_horizontal: 16,
						set_overflow: gtk::Overflow::Visible,
						gtk::Box {
							set_css_classes: &["p-8"],
							set_align: gtk::Align::Center,
							set_orientation: gtk::Orientation::Vertical,
							gtk::Label {
								set_css_classes: &["heading"],
								set_label: "Previous Character",
							},
							gtk::Label {
								set_css_classes: &["text-5xl"],
								#[watch]
								set_label: &model.tests.get(model.current_test.saturating_sub(1)).map_or_else(|| "No previous character.".into(), |q| q.char.to_string())
							},
							gtk::Button {
								set_css_classes: &["mt-2"],
								set_label: "Go back",
								connect_clicked => {
									Message::GoBack
								}
							}
						}
					}
				},
				gtk::Label {
					set_css_classes: &["card", "text-9xl", "p-8"],
					#[watch]
					set_label: &model.tests.get(model.current_test).map_or_else(|| "nil".into(), |q| q.char.to_string())
				},
				gtk::Separator {
					set_css_classes: &["spacer"],
				},
			},
			gtk::Box {
				set_css_classes: &["m-8", "linked"],
				set_orientation: gtk::Orientation::Horizontal,
				set_hexpand: true,
				set_halign: gtk::Align::Fill,
				set_homogeneous: true,
				gtk::Button {
					set_css_classes: &["error"],
					set_label: "I don't know",
					connect_clicked => Message::Answer(Recalled::Unknown),
				},
				gtk::Button {
					set_css_classes: &["success"],
					set_label: "I know it",
					connect_clicked => Message::Answer(Recalled::Known),
				}
			}
		}
	}

	async fn init(
		db: Self::Init,
		widgets: Self::Root,
		_sender: AsyncComponentSender<Self>,
	) -> AsyncComponentParts<Self> {
		let model = Self {
			tests: Vec::new(),
			db: db.clone(),
			current_test: usize::try_from(db.get_test_progress().await.expect("query failed"))
				.expect("Integer conversion failed"),
		};

		let widgets = view_output!();

		AsyncComponentParts { model, widgets }
	}

	async fn update(&mut self, message: Self::Input, sender: AsyncComponentSender<Self>) {
		match message {
			Message::StartTest(tests) => {
				self.tests = tests;
			}
			Message::Finish(known_chars) => {
				self.current_test = 0;
				sender
					.output(OutputMessage::Finish(known_chars))
					.expect("sending finished failed");
			}
			Message::GoBack => {
				self.current_test -= 1;
				self.db
					.set_test_progress(self.current_test)
					.await
					.expect("failed");
			}
			Message::Answer(recalled) => {
				let finish = |tests: &[Test]| {
					sender.input(Message::Finish(
						tests
							.iter()
							.filter_map(|c| match c.recalled {
								Recalled::Known => Some(c.char),
								Recalled::Unknown => None,
							})
							.collect(),
					));
				};
				if let Some(c) = self.tests.get_mut(self.current_test) {
					c.recalled = recalled;
					self.current_test += 1;
					self.db
						.set_test_progress(self.current_test)
						.await
						.expect("failed");
					if self.current_test == self.tests.len() {
						finish(&self.tests);
					}
				} else {
					finish(&self.tests);
				}
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	StartTest(Vec<Test>),
	Finish(Vec<char>),
	Answer(Recalled),
	GoBack,
}

#[derive(Debug, Clone)]
pub enum OutputMessage {
	Finish(Vec<char>),
}
