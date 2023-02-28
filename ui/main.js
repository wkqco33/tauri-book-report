const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  let htmlCon = new HTMLController();
  htmlCon.includeHTML();

  /** block for local testing
  let bookRankCon = new BookRankController();
  let book_rank_data = await bookRankCon.request_rank_data(1);
  bookRankCon.show_rank_data(book_rank_data);
   */

  let bookReportCon = new BookReportController();
  let book_report_data = await bookReportCon.request_all_report_data();
  bookReportCon.show_report_data(book_report_data);
});
