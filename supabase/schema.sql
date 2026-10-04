-- ============================================================================
-- CashFlow – database schema for Supabase (schema version 1)
--
-- Run once in the Supabase dashboard: SQL Editor → New query → paste this
-- whole file → Run. The script is idempotent: running it again is harmless.
--
-- Security model
--   * The desktop app only ships the project's public ("publishable") key.
--   * Every table has row level security. Access requires a signed-in user
--     whose id is listed in public.app_users (see docs/SUPABASE_SETUP.md).
--   * Contribution amounts are computed by the database (generated columns),
--     so stored payroll figures are always consistent with their rates.
-- ============================================================================

begin;

-- ---------------------------------------------------------------------------
-- Helpers
-- ---------------------------------------------------------------------------

-- Who may use the app. Managed by the admin in the SQL editor only.
create table if not exists public.app_users (
    user_id    uuid primary key references auth.users (id) on delete cascade,
    note       text not null default '',
    created_at timestamptz not null default now()
);
alter table public.app_users enable row level security;
revoke all on table public.app_users from anon, authenticated;

create or replace function public.is_app_user()
returns boolean
language sql
stable
security definer
set search_path = ''
as $$
    select exists (select 1 from public.app_users where user_id = (select auth.uid()));
$$;
revoke all on function public.is_app_user() from public, anon;
grant execute on function public.is_app_user() to authenticated;

-- Called by the app right after signing in.
create or replace function public.app_status()
returns json
language sql
stable
set search_path = ''
as $$
    select json_build_object('schema_version', 1, 'is_app_user', public.is_app_user());
$$;
revoke all on function public.app_status() from public, anon;
grant execute on function public.app_status() to authenticated;

-- Harmless endpoint for an optional keep-alive job (free projects pause after
-- a week without activity). Exposes no data.
create or replace function public.ping()
returns text
language sql
stable
set search_path = ''
as $$
    select 'ok'::text;
$$;
revoke all on function public.ping() from public;
grant execute on function public.ping() to anon, authenticated;

-- Maintains created_by / updated_at / updated_by on every write.
create or replace function public.touch_row()
returns trigger
language plpgsql
set search_path = ''
as $$
begin
    if tg_op = 'INSERT' then
        new.created_at := now();
        new.created_by := auth.uid();
    else
        new.created_at := old.created_at;
        new.created_by := old.created_by;
    end if;
    new.updated_at := now();
    new.updated_by := auth.uid();
    return new;
end;
$$;

-- ---------------------------------------------------------------------------
-- Employers (Arbeitgeber)
-- ---------------------------------------------------------------------------
create table if not exists public.employers (
    id              uuid primary key default gen_random_uuid(),
    name            text not null,
    representative  text not null default '',   -- "Vertretung" (second name line)
    company_number  text not null default '',   -- Betriebsnummer, 8 digits
    tax_number      text not null default '',   -- Steuernummer
    street          text not null default '',
    house_number    text not null default '',
    postal_code     text not null default '',
    city            text not null default '',
    created_at      timestamptz not null default now(),
    created_by      uuid,
    updated_at      timestamptz not null default now(),
    updated_by      uuid,
    constraint employers_name_not_blank check (btrim(name) <> ''),
    constraint employers_company_number_format check (company_number = '' or company_number ~ '^[0-9]{8}$'),
    constraint employers_postal_code_format check (postal_code = '' or postal_code ~ '^[0-9]{5}$')
);
create unique index if not exists employers_name_key on public.employers (lower(btrim(name)));

