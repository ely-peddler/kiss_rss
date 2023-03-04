const { invoke } = window.__TAURI__.tauri;

async function load_subscriptions() {
  let loaded = await invoke("load_subscriptions");
  if(loaded) {
    await sync_all_subscriptions();
    document.getElementById("items_toggle").click();
  } else {
    await update_subscriptions();
    document.getElementById("subscriptions_toggle").click();
  }
}

async function add_subscription_from_url() {
  let url = document.getElementById('subscription_url').value;
  document.getElementById('new_subscription_url').value = "";
  invoke("add_subscription", { url: url });
  await sync_subscription(url);
}

async function sync_subscription(url) {
  await invoke("sync_subscription", { url: url });
  await update_subscriptions();
  await update_items();
}

async function sync_all_subscriptions() {
  await invoke("sync_all_subscriptions");
  await update_subscriptions();
  await update_items();
}

async function update_subscriptions() {
  document.getElementById("subscriptions").innerHTML = await invoke("get_subscriptions_table");
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
//   update_subscriptions_display();
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
    .addEventListener("click", () => sync_all_subscriptions());
  console.log("add click handler to add_subscription_from_url")
  document
    .getElementById("add_subscription_from_url")
    .addEventListener("click", () => add_subscription_from_url());
  tab_toggles = document.getElementsByClassName("tab_toggle");
  for (i = 0; i < tab_toggles.length; i++) {
    let name = tab_toggles[i].value
    console.log("add click handler to "+name)
    tab_toggles[i].addEventListener("click", () => toggle_tab(name));
  }
  load_subscriptions();
})
