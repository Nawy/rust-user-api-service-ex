# User CRUD REST API Service
### Create a new user

```sh
POST /user

```
Body:
```json
{
    "email": "john@mail.com",
    "name": "John Doe"
}
```
### Delete a user
```sh
DELETE /user/{user-id}
```

### Get one user
```sh
GET /user/{user-id}
```

### List all users

```sh
GET /user
```
