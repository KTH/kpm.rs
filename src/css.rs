/// The content of the css for the menu.  Fetch from the javascript.
pub static MENU_CSS: &str = include_str!("../style/menu.css");

/// The content of the css for the application.
///
/// This is from the kth-style npm package, fetched by a script.
pub static PAGE_CSS: &str = include_str!("../style/kth-bootstrap.css");

/// The file name (including a hash) of the css for the menu.
pub fn menu_css_name() -> String {
    format!("menu-{}.css", hash(MENU_CSS))
}

/// The file name (including a hash) of the css for the page.
pub fn page_css_name() -> String {
    format!("page-{}.css", hash(PAGE_CSS))
}

fn hash(data: &str) -> String {
    let digest = md5::compute(data);
    let mut digest = format!("{:x}", digest);
    digest.truncate(8);
    digest
}
