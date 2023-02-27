class BookReportController {
    constructor() { }

    request_all_report_data() {
        return new Promise((resolve, _) => {
            resolve(invoke("request_all_report_data"));
        });
    }

    #create_report_list_item(report) {
        let report_item = document.createElement("div");
        report_item.className = "m-2";

        let report_card = report_item.appendChild(document.createElement("div"));
        report_card.className = "card";

        let report_card_body = report_card.appendChild(document.createElement("div"));
        report_card_body.className = "card-body";

        let report_title = report_card_body.appendChild(document.createElement("h5"));
        report_title.className = "card-title";
        report_title.innerHTML = report.title;

        let report_book_name = report_card_body.appendChild(document.createElement("p"));
        report_book_name.className = "card-text";
        report_book_name.innerHTML = report.book_name;

        let report_author = report_card_body.appendChild(document.createElement("h6"));
        report_author.className = "card-subtitle mb-2 text-muted";
        report_author.innerHTML = report.author;

        let report_delete_button = report_card_body.appendChild(document.createElement("a"));
        report_delete_button.className = "btn btn-danger btn-sm float-end";
        report_delete_button.innerHTML = "삭제";
        report_delete_button.addEventListener("click", () => {
            console.log("delete report " + report.id);
            invoke("request_delete_report", { id: report.id }).then((result) => {
                if (result) {
                    alert("삭제되었습니다.");
                    window.location.reload();
                } else {
                    alert("삭제에 실패했습니다.");
                }
            });
        });

        return report_item;
    }

    #create_no_report_item() {
        let no_report_div = document.createElement("div");
        no_report_div.className = "m-2";

        let no_report_card = no_report_div.appendChild(document.createElement("div"));
        no_report_card.className = "card p-5";

        let no_report_card_body = no_report_card.appendChild(document.createElement("div"));
        no_report_card_body.className = "card-body";

        let no_report_text = no_report_card_body.appendChild(document.createElement("p"));
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
            console.log(report);
            report_list.appendChild(this.#create_report_list_item(report));
        });
    }
}