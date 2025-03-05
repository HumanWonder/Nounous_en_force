--
-- PostgreSQL database dump
--

-- Dumped from database version 16.6 (Ubuntu 16.6-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.6 (Ubuntu 16.6-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: myuser
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO myuser;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: myuser
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO myuser;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO myuser;

--
-- Name: nurseries; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.nurseries (
    id integer NOT NULL,
    referent_id integer NOT NULL,
    name character varying(255) NOT NULL,
    address text NOT NULL,
    organization_type text,
    tel_number character varying(20),
    mail_address text,
    website text
);


ALTER TABLE public.nurseries OWNER TO myuser;

--
-- Name: nurseries_id_seq; Type: SEQUENCE; Schema: public; Owner: myuser
--

CREATE SEQUENCE public.nurseries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.nurseries_id_seq OWNER TO myuser;

--
-- Name: nurseries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: myuser
--

ALTER SEQUENCE public.nurseries_id_seq OWNED BY public.nurseries.id;


--
-- Name: owners; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.owners (
    id integer NOT NULL,
    client_id integer NOT NULL,
    last_name character varying(255) NOT NULL,
    first_name character varying(255) NOT NULL,
    job_position text,
    tel_number character varying(20),
    address text
);


ALTER TABLE public.owners OWNER TO myuser;

--
-- Name: owners_id_seq; Type: SEQUENCE; Schema: public; Owner: myuser
--

CREATE SEQUENCE public.owners_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.owners_id_seq OWNER TO myuser;

--
-- Name: owners_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: myuser
--

ALTER SEQUENCE public.owners_id_seq OWNED BY public.owners.id;


--
-- Name: temps; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.temps (
    id integer NOT NULL,
    client_id integer NOT NULL,
    last_name character varying(255) NOT NULL,
    first_name character varying(255) NOT NULL,
    tel_number character varying(20),
    address text,
    disponibilities jsonb DEFAULT '{}'::jsonb NOT NULL
);


ALTER TABLE public.temps OWNER TO myuser;

--
-- Name: temps_id_seq; Type: SEQUENCE; Schema: public; Owner: myuser
--

CREATE SEQUENCE public.temps_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.temps_id_seq OWNER TO myuser;

--
-- Name: temps_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: myuser
--

ALTER SEQUENCE public.temps_id_seq OWNED BY public.temps.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.users (
    id integer NOT NULL,
    email text NOT NULL,
    hashed_password text NOT NULL,
    is_validated boolean DEFAULT false,
    created_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP,
    role character varying(50) DEFAULT 'user'::character varying NOT NULL,
    is_profile_validated boolean DEFAULT false
);


ALTER TABLE public.users OWNER TO myuser;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: myuser
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO myuser;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: myuser
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: work_schedule; Type: TABLE; Schema: public; Owner: myuser
--

CREATE TABLE public.work_schedule (
    id integer NOT NULL,
    nursery_id integer NOT NULL,
    date timestamp without time zone NOT NULL,
    address text NOT NULL
);


ALTER TABLE public.work_schedule OWNER TO myuser;

--
-- Name: work_schedule_id_seq; Type: SEQUENCE; Schema: public; Owner: myuser
--

CREATE SEQUENCE public.work_schedule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.work_schedule_id_seq OWNER TO myuser;

--
-- Name: work_schedule_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: myuser
--

ALTER SEQUENCE public.work_schedule_id_seq OWNED BY public.work_schedule.id;


--
-- Name: nurseries id; Type: DEFAULT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.nurseries ALTER COLUMN id SET DEFAULT nextval('public.nurseries_id_seq'::regclass);


--
-- Name: owners id; Type: DEFAULT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.owners ALTER COLUMN id SET DEFAULT nextval('public.owners_id_seq'::regclass);


--
-- Name: temps id; Type: DEFAULT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.temps ALTER COLUMN id SET DEFAULT nextval('public.temps_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: work_schedule id; Type: DEFAULT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.work_schedule ALTER COLUMN id SET DEFAULT nextval('public.work_schedule_id_seq'::regclass);


--
-- Data for Name: __diesel_schema_migrations; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.__diesel_schema_migrations (version, run_on) FROM stdin;
00000000000000	2025-02-21 10:12:34.996725
20250221094436	2025-02-21 13:34:19.395723
20250224124731	2025-02-24 13:27:53.416505
20250226144242	2025-02-26 14:48:23.460881
\.


--
-- Data for Name: nurseries; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.nurseries (id, referent_id, name, address, organization_type, tel_number, mail_address, website) FROM stdin;
\.


--
-- Data for Name: owners; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.owners (id, client_id, last_name, first_name, job_position, tel_number, address) FROM stdin;
\.


--
-- Data for Name: temps; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.temps (id, client_id, last_name, first_name, tel_number, address, disponibilities) FROM stdin;
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.users (id, email, hashed_password, is_validated, created_at, role, is_profile_validated) FROM stdin;
\.


--
-- Data for Name: work_schedule; Type: TABLE DATA; Schema: public; Owner: myuser
--

COPY public.work_schedule (id, nursery_id, date, address) FROM stdin;
\.


--
-- Name: nurseries_id_seq; Type: SEQUENCE SET; Schema: public; Owner: myuser
--

SELECT pg_catalog.setval('public.nurseries_id_seq', 1, false);


--
-- Name: owners_id_seq; Type: SEQUENCE SET; Schema: public; Owner: myuser
--

SELECT pg_catalog.setval('public.owners_id_seq', 1, false);


--
-- Name: temps_id_seq; Type: SEQUENCE SET; Schema: public; Owner: myuser
--

SELECT pg_catalog.setval('public.temps_id_seq', 1, false);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: myuser
--

SELECT pg_catalog.setval('public.users_id_seq', 34, true);


--
-- Name: work_schedule_id_seq; Type: SEQUENCE SET; Schema: public; Owner: myuser
--

SELECT pg_catalog.setval('public.work_schedule_id_seq', 1, false);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: nurseries nurseries_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.nurseries
    ADD CONSTRAINT nurseries_pkey PRIMARY KEY (id);


--
-- Name: owners owners_client_id_key; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.owners
    ADD CONSTRAINT owners_client_id_key UNIQUE (client_id);


--
-- Name: owners owners_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.owners
    ADD CONSTRAINT owners_pkey PRIMARY KEY (id);


--
-- Name: temps temps_client_id_key; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.temps
    ADD CONSTRAINT temps_client_id_key UNIQUE (client_id);


--
-- Name: temps temps_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.temps
    ADD CONSTRAINT temps_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: work_schedule work_schedule_pkey; Type: CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.work_schedule
    ADD CONSTRAINT work_schedule_pkey PRIMARY KEY (id);


--
-- Name: nurseries nurseries_referent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.nurseries
    ADD CONSTRAINT nurseries_referent_id_fkey FOREIGN KEY (referent_id) REFERENCES public.owners(client_id) ON DELETE SET NULL;


--
-- Name: owners owners_client_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.owners
    ADD CONSTRAINT owners_client_id_fkey FOREIGN KEY (client_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: temps temps_client_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.temps
    ADD CONSTRAINT temps_client_id_fkey FOREIGN KEY (client_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: work_schedule work_schedule_nursery_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: myuser
--

ALTER TABLE ONLY public.work_schedule
    ADD CONSTRAINT work_schedule_nursery_id_fkey FOREIGN KEY (nursery_id) REFERENCES public.nurseries(id) ON DELETE CASCADE;


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT ALL ON SCHEMA public TO myuser;


--
-- PostgreSQL database dump complete
--

