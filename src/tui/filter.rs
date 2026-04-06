//! Fzf-style subsequence matching (case-insensitive) for filtering password names.

/// Returns `true` if every character of `query` appears in `candidate` in order.
///
/// Matching is **case-insensitive** for every query character, so e.g. `mypass`,
/// `myPass`, and `MYPASS` all match a stored name `MyPass`.
pub fn matches_subsequence(query: &str, candidate: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let mut q = query.chars();
    let mut nc = match q.next() {
        None => return true,
        Some(c) => c,
    };
    for tc in candidate.chars() {
        let ok = chars_equal_ignore_case(nc, tc);
        if ok {
            match q.next() {
                None => return true,
                Some(c) => nc = c,
            }
        }
    }
    false
}

fn chars_equal_ignore_case(a: char, b: char) -> bool {
    if a.is_ascii() && b.is_ascii() {
        a.eq_ignore_ascii_case(&b)
    } else {
        let al: String = a.to_lowercase().collect();
        let bl: String = b.to_lowercase().collect();
        al == bl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_matches_all() {
        assert!(matches_subsequence("", "anything"));
    }

    #[test]
    fn subsequence_basic() {
        assert!(matches_subsequence("gh", "github"));
        assert!(!matches_subsequence("ghx", "github"));
    }

    #[test]
    fn case_insensitive() {
        assert!(matches_subsequence("git", "GitHub"));
        assert!(matches_subsequence("hub", "GitHub"));
        assert!(matches_subsequence("HUB", "GitHub"));
    }

    #[test]
    fn mixed_case_query_matches_stored_name() {
        assert!(matches_subsequence("mypass", "MyPass"));
        assert!(matches_subsequence("myPass", "MyPass"));
        assert!(matches_subsequence("MYPASS", "MyPass"));
    }
}
