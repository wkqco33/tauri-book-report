class BookReportController {
    constructor() { }

    request_rank_data(page) {
        return new Promise((resolve, _) => {
            resolve(invoke("request_rank_data", { page: page }));
        });
    }
}