class BookController {
    constructor() { }

    createNoItemCard(msg) {
        let noItemCardDiv = document.createElement("div");
        noItemCardDiv.className = "m-2";

        let noItemCard = noItemCardDiv.appendChild(document.createElement("div"));
        noItemCard.className = "card p-5";

        let noItemCardBody = noItemCard.appendChild(document.createElement("div"));
        noItemCardBody.className = "card-body";

        let noItemText = noItemCardBody.appendChild(document.createElement("p"));
        noItemText.className = "text-center";
        noItemText.innerHTML = msg;

        return noItemCardDiv;
    }
}