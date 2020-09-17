use md5;

pub static CSS: &str = "
html, body {
    scroll-padding-top: $toolbarheight;
}
body{
    margin-top: 2rem;
}
nav#kpm {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
}
nav#kpm>.container{
    line-height: 2rem;
    margin: 0;
    height: 2rem;
    background: peru;
    color: white;
}\n";

pub fn hash() -> String {
  let digest = md5::compute(CSS);
  let mut digest = format!("{:x}", digest);
  digest.truncate(8);
  digest
}
