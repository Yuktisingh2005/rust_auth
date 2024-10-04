# Rust Authentication API

A simple authentication API built with Rust, using the Diesel ORM for database interactions. This API provides endpoints for user registration and updates.

## Table of Contents

- [Rust Authentication API](#rust-authentication-api)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Technologies Used](#technologies-used)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)

## Features

- User registration with email and password.
- User profile update functionality.
- Basic authentication checks.
- Database integration using Diesel ORM.

## Technologies Used

- **Rust**: The programming language used for building the API.
- **Diesel**: The ORM used for interacting with the PostgreSQL database.
- **PostgreSQL**: The database used for storing user data.
- **dotenv**: For managing environment variables.

## Getting Started

### Prerequisites

To get started, you need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (including Cargo)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (optional, for managing migrations)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Yuktisingh2005/rust_auth_api.git
   cd rust_auth_api
