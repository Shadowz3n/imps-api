CREATE TABLE users_addresses(
  id BIGSERIAL PRIMARY KEY,
  address VARCHAR NOT NULL,
  address_number VARCHAR NULL,
  address_complement VARCHAR NULL,
  city_id BIGINT NOT NULL,
  estate_id BIGINT NOT NULL,
  country_id BIGINT NOT NULL,
  zip_code VARCHAR NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NULL,
  deleted_at TIMESTAMP WITHOUT TIME ZONE NULL,
  user_id BIGINT NOT NULL,
  CONSTRAINT users_addresses_user_id_fkey FOREIGN KEY (user_id)
      REFERENCES users (id) MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)