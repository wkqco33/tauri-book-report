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

        let report_description_accordion = report_card_body.appendChild(document.createElement("div"));
        report_description_accordion.className = "accordion accordion-flush";
        report_description_accordion.id = "book_report_accordion";

        let report_description_accordion_item = report_description_accordion.appendChild(document.createElement("div"));
        report_description_accordion_item.className = "accordion-item";

        let report_description_accordion_header = report_description_accordion_item.appendChild(document.createElement("h2"));
        report_description_accordion_header.className = "accordion-header";
        report_description_accordion_header.id = "header_" + report.id;

        let report_description_accordion_button = report_description_accordion_header.appendChild(document.createElement("button"));
        report_description_accordion_button.className = "accordion-button collapsed";
        report_description_accordion_button.setAttribute("data-bs-toggle", "collapse");
        report_description_accordion_button.setAttribute("data-bs-target", "#collapse_" + report.id);
        report_description_accordion_button.setAttribute("aria-expanded", "false");
        report_description_accordion_button.setAttribute("aria-controls", "collapse_" + report.id);
        report_description_accordion_button.id = "button_" + report.id;
        report_description_accordion_button.type = "button";
        report_description_accordion_button.innerHTML = "감상문 내용 보기";

        let report_description_accordion_collapse = report_description_accordion_item.appendChild(document.createElement("div"));
        report_description_accordion_collapse.className = "accordion-collapse collapse";
        report_description_accordion_collapse.id = "collapse_" + report.id;
        report_description_accordion_collapse.setAttribute("aria-labelledby", "header_" + report.id);
        report_description_accordion_collapse.setAttribute("data-bs-parent", "#book_report_accordion");

        let report_description_accordion_body = report_description_accordion_collapse.appendChild(document.createElement("div"));
        report_description_accordion_body.className = "accordion-body";
        report_description_accordion_body.innerHTML = report.description;

        let report_button_group = report_description_accordion_body.appendChild(document.createElement("div"));
        report_button_group.className = "d-grid gap-2 d-md-flex justify-content-md-end";

        let report_update_button = report_button_group.appendChild(document.createElement("a"));
        report_update_button.className = "btn btn-primary me-md-2";
        report_update_button.innerHTML = "수정";
        report_update_button.addEventListener("click", () => {
            console.log("update report " + report.id);
            // window.location.href = "book_write.html?id=" + report.id;
        });

        let report_delete_button = report_button_group.appendChild(document.createElement("a"));
        report_delete_button.className = "btn btn-danger";
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