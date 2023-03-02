class BookRankController extends BookController {
    constructor() {
        super();
    }

    requestRankData(page) {
        return new Promise((resolve, _) => {
            resolve(invoke("request_rank_data", { page: page }));
        });
    }

    #createBookCard(data) {
        let book_card = document.createElement("div");
        book_card.className = "card p-2 m-2";

        let book_card_row = book_card.appendChild(document.createElement("div"));
        book_card_row.className = "row g-0";

        let book_card_col = book_card_row.appendChild(document.createElement("div"));
        book_card_col.className = "col-md-4";

        let book_card_img = book_card_col.appendChild(document.createElement("img"));
        book_card_img.className = "img-fluid rounded-start";
        book_card_img.src = data.image_url;
        book_card_img.alt = "...";
        book_card_img.style = "min-width: 200px;";

        let book_card_col2 = book_card_row.appendChild(document.createElement("div"));
        book_card_col2.className = "col-md-8";

        let book_card_body = book_card_col2.appendChild(document.createElement("div"));
        book_card_body.className = "card-body m-2";

        let book_card_title = book_card_body.appendChild(document.createElement("h5"));
        book_card_title.className = "card-title text-center";
        book_card_title.innerHTML = data.book_name + (data.volume == null || data.volume == "" ? "" : " - <small>" + data.volume + "권</small>");

        let book_card_publisher = book_card_body.appendChild(document.createElement("p"));
        book_card_publisher.className = "card-text text-end mt-3";
        book_card_publisher.innerHTML = data.publisher;

        let book_card_author = book_card_body.appendChild(document.createElement("p"));
        book_card_author.className = "card-text text-end";
        book_card_author.innerHTML = "<strong>" + data.author + "</strong>";

        let book_wish_btn = book_card_body.appendChild(document.createElement("button"));
        book_wish_btn.className = "btn btn-info position-absolute bottom-0 end-0 m-3";
        book_wish_btn.innerHTML = "위시리스트에 추가";
        book_wish_btn.onclick = () => {

        };

        return book_card;
    }

    showRankData(data) {
        let reportList = document.querySelector("#book_rank_list");

        if (data == null || Object.keys(data).length == 0) {
            reportList.appendChild(this.createNoItemCard("랭킹 데이터를 불러올 수 없습니다."));
            return;
        }

        data.forEach(element => {
            reportList.appendChild(this.#createBookCard(element));
        });
    }
}