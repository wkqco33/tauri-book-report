class BookReportController extends BookController {
    constructor() {
        super();
    }

    requestAllReportData() {
        return new Promise((resolve, _) => {
            resolve(invoke("request_all_report_data"));
        });
    }

    #createReportListItem(data) {
        let reportItem = document.createElement("div");
        reportItem.className = "m-2";

        let reportCard = reportItem.appendChild(document.createElement("div"));
        reportCard.className = "card";

        let reportCardBody = reportCard.appendChild(document.createElement("div"));
        reportCardBody.className = "card-body";

        let reportTitle = reportCardBody.appendChild(document.createElement("h5"));
        reportTitle.className = "card-title";
        reportTitle.innerHTML = data.title;

        let reportBookName = reportCardBody.appendChild(document.createElement("p"));
        reportBookName.className = "card-text";
        reportBookName.innerHTML = data.book_name;

        let reportAuthor = reportCardBody.appendChild(document.createElement("h6"));
        reportAuthor.className = "card-subtitle mb-2 text-muted";
        reportAuthor.innerHTML = data.author;

        let reportDescriptionAccordion = reportCardBody.appendChild(document.createElement("div"));
        reportDescriptionAccordion.className = "accordion accordion-flush";
        reportDescriptionAccordion.id = "book_report_accordion";

        let reportDescriptionAccordionItem = reportDescriptionAccordion.appendChild(document.createElement("div"));
        reportDescriptionAccordionItem.className = "accordion-item";

        let reportDescriptionAccordionHeader = reportDescriptionAccordionItem.appendChild(document.createElement("h2"));
        reportDescriptionAccordionHeader.className = "accordion-header";
        reportDescriptionAccordionHeader.id = "header_" + data.id;

        let reportDescriptionAccordionButton = reportDescriptionAccordionHeader.appendChild(document.createElement("button"));
        reportDescriptionAccordionButton.className = "accordion-button collapsed";
        reportDescriptionAccordionButton.setAttribute("data-bs-toggle", "collapse");
        reportDescriptionAccordionButton.setAttribute("data-bs-target", "#collapse_" + data.id);
        reportDescriptionAccordionButton.setAttribute("aria-expanded", "false");
        reportDescriptionAccordionButton.setAttribute("aria-controls", "collapse_" + data.id);
        reportDescriptionAccordionButton.id = "button_" + data.id;
        reportDescriptionAccordionButton.type = "button";
        reportDescriptionAccordionButton.innerHTML = "감상문 내용 보기";

        let reportDescriptionAccordionCollapse = reportDescriptionAccordionItem.appendChild(document.createElement("div"));
        reportDescriptionAccordionCollapse.className = "accordion-collapse collapse";
        reportDescriptionAccordionCollapse.id = "collapse_" + data.id;
        reportDescriptionAccordionCollapse.setAttribute("aria-labelledby", "header_" + data.id);
        reportDescriptionAccordionCollapse.setAttribute("data-bs-parent", "#book_report_accordion");

        let reportDescriptionAccordionBody = reportDescriptionAccordionCollapse.appendChild(document.createElement("div"));
        reportDescriptionAccordionBody.className = "accordion-body";
        reportDescriptionAccordionBody.innerHTML = data.description;

        let reportButtonGroup = reportDescriptionAccordionBody.appendChild(document.createElement("div"));
        reportButtonGroup.className = "d-grid gap-2 d-md-flex justify-content-md-end";

        let reportUpdateButton = reportButtonGroup.appendChild(document.createElement("a"));
        reportUpdateButton.className = "btn btn-primary me-md-2";
        reportUpdateButton.innerHTML = "수정";
        reportUpdateButton.addEventListener("click", () => {
            console.log("update report " + data.id);
            window.location.href = `report.html?update=true&id=${data.id}`;
        });

        let reportDeleteButton = reportButtonGroup.appendChild(document.createElement("a"));
        reportDeleteButton.className = "btn btn-danger";
        reportDeleteButton.innerHTML = "삭제";
        reportDeleteButton.addEventListener("click", () => {
            console.log("delete report " + data.id);
            invoke("request_delete_report", { id: data.id }).then((result) => {
                if (result) {
                    alert("삭제되었습니다.");
                    window.location.reload();
                } else {
                    alert("삭제에 실패했습니다.");
                }
            });
        });

        return reportItem;
    }

    showReportData(data) {
        let reportList = document.getElementById("book_report_list");

        if (data == null || Object.keys(data).length == 0) {
            reportList.appendChild(this.createNoItemCard("작성된 감상문이 없습니다. 감상문을 작성해보세요!"));
            return;
        }

        data.forEach((report) => {
            console.log(report);
            reportList.appendChild(this.#createReportListItem(report));
        });
    }
}