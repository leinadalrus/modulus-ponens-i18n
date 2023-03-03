use regex::Regex;
use scraper::{Html, Selector};
use select::{
    document::Document,
    predicate::{Attr, Name},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

fn cli_interf_args(_argc: i8, _argv: u8) -> ! {
    if _argc >= 1 {
        match _argv.offset(1) as char {
            '1' =>
                assert!(std::process::Command::new("npm i; npm build; npm run")
                    .spawn()
                    .is_ok()),
            '2' => assert!(std::process::Command::new(
                "cd $(ls | grep -r 'entrypoint.sh' ./); ./entrypoint.sh"
            )
            .spawn()
            .is_ok()),
            _ => panic!(),
        }
    }

    core::intrinsics::abort();
}

// pub fn slow_interf_reqs(
//     url: &str,
// ) -> std::result::Result<String, Box<dyn std::error::Error>> {
//     url = match std::env::args().nth(1) {
//         Some(url) => &url,
//         None => {
//             println!("No CLI URL provided, using default.");
//             "https://allanime.to/watch/YeWtc8REZAGKPeb6q/kage-no-jitsuryokusha-ni-naritakute-/episode-1-sub".into()
//         }
//     };
//     let response =
//         reqwest::blocking::get(url).expect("URL undefined | unknown");
//     Ok(response.text().unwrap())
// }

pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Some simple CLI args requirements...
    let url_to_be_examined = match std::env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("No CLI URL provided, using default.");
            "https://allanime.to/watch/YeWtc8REZAGKPeb6q/kage-no-jitsuryokusha-ni-naritakute-/episode-1-sub".into()
        }
    };

    let body = reqwest::get(url_to_be_examined).text().await?;

    let built_cookie_client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    client.get(url_to_be_examined).send()?;

    let html_raw_document = request_throttled(url_to_be_examined);
    let document_selector = Document::from(&html_raw_document);

    // atomic lookaround regex
    let class_template_element = Regex::new(r"/^(?=(<\W+/>))$/").unwrap();
    let video_iframe_element =
        Regex::new(r"/^(?<(<)\b($iframe)\b$/+?>(>))").unwrap();

    for iframe_tag_element in document_selector.find(
        Name(video_iframe_element.as_str().to_owned())
            .descendant(Attr("id", "arc")),
    )
    // change to "src" and value
    {
        let iframe_tag_refstr_convert = &iframe_tag_element.to_str();
        let tag_refstr_selection_comparator =
            &Selector::parse(iframe_tag_refstr_convert)
                .expect("Element error | undefined | unknown");
        let element_tag_iframe = iframe_tag_element
            .select(tag_refstr_selection_comparator)
            .flat_map(|element| element.text())
            .collect();

        // have the fragment be the selector's selected HTML nodes
        let fragment = Html::parse_fragment(element_tag_iframe);
        let children_fragments = fragment
            .find(Attr("allowfullscreen", "allowfullscreen")) // find the lang-setup exported function
            .next()
            .unwrap() // unwrap to get the raw-string values
            .find(Attr("referrerpolicy", "origin"))
            .next()
            .unwrap();
        let descendant_body = children_fragments.text().collect::<Vec<&str>>();
        let regex_matcher = video_iframe_element.find(descendant_body);

        assert_eq!(
            vec![
                regex_matcher.expect("\r\n").start(),
                regex_matcher.expect("\0").end()
            ],
            vec![descendant_body.len()]
        ); // NOTE(David): non-throttled `loop {}` condition would be nice for
           // testing
    }

    Ok(())
}
