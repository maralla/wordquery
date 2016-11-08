pub fn is_subseq(src: &str, target: &str) -> (i32, bool) {
    let mut score = 0;
    let mut src_iter = src.chars();
    let mut ch = match src_iter.next() {
        Some(e) => e,
        None => return (0, false),
    };

    for (i, c) in target.char_indices() {
        if c.len_utf8() != ch.len_utf8() {
            continue;
        }

        if c.to_lowercase()
            .zip(ch.to_lowercase())
            .filter(|&(s, t)| s != t)
            .next()
            .is_none() {
            match i {
                0 => score = -999,
                _ => score += i as i32,
            }

            match src_iter.next() {
                Some(c) => ch = c,
                None => return (score, true),
            }
        }
    }
    (0, false)
}

#[test]
fn test_subseq() {
    assert_eq!(is_subseq("wop", "world"), (0, false));
    assert_eq!(is_subseq("", "world"), (0, false));

    assert_eq!(is_subseq("w", "world"), (-999, true));
    assert_eq!(is_subseq("wld", "world"), (-992, true));
    assert_eq!(is_subseq("d", "world"), (4, true));
    assert_eq!(is_subseq("od", "world"), (5, true));
    assert_eq!(is_subseq("Od", "world"), (5, true));
}
