pub static MENU_CSS: &str = include_str!("../style/menu.css");
pub static PAGE_CSS: &str = include_str!("../style/kth-bootstrap.css");

pub fn menu_css_name() -> String {
    format!("page-{}.css", hash(MENU_CSS))
}
pub fn page_css_name() -> String {
    format!("page-{}.css", hash(PAGE_CSS))
}

fn hash(data: &str) -> String {
  let digest = md5::compute(data);
  let mut digest = format!("{:x}", digest);
  digest.truncate(8);
  digest
}
