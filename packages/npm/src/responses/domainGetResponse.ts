export type DomainGetResponseRaw = {
    /**
     * The domain that was checked
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
     * The public notes of the domain left by the moderators
     */
    notes: string,
    /**
     * The user that submitted the domain
     */
    submitted_by: string,
    /**
     * The date the domain was submitted
     */
    submitted_at: string
}

export type DomainGetResponse =
    Omit<DomainGetResponseRaw, "submitted_at">
    & Omit<DomainGetResponseRaw, "submitted_by">
    & { submittedBy: string, submittedAt: string }

export type DomainGetAdvancedResponseRaw = {
    /**
     * The internal ID of the domain
     */
    id: number,
    /**
     * The domain that was checked
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
     * The note the moderator left when approving the domain
     */
    notes: string,
    /**
     * The user that submitted the domain
     */
    submitted_by: string,
    /**
     * The date the domain was submitted
     */
    submitted_at: string,
    /**
     * The public notes of the domain left by the moderators
     */
    public_notes: string,
    /**
     * The reason the user gave when submitting the domain
     */
    submitted_reason: string,
    /**
     * The moderator that approved the domain
     */
    approved_by: string,
    /**
     * How many times the domain has been consulted
     */
    times_consulted: number
}

export type DomainGetAdvancedResponse =
    Omit<DomainGetAdvancedResponseRaw, "submitted_at">
    & Omit<DomainGetAdvancedResponseRaw, "submitted_by">
    & Omit<DomainGetAdvancedResponseRaw, "times_consulted">
    & Omit<DomainGetAdvancedResponseRaw, "approved_by">
    & Omit<DomainGetAdvancedResponseRaw, "submitted_reason">
    & Omit<DomainGetAdvancedResponseRaw, "public_notes">
    & {
    submittedBy: string,
    submittedAt: string,
    timesConsulted: number,
    approvedBy: string,
    submittedReason: string,
    publicNotes: string
}


export type DomainGetMasterResponseRaw = {
    /**
     * The internal ID of the domain
     */
    id: number,
    /**
     * The domain that was checked
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
     * The note the moderator left when approving the domain
     */
    notes: string,
    /**
     * The user that submitted the domain
     */
    submitted_by: string,
    /**
     * The date the domain was submitted
     */
    submitted_at: string,
    /**
     * The public notes of the domain left by the moderators
     */
    public_notes: string,
    /**
     * The reason the user gave when submitting the domain
     */
    submitted_reason: string,
    /**
     * The moderator that approved the domain
     */
    approved_by: string,
    /**
     * How many times the domain has been consulted
     */
    times_consulted: number,
    /**
     * The IP address of the user that submitted the domain, only if the key is a master key
     */
    submitted_ip: string,
    /**
     * The user agent of the user that submitted the domain, only if the key is a master key
     */
    submitted_user_agent: string,
    /**
     * The key that was used to approve the domain, only if the key is a master key
     */
    approved_key: string,
}

export type DomainGetMasterResponse =
    Omit<DomainGetMasterResponseRaw, "submitted_at">
    & Omit<DomainGetMasterResponseRaw, "submitted_by">
    & Omit<DomainGetMasterResponseRaw, "times_consulted">
    & Omit<DomainGetMasterResponseRaw, "approved_by">
    & Omit<DomainGetMasterResponseRaw, "submitted_reason">
    & Omit<DomainGetMasterResponseRaw, "public_notes">
    & Omit<DomainGetMasterResponseRaw, "approved_key">
    & Omit<DomainGetMasterResponseRaw, "submitted_user_agent">
    & Omit<DomainGetMasterResponseRaw, "submitted_ip">
    & {
    submittedBy: string,
    submittedAt: string,
    timesConsulted: number,
    approvedBy: string,
    submittedReason: string,
    publicNotes: string,
    approvedKey: string,
    submittedUserAgent: string,
    submittedIp: string
}