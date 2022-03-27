## TODOs Example
#### Setup
1. Declare the database URL

```export DATABASE_URL="mysql://root:password@localhost/todos"```
2. Create the database.

```$ sqlx db create```
3. Run sql migrations

$ sqlx migrate run

#### Usage
1. Add a todo

```cargo run -- add "todo description"```
2. Complete a todo.

```cargo run -- update <todo id>```
3. List all todos

```cargo run```