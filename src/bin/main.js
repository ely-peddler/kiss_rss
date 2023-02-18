const { invoke } = window.__TAURI__.tauri;

//let items_element;

async function refresh() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  document
    .querySelector("#refresh").className = "active";
  document.querySelector("#items").innerHTML = await invoke("refresh");
  document
    .querySelector("#refresh").className = "passive";
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#refresh")
    .addEventListener("click", () => refresh());
});

document.addEventListener('DOMContentLoaded', () => {
  refresh();
})



