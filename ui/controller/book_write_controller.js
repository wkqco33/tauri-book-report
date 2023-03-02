const { invoke } = window.__TAURI__.tauri;

class BookWriteController {
    constructor() {
        this.title = document.getElementById("inputTitle");
        this.bookName = document.getElementById("inputBookName");
        this.bookAuthor = document.getElementById("inputAuthor");
        this.bookPublisher = document.getElementById("inputPublisher");
        this.bookStartDate = document.getElementById("inputStartDate");
        this.bookEndDate = document.getElementById("inputEndDate");
        this.bookReport = document.getElementById("inputBookReport");
    }

    setReportData(data) {
        this.title.value = data.title;
        this.bookName.value = data.book_name;
        this.bookAuthor.value = data.author;
        this.bookPublisher.value = data.publisher;
        this.bookStartDate.value = data.start_date;
        this.bookEndDate.value = data.end_date;
        this.bookReport.value = data.description;
    }

    onSaveReportButtonClick(reportId) {
        if (this.title.value == "" || this.bookName.value == "" || this.bookReport.value == "") {
            alert("제목과 책이름, 내용을 입력해주세요.");
            return;
        }

        let bookReportData = {
            "id": reportId,
            "title": this.title.value,
            "book_name": this.bookName.value,
            "author": this.bookAuthor.value,
            "publisher": this.bookPublisher.value,
            "start_date": this.bookStartDate.value,
            "end_date": this.bookEndDate.value,
            "description": this.bookReport.value,
            "created_at": "",
            "updated_at": ""
        };

        let requestType = reportId === 0 ? "request_save_report" : "request_update_report";

        invoke(requestType, { bookReport: bookReportData }).then((result) => {
            if (result) {
                alert("저장되었습니다.");
                window.location.href = "index.html";
            }
            else {
                alert("저장에 실패하였습니다.");
            }
        });
    }
}

const urlParams = new URLSearchParams(window.location.search);
let bookWriteCon = new BookWriteController();
let reportId = 0

if (urlParams.has("id")) {
    reportId = parseInt(urlParams.get("id"));
    invoke("request_report_data", { id: reportId }).then((result) => {
        bookWriteCon.setReportData(result);
    });
}

document.querySelector("#save_report").addEventListener("click", () => {
    console.log("save_report button clicked");
    bookWriteCon.onSaveReportButtonClick(reportId);
});