//! get emoji from description
use std::collections::HashMap;

pub(crate) fn determine_emoji(description: &str) -> &str {
    let emoji_map: HashMap<&str, &str> = [
        ("grocery", "🛒"),
        ("work", "💼"),
        ("study", "📚"),
        ("exercise", "🏋️‍♂️"),
    ]
    .iter()
    .cloned()
    .collect();

    for (keyword, emoji) in emoji_map {
        if description.contains(keyword) {
            return emoji;
        }
    }

    "🤷"
}
