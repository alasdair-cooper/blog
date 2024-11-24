use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Timelike;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use reqwest::Client;
use sauron::{
    article, main, wasm_bindgen, wasm_bindgen_futures, window, Application, Cmd, JsValue, Node,
    Program,
};
use sauron_html_parser::parse_html;

enum Msg {
    GetArticles,
    GotArticles(Vec<String>),
}

struct App {
    articles: Vec<String>,
}

async fn get_articles() -> Vec<String> {
    let client = Client::new();

    let mut articles: Vec<String> = [].to_vec();

    for url in ["/one_of.md"] {
        let origin = window().origin();
        let ticks = chrono::Local::now().timestamp();
        let res = client
            .get(format!("{}{}?v={}", origin, url, ticks))
            .send()
            .await
            .unwrap();
        if let Some(text) = res.text().await.ok() {
            let html =
                markdown_to_html_with_plugins(&text, &Options::default(), &Plugins::default());
            articles.push(html);
        }
    }

    articles
}

impl App {
    fn new() -> Self {
        App { articles: vec![] }
    }
}

impl Application for App {
    type MSG = Msg;

    fn init(&mut self) -> Cmd<Msg> {
        Cmd::new(async move { Msg::GetArticles })
    }

    fn view(&self) -> Node<Msg> {
        main(
            [],
            self.articles.iter().map(|x| {
                article(
                    [],
                    [parse_html::<Msg>(x)
                        .expect("should have no parsing error")
                        .expect("should have at least one node")],
                )
            }),
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Msg> {
        match msg {
            Msg::GetArticles => Cmd::new(async move {
                let articles = get_articles().await;
                Msg::GotArticles(articles)
            }),
            Msg::GotArticles(articles) => {
                self.articles = articles;
                Cmd::none()
            }
        }
    }
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
    Ok(())
}
