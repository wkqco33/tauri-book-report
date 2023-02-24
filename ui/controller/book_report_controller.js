class BookReportController {
    constructor() { }

    request_all_report_data() {
        return new Promise((resolve, _) => {
            resolve(invoke("request_all_report_data"));
        });
    }

    #create_report_list_item(report) {
        // TODO: 감상문 리스트 아이템 생성
    }

    #create_no_report_item() {
        let no_report_div = document.createElement("div");
        no_report_div.className = "text-center m-5";
        let no_report_text = no_report_div.appendChild(document.createElement("p"));
        no_report_text.className = "text-center";
        no_report_text.innerHTML = "작성된 감상문이 없습니다. 감상문을 작성해보세요!";
        return no_report_div;
    }

    show_report_data(report_data) {
        let report_list = document.getElementById("book_report_list");

        if (report_data == null || Object.keys(report_data).length == 0) {
            report_list.appendChild(this.#create_no_report_item());
            return;
        }

        report_data.forEach((report) => {
            report_list.appendChild(this.#create_report_list_item(report));
        });
    }
}