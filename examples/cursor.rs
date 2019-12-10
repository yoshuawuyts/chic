use chic::Error;
use std::io::Cursor;

fn main() {
    let cursor = Cursor::new(
        r#"This is an example
content of the slice
which will be annotated
with the list of annotations below.
"#,
    );

    let line = 1;
    let start = cursor.position() as usize;
    let end = cursor.get_ref().len() as usize;
    let code = cursor.into_inner().to_string();

    let msg = Error::new("expected type, found `x`".to_string())
        .error(line, start, end, code, "found `x`".to_string())
        .help("try using a foobs intead".to_string())
        .to_string();

    println!("{}", msg);
}
