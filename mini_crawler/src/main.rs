use reqwest::blocking::ClientBuilder;
use url::Url;
use mini_crawler::LinkExtractor;
use mini_crawler::crawler::Crawler;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Maximum number of pages to be crawled
    #[structopt(short="n")]
    maximum_pages: usize,
    /// URL where this program starts crawling
    start_page: Url,
}
/*
fn on_response(res: reqwest::blocking::Response) {
    match res.error_for_status() {
        Ok(_res) => (),
        Err(err) => {
            // asserting a 400 as an example
            // it could be any status between 400...599
            assert_eq!(
                err.status(),
                Some(reqwest::StatusCode::BAD_REQUEST)
            );
        }
    }
}
*/
fn main() -> eyre::Result<()> {
    /*let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;

    println!("body = {:?}", body);

    let doc = Document::from(body.as_str());
    for a in doc.find(Name("a")) {
        println!("{:?}", a);
    }
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        println!("{:?}", href);
    }
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {

        match Url::parse(href) {
            Ok(url) => { println!("{}", url); },
            Err(UrlParseError::RelativeUrlWithoutBase) => {
            // `href`を絶対URLに変換する。
            },
            Err(e) => {},
        }
    }*/

    //演習
    //let f = reqwest::blocking::get("https://www.rust-")?;
    //on_response(f);

    /*
    let response = reqwest::blocking::get("https://www.rust-lang.org")?;
    let base_url = response.url().clone();
    let body = response.text()?;
    let doc = Document::from(body.as_str());
    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        match Url::parse(href) {
            Ok(url) => { println!("{}", url); },
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                println!("{}", url);
            },
            Err(e) => { println!("Error: {}", e); },
        }
    }
    let url = Url::parse("https://www.rust-lang.org")?;
    let client = ClientBuilder::new()
        .build()?;
    let extractor = LinkExtractor::from_client(client);

    let links = extractor.get_links(url)?;
    for link in links.iter() {
        println!("{}", link);
    }*/
    env_logger::init();

    let opt = Opt::from_args();

    let client = ClientBuilder::new()
        .build()?;
    let extractor = LinkExtractor::from_client(client);

    let crawler = Crawler::new(&extractor, opt.start_page);

    let wait = Duration::from_millis(100);

    for url in crawler.take(opt.maximum_pages) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
