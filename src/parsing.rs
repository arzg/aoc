pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    s.strip_prefix(starting_text)
        .ok_or_else(|| format!("expected {}", starting_text))
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "expected digits".to_string())
}

pub(crate) fn extract_lowercase(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |c| c.is_ascii_lowercase(),
        s,
        "expected lowercase letters".to_string(),
    )
}

pub(crate) fn extract_char(s: &str) -> Result<(&str, char), String> {
    let gen_error_msg = || "expected a character".to_string();

    let c = s.chars().next().ok_or_else(gen_error_msg)?;
    let (idx, _) = s.char_indices().nth(1).ok_or_else(gen_error_msg)?;

    Ok((&s[idx..], c))
}

fn take_while1(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_msg: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);

    if extracted.is_empty() {
        Err(error_msg)
    } else {
        Ok((remainder, extracted))
    }
}

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}
