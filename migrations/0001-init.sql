--
-- PostgreSQL database dump
--

-- Dumped from database version 14.22 (Homebrew)
-- Dumped by pg_dump version 14.18 (Homebrew)

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
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: vms
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO vms;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: vms
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


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO vms;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO vms;

--
-- Name: accounts; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.accounts (
    id integer NOT NULL,
    username text NOT NULL,
    password text NOT NULL,
    pin text,
    pic text,
    last_login_at timestamp without time zone,
    gender_wz smallint NOT NULL,
    accepted_tos boolean DEFAULT false NOT NULL,
    banned boolean DEFAULT false NOT NULL,
    admin boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.accounts OWNER TO vms;

--
-- Name: accounts_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.accounts_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.accounts_id_seq OWNER TO vms;

--
-- Name: accounts_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: vms
--

ALTER SEQUENCE public.accounts_id_seq OWNED BY public.accounts.id;


--
-- Name: character_limits; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.character_limits (
    id integer NOT NULL,
    acc_id integer NOT NULL,
    world_id smallint NOT NULL,
    char_max smallint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.character_limits OWNER TO vms;

--
-- Name: characters; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.characters (
    id integer NOT NULL,
    acc_id integer NOT NULL,
    world_id smallint NOT NULL,
    map_wz integer NOT NULL,
    ign text NOT NULL,
    level smallint DEFAULT 1 NOT NULL,
    exp integer DEFAULT 0 NOT NULL,
    strength smallint DEFAULT 4 NOT NULL,
    dexterity smallint DEFAULT 4 NOT NULL,
    luck smallint DEFAULT 4 NOT NULL,
    intelligence smallint DEFAULT 4 NOT NULL,
    hp integer DEFAULT 50 NOT NULL,
    mp smallint DEFAULT 5 NOT NULL,
    max_hp integer DEFAULT 50 NOT NULL,
    max_mp smallint DEFAULT 0 NOT NULL,
    ap smallint DEFAULT 0 NOT NULL,
    sp smallint DEFAULT 0 NOT NULL,
    fame smallint DEFAULT 0 NOT NULL,
    meso integer DEFAULT 0 NOT NULL,
    job_wz smallint NOT NULL,
    face_wz integer NOT NULL,
    hair_wz integer NOT NULL,
    hair_color_wz integer NOT NULL,
    skin_wz integer NOT NULL,
    gender_wz smallint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    last_portal smallint DEFAULT 0 NOT NULL
);


ALTER TABLE public.characters OWNER TO vms;

--
-- Name: characters_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.characters_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.characters_id_seq OWNER TO vms;

--
-- Name: characters_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: vms
--

ALTER SEQUENCE public.characters_id_seq OWNED BY public.characters.id;


--
-- Name: drops; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.drops (
    id bigint NOT NULL,
    mob_wz integer NOT NULL,
    item_wz integer DEFAULT 0 NOT NULL,
    minimum_quantity integer DEFAULT 1 NOT NULL,
    maximum_quantity integer DEFAULT 1 NOT NULL,
    chance integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.drops OWNER TO vms;

--
-- Name: drops_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.drops_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.drops_id_seq OWNER TO vms;

--
-- Name: drops_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: vms
--

ALTER SEQUENCE public.drops_id_seq OWNED BY public.drops.id;


--
-- Name: inventory_capacity; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.inventory_capacity (
    id integer NOT NULL,
    char_id integer NOT NULL,
    equip_slot_capacity smallint NOT NULL,
    use_slot_capacity smallint NOT NULL,
    etc_slot_capacity smallint NOT NULL,
    setup_slot_capacity smallint NOT NULL,
    cash_slot_capacity smallint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.inventory_capacity OWNER TO vms;

--
-- Name: inventory_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.inventory_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.inventory_id_seq OWNER TO vms;

--
-- Name: inventory_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: vms
--

ALTER SEQUENCE public.inventory_id_seq OWNED BY public.inventory_capacity.id;


--
-- Name: items; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.items (
    id integer NOT NULL,
    char_id integer,
    wz integer NOT NULL,
    ipos smallint,
    strength smallint DEFAULT 0 NOT NULL,
    dexterity smallint DEFAULT 0 NOT NULL,
    intelligence smallint DEFAULT 0 NOT NULL,
    luck smallint DEFAULT 0 NOT NULL,
    attack smallint DEFAULT 0 NOT NULL,
    weapon_defense smallint DEFAULT 0 NOT NULL,
    magic smallint DEFAULT 0 NOT NULL,
    magic_defense smallint DEFAULT 0 NOT NULL,
    hp smallint DEFAULT 0 NOT NULL,
    mp smallint DEFAULT 0 NOT NULL,
    accuracy smallint DEFAULT 0 NOT NULL,
    avoid smallint DEFAULT 0 NOT NULL,
    hands smallint DEFAULT 0 NOT NULL,
    speed smallint DEFAULT 0 NOT NULL,
    jump smallint DEFAULT 0 NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    slots integer NOT NULL,
    expire bigint NOT NULL,
    level smallint NOT NULL,
    item_level smallint NOT NULL,
    flag smallint NOT NULL,
    item_exp smallint NOT NULL,
    vicious integer NOT NULL,
    equipped boolean NOT NULL
);


ALTER TABLE public.items OWNER TO vms;

--
-- Name: items_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.items_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.items_id_seq OWNER TO vms;

--
-- Name: items_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: vms
--

ALTER SEQUENCE public.items_id_seq OWNED BY public.items.id;


--
-- Name: keybindings_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.keybindings_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.keybindings_id_seq OWNER TO vms;

--
-- Name: keybindings; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.keybindings (
    id integer DEFAULT nextval('public.keybindings_id_seq'::regclass) NOT NULL,
    char_id integer NOT NULL,
    key integer NOT NULL,
    bind_type smallint NOT NULL,
    action integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.keybindings OWNER TO vms;

--
-- Name: skills_id_seq; Type: SEQUENCE; Schema: public; Owner: vms
--

CREATE SEQUENCE public.skills_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.skills_id_seq OWNER TO vms;

--
-- Name: skills; Type: TABLE; Schema: public; Owner: vms
--

CREATE TABLE public.skills (
    id integer DEFAULT nextval('public.skills_id_seq'::regclass) NOT NULL,
    char_id integer NOT NULL,
    wz integer NOT NULL,
    level smallint NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.skills OWNER TO vms;

--
-- Name: accounts id; Type: DEFAULT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.accounts ALTER COLUMN id SET DEFAULT nextval('public.accounts_id_seq'::regclass);


--
-- Name: characters id; Type: DEFAULT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.characters ALTER COLUMN id SET DEFAULT nextval('public.characters_id_seq'::regclass);


--
-- Name: drops id; Type: DEFAULT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.drops ALTER COLUMN id SET DEFAULT nextval('public.drops_id_seq'::regclass);


--
-- Name: inventory_capacity id; Type: DEFAULT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.inventory_capacity ALTER COLUMN id SET DEFAULT nextval('public.inventory_id_seq'::regclass);


--
-- Name: items id; Type: DEFAULT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.items ALTER COLUMN id SET DEFAULT nextval('public.items_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: accounts accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id);


--
-- Name: accounts accounts_unique; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_unique UNIQUE (username);


--
-- Name: character_limits character_limits_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.character_limits
    ADD CONSTRAINT character_limits_pkey PRIMARY KEY (id);


--
-- Name: character_limits character_limits_unique; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.character_limits
    ADD CONSTRAINT character_limits_unique UNIQUE (acc_id, world_id);


--
-- Name: characters characters_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_pkey PRIMARY KEY (id);


--
-- Name: characters characters_unique; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_unique UNIQUE (ign);


--
-- Name: drops drops_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.drops
    ADD CONSTRAINT drops_pkey PRIMARY KEY (id);


--
-- Name: items equip_items_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT equip_items_pkey PRIMARY KEY (id);


--
-- Name: inventory_capacity inventory_capacity_id_char_id_key; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.inventory_capacity
    ADD CONSTRAINT inventory_capacity_id_char_id_key UNIQUE (id, char_id);


--
-- Name: inventory_capacity inventory_capacity_key; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.inventory_capacity
    ADD CONSTRAINT inventory_capacity_key UNIQUE (char_id);


--
-- Name: keybindings keybindings_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.keybindings
    ADD CONSTRAINT keybindings_pkey PRIMARY KEY (id);


--
-- Name: keybindings keybindings_unique; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.keybindings
    ADD CONSTRAINT keybindings_unique UNIQUE (char_id, key);


--
-- Name: skills skills_pkey; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.skills
    ADD CONSTRAINT skills_pkey PRIMARY KEY (id);


--
-- Name: skills skills_unique; Type: CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.skills
    ADD CONSTRAINT skills_unique UNIQUE (char_id, wz);


--
-- Name: character_limits character_limits_acc_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.character_limits
    ADD CONSTRAINT character_limits_acc_id_fkey FOREIGN KEY (acc_id) REFERENCES public.accounts(id) ON DELETE CASCADE;


--
-- Name: characters characters_acc_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.characters
    ADD CONSTRAINT characters_acc_id_fkey FOREIGN KEY (acc_id) REFERENCES public.accounts(id) ON DELETE CASCADE;


--
-- Name: items equip_items_char_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.items
    ADD CONSTRAINT equip_items_char_id_fkey FOREIGN KEY (char_id) REFERENCES public.characters(id) ON DELETE CASCADE;


--
-- Name: keybindings keybindings_char_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.keybindings
    ADD CONSTRAINT keybindings_char_id_fkey FOREIGN KEY (char_id) REFERENCES public.characters(id) ON DELETE CASCADE;


--
-- Name: skills skills_char_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: vms
--

ALTER TABLE ONLY public.skills
    ADD CONSTRAINT skills_char_id_fkey FOREIGN KEY (char_id) REFERENCES public.characters(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

