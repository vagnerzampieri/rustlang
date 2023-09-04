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

## Run

`docker network create my_pocket`
`docker compose up -d`
`cargo run`

## Stop

`docker compose stop`
