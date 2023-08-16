-- Your SQL goes here

create table links
(
    id serial primary key not null,
    domain text not null,
    category varchar(255) not null default 'unknown',
    priority int not null default 0,
    public_notes text not null default '',

    submitted_by text not null, -- the username of the user who submitted the link
    submitted_at timestamp default current_timestamp, -- the time the link was submitted
    submitted_ip text, -- the IP address of the user who submitted the link
    submitted_user_agent text, -- the user agent of the user who submitted the link
    submitted_reason text not null, -- the reason the user submitted the link (e.g. "Phishing link found on <server>")

    approved_by text, -- the username of the user who approved the link
    approved_at timestamp, -- the time the link was approved
    approved_key text, -- the key used to approve the link, references keys(key)

    notes text not null default '' -- the internal notes for the link
);