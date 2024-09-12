struct Callbacks;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;

impl gcode::Callbacks for Callbacks {}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Expected `gcode-thumb [input.gcode] [output.png]`");
    }

    let input = &args[1];
    let output = &args[2];

    let mut images = Vec::new();

    // TODO: Only read the first N kB
    let content = std::fs::read_to_string(input).unwrap();

    let mut base = None;
    for line in gcode::full_parse_with_callbacks(&content, Callbacks) {
        for comment in line.comments() {
            if comment.value.contains("thumbnail begin") {
                // Start.
                base = Some(String::new());
            } else if comment.value.contains("thumbnail end") {
                let Some(base) = base.take() else {
                    continue;
                };

                let parsed = STANDARD.decode(base).unwrap();
                images.push(parsed);
            } else if base.is_some() && comment.value.starts_with("; ") {
                let comment = comment.value.strip_prefix("; ").unwrap();
                base.as_mut().unwrap().push_str(comment);
            }
            // TODO: Exit early the moment we see a non-comment
        }
    }

    // Pick the largest image (prusaslicer generates 2 thumbnails)
    let best_image = images.iter().max_by_key(|bytes| bytes.len()).unwrap();

    std::fs::write(output, best_image).unwrap();
}
