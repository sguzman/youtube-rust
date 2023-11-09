use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, info, warn};
use scraper::{ElementRef, Html, Selector};
use std::io::Write;

// Make an HTTP request
use reqwest::blocking::Client;

// Use serde_json to parse JSON
use youtube_serde::Youtube;

fn get(url: &str) -> String {
    info!("GET {}", url);
    let client = Client::new();
    let res = client.get(url).send().unwrap();

    debug!("Status: {}", res.status());
    res.text().expect(&std::format!("Failed to get {}", url))
}

fn parser(html: String) -> Html {
    info!("Parsing");
    Html::parse_document(&html)
}

fn get_scripts<'a>(html: &'a Html) -> Vec<ElementRef<'a>> {
    info!("Getting scripts");
    let selector = Selector::parse("script").unwrap();
    let mut scripts: Vec<ElementRef> = Vec::new();

    for element in html.select(&selector) {
        scripts.push(element);
    }

    info!("Found {} scripts", scripts.len());
    scripts
}

// Get Script element that contains var ytInitialData
fn get_data<'a>(scripts: &'a Vec<ElementRef<'a>>) -> Option<&'a ElementRef<'a>> {
    info!("Getting ytInitialData script");

    for script in scripts {
        let text = script.text().collect::<String>();
        if text.starts_with("var ytInitialData") {
            info!("Found ytInitialData script");
            return Some(script);
        } else {
            debug!("Skipping script");
        }
    }

    warn!("Did not find ytInitialData script");
    None
}

// Function to get text from ytInitialData script element
fn get_text<'a>(element: Option<&'a ElementRef<'a>>) -> String {
    info!("Getting text from ytInitialData script");
    match element {
        Some(script) => {
            let text = script.text().collect::<String>();
            info!("Found text");
            text
        }
        None => panic!("Did not find text"),
    }
}

// Function to extract JSON from ytInitialData script text
fn get_json(text: String) -> String {
    info!("Getting JSON from ytInitialData script text");
    let start = text.find("{");
    let end = text.rfind("}");

    match (start, end) {
        (Some(start), Some(end)) => {
            let json = text[start..end].to_string();
            info!("Found JSON");
            json
        }
        _ => panic!("Did not find JSON"),
    }
}

// Use serde_json to parse JSON
fn parse_json(json: String) -> Youtube {
    info!("Parsing JSON");
    let v: Youtube = serde_json::from_str(&json).unwrap();
    info!("Parsed JSON");

    v
}

fn op() {
    const URL: &str = "https://www.youtube.com/channel/UC-lHJZR3Gqxm24_Vd_AJ5Yw/videos";
    let html = get(URL);
    let body = parser(html);
    let scripts = get_scripts(&body);
    let element = get_data(&scripts);
    let text = get_text(element);
    let json = get_json(text);
    let v = parse_json(json);
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(32)
        .build_global()
        .unwrap();

    // Initialize logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    log::info!("Starting");
    op();
    log::info!("end");
}
