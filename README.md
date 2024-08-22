# metw-mailer

metw-mailer is a templated e-mail sender REST API. It requires a e-mail relay.

Example .env file:
```env
HTTP_HOST="0.0.0.0:3000"
MAILER_RELAY="example.com"
MAILER_NOREPLY="no-reply@example.com"
TOKEN="example"
```
