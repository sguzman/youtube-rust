extern crate rayon;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, info, warn};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Cache {
    data: HashMap<String, String>, // Adjust types according to your needs
}

const CACHE: &str = ".cache/data.json";

// Make an HTTP request
use reqwest::blocking::Client;

fn get(url: String) -> String {
    let url: String = format!("https://www.youtube.com/channel/{url}/videos");
    info!("GET {}", url);
    let client = Client::new();
    let res = client.get(url.clone()).send().unwrap();

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

fn op(cache: &mut Cache) {
    let arg: String = "UC-lHJZR3Gqxm24_Vd_AJ5Yw".to_string();
    // Cache get html
    let html = cache_logic(arg, cache, |url| get(url));
    let body = parser(html);
    let scripts = get_scripts(&body);
    let element = get_data(&scripts);
    let text = get_text(element);
    let json = get_json(text);
}

// Init systems
fn init() -> Cache {
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
    Cache::load()
}

impl Cache {
    fn load() -> Cache {
        let path = Path::new(CACHE);
        if path.exists() {
            let mut file = File::open(path).unwrap();

            let cache: Cache =
                serde_json::from_reader(&mut file).expect("Failed to deserialize cache");

            // Log number of keys in cache
            log::info!("Loaded {} keys from cache", cache.data.len());

            cache
        } else {
            Cache {
                data: HashMap::new(),
            }
        }
    }

    fn save(&self) {
        let path = Path::new(CACHE);
        let contents = serde_json::to_string(&self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes())
            .expect("Failed to write cache to file")
    }
}

fn cache_logic<F, Arg, Res>(arg: Arg, cache: &mut Cache, compute: F) -> Res
where
    F: FnOnce(Arg) -> Res,
    Arg: std::hash::Hash + Eq + Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
    Res: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    let key = serde_json::to_string(&arg).expect("Failed to serialize key");

    if let Some(result) = cache.data.get(&key) {
        serde_json::from_str(result).expect("Failed to deserialize value")
    } else {
        let result = compute(arg.clone());
        let serialized_result = serde_json::to_string(&result).expect("Failed to serialize value");
        cache.data.insert(key, serialized_result);
        result
    }
}

fn main() -> io::Result<()> {
    let mut cache = init();
    op(&mut cache);
    cache.save();
    log::info!("end");

    Ok(())
}
