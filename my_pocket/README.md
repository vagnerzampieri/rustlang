# My Pocket

## Description

This is a simple app to save your favorite links with a description and a tag. It's possible to remove the link, edit the description and the tag.

## Models

Pocket

```
{
  description: String,
  link: String,
  tags: [Tag]
}
```

Tag

```
{
  name: String
}
```
## API

GET `/pockets?tag=tag_name`
```
[
  {
    description: String,
    link: String,
    tags: [Tag]
  }
]
```

GET `/pockets/:id`
```
{
  description: String,
  link: String,
  tags: [Tag]
}
```

POST `/pockets`
```
{
  description: String,
  link: String,
  tags: [Tag]
}
```

PUT `/pockets/:id`
```
{
  description: String,
  link: String,
  tags: [Tag]
}
```

DELETE `/pockets/:id`
```
{
  description: String,
  link: String,
  tags: [Tag]
}
```

GET `/tags`
```
[
  {
    name: String
  }
]
```

GET `/tags/:id`
```
{
  name: String
}
```

POST `/tags`
```
{
  name: String
}
```

PUT `/tags/:id`
```
{
  name: String
}
```

DELETE `/tags/:id`
```
{
  name: String
}
```

## Run

`docker network create my_pocket`
`docker compose up -d`
`cargo run`

## Stop

`docker compose stop`

## Run with make

`make dev`
`make dev-down`
`make start-server`
`make install`

## References

- https://codevoweb.com/build-a-crud-api-with-rust-and-mongodb/
- https://blog.devgenius.io/build-a-rest-api-with-rust-and-mongodb-actix-web-version-a275215c262a
- https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-user-registration-580h
