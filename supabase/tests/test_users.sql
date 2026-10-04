-- Fixed users for the local integration tests (scripts/dev-db.sh only).
-- "allowed" is unlocked in app_users, "blocked" is a signed-in user without access.
insert into auth.users (id, email) values
    ('00000000-0000-0000-0000-00000000a11d', 'allowed@test.local'),
    ('00000000-0000-0000-0000-0000000b10cd', 'blocked@test.local')
on conflict (id) do nothing;

insert into public.app_users (user_id, note)
values ('00000000-0000-0000-0000-00000000a11d', 'integration tests')
on conflict (user_id) do nothing;