-- ---------------------------------------------------------------------------
-- Employees (Arbeitnehmer)
-- Rates are percentages, e.g. 13.0000 = 13 %.
-- ---------------------------------------------------------------------------
create table if not exists public.employees (
    id                      uuid primary key default gen_random_uuid(),
    employer_id             uuid not null references public.employers (id) on delete restrict,
    personnel_number        text not null,               -- Personalnummer (unique, used in PDF file names)
    first_name              text not null,
    last_name               text not null,
    birth_name              text not null default '',
    street                  text not null default '',
    house_number            text not null default '',
    postal_code             text not null default '',
    city                    text not null default '',
    birth_date              date,
    gender                  text,                        -- m / w / d / x, null = keine Angabe
    nationality             text not null default '',
    social_security_number  text not null default '',   -- SV-Nummer, normalized (no spaces)
    tax_id                  text not null default '',   -- Steuer-ID, 11 digits
    activity_key            text not null default '',   -- Tätigkeitsschlüssel, 9 digits
    contribution_group_key  text not null default '',   -- Beitragsgruppenschlüssel (BGRS), 4 digits
    person_group            text not null default '',   -- Personengruppenschlüssel (PGRS), 3 digits
    transition_zone         smallint,                    -- Übergangsbereich/Gleitzone: 0, 1, 2
    occupation              text not null default '',   -- Berufsbezeichnung
    health_insurer          text not null default '',   -- Krankenkasse (shown on the payslip)
    employment_start        date,                        -- Beschäftigungsbeginn
    employment_end          date,                        -- Austritt
    monthly_salary          numeric(10, 2),              -- mtl. Vergütung
    hourly_rate             numeric(8, 2),               -- Stundensatz (Regiestunden)
    rate_health             numeric(7, 4) not null default 0,  -- Krankenversicherung
    rate_pension            numeric(7, 4) not null default 0,  -- Rentenversicherung
    rate_u1                 numeric(7, 4) not null default 0,  -- Umlage U1
    rate_u2                 numeric(7, 4) not null default 0,  -- Umlage U2
    rate_insolvency         numeric(7, 4) not null default 0,  -- Insolvenzgeldumlage
    rate_flat_tax           numeric(7, 4) not null default 0,  -- Pauschalsteuer
    created_at              timestamptz not null default now(),
    created_by              uuid,
    updated_at              timestamptz not null default now(),
    updated_by              uuid,
    constraint employees_personnel_number_format check (personnel_number ~ '^[A-Za-z0-9][A-Za-z0-9._-]{0,19}$'),
    constraint employees_first_name_not_blank check (btrim(first_name) <> ''),
    constraint employees_last_name_not_blank check (btrim(last_name) <> ''),
    constraint employees_gender_values check (gender is null or gender in ('m', 'w', 'd', 'x')),
    constraint employees_postal_code_format check (postal_code = '' or postal_code ~ '^[0-9]{5}$'),
    constraint employees_ssn_format check (social_security_number = '' or social_security_number ~ '^[0-9]{8}[A-Z][0-9]{3}$'),
    constraint employees_tax_id_format check (tax_id = '' or tax_id ~ '^[1-9][0-9]{10}$'),
    constraint employees_activity_key_format check (activity_key = '' or activity_key ~ '^[0-9]{9}$'),
    constraint employees_contribution_group_key_format check (contribution_group_key = '' or contribution_group_key ~ '^[0-9]{4}$'),
    constraint employees_person_group_format check (person_group = '' or person_group ~ '^[0-9]{3}$'),
    constraint employees_transition_zone_values check (transition_zone is null or transition_zone between 0 and 2),
    constraint employees_employment_period check (employment_end is null or employment_start is null or employment_end >= employment_start),
    constraint employees_monthly_salary_positive check (monthly_salary is null or monthly_salary >= 0),
    constraint employees_hourly_rate_positive check (hourly_rate is null or hourly_rate >= 0),
    constraint employees_rates_range check (
        rate_health between 0 and 100 and rate_pension between 0 and 100 and rate_u1 between 0 and 100
        and rate_u2 between 0 and 100 and rate_insolvency between 0 and 100 and rate_flat_tax between 0 and 100
    ),
    constraint employees_personnel_number_key unique (personnel_number)
);
create index if not exists employees_employer_idx on public.employees (employer_id);

-- ---------------------------------------------------------------------------
-- Monthly wage records (Lohnerfassung). One row per employee and month.
-- The rates are a snapshot taken when the month was recorded, so later rate
-- changes never alter past payslips.
-- ---------------------------------------------------------------------------
create table if not exists public.payroll_records (
    id                  uuid primary key default gen_random_uuid(),
    employee_id         uuid not null references public.employees (id) on delete cascade,
    year                smallint not null,
    month               smallint not null,
    statement_date      date not null,                      -- Abrechnungsdatum (printed on the payslip)
    base_pay            numeric(10, 2) not null,            -- Brutto-Bezüge
    extra_pay           numeric(10, 2) not null default 0,  -- Sonstige Brutto-Bezüge (Regiestunden)
    payout              numeric(10, 2) not null,            -- Auszahlungsbetrag
    rate_health         numeric(7, 4) not null,
    rate_pension        numeric(7, 4) not null,
    rate_u1             numeric(7, 4) not null,
    rate_u2             numeric(7, 4) not null,
    rate_insolvency     numeric(7, 4) not null,
    rate_flat_tax       numeric(7, 4) not null,
    gross_total         numeric(10, 2) generated always as (base_pay + extra_pay) stored,
    amount_health       numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_health / 100, 2)) stored,
    amount_pension      numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_pension / 100, 2)) stored,
    amount_u1           numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_u1 / 100, 2)) stored,
    amount_u2           numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_u2 / 100, 2)) stored,
    amount_insolvency   numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_insolvency / 100, 2)) stored,
    amount_flat_tax     numeric(10, 2) generated always as (round((base_pay + extra_pay) * rate_flat_tax / 100, 2)) stored,
    total_contribution  numeric(10, 2) generated always as (
        round((base_pay + extra_pay) * rate_health / 100, 2)
        + round((base_pay + extra_pay) * rate_pension / 100, 2)
        + round((base_pay + extra_pay) * rate_u1 / 100, 2)
        + round((base_pay + extra_pay) * rate_u2 / 100, 2)
        + round((base_pay + extra_pay) * rate_insolvency / 100, 2)
        + round((base_pay + extra_pay) * rate_flat_tax / 100, 2)
    ) stored,
    created_at          timestamptz not null default now(),
    created_by          uuid,
    updated_at          timestamptz not null default now(),
    updated_by          uuid,
    constraint payroll_records_period_key unique (employee_id, year, month),
    constraint payroll_records_year_range check (year between 2000 and 2099),
    constraint payroll_records_month_range check (month between 1 and 12),
    constraint payroll_records_amounts_positive check (base_pay >= 0 and extra_pay >= 0 and payout >= 0),
    constraint payroll_records_payout_not_above_gross check (payout <= base_pay + extra_pay),
    constraint payroll_records_rates_range check (
        rate_health between 0 and 100 and rate_pension between 0 and 100 and rate_u1 between 0 and 100
        and rate_u2 between 0 and 100 and rate_insolvency between 0 and 100 and rate_flat_tax between 0 and 100
    )
);

