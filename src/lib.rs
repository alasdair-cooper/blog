use comrak::{
    markdown_to_html, markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, Options,
    Plugins,
};
use reqwest::Client;
use sauron::{
    class, document,
    html::{text, units::px},
    input, jss, main, node, on_click, on_readystatechange, r#type, value, wasm_bindgen,
    wasm_bindgen_futures,
    web_sys::console::{info, info_0, info_1},
    window, Application, Closure, Cmd, Element, JsCast, JsValue, Node, Program,
};
use sauron_html_parser::raw_html;

enum Msg {
    GetArticles,
    GotArticles(Vec<String>),
}

struct App {
    articles: Vec<String>,
}

async fn get_articles() -> Vec<String> {
    let mut plugins = Plugins::default();
    let adapter = SyntectAdapter::new(None);
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let client = Client::new();

    let mut articles: Vec<String> = [].to_vec();

    for url in ["/one_of.md"] {
        let origin = window().origin();
        let res = client
            .get(format!("{}{}", origin, url))
            .send()
            .await
            .unwrap();
        if let Some(text) = res.text().await.ok() {
            let html = markdown_to_html_with_plugins(&text, &Options::default(), &plugins);
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
        main([], self.articles.iter().map(|x| raw_html(x)))
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
