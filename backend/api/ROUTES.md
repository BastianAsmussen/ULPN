# API Routes Documentation

## Descriptor Table

| Route                  | Method | Description                            |
|------------------------|--------|----------------------------------------|
| /sessions/oauth/google | GET    | Redirects to Google OAuth2 login page. |
| /auth/login            | POST   | Logs in a user.                        |
| /auth/register         | POST   | Registers a user.                      |
| /auth/logout           | GET    | Logs out a user.                       |
| /forum                 | POST   | Creates a new forum.                   |
| /forum                 | GET    | Gets all forums.                       |
| /forum/:id             | GET    | Gets a forum by ID.                    |
| /forum/:id             | PUT    | Updates a forum by ID.                 |
| /forum/:id             | DELETE | Deletes a forum by ID.                 |
| /message               | POST   | Creates a new message.                 |
| /message               | GET    | Gets all messages.                     |
| /message/:id           | GET    | Gets a message by ID.                  |
| /message/:id           | PUT    | Updates a message by ID.               |
| /message/:id           | DELETE | Deletes a message by ID.               |
| /settings              | GET    | Gets all active app settings.          |
| /settings/:key         | GET    | Gets a specific setting by its key.    |
| /settings              | POST   | Create a new app setting.              |
| /settings/:key         | PUT    | Overwrite/update an app setting.       |
| /settings/:key         | DELETE | Delete an app setting by its key       |

