CREATE SEQUENCE video_servers_id_seq;

CREATE TABLE public.video_servers
(
    id integer NOT NULL DEFAULT nextval('video_servers_id_seq'::regclass),
    name text COLLATE pg_catalog."default" NOT NULL,
    url text COLLATE pg_catalog."default" NOT NULL,
    episode_id integer NOT NULL,
    CONSTRAINT video_servers_pkey PRIMARY KEY (id),
    CONSTRAINT video_servers_episode_id_fkey FOREIGN KEY (episode_id)
        REFERENCES public.episodes (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

ALTER SEQUENCE video_servers_id_seq
OWNED BY public.video_servers.id;
