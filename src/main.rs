use base64::{engine::general_purpose::STANDARD, Engine as _};

struct Callbacks;
impl gcode::Callbacks for Callbacks {}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Expected `gcode-thumb [input.gcode] [output.png]`");
    }

    let input = &args[1];
    let output = &args[2];

    let file = std::fs::File::open(input).unwrap();
    let mmap = unsafe { memmap::Mmap::map(&file).unwrap() };

    // TODO: Only validate what we actually read? So the OS doesn't need to read the entire file
    // for the memmap.
    // What if the file is not actually utf8? _In this script_ we don't rely on utf8 guarantees.
    let content = unsafe { std::str::from_utf8_unchecked(&mmap) };

    // Accumulate all the potential thumbnails, then pick the best one.
    let mut images = Vec::new();

    // Accumulate lines of base64-encoded data
    let mut base = None;

    for line in gcode::full_parse_with_callbacks(content, Callbacks) {
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
        }
        // Exit early the moment we see a non-comment
        if !line.gcodes().is_empty() {
            break;
        }
    }

    // Pick the largest image (prusaslicer generates 2 thumbnails)
    let best_image = images.iter().max_by_key(|bytes| bytes.len()).unwrap();

    std::fs::write(output, best_image).unwrap();
}
