use std::borrow::Cow;

/// Joins the version parts with `.`, stopping at the first part which is missing.
pub(crate) fn join_version<'a>(parts: &[Option<&'a str>]) -> Option<Cow<'a, str>> {
    // A version never has a hole in it, so the first missing part ends it.
    let end = parts.iter().position(Option::is_none).unwrap_or(parts.len());
    let parts = &parts[..end];

    match parts {
        [] => None,
        // A version which only has a major part needs no allocation at all.
        [Some(major)] => Some(Cow::from(*major)),
        _ => {
            // Every part but the last one is followed by a dot.
            let capacity = parts.iter().flatten().map(|part| part.len() + 1).sum::<usize>() - 1;

            let mut version = String::with_capacity(capacity);

            for (index, part) in parts.iter().flatten().enumerate() {
                if index > 0 {
                    version.push('.');
                }

                version.push_str(part);
            }

            Some(Cow::from(version))
        },
    }
}
