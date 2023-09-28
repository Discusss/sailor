export type DomainPostResponse = {
    /**
     * The domain that was submitted
     */
    domain: string,
    /**
     * The category of the domain
     */
    category: string,
    /**
     * The severity of the domain. 0 is the lowest and 10 is the highest depending on how dangerous the domain is.
     * Example: How easy it is for a user to be tricked into giving away their credentials.
     */
    severity: number,
    /**
     * The notes of the domain left by the submitter
     */
    notes: string,
    /**
     * The user that submitted the domain
     */
    submittedBy: string,
    /**
     * The date the domain was submitted
     */
    submittedAt: string
}