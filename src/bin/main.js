const { invoke } = window.__TAURI__.tauri;

//let items_element;

async function update_subscriptions_display() {
  document.getElementById("current-subscriptions").innerHTML = await invoke("get_subscriptions_html");
}

async function load_subscriptions() {
  let loaded = await invoke("load_subscriptions");
  console.log(loaded);
  update_subscriptions_display();
  if(loaded) {
    refresh_feeds();
  } else {
    document.getElementById("subscriptions").click();
  }
}

async function add_subscription() {
  let name = document.getElementById('new-subscription-name').value;
  document.getElementById('new-subscription-name').value = "";
  let url = document.getElementById('new-subscription-url').value;
  document.getElementById('new-subscription-url').value = "";
  await invoke("add_subscription", { name: name, url: url });
  refresh_feeds();
}

async function refresh_feeds() {
  var i, ids;
  ids = [];
  let selected_items = document.getElementsByClassName("selected");
  for (i = 0; i < selected_items.length; i++) {
    var id = selected_items[i].id;
    if(id.length > 0) {
      ids.push(id);
    }
  }
  let refresh_button = document.getElementById("refresh-feeds");
  refresh_button.className = "active";
  let feeds = document.getElementById("feeds");
  feeds.innerHTML = await invoke("refresh_feeds");
  refresh_button.className = "passive";
  if (ids.length > 0) {
    for (i = 0; i < ids.length; i++) {
      document.getElementById(ids[i]).className += " selected";
    }
  } else {
    document.getElementsByClassName("tab-toggle")[0].click();
  }
  update_subscriptions_display();
}

function open_tab(name) {
  var i, tabs, tab_toggles;
  tabs = document.getElementsByClassName("tab");
  for (i = 0; i < tabs.length; i++) {
    tabs[i].className = tabs[i].className.replace(" selected", "");
  }
  tab_toggles = document.getElementsByClassName("tab-toggle");
  for (i = 0; i < tab_toggles.length; i++) {
    tab_toggles[i].className = tab_toggles[i].className.replace(" selected", "");
  }
  document.getElementById("tab-"+name).className += " selected";
  document.getElementById(name).className += " selected";
}

window.addEventListener("DOMContentLoaded", () => {
  var i, tab_toggles;
  console.log("add click handler to refresh")
  document
    .getElementById("refresh-feeds")
    .addEventListener("click", () => refresh_feeds());
  console.log("add click handler to refresh")
  document
    .getElementById("add-subscription")
    .addEventListener("click", () => add_subscription());
  tab_toggles = document.getElementsByClassName("tab-toggle");
  for (i = 0; i < tab_toggles.length; i++) {
    let id = tab_toggles[i].id
    console.log("add click handler to "+id)
    tab_toggles[i].addEventListener("click", () => open_tab(id));
  }
  load_subscriptions();
})
