pub const CHAR_COUNT: usize = 5;
pub type TargetWord = [char; CHAR_COUNT];

pub fn wordlist_german() -> Vec<TargetWord> {
    // extracted via:
    // aspell dump master de_DE > wordlist
    let list = include_bytes!("../wordlists/wordlist_german");
    let list = std::str::from_utf8(list).unwrap();
    let list = list
        .lines()
        .map(|x| x.split_once('/').map(|x| x.0).unwrap_or(x))
        .filter(|x| x.len() == CHAR_COUNT && x.is_ascii())
        .map(|x| x.to_lowercase())
        .map(|x| x.chars().collect::<Vec<_>>().try_into().unwrap())
        .collect::<Vec<_>>();
    list
}
