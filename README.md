MING's Curriculum Manager
======

A restful-API backend application for managing content and data meant to create a CV.

I created this project for 2 main reasons:
* I wanted something in my project backlog that showed **rust expertise** beyond advent of coded solutions. While it technically does the trick *kind of* but I believe having a project in my github portfolio that is actually usable and deployable says alot more than a couple of algorithmic solutions to programming puzzles.
* I wanted to actually properly learn the backend-frontend-fullstack development by doing one myself.

As a bonus, the goal is also for this project to create one or two CVs I will actually use.

## Building an running

Clone and build the project. The entire thing is meant to be executed as a cargo project.
Beware though, right now, this thing is just a barebone axum app.

## Running the tests

This project uses the `googletest-rust`  framework for testing.
To run the unit test, execute:
```sh
$ cargo test
```

For integration tests, you currently only have to run
```sh
$ cargo test --features integration
```

This will change though shortly once database integration is upstream
