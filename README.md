# Conspiracy API

Welcome to the **Conspiracy API**!  
This is a simple API built with Axum Rust that serves up 50 different conspiracy theories. I built this a practice project for myself, or just satisfying your curiosity about some of the wildest conspiracies out there!

## Features

- Get a list of 50 unique conspiracy theories via a simple API
- Filter by ID or categories

## Endpoints

### Get All conspiracy theories

```
GET /theories
```
Returns a JSON array of all 50 conspiracy theories.

### Get a conspiracy theory by ID

```
GET /theories/:id
```
Returns the conspiracy theory with the specified ID.

### Get all conspiracy theory in a Category

```
GET theories/?category={category}
```
Returns a list conspiracy theories in the specified category.

### Get all available categories

```
GET /categories
```
Returns a list of all categories

## Getting Started

1. **Clone the repository:**
    ```bash
    git clone https://github.com/raoulkdev/conspiracy-api.git
    cd conspiracy-api
    ```

2. **Install dependencies:**
    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

3. **Start the API server:**
    ```bash
    cargo run
    ```

4. **Visit the API endpoints:**
    - `http://localhost:3000/theories`
    - `http://localhost:3000/theories/1` (replace `1` with the desired ID)
    - `http://localhost:3000/theories/?category=space` (replace `space` with the desired category)
    - `http://localhost:3000/categories`

## Example Response

```json
{
    "id": 1,
    "title": "The Moon Landing Was Filmed in a Studio",
    "category": "space",
    "description": "Some believe Stanley Kubrick directed the moon landing in a secret NASA sound-stage."
}
```

## Why This Exists

This project was created for me to practice Axum and fun. It can be used as:
- A mock API for frontend or mobile app development
- A demonstration of basic API creation
- A quirky dataset for testing or inspiration

## Contributions

Pull requests and suggestions are welcome!  
Feel free to fork the repo and make it even more interesting.

## License

This project is released under the MIT License.
