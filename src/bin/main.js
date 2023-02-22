const { invoke } = window.__TAURI__.tauri;

//let items_element;

async function update_sources_tab() {
  document.getElementById("tab-sources").innerHTML = await invoke("update_sources_tab");
}

async function load() {
  await invoke("load")
  update_sources_tab();
}

async function refresh() {
  var i, ids;
  ids = [];
  let selected_items = document.getElementsByClassName("selected");
  for (i = 0; i < selected_items.length; i++) {
    var id = selected_items[i].id;
    if(id.length > 0) {
      ids.push(id);
    }
  }
  let refresh_button = document.getElementById("button-refresh");
  refresh_button.className = "active";
  let feeds = document.getElementById("feeds");
  feeds.innerHTML = await invoke("refresh");
  refresh_button.className = "passive";
  if (ids.length > 0) {
    for (i = 0; i < ids.length; i++) {
      document.getElementById(ids[i]).className += " selected";
    }
  } else {
    document.getElementsByClassName("tab-toggle")[0].onclick();
  }
  update_sources_tab();
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .getElementById("button-refresh")
    .addEventListener("click", () => refresh());
});

document.addEventListener('DOMContentLoaded', () => {
  load();
  refresh();
})
