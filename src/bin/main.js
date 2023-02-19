const { invoke } = window.__TAURI__.tauri;

//let items_element;

async function refresh() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let refresh_button = document.getElementById("button-refresh");
  refresh_button.className = "active";
  let content = document.getElementById("content");
  content.innerHTML = await invoke("refresh");
  refresh_button.className = "passive";
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .getElementById("button-refresh")
    .addEventListener("click", () => refresh());
});

document.addEventListener('DOMContentLoaded', () => {
  refresh();
})
