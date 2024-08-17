TO DO EMAIL App

What?
We create a TO DO App that sends the to do list to an email.

Why?
Most TO DO Apps, mostly focus on making a list with some form of alarm or notification, but not making an app where you make the list and that list is forwarded to your email for you to receive as apart of the work day mail.

Who?
Enthusiasts, Users, Programmers, interested in making their daily to do list easily accessible.

How? 

## Step 1: Set Up Your Rust Project

1. **Create a new Rust project**:
   ```sh
   cargo new todo_email_app
   cd todo_email_app
   ```

2. **Add dependencies to `Cargo.toml`**:
   ```toml
   [dependencies]
   chrono = "0.4"
   lettre = { version = "0.10.0-rc.4", features = ["smtp-tokio", "builder"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```

## Step 2: Implement the To-Do Application

1. **Create the main logic in `src/main.rs`**:



 _Docs_

* [About Me](https://github.com/josephkb87)* [About Rust Beginners](../docs/README.md)
* [Rust Worked Examples](https://github.com/josephkb87/Beginners/tree/main/RustWorkedExamples/README.md)
* [Rust Projects](https://github.com/josephkb87/RustBeginners/tree/main/Projects/README.md)
* [CHANGELOG](../docs/CHANGELOG.md) 
* [Contribute](../docs/CONTRIBUTING.md)
* [Pull Requests](../docs/blob/PRs.md)  
* [SECURITY](../docs/SECURITY.md) 
* [Attributions](..docs/Attributions.md) 
* [CodeOfConduct](../docs/CodeOfConduct.md) 
* [LICENSE](../LICENSE.md)
* [References](../docs/References.md)