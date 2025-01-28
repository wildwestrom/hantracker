use lib::is_chinese_character;
use lib::load_kanjidic::bootstrap_dict;
use lib::load_kanjidic::Dict2;
use relm4::adw::prelude::*;
use relm4::component;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug, Clone)]
pub enum Message {
	UpdateText(String),
	NewTest,
}

#[derive(Debug, Clone)]
pub enum OutputMessage {
	ResumeTest,
	NewTest(String),
}

#[derive(Debug)]
pub struct InputScreen {
	text: String,
	chinese_character_exists: bool,
	dict: Dict2,
	save_exists: bool,
}

#[component(pub)]
impl SimpleComponent for InputScreen {
	type Init = ();
	type Input = Message;
	type Output = OutputMessage;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			gtk::Box {
				set_css_classes: &["card", "p-2"],
				set_orientation: gtk::Orientation::Horizontal,
				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					gtk::Label {
						set_css_classes: &["heading"],
						set_label: "Input Characters",
					},
					gtk::Label {
						set_css_classes:  &["m-2"],
						set_label: "These are the characters which you will be tested on:",
					},
					gtk::ScrolledWindow {
						set_min_content_width: 200,
						set_min_content_height: 180,
						set_css_classes: &["mr-2", "card", "view"],
						gtk::TextView {
							set_css_classes: &["m-2"],
							set_hexpand: true,
							set_vexpand: true,
							set_wrap_mode: gtk::WrapMode::WordChar,
							#[wrap(Some)]
							set_buffer: buf = &gtk::TextBuffer {
								#[watch]
								set_text: &model.text,
								connect_end_user_action[sender] => move |buf| {
									let text = get_full_text_from_buffer(buf);
									sender.input(Message::UpdateText(text));
								}
							},
						},
					},
					gtk::Label {
						set_css_classes:  &["m-2"],
						#[watch]
						set_label: &format!("{} Characters", {
							#[allow(clippy::needless_borrow, reason = "This is a bug in clippy, the type system demands a reference")]
							let text = get_full_text_from_buffer(&buf);
							text.trim().chars().count()
						}),
					},
				},
				adw::PreferencesGroup {
					set_title: "Presets",
					set_width_request: 320,
					set_hexpand: false,
					set_css_classes: &["card", "boxed-list-separate", "p-4", "view", "ml-2"],
					gtk::Button {
						set_css_classes: &["my-2", "py-2"],
						set_label: "한문 교육용 기초 한자 1800",
						connect_clicked => {
							Message::UpdateText(include_str!("hanmun_hanja_1800.txt").to_string())
						},
					},
					gtk::Button {
						set_css_classes: &["my-2", "py-2"],
						set_label: "日本語能力試験",
						connect_clicked => {
							Message::UpdateText(jlpt.clone())
						},
					},
					gtk::Button {
						set_css_classes: &["my-2", "py-2"],
						set_label: "常用漢字",
						connect_clicked => {
							Message::UpdateText(joyo.clone())
						},
					},
					gtk::Button {
						set_css_classes: &["my-2", "py-2"],
						set_label: "教育漢字",
						connect_clicked => {
							Message::UpdateText(kyoiku.clone())
						},
					},
					gtk::Button {
						set_css_classes: &["my-2", "py-2"],
						set_label: "通用规范汉字表: 一级字表",
						connect_clicked => {
							Message::UpdateText(include_str!("common_cn_tier_1.txt").trim().to_string())
						},
					}
				}
			},
			gtk::Revealer {
				#[watch]
				set_reveal_child: model.save_exists || model.chinese_character_exists,

				match (model.save_exists, model.chinese_character_exists) {
					(true, false) => gtk::Button {
						set_css_classes: &["suggested-action", "pill", "mt-8", "mx-8"],
						set_label: "Resume Test",
						set_hexpand: false,
						connect_clicked[sender] => move |_| {
							sender.output(OutputMessage::ResumeTest).expect("This shouldn't fail");
						}
					},
					(true, true) => gtk::Box {
						set_hexpand: true,
						set_homogeneous: true,
						gtk::Button {
							set_css_classes: &["pill", "mt-8", "mx-8"],
							set_label: "New Test",
							set_hexpand: false,
							connect_clicked[sender] => move |_| {
								sender.input(Message::NewTest);
							}
						},
						gtk::Button {
							set_css_classes: &["suggested-action", "pill", "mt-8", "mx-8"],
							set_label: "Resume Test",
							set_hexpand: false,
							connect_clicked[sender] => move |_| {
								sender.output(OutputMessage::ResumeTest).expect("This shouldn't fail");
							}
						},
					},
					(false, true) => gtk::Button {
						set_css_classes: &["suggested-action", "pill", "mt-8", "mx-8"],
						set_label: "New Test",
						set_hexpand: false,
						connect_clicked[sender] => move |_| {
							sender.input(Message::NewTest);
						}
					},
					(false, false) => gtk::Separator {
						set_css_classes: &["spacer"],
					}
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		widgets: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			text: String::new(),
			chinese_character_exists: false,
			dict: bootstrap_dict().unwrap(),
			save_exists: true,
		};

		let dict = &model.dict;
		let jlpt: String = dict.get_all_jlpt().into_iter().collect();
		let joyo: String = dict.get_all_joyo().into_iter().collect();
		let kyoiku: String = dict.get_all_kyoiku().into_iter().collect();

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Message::UpdateText(s) => {
				self.text = s;
				self.chinese_character_exists =
					self.text.chars().filter(is_chinese_character).count() > 0;
			}
			Message::NewTest => sender
				.output(OutputMessage::NewTest(self.text.clone()))
				.expect("Shouldn't fail"),
		}
	}
}

fn get_full_text_from_buffer(buf: &gtk::TextBuffer) -> String {
	let (start, end) = buf.bounds();
	buf.text(&start, &end, true).to_string()
}
