CREATE TABLE estates(
  estate_id BIGSERIAL PRIMARY KEY,
  estate_name VARCHAR NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NULL,
  deleted_at TIMESTAMP WITHOUT TIME ZONE NULL
)