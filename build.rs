fn main() {
	println!("cargo:rerun-if-changed={{**/*{{.blp,.css}},migrations}}");
}
