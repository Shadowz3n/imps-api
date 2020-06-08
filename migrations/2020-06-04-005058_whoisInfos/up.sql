CREATE TABLE whois_infos(
  whois_info_id BIGSERIAL PRIMARY KEY,
  owner VARCHAR NULL,
  owner_account VARCHAR NULL,
  admin_account VARCHAR NULL,
  tech_account VARCHAR NULL,
  billing_account VARCHAR NULL,
  n_server VARCHAR NULL,
  n_stat VARCHAR NULL,
  n_last_stat VARCHAR NULL,
  n_created VARCHAR NULL,
  n_updated VARCHAR NULL,
  status VARCHAR NULL,
  registrar VARCHAR NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NULL,
  deleted_at TIMESTAMP WITHOUT TIME ZONE NULL,
  whois_id BIGINT NOT NULL,
  CONSTRAINT whois_infos_whois_id_fkey FOREIGN KEY (whois_id)
      REFERENCES whois (whois_id) MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)