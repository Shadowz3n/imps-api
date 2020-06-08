CREATE TABLE whois_users(
  whois_user_id BIGSERIAL PRIMARY KEY,
  role VARCHAR NULL,
  name VARCHAR NULL,
  nic VARCHAR NULL,
  organization VARCHAR NULL,
  street VARCHAR NULL,
  city VARCHAR NULL,
  state VARCHAR NULL,
  postal_code VARCHAR NULL,
  country VARCHAR NULL,
  phone VARCHAR NULL,
  phone_ext VARCHAR NULL,
  fax VARCHAR NULL,
  fax_ext VARCHAR NULL,
  email VARCHAR NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NULL,
  deleted_at TIMESTAMP WITHOUT TIME ZONE NULL,
  whois_info_id BIGINT NOT NULL,
  CONSTRAINT whois_users_whois_info_id_fkey FOREIGN KEY (whois_info_id)
      REFERENCES whois_infos (whois_info_id) MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)