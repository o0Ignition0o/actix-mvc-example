# actix-mvc-example

  <a href="https://travis-ci.org/o0Ignition0o/actix-mvc-example">
      <img src="https://img.shields.io/travis/o0Ignition0o/actix-mvc-example/master.svg" alt="Travis Build Status">
  </a>


This project is using the same Code of Conduct as actix projects do.
It is licensed as Apache2/MIT as well as actix projects.
# License
<p>
  <a href="LICENSE-APACHE">
    <img
    src="https://img.shields.io/badge/license-apache2-green.svg" alt="MPL 2.0 License">
  </a>
  <a href="LICENSE-MIT">
    <img
    src="https://img.shields.io/badge/license-mit-blue.svg" alt="MIT License">
  </a>
</p>


Some really basic tryouts an actix-web mvc boilerplate/example work. Trying to figure out a SOLID implementation

expected outputs : 

Once you started the project, go to your browser, or use curl to make an request to http://127.0.0.1:8088/

```
$ curl http://127.0.0.1:8088/status                      
Status is OK. Server is ready to go.

$ curl http://127.0.0.1:8088/status/service?name=foo                             
Status for service foo is Ok, everything is fine ! :)

$ curl http://127.0.0.1:8088/user
Lots of users right here ! 2 total.
```

# Run the example : 
- cargo run

# Test the example :
- cargo test // Not available yet