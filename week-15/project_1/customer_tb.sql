--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

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

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    cid integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile integer NOT NULL,
    e_id integer NOT NULL,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (cid, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	805508911	102	5
111	LilianJaiya	43	i_jaiya@gmail.com	805518534	100	3
112	ArthurMusa	50	a_musa@gmail.com	705528281	107	10
113	PhilipAkonio	41	p_akonio@gmail.com	905235677	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	80533335	120	5
115	Oghenero Agor	50	o_agor@gmail.com	705556677	117	11
116	Adams Bree	33	a_bree@gmail.com	805676556	102	1
117	OkaforMathias	45	o_mathias@gmail.com	805676542	120	10
118	SamsonAdeleke	65	s_adeleke@gmail.com	905211110	107	5
119	Lawal Tamire	35	l_tamire@gmail.com	705677442	117	11
120	James Job	44	j_job@gmail.com	805969391	100	8
121	Matthew Jakande	21	m_jakamde@gmail.com	705123214	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	805492192	107	5
\.


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (cid);


--
-- PostgreSQL database dump complete
--

