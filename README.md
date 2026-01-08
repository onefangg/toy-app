# Toy App

Sample template for a basic web application with Axum and HTMX, hence the name toy app.

## Utility commands

*(Primarily for reference)*

Hot Reload:
`cargo watch -w src/ -w ui/ -x run`

Cert

(using mkcert)
```
sudo apt install -y mkcert libnss3-tools
mkcert -install
mkcert localhost 127.0.0.1 ::1
```

Make windows import the cert thereafter.