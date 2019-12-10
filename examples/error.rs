use chic::Error;

fn main() {
    let src = r#"This is an example
content of the slice
which will be annotated
with the list of annotations below.
"#;

    let msg = Error::new("expected type, found `x`")
        .error(260, 0, 12, src, "found `x`")
        .help("try using a foobs intead")
        .to_string();

    println!("{}", msg);
}
