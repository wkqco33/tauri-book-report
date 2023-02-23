const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  let bookReportCon = new BookReportController();
  let book_rank_data = await bookReportCon.request_rank_data(1);
  
  book_rank_data.forEach(element => {
    let book_name = document.createElement("div");
    book_name.innerHTML = element.book_name;
    document.querySelector("#book_rank_list").appendChild(book_name)
  });
});
