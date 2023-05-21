# Rust Apps - TODO

Build Rust App - TODO (A sophisticated one)

## Overview

### Features

- [x] TODO w/o DB

  > The TODO struct is wrapped with a Mutex to make it thread-safe. The Mutex is a mutual exclusion lock, which allows only one thread to access the data at a time. This is necessary because Rust doesn't know at compile time which thread will call the code. If we don't use Mutex, Rust will complain that the code is not thread-safe.
  >
  > The MutexGuard is a smart pointer that points to the data inside the Mutex. It implements Deref to point to the inner data, and Drop to release the lock automatically when it goes out of scope.
  >
  > The DB is not persistent and will be lost when the server is restarted.

### APIs

| Route       | API endpoint       | HTTP Method | CRUD Method | Description   |
| ----------- | ------------------ | ----------- | ----------- | ------------- |
| /tasks      | GET /tasks         | GET         | READ        | Get all tasks |
| /tasks      | POST /tasks        | POST        | CREATE      | Create a task |
| /tasks/{id} | PUT /tasks/{id}    | PUT         | UPDATE      | Update a task |
| /tasks/{id} | DELETE /tasks/{id} | DELETE      | DELETE      | Delete a task |

> 2 routes, 4 API endpoints.

### API

## Installation

```sh
$ cargo add actix-web
$ cargo add serde --features derive
$ cargo add seder_json
$ cargo add serde_json
```

## Usage

### Build

### Run

### Test

### Deploy
