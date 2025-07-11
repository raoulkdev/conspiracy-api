# Conspiracy API

This a fun and simple HTTP API built with Rust + Axum that return **50 unique conspiracy theories**.

You can try the API live here: https://conspiracy-api.onrender.com

---
## Built With

- 🦀 Rust (2021 edition)
- 🕸️ [Axum](https://github.com/tokio-rs/axum) — Web framework
- 📦 Serde — JSON serialization
- 🔧 Tokio — Async runtime

---
## Features

- Retrieve 50 unique conspiracy theories
- Filter by ID or category
- Clean, readable JSON responses
- Data loaded from a local JSON file

---

## API Endpoints

### Get all conspiracy theories  
`GET /theories`  
Returns an array of all conspiracy theories.

### Get a theory by ID  
`GET /theories/:id`  
Returns the conspiracy theory with the specified ID.

### Filter theories by category  
`GET /theories?category={category}`  
Returns theories in the specified category (e.g. `space`, `tech`, `government`, etc.)

### Get all categories  
`GET /categories`  
Returns a list of available categories.

---

## Example Response

```json
{
  "id": 1,
  "title": "The Moon Landing Was Filmed in a Studio",
  "category": "space",
  "description": "Some believe Stanley Kubrick directed the moon landing in a secret NASA soundstage."
}
```

---

## Getting Started

### 1. Clone the repo
```bash
git clone https://github.com/raoulkdev/conspiracy-api.git
cd conspiracy-api
```

### 2. Install Rust (if you don’t have it)
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### 3. Run the API
```bash
cargo run
```

### 4. Visit the endpoints
- http://localhost:3000/theories  
- http://localhost:3000/theories/1  
- http://localhost:3000/theories?category=space  
- http://localhost:3000/categories  

---

## Why This Exists

I built this project to:
- Practice building APIs with **Axum**
- Create a fun API
- Demonstrate clean API structure and JSON handling in Rust

---

## Contributions

Pull requests and suggestions are welcome!  
Feel free to fork it and add even more wild theories.

---

## License

MIT License
