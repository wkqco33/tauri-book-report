const { invoke } = window.__TAURI__.tauri;

async function greet() {
  invoke("greet", { name: "Test" }).then((res) => {
    console.log(res);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  greet();
});
