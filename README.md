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
- [x] Make retro console style of website
- [x] Improve input interaction, always focus on input
- [x] Add automatically scrolling to the bottom of the conversation
- [ ] Open keyboard on mobile devices
- [ ] Add navigation by assistant
- [ ] Add history to conversation
- [ ] Add error handling
- [ ] Add tests
- [ ] Utilize built-in semantic router of LangChain
