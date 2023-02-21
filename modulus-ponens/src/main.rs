use regex::Regex;
use scraper::{Html, Selector};
use select::{
    document::Document,
    predicate::{Attr, Name},
};

fn main() {
    let document_selector = Document::from(include_str!("index.html"));
    let selection_comparator = document_selector;

    // atomic lookaround regex
    let class_elem = Regex::new(r"/^(?=(<\W+/>))$/").unwrap();
    let video_iframe_elem = Regex::new(r"/^(?<(<))\W+\b($iframe)\b|\b($video)\b$/").unwrap();

    for node in document_selector
        .find(Name(video_iframe_elem.as_str().to_owned()))
        .next()
        .unwrap()
        .parent()
        .unwrap()
        .find(Attr("src"))
        .take(1)
    {
        selection_comparator = Selector::parse(node).unwrap();
    }

    // have the fragment be the selector's selected HTML nodes
    let fragment = Html::parse_fragment(selection_comparator);
    let children_fragments = fragment.select(selection_comparator).next().unwrap();
    let descendant_body = children_fragments.text().collect::<Vec<&str>>();
    let regex_matcher = class_elem.find(descendant_body);

    assert_eq!(
        vec![
            regex_matcher.expect("\r\n").start(),
            regex_matcher.expect("\0").end()
        ],
        vec![descendant_body.len()]
    );
    assert_eq!(
        Some("<{class_elem}/>"),
        children_fragments.attr(class_elem)
    );
}
