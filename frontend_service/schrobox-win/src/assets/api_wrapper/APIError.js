
export default class APIError {
    constructor(message="", reason="", statuscode=500) {
        this.message = message;
        this.reason = reason;
        this.status = statuscode
    }

    log() {
        console.error(this.type, ": ", this.message)
    }
}