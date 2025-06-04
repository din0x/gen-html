pub(crate) fn escape_html(s: &str) -> String {
    let mut buf = String::new();

    for ch in s.chars() {
        match ch {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            ch => buf.push(ch),
        }
    }

    buf
}
