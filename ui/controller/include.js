class HTMLController {
    constructor() { }

    includeHTML() {
        const includeArea = document.querySelectorAll('[include-html]');

        for (let dom of includeArea) {
            const url = dom.dataset.include;
            fetch(url)
                .then(response => response.text())
                .then(data => {
                    dom.innerHTML = data;
                    dom.removeAttribute('include-html');
                });
        }
    }
}