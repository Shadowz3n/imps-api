# IMPS API

## Database migrations

`$ cargo install diesel_cli --no-default-features --features "postgres sqlite mysql"`
`$ diesel setup`

## Run

`$ docker build -t imps-api .`
`$ docker run -it --rm --name imps-running-api imps-api`