-- ---------------------------------------------------------------------------
-- Monthly time records (Zeiterfassung). Independent of the wage record.
-- ---------------------------------------------------------------------------
create table if not exists public.time_records (
    id              uuid primary key default gen_random_uuid(),
    employee_id     uuid not null references public.employees (id) on delete cascade,
    year            smallint not null,
    month           smallint not null,
    hours_worked    numeric(6, 2) not null,               -- Arbeitszeit
    extra_hours     numeric(6, 2) not null default 0,     -- Arbeitszeit Regie
    total_hours     numeric(6, 2) generated always as (hours_worked + extra_hours) stored,
    vacation_days   numeric(4, 1) not null default 0,     -- Urlaubstage
    sick_days       numeric(4, 1) not null default 0,     -- Krankheitstage
    created_at      timestamptz not null default now(),
    created_by      uuid,
    updated_at      timestamptz not null default now(),
    updated_by      uuid,
    constraint time_records_period_key unique (employee_id, year, month),
    constraint time_records_year_range check (year between 2000 and 2099),
    constraint time_records_month_range check (month between 1 and 12),
    constraint time_records_hours_range check (hours_worked between 0 and 744 and extra_hours between 0 and 744),
    constraint time_records_days_range check (vacation_days between 0 and 31 and sick_days between 0 and 31)
);

-- ---------------------------------------------------------------------------
-- Shared settings (exactly one row): defaults for new employees.
-- Initial values: commercial Minijob rates 2026 (editable in the app).
-- ---------------------------------------------------------------------------
create table if not exists public.settings (
    id                      smallint primary key default 1,
    default_rate_health     numeric(7, 4) not null default 13,
    default_rate_pension    numeric(7, 4) not null default 15,
    default_rate_u1         numeric(7, 4) not null default 0.8,
    default_rate_u2         numeric(7, 4) not null default 0.22,
    default_rate_insolvency numeric(7, 4) not null default 0.15,
    default_rate_flat_tax   numeric(7, 4) not null default 2,
    default_health_insurer  text not null default '',
    created_at              timestamptz not null default now(),
    created_by              uuid,
    updated_at              timestamptz not null default now(),
    updated_by              uuid,
    constraint settings_single_row check (id = 1),
    constraint settings_rates_range check (
        default_rate_health between 0 and 100 and default_rate_pension between 0 and 100
        and default_rate_u1 between 0 and 100 and default_rate_u2 between 0 and 100
        and default_rate_insolvency between 0 and 100 and default_rate_flat_tax between 0 and 100
    )
);
insert into public.settings (id) values (1) on conflict (id) do nothing;

-- ---------------------------------------------------------------------------
-- Triggers, grants and row level security for all data tables
-- ---------------------------------------------------------------------------
do $$
declare
    t text;
begin
    foreach t in array array['employers', 'employees', 'payroll_records', 'time_records', 'settings'] loop
        execute format('drop trigger if exists touch_row on public.%I', t);
        execute format('create trigger touch_row before insert or update on public.%I
                        for each row execute function public.touch_row()', t);

        execute format('alter table public.%I enable row level security', t);
        execute format('revoke all on table public.%I from anon, authenticated', t);
        execute format('drop policy if exists app_users_only on public.%I', t);
        execute format('create policy app_users_only on public.%I for all to authenticated
                        using ((select public.is_app_user())) with check ((select public.is_app_user()))', t);
    end loop;
end
$$;

grant select, insert, update, delete on table
    public.employers, public.employees, public.payroll_records, public.time_records
    to authenticated;
-- The settings row is only read and updated, never created or deleted by the app.
grant select, update on table public.settings to authenticated;

commit;
