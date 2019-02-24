#/bin.bash
systemfd --no-pid -s http::0.0.0.0:8080 -- cargo watch -x run