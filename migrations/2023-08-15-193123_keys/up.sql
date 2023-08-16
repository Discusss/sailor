-- Your SQL goes here

create table keys
(
    key text primary key unique not null, -- the key itself

    created_at timestamp not null default current_timestamp, -- when the key was created
    expires_at timestamp not null default current_timestamp + interval '1 year', -- when the key expires, default 1 year
    last_used_at timestamp not null default current_timestamp, -- when the key was last used

    owner text not null default 'internal', -- username of the owner of the key
    uses integer not null default 0, -- how many times the key has been used

    ips text[] not null default '{}', -- which IPs have used the key
    user_agent text not null default 'unknown', -- which user agents have used the key (last one)

    created_by text not null default 'system', -- who created the key
    notes text not null default '' -- internal notes about the key
);