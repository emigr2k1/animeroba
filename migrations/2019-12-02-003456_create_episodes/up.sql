CREATE SEQUENCE episodes_id_seq;

CREATE TABLE public.episodes
(
    id integer NOT NULL DEFAULT nextval('episodes_id_seq'::regclass),
    "number" integer NOT NULL,
    anime_id integer NOT NULL,
    CONSTRAINT episodes_pkey PRIMARY KEY (id),
    CONSTRAINT anime_id FOREIGN KEY (anime_id)
        REFERENCES public.animes (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

ALTER SEQUENCE episodes_id_seq
OWNED BY public.episodes.id;
