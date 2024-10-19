use std::fmt::Display;

pub fn join_humanized<I, T>(strings: I) -> String
where
    I: IntoIterator<Item = T>,
    I::IntoIter: DoubleEndedIterator,
    T: Display,
{
    let mut iter = strings.into_iter();
    let first = iter.next();
    let last = iter.next_back();
    match (first, last) {
        (None, _) => String::new(),
        (Some(first), None) => format!("{}", first),
        (Some(first), Some(last)) => {
            let separated = iter.map(|x| format!(", {x}")).collect::<String>();
            format!("{first}{separated} and {last}")
        }
    }
}
