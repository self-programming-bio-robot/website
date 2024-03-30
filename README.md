# How to run
First run server:
```shell
run --package server --bin server
```

Next run web server:
```shell
cd web
trunk serve --proxy-backend=http://localhost:3000/ --proxy-rewrite=/api/
```

# Todo:
- [ ] Make retro console style of website
- [ ] Improve input interaction, always focus on input
- [ ] Add automatically scrolling to the bottom of the conversation
- [ ] Add navigation by assistant
- [ ] Add history to conversation
- [ ] Add error handling
- [ ] Add tests
