# Usage

## Environment

Create a `.env` file with the following:

| Variable                    | Description                |
|-----------------------------|----------------------------|
| `DATABASE_URL`              | URL for the database.      |
| `CLIENT_ORIGIN`             | Base URL for the frontend. |
| `JWT_SECRET`                | JWT secret.                |
| `TOKEN_EXPIRED_IN`          | Token expiration time.     |
| `TOKEN_MAXAGE`              | Token max age.             |
| `GOOGLE_OAUTH_CLIENT_ID`    | Google OAuth client ID.    |
| `GOOGLE_OAUTH_SECRET`       | Google OAuth secret.       |
| `GOOGLE_OAUTH_REDIRECT_URL` | Google OAuth redirect URL. |

You can use the [.env.example](.env.example) file as a template.

## Secrets

Create a file at `secrets/db/password.txt` containing the password for the database.
