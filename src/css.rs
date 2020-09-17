use md5;

pub static CSS: &str = "
html, body {
    scroll-padding-top: calc(2rem + 3px);
}
body{
    margin-top: calc(2rem + 1px);
}
nav#kpm {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 9999;
}
nav#kpm>.kpmbar{
    line-height: 2rem;
    margin: 0;
    padding: 0 1rem;
    height: 2rem;
    background: peru;
    color: white;
    border-bottom: solid 1px #fff;
}\n";

pub fn hash() -> String {
  let digest = md5::compute(CSS);
  let mut digest = format!("{:x}", digest);
  digest.truncate(8);
  digest
}
