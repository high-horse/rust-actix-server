# Rust actix server

A brief description of what this project does and who it's for.

## Installation 

Install my-project with npm

```bash 
  cargo build
```

## Usage/Examples

```bash
  cargo run
```

## API Reference

#### Get all items

```http
  GET /api/todos
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `api_key` | `string` | **Required**. Your API key |

#### Get item

```http
  GET /api/items/${id}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `id`      | `string` | **Required**. Id of item to fetch |

#### Add item

```http
  POST /api/items
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `api_key` | `string` | **Required**. Your API key |
| `name`    | `string` | **Required**. Item name |
| `desc`    | `string` | **Required**. Item description |

## Contributing

Contributions are always welcome!

See `contributing.md` for ways to get started.

Please adhere to this project's `code of conduct`.
