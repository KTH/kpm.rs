(function(d){
  function create_menu() {
    var el = d.createElement('nav');
    el.id = "kpm"
    el.innerHTML= '<div class="container">hejsan</div>';
    d.body.insertBefore(el, d.body.firstChild);
  }
  function fetch_css() {
    var base = (d.currentScript || d.querySelector('script[src*=kpm]')).src;
    var el = d.createElement('link');
    el.href = base.substr(0, 1+base.lastIndexOf('/'))+'index-{{ js_hash }}.css';
    el.rel = "stylesheet";
    el.type = "text/css";
    d.head.appendChild(el);
  }
  fetch_css();
  create_menu();
})(document)

