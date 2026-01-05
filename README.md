# axum-hateoas

Libreria HATEOAS per Axum - Implementazione standardizzata di Hypermedia as the Engine of Application State.

## Features

- **HateoasResponse**: Wrapper per singole risorse con link HATEOAS
- **CollectionResponse**: Wrapper per collezioni con paginazione e link
- **Link**: Builder pattern per creare link con href, method, e title

## Struttura

```
src/
├── lib.rs           # Exports pubblici
├── link.rs          # Link struct e builder
├── resource.rs      # HateoasResponse per singole risorse
└── collection.rs    # CollectionResponse per liste paginate
```

## Usage

### Single Resource con HATEOAS

```rust
use axum_hateoas::{HateoasResponse, Link};
use axum::Json;

pub async fn get_user() -> Result<Json<HateoasResponse<User>>> {
    let user = User { id: 1, name: "Mario".to_string() };

    let response = HateoasResponse::new(user)
        .add_self("/api/v1/users/1")
        .add_collection("/api/v1/users")
        .add_update("/api/v1/users/1")
        .add_delete("/api/v1/users/1")
        .add_link("avatar", Link::new("/api/v1/users/1/avatar").title("Avatar"));

    Ok(Json(response))
}
```

**Response:**
```json
{
  "data": { "id": 1, "name": "Mario" },
  "links": {
    "self": { "href": "/api/v1/users/1" },
    "collection": { "href": "/api/v1/users" },
    "update": { "href": "/api/v1/users/1", "method": "PATCH" },
    "delete": { "href": "/api/v1/users/1", "method": "DELETE" },
    "avatar": { "href": "/api/v1/users/1/avatar", "title": "Avatar" }
  }
}
```

### Collection con Paginazione

```rust
use axum_hateoas::{CollectionResponse, Link};
use axum::{Json, extract::Query};

pub async fn get_users(
    Query(params): Query<UsersQuery>,
) -> Result<Json<CollectionResponse<User>>> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let total = 100;

    let users = vec![...]; // Get users from DB

    let mut response = CollectionResponse::new(users, total, page, limit);

    response = response
        .with_pagination_links("/api/v1/users", page, limit, total_pages)
        .add_link("create", Link::new("/api/v1/users").method("POST"));

    Ok(Json(response))
}
```

**Response:**
```json
{
  "data": [...],
  "links": {
    "self": { "href": "/api/v1/users?page=2&limit=10" },
    "first": { "href": "/api/v1/users?page=1&limit=10" },
    "prev": { "href": "/api/v1/users?page=1&limit=10" },
    "next": { "href": "/api/v1/users?page=3&limit=10" },
    "last": { "href": "/api/v1/users?page=10&limit=10" },
    "create": { "href": "/api/v1/users", "method": "POST" }
  },
  "meta": {
    "total": 100,
    "page": 2,
    "limit": 10,
    "total_pages": 10
  }
}
```

## Link Standard

| Relazione | Descrizione | Method |
|----------|-------------|---------|
| `self` | URL della risorsa corrente | GET |
| `collection` | URL della collezione padre | GET |
| `create` | Crea nuova risorsa | POST |
| `update` | Aggiorna risorsa | PATCH |
| `delete` | Elimina risorsa | DELETE |
| `first` | Prima pagina (collections) | GET |
| `prev` | Pagina precedente (collections) | GET |
| `next` | Pagina successiva (collections) | GET |
| `last` | Ultima pagina (collections) | GET |

## Integration con utoipa

La crate supporta utoipa per OpenAPI docs:

```rust
#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, description = "User found", body = HateoasResponse<User>)
    )
)]
pub async fn get_user(...) -> Result<Json<HateoasResponse<User>>> {
    // ...
}
```

## License

MIT
