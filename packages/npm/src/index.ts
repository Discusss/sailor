import axios, {AxiosError} from "axios";

import {
    DomainGetAdvancedResponse,
    DomainGetAdvancedResponseRaw, DomainGetMasterResponse, DomainGetMasterResponseRaw,
    DomainGetResponse,
    DomainGetResponseRaw
} from "./responses/domainGetResponse";
import {BadRequestError} from "./exceptions/badRequestError";
import {BlacklistedError} from "./exceptions/blacklistedError";
import {InternalServerError} from "./exceptions/internalServerError";
import {Categories} from "./categories";
import {DomainPostResponse} from "./responses/domainPostResponse";

const API_URL = "https://phishing.lacabra.app";

/**
 * Parses a domain to get its netloc.
 * @param rawDomain
 * @returns string
 */
function formatDomain(rawDomain: string): string {
    return rawDomain
        .replace(/https?:\/\//, "")
        .replace(/http?:\/\//, "")
        .split("/")[0];
}

/**
 * Gets basic information about a domain.
 * @param {string} rawDomain 
 * @param {string} urlOverride 
 * @returns {Promise<DomainGetResponse | null>} The found information if any.
 * @throws {BadRequestError} The request wasn't successful processed.
 * @throws {BlacklistedError} Your IP is currently blacklisted.
 * @throws {InternalServerError} There was a server error.
 * @throws {Error} A generic error happened.
 */
export async function getDomain(rawDomain: string, urlOverride: string = API_URL): Promise<DomainGetResponse | null> {
    const domain = formatDomain(rawDomain);

    try {
        const response = await axios.get(`${urlOverride}/api/domain`, {params: {domain}});

        const data: DomainGetResponseRaw = response.data.data;

                return {
                    ...data,
                    submittedBy: data.submitted_by,
                    submittedAt: data.submitted_at
                } as DomainGetResponse;
    } catch (e) {
        if (!(e instanceof AxiosError) || !e.response)
            return null

        switch (e.response.status) {
            case 400:
                throw new BadRequestError(e.response.statusText);
            case 403:
                throw new BlacklistedError(e.response.statusText);
            case 404:
                return null;
            case 500:
                throw new InternalServerError(e.response.statusText);
            default:
                throw new Error(`Unknown error: ${e.response.statusText}`);
        }
    }
}

/**
 * Gets advanced information about a domain.
 * @param {string} rawDomain 
 * @param {string} key
 * @param {string} urlOverride
 * @returns {Promise<DomainGetAdvancedResponse | null>} The found information if any.
 * @throws {BadRequestError} The request wasn't successful processed.
 * @throws {BlacklistedError} Your IP is currently blacklisted.
 * @throws {InternalServerError} There was a server error.
 * @throws {Error} A generic error happened.
 */
export async function getDomainWithApiKey(rawDomain: string, key: string, urlOverride: string): Promise<DomainGetAdvancedResponse | null> {
    const domain = formatDomain(rawDomain);

    try {
        const response = await axios.get(`${urlOverride}/api/domain`, {
            params: {domain},
            headers: {Authorization: `Bearer ${key}`}
        });

        const data: DomainGetAdvancedResponseRaw = response.data;

        return {
            ...data,
            submittedBy: data.submitted_by,
            submittedAt: data.submitted_at,
            timesConsulted: data.times_consulted,
            approvedBy: data.approved_by,
            submittedReason: data.submitted_reason,
            publicNotes: data.public_notes
        } as DomainGetAdvancedResponse;
    } catch (e) {
        if (!(e instanceof AxiosError) || !e.response)
            return null

        switch (e.response.status) {
            case 400:
                throw new BadRequestError(e.response.statusText);
            case 403:
                throw new BlacklistedError(e.response.statusText);
            case 404:
                return null;
            case 500:
                throw new InternalServerError(e.response.statusText);
            default:
                throw new Error(`Unknown error: ${e.response.statusText}`);
        }
    }
}

/**
 * Gets all information about a domain.
 * @param {string} rawDomain 
 * @param {string} key
 * @param {string} urlOverride
 * @returns {Promise<DomainGetAdvancedResponse | null>} The found information if any.
 * @throws {BadRequestError} The request wasn't successful processed.
 * @throws {BlacklistedError} Your IP is currently blacklisted.
 * @throws {InternalServerError} There was a server error.
 * @throws {Error} A generic error happened.
 */
export async function getDomainWithMasterKey(rawDomain: string, masterKey: string, urlOverride: string = API_URL): Promise<DomainGetMasterResponse | null> {
    const domain = formatDomain(rawDomain);

    try {
        const response = await axios.get(`${urlOverride}/api/domain`, {
            params: {domain},
            headers: {Authorization: `Bearer ${masterKey}`}
        });

        const data: DomainGetMasterResponseRaw = response.data.data;

        return {
            ...data,
            submittedBy: data.submitted_by,
            submittedAt: data.submitted_at,
            timesConsulted: data.times_consulted,
            approvedBy: data.approved_by,
            submittedReason: data.submitted_reason,
            publicNotes: data.public_notes,
            submittedIp: data.submitted_ip,
            submittedUserAgent: data.submitted_user_agent,
            approvedKey: data.approved_key
        } as DomainGetMasterResponse;
    } catch (e) {
        if (!(e instanceof AxiosError) || !e.response)
            return null

        switch (e.response.status) {
            case 400:
                throw new BadRequestError(e.response.statusText);
            case 403:
                throw new BlacklistedError(e.response.statusText);
            case 404:
                return null;
            case 500:
                throw new InternalServerError(e.response.statusText);
            default:
                throw new Error(`Unknown error: ${e.response.statusText}`);
        }
    }
}

/**
 * 
 * @param rawDomain 
 * @param category 
 * @param severity 
 * @param notes 
 * @param submittedBy 
 * @param reason 
 * @param urlOverride 
 * @returns {Promise<DomainPostResponse>} The sent domain information.
 * @throws {BadRequestError} The request wasn't successful processed.
 * @throws {BlacklistedError} Your IP is currently blacklisted.
 * @throws {InternalServerError} There was a server error.
 * @throws {Error} A generic error happened.
 */
export async function submitDomain(
    rawDomain: string,
    category: Categories = Categories.Other,
    severity: number = 0,
    notes: string = "",
    submittedBy: string,
    reason: string,
    urlOverride: string = API_URL
): Promise<DomainPostResponse> {
    const domain = formatDomain(rawDomain);

    try {
        const response = await axios.post(`${urlOverride}/api/domain`, {
            domain,
            category: category.valueOf(),
            severity,
            notes,
            submitted_by: submittedBy,
            reason
        });

        const data: DomainGetResponseRaw = response.data;

        return {
            ...data,
            submittedBy: data.submitted_by,
            submittedAt: data.submitted_at
        } as DomainPostResponse;
    } catch (e) {
        if (!(e instanceof AxiosError) || !e.response)
            throw new Error(`Unknown error: ${e}`);

        switch (e.response.status) {
            case 400:
                throw new BadRequestError(e.response.statusText);
            case 403:
                throw new BlacklistedError(e.response.statusText);
            case 500:
                throw new InternalServerError(e.response.statusText);
            default:
                throw new Error(`Unknown error: ${e.response.statusText}`);
        }
    }
}

