use anyhow::Result;

use relm4::component;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

gtk_blueprint::gen_blp_map!("");

const APP_ID: &str = "xyz.westrom.hantracker";
const TITLE: &str = "漢tracker";

#[derive(Debug)]
struct Ht {
	unknown: Vec<char>,
	known: Vec<char>,
	current_section: Section,
	current_test: usize,
	editor: String,
}

impl Default for Ht {
	fn default() -> Self {
		Self {
			unknown: vec![],
			known: vec![],
			current_section: Section::TextInput,
			current_test: 0,
			editor: String::new(),
		}
	}
}

impl SimpleComponent for Ht {
	type Init = lib::load_kanjidic::Dict2;
	type Input = ();
	type Output = ();
	type Root = gtk::ApplicationWindow;
	type Widgets = gtk::Window;

	fn init_root() -> Self::Root {
		gtk::ApplicationWindow::builder().build()
	}

	fn init(
		dict: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let joyo_kanji = dict.get_all_joyo();
		let joyo_string = joyo_kanji.join(" ");

		let mut model = Ht::default();
		model.editor = joyo_string;

		let window = gtk::Builder::from_string(gtk_blueprint::get_blp!("ui/main.blp"))
			.object::<gtk::Window>(TITLE);

		root.set_child(window.as_ref());
		let widgets = root.into();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		todo!()
	}
}

#[derive(Debug, Clone)]
enum Message {
	LoadTestChars,
	NextSection(Section),
	Known(usize),
	Unknown(usize),
	Edited(String),
}

#[derive(Debug, Clone, Default)]
enum Section {
	#[default]
	TextInput,
	Test,
	Results,
}

fn main() -> Result<()> {
	let dict = lib::load_kanjidic::bootstrap_dict()
		.expect("Something failed when starting the dictionary");

	let app = RelmApp::new(APP_ID);
	app.run::<Ht>(dict);
	Ok(())
}

fn is_cjk(c: &char) -> bool {
	let c = *c as u32;
	c >= 0x3400 && c <= 0x4DBF || // CJK Unified Ideographs Extension A
	c >= 0x4E00 && c <= 0x9FFF || // CJK Unified Ideographs
    c >= 0x20000 && c <= 0x2A6D || // CJK Unified Ideographs Extension B
    c >= 0x2A700 && c <= 0x2B73 || // CJK Unified Ideographs Extension C
    c >= 0x2B740 && c <= 0x2B81 || // CJK Unified Ideographs Extension D
    c >= 0x2B820 && c <= 0x2CEA || // CJK Unified Ideographs Extension E
    c >= 0x2CEB0 && c <= 0x2EBE || // CJK Unified Ideographs Extension F
    c >= 0x2EBF0 && c <= 0x2EE5 || // CJK Unified Ideographs Extension I
    c >= 0x2F800 && c <= 0x2FA1 || // CJK Compatibility Ideographs Supplement
    c >= 0x30000 && c <= 0x3134 || // CJK Unified Ideographs Extension G
    c >= 0x31350 && c <= 0x323A // CJK Unified Ideographs Extension H
}

// fn update(state: &mut Ht, message: Message) {
// 	match message {
// 		Message::LoadTestChars => {
// 			state.unknown = state.editor.chars().filter(is_cjk).collect();
// 			state.current_section = Section::Test;
// 		}
// 		Message::NextSection(sec) => state.current_section = sec,
// 		Message::Known(..) => todo!(),
// 		Message::Unknown(..) => todo!(),
// 		Message::Edited(text) => state.editor = text,
// 	}
// }

// fn view(state: &Ht) -> Element<Message> {
// 	container(match state.current_section {
// 		Section::TextInput => column(vec![
// 			text_input(
// 				"This is where you insert the characters you wish to test yourself on.",
// 				&state.editor,
// 			)
// 			.on_input(|new_text| Message::Edited(new_text))
// 			.on_submit(Message::NextSection(Section::Test))
// 			.into(),
// 			button("Next section")
// 				.on_press(Message::LoadTestChars)
// 				.into(),
// 		])
// 		.spacing(16)
// 		.padding(16),
// 		Section::Test => column(if let Some(c) = state.unknown.get(state.current_test) {
// 			vec![
// 				text(c.to_string()).size(36).align_x(Center).into(),
// 				button("I know it").into(),
// 				button("I don't know").into(),
// 			]
// 		} else {
// 			vec![button("See results").into()]
// 		})
// 		.into(),
// 		Section::Results => todo!(),
// 	})
// 	.into()
// }
