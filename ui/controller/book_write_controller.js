const { invoke } = window.__TAURI__.tauri;

class BookWriteController {
    constructor() { }

    on_save_report_button_click() {
        let title = document.getElementById("inputTitle").value;
        let bookName = document.getElementById("inputBookName").value;
        let bookAuthor = document.getElementById("inputAuthor").value;
        let bookPublisher = document.getElementById("inputPublisher").value;
        let bookStartDate = document.getElementById("inputStartDate").value;
        let bookEndDate = document.getElementById("inputEndDate").value;
        let bookReport = document.getElementById("inputBookReport").value;

        if (title == "" || bookName == "" || bookReport == "") {
            alert("제목과 책이름, 내용을 입력해주세요.");
            return;
        }

        let book_report_data = {
            "id": 0,
            "title": title,
            "book_name": bookName,
            "author": bookAuthor,
            "publisher": bookPublisher,
            "start_date": bookStartDate,
            "end_date": bookEndDate,
            "description": bookReport
        };

        invoke("request_save_report", { bookReport: book_report_data }).then((result) => {
            if (result) {
                alert("저장되었습니다.");
                window.location.href = "../index.html";
            }
            else {
                alert("저장에 실패하였습니다.");
            }
        });
    }
}

document.querySelector("#save_report").addEventListener("click", () => {
    console.log("save_report button clicked");
    let bookWriteCon = new BookWriteController();
    bookWriteCon.on_save_report_button_click();
});