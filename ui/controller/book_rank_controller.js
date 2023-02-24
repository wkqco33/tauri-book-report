class BookRankController {
    constructor() { }

    request_rank_data(page) {
        return new Promise((resolve, _) => {
            resolve(invoke("request_rank_data", { page: page }));
        });
    }

    #create_book_card(data) {
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
        book_card_body.className = "card-body";

        let book_card_title = book_card_body.appendChild(document.createElement("h5"));
        book_card_title.className = "card-title";
        book_card_title.innerHTML = data.book_name + (data.volume == null || data.volume == "" ? "" : " - <small>" + data.volume + "권</small>");

        let book_card_author = book_card_body.appendChild(document.createElement("p"));
        book_card_author.className = "card-text";
        book_card_author.innerHTML = data.author;

        let book_card_publisher = book_card_body.appendChild(document.createElement("p"));
        book_card_publisher.className = "card-text";
        book_card_publisher.innerHTML = data.publisher;

        return book_card;
    }

    show_rank_data(data) {
        data.forEach(element => {
            document.querySelector("#book_rank_list").appendChild(this.#create_book_card(element));
        });
    }
}