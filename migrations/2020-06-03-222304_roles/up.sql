CREATE TABLE roles(
  role_id BIGSERIAL PRIMARY KEY,
  role_name VARCHAR UNIQUE NOT NULL,
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITHOUT TIME ZONE NULL,
  deleted_at TIMESTAMP WITHOUT TIME ZONE NULL
)