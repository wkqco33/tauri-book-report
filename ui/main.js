const { invoke } = window.__TAURI__.tauri;

async function request_rank_data() {
  invoke("request_rank_data", { page: 1 }).then((res) => {
    console.log(res);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  // request_rank_data();
});
