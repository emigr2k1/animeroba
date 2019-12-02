CREATE SEQUENCE animes_id_seq;

CREATE TABLE public.animes
(
    id integer NOT NULL DEFAULT nextval('animes_id_seq'::regclass),
    name text COLLATE pg_catalog."default" NOT NULL,
    code_name text COLLATE pg_catalog."default" NOT NULL,
    score integer NOT NULL,
    synopsis text COLLATE pg_catalog."default",
    release_date date NOT NULL DEFAULT '1900-01-01'::date,
    kind integer NOT NULL,
    cover text COLLATE pg_catalog."default",
    status integer NOT NULL,
    genres integer[] NOT NULL,
    CONSTRAINT animes_pkey PRIMARY KEY (id),
    CONSTRAINT code_name UNIQUE (code_name)
);

ALTER SEQUENCE animes_id_seq
OWNED BY public.animes.id;
