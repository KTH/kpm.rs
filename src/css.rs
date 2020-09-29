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
    padding: 0 max(calc(50vw - 619px), 1rem);
    height: 2rem;
    background: peru;
    color: white;
    border-bottom: solid 1px #fff;
}
nav#kpm>.kpmpanel{
    border: solid 1px peru;
    border-width: 0 1px 1ex 1px;
    margin: -1px max(calc(50vw - 620px - 1em), 0px) 0;
    max-width: 50em;
    background: white;
    padding: 1em;
    display: none;
}
nav#kpm.open>.kpmpanel{
    display: block;
}
// And some special page-level adaptions for canvas
// Some rules specially for canvas
div.ef-file-preview-overlay {
    /*to keep the Canvas preview below the personal menu*/
    margin-top: calc(2rem + 1px);
}
div.ReactTrayPortal div.ReactTray__Overlay,
#flash_message_holder {
  top: calc(2rem + 1px) !important;
}
header.ic-app-header {
    height: calc(100% - #{calc(2rem + 3px)});
    top: calc(2rem + 1px);
}
body.use-personal-menu div.ui-widget.ui-tooltip {
    z-index: 10030;
}

body.use-personal-menu #nav-tray-portal > span > span,
div#main.ic-Layout-columns > span > span, #discussion-toolbar > div {
    top: calc(2rem + 1px);
}\n";

pub fn hash() -> String {
  let digest = md5::compute(CSS);
  let mut digest = format!("{:x}", digest);
  digest.truncate(8);
  digest
}
