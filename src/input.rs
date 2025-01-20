use relm4::adw::prelude::*;
use relm4::component;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct InputScreen {
	buf: gtk::TextBuffer,
	text: String,
}

#[component(pub)]
impl SimpleComponent for InputScreen {
	type Init = ();
	type Input = Message;
	type Output = String;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			gtk::Label {
				set_css_classes: &["heading"],
				set_label: "Input Characters",
			},
			gtk::Label {
				set_css_classes:  &["mb-2"],
				set_label: "These are the characters which you will be tested on:",
			},
			gtk::Frame {
				gtk::ScrolledWindow {
					set_hexpand: true,
					set_vexpand: true,
					set_width_request: 416,
					set_height_request: 180,
					gtk::TextView {
						set_css_classes: &["m-2"],
						set_hexpand: true,
						set_wrap_mode: gtk::WrapMode::WordChar,
						#[wrap(Some)]
						set_buffer: buf = &gtk::TextBuffer {
							set_text: &model.text,
							connect_end_user_action[sender] => move |buf| {
								let text = get_full_text_from_buffer(&buf);
								sender.input(Message::UpdateText(text));
							}
						},
					},
				},
			},
			gtk::Button {
				set_css_classes: &["suggested-action", "pill", "m-8"],
				set_label: "Test me",
				set_hexpand: false,
				connect_clicked[sender] => move |_| {
					sender.input(Message::MoveToTestSection);
				}
			}
		}
	}

	fn init(
		chars: Self::Init,
		widgets: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let buf = gtk::TextBuffer::new(None);

		let text = get_full_text_from_buffer(&buf);
		let model = Self { buf, text };

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Message::MoveToTestSection => sender.output(self.text.clone()).expect("Shouldn't fail"),
			Message::UpdateText(s) => self.text = s,
		}
	}
}

fn get_full_text_from_buffer(buf: &gtk::TextBuffer) -> String {
	let (start, end) = buf.bounds();
	buf.text(&start, &end, true).to_string()
}

#[derive(Debug, Clone)]
pub enum Message {
	MoveToTestSection,
	UpdateText(String),
}
