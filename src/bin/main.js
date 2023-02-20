const { invoke } = window.__TAURI__.tauri;

//let items_element;

async function refresh() {
  var i, ids;
  ids = [];
  let selected_items = document.getElementsByClassName("selected");
  console.log("before");
  for (i = 0; i < selected_items.length; i++) {
    var id = selected_items[i].id;
    if(id.length > 0) {
      console.log(id);
      ids.push(id);
    }
  }
  let refresh_button = document.getElementById("button-refresh");
  refresh_button.className = "active";
  let content = document.getElementById("content");
  content.innerHTML = await invoke("refresh");
  refresh_button.className = "passive";
  console.log("after");
  for (i = 0; i < ids.length; i++) {
    console.log(ids[i]);
    document.getElementById(ids[i]).className += " selected";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .getElementById("button-refresh")
    .addEventListener("click", () => refresh());
});

document.addEventListener('DOMContentLoaded', () => {
  refresh();
})
