use core::panic;
use std::collections::HashSet;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Line {
    word: String,
    link: String,
}
fn main() {
    std::fs::create_dir_all("target/cache_words").unwrap();
    let words: Vec<Line> =
        serde_json::from_slice(&std::fs::read("target/words_wiktionary_de.json").unwrap()).unwrap();
    dbg!(words.len());

    let client = reqwest::blocking::Client::new();
    let base_url = "https://de.wiktionary.org/wiki/";
    let mut to_keep = Vec::new();
    let mut sublists = HashSet::new();
    for word in words.into_iter() {
        dbg!(&word.word);
        let body = {
            let path_cache = format!("target/cache_words/{}.html", word.word);
            if std::fs::exists(&path_cache).unwrap() {
                String::from_utf8(std::fs::read(&path_cache).unwrap()).unwrap()
            } else {
                let url_full: String = format!("{base_url}{}", word.link);
                let res = client.get(url_full).send().unwrap();
                let body = res.text().unwrap();
                std::fs::write(&path_cache, &body).unwrap();
                body
            }
        };

        let start = format!("<li id=\"toc-{}_", word.word);
        let entry = {
            let mut possibles = body
                .split(&start)
                .filter_map(|x| x.strip_prefix("(Deutsch)\""))
                .collect::<Vec<_>>();
            let entry = match possibles.len() {
                0 => {
                    dbg!("No german available: skip", &word.word);
                    continue;
                }
                1 => possibles.pop().unwrap(),
                _ => {
                    panic!()
                }
            };
            entry
        };

        let sublist = {
            let Some(sublist) = entry
                .split_once(&format!("<ul id=\"toc-{}_(Deutsch)-sublist\"", word.word))
                .and_then(|(_, x)| x.split_once("<li id=\"toc-"))
                .and_then(|(_, x)| x.split_once("\""))
                .map(|x| x.0)
            else {
                dbg!("No sublist", &word.word);
                continue;
            };
            sublist
        };
        let skiplist = [
            "Abkürzung",
            "Eigenname",
            "Grußformel",
            "Nachname",
            "Substantiv®",
            "Toponym",
            "Vorname",
        ];
        if skiplist.iter().any(|x| sublist.contains(x)) {
            continue;
        }
        if sublist == "Substantiv,_m,_Katze" {
            dbg!("katze", &word);
        }
        dbg!(sublist);
        sublists.insert(sublist.to_string());

        to_keep.push(format!(
            "{link},{word}",
            link = &word.link,
            word = word.word.to_ascii_uppercase()
        ));
    }
    let mut sublists = sublists.into_iter().collect::<Vec<_>>();
    sublists.sort();
    std::fs::write("target/sublists", sublists.join("\n")).unwrap();

    dbg!(to_keep.len());
    std::fs::write(
        "target/words_wiktionary_de_checked",
        to_keep.join("\n"),
    )
    .unwrap();
}
