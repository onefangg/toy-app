

Hot Reload:
`cargo watch -w src/ -w ui/ -x run`


Cert

(using mkcert)
```
sudo apt install -y mkcert libnss3-tools
mkcert -install
mkcert localhost 127.0.0.1 ::1
```
* get windows to import the cert