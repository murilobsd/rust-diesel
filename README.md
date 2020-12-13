# Rust Diesel

## install postresql (openbsd)

```
doas pkg_add postgresql-server
```

init db

```
initdb -D /var/postgresql/data/ -U postgres
```

start server

```
doas rcctl -f start postgresql
```

## install docker-compose

```
docker-compose up
```

## install diesel client

```
cargo install diesel_cli --no-default-features --features postgres
```

copy and change .env_example

```
cp .env_example .env
# change DATABASEURL
```

setup diesel (create migrations)

```
diesel setup
diesel migration run
```
