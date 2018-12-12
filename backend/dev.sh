#/bin.bash
systemfd --no-pid -s http::0.0.0.0:80 -- cargo watch -x run