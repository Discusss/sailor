export class BlacklistedError extends Error {
    constructor(message: string) {
        super(message);
        this.name = "BlacklistedError";
    }
}