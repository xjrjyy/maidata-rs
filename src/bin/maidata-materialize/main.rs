fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args()
        .nth(1)
        .expect("usage: $0 <input> <output> <offset?>");
    let output = std::env::args()
        .nth(2)
        .expect("usage: $0 <input> <output> <offset?>");
    let offset = std::env::args()
        .nth(3)
        .map(|x| x.parse::<f32>().expect("parsing offset failed"))
        .unwrap_or(0.0);

    let content = read_file(input);
    let (insns, state) = maidata::container::parse_maidata_insns(&content);

    let mut mcx = maidata::materialize::MaterializationContext::with_offset(offset);
    let notes = mcx.materialize_insns(insns.iter());

    let messages_to_value = |messages: &[maidata::ParseMessage]| -> serde_json::Value {
        messages
            .iter()
            .map(|msg| {
                serde_json::json!({
                    "span": {
                        "start_line": msg.span.line,
                        "start_col": msg.span.col,
                        "end_line": msg.span.end_line,
                        "end_col": msg.span.end_col,
                    },
                    "message": msg.message,
                })
            })
            .collect()
    };

    let json = serde_json::json!({
        "chart": notes,
        "warnings": messages_to_value(&state.warnings),
        "errors": messages_to_value(&state.errors),
    });
    let json_str = serde_json::to_string_pretty(&json).expect("serializing json failed");
    std::fs::write(output, json_str).expect("writing json file failed");

    Ok(())
}

fn read_file<P: AsRef<std::path::Path>>(path: P) -> String {
    let content = std::fs::read(path.as_ref()).expect("file reading failed");
    String::from_utf8(content).expect("decoding file content as utf-8 failed")
}
