# Rust Web Server

An async web server built with Rust using Warp framework.

## Features

- **Async/Await**: Built on Tokio runtime
- **REST API**: CRUD operations for users
- **JSON**: Request/response handling
- **In-Memory Storage**: HashMap-based data store
- **CORS**: Cross-origin resource sharing
- **Health Checks**: Service monitoring endpoint

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/users` | List all users |
| POST | `/users` | Create new user |
| GET | `/users/:id` | Get user by ID |
| DELETE | `/users/:id` | Delete user |
| GET | `/health` | Health check |

## Usage

### Start the server
```bash
cargo run
```

Server runs on `http://127.0.0.1:3030`

### API Examples

Create a user:
```bash
curl -X POST http://127.0.0.1:3030/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

List users:
```bash
curl http://127.0.0.1:3030/users
```

Get user:
```bash
curl http://127.0.0.1:3030/users/{user-id}
```

## Key Learning Points

1. **Async Programming**: Using `async/await` with Tokio
2. **Warp Framework**: Filter-based web framework
3. **Shared State**: Arc<Mutex<T>> for thread-safe state
4. **Error Handling**: Rust's Result type with Warp rejections
5. **Serialization**: Serde for JSON handling

## Architecture

```
HTTP Request -> Warp Filters -> Handler Functions -> Shared Database -> JSON Response
```

## Dependencies

- `tokio`: Async runtime
- `warp`: Web framework
- `serde`: Serialization framework
- `uuid`: Unique ID generation
- `chrono`: Date/time handling

## Performance Notes

Rust + Tokio provides excellent performance for concurrent workloads. The type system ensures memory safety without garbage collection overhead.