#![feature(core_intrinsics)]
#![feature(dec2flt)]
use core::num::dec2flt::parse;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use select::{
    document::Document,
    predicate::{Attr, Name},
}; // use separately from Scraper-Rs
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

fn cli_interf_args(_argc: isize, _argv: *const *const u32) -> ! {
    if _argc >= 1 {
        unsafe {
            match char::from_u32(**_argv) {
                Some('1') => assert!(std::process::Command::new(
                    "cd $(ls | grep -r 'entrypoint.sh' ./); ./entrypoint.sh; \
                     ./entrypoint.sh init"
                )
                .spawn()
                .is_ok()),
                Some('2') => assert!(std::process::Command::new(
                    "npm i; npm build; npm run"
                )
                .spawn()
                .is_ok()),
                _ => panic!(),
            }
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

    let content_body = reqwest::get(url_to_be_examined);
    let built_cookie_client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    built_cookie_client.get(url_to_be_examined).send()?;

    // atomic lookaround regex
    let class_template_element = Regex::new(r"/^(?=(<\W+/>))$/").unwrap();
    let video_iframe_element =
        Regex::new(r"/^(?<(<)\b($iframe)\b$/+?>(>))").unwrap();

    let parsed_fragment =
        Html::parse_fragment(&video_iframe_element.to_string());
    let iframe_selector = Selector::parse("iframe").unwrap();
    let arc_selector = Selector::parse("arc").unwrap();
    let arc_id = parsed_fragment.select(&iframe_selector).next().unwrap();

    // change to "src" and value
    let iframe_tag_refstr_convert = &iframe_selector;
    let tag_refstr_selection_comparator =
        &Selector::parse(iframe_tag_refstr_convert)
            .expect("Element error | undefined | unknown");
    let elements = iframe_selector.ne(&tag_refstr_selection_comparator);

    for element in elements {
        if element == elements {
            return Err(element);
        }
    }

    for attribute in arc_id.select(&arc_selector) {
        let arc_attributes = attribute.value().attrs;
        let descendant_body = arc_attributes.values().next();
        let regex_matcher = video_iframe_element.find(descendant_body);

        assert_eq!(
            vec![
                regex_matcher.expect("\r\n").start(),
                regex_matcher.expect("\0").end()
            ],
            vec![descendant_body]
        ); // NOTE(David): non-throttled `loop {}` condition would be nice for
           // testing
    }

    Ok(())
}
