use readable_readability::Readability;
use url::Url;

fn main() {
    let urls = [
        "https://www.elmundo.es/internacional/2023/03/28/6422bee8fc6c83b81d8b45aa.html"
        // "https://elpais.com/sociedad/2023-03-28/el-consejo-de-ministros-aprueba-la-ley-de-familias-sin-la-prohibicion-expresa-del-veto-parental.html"
        // "http://www.bbc.co.uk/news/uk-politics-33647154",
        // "https://www.bbc.co.uk/news/uk-scotland-scotland-politics-65105951"
        //"https://www.theguardian.com/global-development/2023/mar/28/security-guards-in-qatar-still-being-paid-as-little-as-35p-an-hour"
        ];
    for url in urls {
        let html = match reqwest::blocking::get(url) {
            Ok(resp) => match resp.text() {
                Ok(html) => html.to_string(),
                Err(_) => String::new()
            },
            Err(_) => String::new()
        };
        println!("{}", html.len());
        let mut r = Readability::new();
        let (actual_tree, actual_meta) = r.base_url(Url::parse(url).unwrap()).parse(&html);
        println!("{}", actual_tree.text_contents());
        println!("{}", actual_meta.article_title.unwrap_or_default());
        println!("{}", actual_meta.page_title.unwrap_or_default());
        println!("{}", actual_meta.description.unwrap_or_default());

    }
}

