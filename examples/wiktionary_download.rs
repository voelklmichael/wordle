fn main() {
    let client = reqwest::blocking::Client::new();
    let base_url =
        //"https://de.wiktionary.org/w/index.php?title=Kategorie:Grundformeintrag_(Deutsch)";
        "https://de.wiktionary.org/wiki/Kategorie:Partikel_(Deutsch)";
    let wiktionary_kind = "partikel";
    let mut url_suffix = "".to_string();
    //let mut bodies = vec![];
    let mut words = Vec::new();
    loop {
        let url_full = format!("{base_url}{url_suffix}");
        dbg!(&url_full);
        let res = client.get(url_full).send().unwrap();
        let body = res.text().unwrap();
        let count_number: usize = body
            .split_once("<p>Folgende ")
            .unwrap()
            .1
            .split_once(" Seiten sind in dieser Kategorie, von ")
            .unwrap()
            .0
            .parse()
            .unwrap();

        let entry = body
            .split_once("<h2>Seiten in der Kategorie ")
            .unwrap()
            .1;
        let entry = entry
            .split_once(r#"<div class="printfooter" data-nosnippet="">"#)
            .unwrap()
            .0
            .split("<li><a href=\"")
            .skip(1)
            .map(|x| x.split_once("\">").unwrap().0.to_string())
            .collect::<Vec<_>>();
        assert_eq!(count_number, entry.len());
        words.extend(entry);

        let lines = body.split("\n").collect::<Vec<_>>();
        //let html = lines.join("\n");
        //std::fs::write(format!("target/lines_{}.html", bodies.len()), &html).unwrap();
        //bodies.push(html);
        if let Some(line) = lines.iter().find(|x| x.contains("nächste")) {
            let next = line
                .split("<a href=\"")
                .last()
                .map(|x| x.split_once("</a>").unwrap().0)
                .unwrap();
            if !next.contains("nächste") {
                break;
            }
            let next = next
                .split_once("\"")
                .unwrap()
                .0
                .strip_prefix(
                    "/w/index.php?title=Kategorie:Grundformeintrag_(Deutsch)&amp;pagefrom=",
                )
                .unwrap()
                .split_once("#")
                .unwrap()
                .0;
            url_suffix = format!("&pagefrom={}", next);
        } else {
            break;
        }

        //dbg!(bodies.len());
        dbg!(words.len());
    }
    dbg!(words.len());
    std::fs::write(
        format!("target/words_wiktionary_de_{wiktionary_kind}"),
        words.join("\n"),
    )
    .unwrap();
}
