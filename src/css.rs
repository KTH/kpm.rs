use md5;

pub static CSS: &str = "
html, body {
    scroll-padding-top: calc(2rem + 3px);
}
body{
    margin-top: calc(2rem + 1px);
    position: relative !important;
    border-top: none !important;
    padding-top: 0;
}
nav#kpm {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 9999;
}
nav#kpm.open {
    height: 100vh;
    background: rgba(0,0,0,0.3);
}
nav#kpm>.kpmbar{
    line-height: 2rem;
    margin: 0;
    padding: 0 max(calc(50vw - 590px), 1rem);
    height: 2rem;
    background: peru;
    color: white;
    border-bottom: solid 1px #fff;
}
nav#kpm>.kpmpanel{
    border: solid 1px peru;
    border-width: 0 1px 1ex 1px;
    margin: -1px max(calc(50vw - 590px), 1rem) 0;
    max-width: 50em;
    background: white;
    padding: 1em;
    display: none;
}
nav#kpm.open>.kpmpanel{
    display: block;
}\n";

pub fn hash() -> String {
  let digest = md5::compute(CSS);
  let mut digest = format!("{:x}", digest);
  digest.truncate(8);
  digest
}
