use std::rc::Rc;

use relm4::adw::prelude::*;
use relm4::component;
use relm4::gtk;
use relm4::prelude::*;

#[derive(Debug)]
pub struct ResultScreen {
	buf: gtk::TextBuffer,
}

#[component(pub)]
impl SimpleComponent for ResultScreen {
	type Init = ();
	type Input = Message;
	type Output = Message;

	view! {
		#[root]
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_hexpand: true,
			gtk::Label {
				set_css_classes: &["heading"],
				set_label: "Result",
			},
			gtk::Label {
				set_css_classes:  &["mb-2"],
				set_label: "The ones in blue are the ones you know.",
			},
			gtk::Frame {
				gtk::ScrolledWindow {
				set_hexpand: true,
				set_vexpand: true,
				gtk::TextView {
					set_css_classes: &["m-2"],
					set_hexpand: true,
					set_wrap_mode: gtk::WrapMode::WordChar,
					set_editable: false,
					set_buffer: Some(&model.buf),
				},
				set_width_request: 416,
				set_height_request: 180,
				}
			},
			gtk::Button {
				set_css_classes: &["suggested-action", "pill", "m-8"],
				set_label: "Start Over",
				set_hexpand: false,
				connect_clicked[sender] => move |_| {
					sender.input(Message::StartOver);
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		widgets: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let tag_table = Rc::new(gtk::TextTagTable::new());
		let buf = gtk::TextBuffer::new(Some(&tag_table));
		let model = Self { buf };

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Message::StartOver => sender.output(Message::StartOver).unwrap(),
			Message::ShowResults(initial_input, known_chars) => {
				self.buf.set_text(&initial_input);
				let known_tag = self
					.buf
					.create_tag(None, &[("foreground", &"blue")])
					.unwrap();
				for cjk_c in known_chars {
					let mut txt_iter = self.buf.start_iter();
					let end_iter = self.buf.end_iter();
					while let Some((match_start, match_end)) = txt_iter.forward_search(
						&cjk_c.to_string(),
						gtk::TextSearchFlags::CASE_INSENSITIVE,
						Some(&end_iter),
					) {
						self.buf.apply_tag(&known_tag, &match_start, &match_end);
						txt_iter = match_end;
					}
				}
			}
		}
	}
}

#[derive(Debug, Clone)]
pub enum Message {
	StartOver,
	ShowResults(String, Vec<char>),
}
