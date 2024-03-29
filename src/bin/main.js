const { invoke } = window.__TAURI__.tauri;

async function display_readable(title, url) {
  document.getElementById("reader_title").innerHTML = title;
  document.getElementById("reader").innerHTML = await invoke("get_readable", { url: url});
  toggle_tab("reader");
}

async function load_known_sources() {
  document.getElementById("known_source").innerHTML = await invoke("load_known_sources");
}

async function load_user_sources() {
  let loaded = await invoke("load_user_sources");
  await update_sources();
  if(loaded) {
    await sync_all_sources();
    document.getElementById("items_toggle").click();
  } else {
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

async function remove_source(name, url) {
  let confirmed = await confirm('Remove '+name+'?', 'Confirm');
  if(confirmed) {
    invoke("remove_source", { url: url });
    await update_sources();
    await update_items();
  }
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

function create_div(class_name, text='', id='') {
  let div = document.createElement('div');
  div.className = class_name;
  if(text.length > 0) {
    div.innerText = text;
  }
  if(id.length > 0) {
    div.id = id;
  }
  return div;
}

function create_button(class_name, text, onclick) {
  let button = document.createElement('button');
  button.className = class_name;
  button.innerText = text;
  button.addEventListener('click', onclick);
  return button;
}

async function update_sources() {
  var source_list = JSON.parse(await invoke("get_source_list_as_json"));
  let sources_element = document.getElementById("sources");
  sources_element.innerHTML = '';
  let source_count = source_list.sources.length;
  console.log(source_count)
  for (let i = 0; i < source_count; i++) {
    let source = source_list.sources[i];
    let source_element = create_div('source');
    let source_info_element = create_div('info');
    source_info_element.appendChild(create_div('name', source.name, source.url));
    source_info_element.appendChild(create_div('timestamp', source.last_sync));
    source_info_element.appendChild(create_div('update_rate', Math.floor(source.update_rate*24.0).toString() + ' / day'));
    console.log(source.name, source.status, source.update_rate, Math.floor(source.update_rate*24.0).toString());
    let status = '✕'
    if(source.status == 'Ok') {
      status = '✓';
    } else if (source.status == 'Unknown') {
      status = '?';
    }
    source_info_element.appendChild(create_div('icon', status));
    source_info_element.appendChild(create_button('icon', '🖉', () => rename_source(source.url)));
    source_info_element.appendChild(create_button('icon', '🗑', () => remove_source(source.name, source.url)));
    source_element.appendChild(source_info_element);
    sources_element.appendChild(source_element);
  }
}

async function update_items() {
  var item_list = JSON.parse(await invoke("get_item_list_as_json"));
  let items_element = document.getElementById("items");
  items_element.innerHTML = '';
  let item_count = item_list.items.length;
  console.log(item_count)
  for (let i = 0; i < item_count; i++) {
    let item = item_list.items[i];
    let item_element = create_div('news_item');
    item_element.appendChild(create_div('source_name', item.source));
    item_element.appendChild(create_div('timestamp', item.timestamp));
    // let encoded_url = encodeURIComponent(item.url);
    // let item_url = new URL(item.url);
    // console.log(encoded_url);
    // let reader_url = "read:"+item.url // "read://"+item_url.protocol.replace(":", "")+"_"+item_url.host+"/?url="+encoded_url;
    // console.log(reader_url);
    // let a = document.createElement("a")
    // a.href=reader_url
    // a.innerText=item.title
    // a.target="_blank"
    // let t = create_div('title');
    // t.appendChild(a)
    // item_element.appendChild(t);
    item_element.appendChild(create_button('title', item.title, () => display_readable(item.title, item.url)));
    item_element.appendChild(create_div('summary', await invoke("get_short_summary", { htmlSummary: item.summary, len: 100 })));
    items_element.appendChild(item_element);
  }
}

// async function update_items() {
//   // TODO add filter here
//   document.getElementById("items").innerHTML = await invoke("get_items");
// }

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
  if(name != "reader") {
    document.getElementById("reader").innerHTML = "";
  }
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
