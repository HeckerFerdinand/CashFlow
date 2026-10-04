-- ============================================================================
-- Local stand-in for the parts of Supabase that schema.sql relies on.
--
-- ONLY for the local test database started by scripts/dev-db.sh.
-- Never run this on the real Supabase project: there these objects already
-- exist (and are managed by Supabase).
--
-- Mirrors Supabase's definitions of the API roles, the auth.users table and
-- the auth.uid() / auth.role() / auth.jwt() helper functions.
-- ============================================================================

do $$
begin
    if not exists (select 1 from pg_roles where rolname = 'anon') then
        create role anon nologin noinherit;
    end if;
    if not exists (select 1 from pg_roles where rolname = 'authenticated') then
        create role authenticated nologin noinherit;
    end if;
    if not exists (select 1 from pg_roles where rolname = 'service_role') then
        create role service_role nologin noinherit bypassrls;
    end if;
    if not exists (select 1 from pg_roles where rolname = 'authenticator') then
        create role authenticator login noinherit password 'authenticator';
    end if;
end
$$;

grant anon, authenticated, service_role to authenticator;

create schema if not exists auth;
grant usage on schema auth to anon, authenticated, service_role;

create table if not exists auth.users (
    id    uuid primary key,
    email text unique
);

create or replace function auth.uid() returns uuid
language sql stable as $$
    select coalesce(
        nullif(current_setting('request.jwt.claim.sub', true), ''),
        (nullif(current_setting('request.jwt.claims', true), '')::jsonb ->> 'sub')
    )::uuid
$$;

create or replace function auth.role() returns text
language sql stable as $$
    select coalesce(
        nullif(current_setting('request.jwt.claim.role', true), ''),
        (nullif(current_setting('request.jwt.claims', true), '')::jsonb ->> 'role')
    )::text
$$;

create or replace function auth.jwt() returns jsonb
language sql stable as $$
    select coalesce(
        nullif(current_setting('request.jwt.claim', true), ''),
        nullif(current_setting('request.jwt.claims', true), '')
    )::jsonb
$$;

grant execute on function auth.uid(), auth.role(), auth.jwt() to anon, authenticated, service_role;

-- Supabase installs pgcrypto/uuid helpers in the "extensions" schema; Postgres 13+
-- has gen_random_uuid() built in, which is all schema.sql uses.
