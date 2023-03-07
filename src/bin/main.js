const { invoke } = window.__TAURI__.tauri;

async function load_known_sources() {
  document.getElementById("known_source").innerHTML = await invoke("load_known_sources");
}

async function load_user_sources() {
  let loaded = await invoke("load_user_sources");
  if(loaded) {
    await sync_all_sources();
    document.getElementById("items_toggle").click();
  } else {
    await update_sources();
    document.getElementById("sources_toggle").click();
  }
}

async function add_source_from_url() {
  let url = document.getElementById('source_url').value;
  document.getElementById('source_url').value = "";
  invoke("add_source", { url: url });
  await sync_source(url);
}

async function add_known_source() {
  let url = document.getElementById('known_source').value;
  invoke("add_source", { url: url });
  await sync_source(url);
}

async function sync_source(url) {
  await invoke("sync_source", { url: url });
  await update_sources();
  await update_items();
}

async function sync_all_sources() {
  await invoke("sync_all_sources");
  await update_sources();
  await update_items();
}

async function update_sources() {
  document.getElementById("sources").innerHTML = await invoke("get_sources_table");
}

async function update_items() {
  // TODO add filter here
  document.getElementById("items").innerHTML = await invoke("get_items");
}

// async function sync_all() {
//   var i, ids;
//   ids = [];
//   let selected_items = document.getElementsByClassName("selected");
//   for (i = 0; i < selected_items.length; i++) {
//     var id = selected_items[i].id;
//     if(id.length > 0) {
//       ids.push(id);
//     }
//   }
//   let refresh_button = document.getElementById("refresh-feeds");
//   refresh_button.className = "active";
//   let feeds = document.getElementById("feeds");
//   feeds.innerHTML = await invoke("sync_all");
//   refresh_button.className = "passive";
//   if (ids.length > 0) {
//     for (i = 0; i < ids.length; i++) {
//       document.getElementById(ids[i]).className += " selected";
//     }
//   } else {
//     document.getElementsByClassName("tab-toggle")[0].click();
//   }
//   update_sources_display();
// }

function clear_selected(elements) {
  var i;
  var selected = "selected";
  for (i = 0; i < elements.length; i++) {
    while(elements[i].className.indexOf(selected) > 0) {
      elements[i].className = elements[i].className.replace(selected, "").trim();
    }
    elements[i].className = elements[i].className.replace(/\s\s+/g, " ");
  }
}

function toggle_tab(name) {
  var i, tabs, tab_toggles;
  clear_selected(document.getElementsByClassName("tab"));
  clear_selected(document.getElementsByClassName("tab_toggle"));
  document.getElementById(name+"_tab").className += " selected";
  document.getElementById(name+"_toggle").className += " selected";
}

window.addEventListener("DOMContentLoaded", () => {
  var i, tab_toggles;
  console.log("add click handler to sync_all")
  document
    .getElementById("sync_all")
    .addEventListener("click", () => sync_all_sources());
    
  console.log("add click handler to add_source_from_url")
  document
    .getElementById("add_source_from_url")
    .addEventListener("click", () => add_source_from_url());
  console.log("add click handler to add_known_source")
  document
    .getElementById("add_known_source")
    .addEventListener("click", () => add_known_source());
  tab_toggles = document.getElementsByClassName("tab_toggle");
  for (i = 0; i < tab_toggles.length; i++) {
    let name = tab_toggles[i].value
    console.log("add click handler to "+name)
    tab_toggles[i].addEventListener("click", () => toggle_tab(name));
  }
  load_known_sources();
  load_user_sources();
})
