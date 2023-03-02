const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", async () => {
  let bookRankCon = new BookRankController();
  let bookRankData = await bookRankCon.requestRankData(1);
  bookRankCon.showRankData(bookRankData);
  // bookRankCon.showRankData(""); // Test code

  let bookReportCon = new BookReportController();
  let bookReportData = await bookReportCon.requestAllReportData();
  bookReportCon.showReportData(bookReportData);
});
