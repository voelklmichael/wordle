pub const CHAR_COUNT: usize = 5;
pub type TargetWord = [char; CHAR_COUNT];

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct WordWithLink {
    pub link: String,
    pub word: TargetWord,
}

pub fn wordlist_german() -> Vec<WordWithLink> {
    // extracted via:
    // aspell dump master de_DE > wordlist
    let list = include_bytes!("../wordlists/words_wiktionary_de_checked");
    let list = std::str::from_utf8(list).unwrap();
    let list = list
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(link, word)| WordWithLink {
            link: link.to_string(),
            word: word.chars().collect::<Vec<_>>().try_into().unwrap(),
        })
        .collect::<Vec<_>>();
    list
}
