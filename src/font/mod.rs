use font_kit::family_name::FamilyName;
use font_kit::handle::Handle;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;

pub const DEFAULT_FONT: &[u8] = include_bytes!("JetBrainsMonoNL-Regular.ttf");

pub fn load(name: &str) -> Option<&'static [u8]> {
    let mut families = Vec::new();
    for family in name.split(',') {
        let family = family.replace('\'', "");
        let family = family.trim();
        families.push(match family {
            "serif" => FamilyName::Serif,
            "sans-serif" => FamilyName::SansSerif,
            "monospace" => FamilyName::Monospace,
            "cursive" => FamilyName::Cursive,
            "fantasy" => FamilyName::Fantasy,
            _ => FamilyName::Title(family.to_string()),
        });
    }

    let properties = Properties::default();
    let source = SystemSource::new();
    let handle = source.select_best_match(&families, &properties);
    match handle {
        Ok(handle) => match handle {
            Handle::Path { ref path, .. } => {
                let contents = std::fs::read(path).unwrap();
                let contents = Box::new(contents);
                let contents = Box::leak(contents);

                Some(contents.as_slice())
            }
            _ => None,
        },
        Err(err) => {
            eprintln!("Failed to load fond {name}: {err}");
            None
        }
    }
}
