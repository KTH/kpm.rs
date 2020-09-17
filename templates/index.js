(function(d){
  function create_menu() {
    var el = d.createElement('nav');
    el.id = "kpm"
    el.innerHTML= '<div class="container">hejsan</div>';
    d.body.insertBefore(el, d.body.firstChild);
  }
  function fetch_css() {
    var el = d.createElement('link');
    el.href = '/kpm/index-{{ js_hash }}.css';
    el.rel = "stylesheet";
    el.type = "text/css";
    d.head.appendChild(el);
  }
  fetch_css();
  create_menu();
})(document)

