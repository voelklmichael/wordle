fn main() {
    let words = String::from_utf8(std::fs::read("target/words_wiktionary_de").unwrap())
        .unwrap()
        .lines()
        .map(|x| {
            let (link, word) = x.split_once("\" title=\"").unwrap();
            let link = link.strip_prefix("/wiki/").unwrap();
            Line {
                word: word.to_string(), //.to_ascii_uppercase(),
                link: link.to_string(),
            }
        })
        .filter(|x| x.word.len() <= 5)
        .filter(|x| x.word.chars().all(|c| c.is_ascii_alphabetic()))
        .filter(|x| x.word.len() == 5)
        .collect::<Vec<_>>();
    std::fs::write(
        "target/words_wiktionary_de.json",
        serde_json::to_string(&words).unwrap(),
    )
    .unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Line {
    word: String,
    link: String,
}
