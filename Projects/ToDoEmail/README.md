<<<<<<< HEAD
TO DO EMAIL App

<<<<<<< HEAD
What?
=======
<<<<<<< HEAD
W
Xhat?
=======
What?
>>>>>>> 818d650 (Add files via upload)
>>>>>>> a9f995b (Add files via upload)
We create a TO DO App that sends the to do list to an email.

Why?
Most TO DO Apps, mostly focus on making a list with some form of alarm or notification, but not making an app where you make the list and that list is forwarded to your email for you to receive as apart of the work day mail.

Who?
Enthusiasts, Users, Programmers, interested in making their daily to do list easily accessible.

How? 

<<<<<<< HEAD
=======
<<<<<<< HEAD
## Step 1: Set Up Your Rust Project

>>>>>>> 3118847 (Update Projects:ToDoEmailApp)
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

<<<<<<< HEAD
## Implement the To-Do Application

1. **Create the main logic in `src/main.rs`**:

=======
## Step 2: Implement the To-Do Application

1. **Create the main logic in `src/main.rs`**:


<<<<<<< HEAD
>>>>>>> 2938dda (Update Projects:ToDoEmailApp)
=======
>>>>>>> 818d650 (Add files via upload)
>>>>>>> a9f995b (Add files via upload)
## Step 3: Run the Application

1. **Build and run the application**:
   ```sh
   cargo run
   ```

2. **Use the application**:
   - Add To-Do items by selecting option 1 and entering descriptions.
   - Send the To-Do list via email by selecting option 2.
   - Exit the application by selecting option 3.

## Step 4: Automate the Script

To automate the script, you can use a task scheduler to run the application daily.

<<<<<<< HEAD
**On Unix-based Systems (Using `cron`)**
=======
**On Unix-based Systems (Using `cron`)**:
<<<<<<< HEAD
>>>>>>> 2938dda (Update Projects:ToDoEmailApp)
=======
>>>>>>> 818d650 (Add files via upload)
>>>>>>> a9f995b (Add files via upload)

1. Open the cron table for editing:
   ```sh
   crontab -e
   ```

2. Add a line to run the Rust executable daily at a specific time (e.g., 8 AM):
   ```sh
   0 8 * * * /path/to/todo_email_app
   ```

**On Windows (Using Task Scheduler)**:

1. Open Task Scheduler and create a new task.
2. In the "General" tab, name your task and provide a description.
3. In the "Triggers" tab, create a new trigger that runs daily at your desired time.
4. In the "Actions" tab, create a new action that runs the Rust executable:
   - Action: Start a program
   - Program/script: `C:\path\to\todo_email_app.exe
   - 
<<<<<<< HEAD
 ´´´´
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
TO DO EMAIL App

What?
We create a TO DO App that sends the to do list to an email.

Why?
Most TO DO Apps, mostly focus on making a list with some form of alarm or notification, but not making an app where you make the list and that list is forwarded to your email for you to receive as apart of the work day mail.

Who?
Enthusiasts, Users, Programmers, interested in making their daily to do list easily accessible.

How? 
>>>>>>> 22cca21 (Add files via upload)

_Docs_
=======
=======
_Docs_

>>>>>>> 3118847 (Update Projects:ToDoEmailApp)
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
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 2938dda (Update Projects:ToDoEmailApp)
=======
>>>>>>> 68b0f92 (Update Projects:ToDoEmailApp)
<<<<<<< HEAD
>>>>>>> 3118847 (Update Projects:ToDoEmailApp)
=======
=======
TO DO EMAIL App

What?
We create a TO DO App that sends the to do list to an email.

Why?
Most TO DO Apps, mostly focus on making a list with some form of alarm or notification, but not making an app where you make the list and that list is forwarded to your email for you to receive as apart of the work day mail.

Who?
Enthusiasts, Users, Programmers, interested in making their daily to do list easily accessible.

How? 

_Docs_

<<<<<<< HEAD
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

## Implement the To-Do Application

1. **Create the main logic in `src/main.rs`**:

## Step 3: Run the Application

1. **Build and run the application**:
   ```sh
   cargo run
   ```

2. **Use the application**:
   - Add To-Do items by selecting option 1 and entering descriptions.
   - Send the To-Do list via email by selecting option 2.
   - Exit the application by selecting option 3.

## Step 4: Automate the Script

To automate the script, you can use a task scheduler to run the application daily.

**On Unix-based Systems (Using `cron`)**

1. Open the cron table for editing:
   ```sh
   crontab -e
   ```

2. Add a line to run the Rust executable daily at a specific time (e.g., 8 AM):
   ```sh
   0 8 * * * /path/to/todo_email_app
   ```

**On Windows (Using Task Scheduler)**:

1. Open Task Scheduler and create a new task.
2. In the "General" tab, name your task and provide a description.
3. In the "Triggers" tab, create a new trigger that runs daily at your desired time.
4. In the "Actions" tab, create a new action that runs the Rust executable:
   - Action: Start a program
   - Program/script: `C:\path\to\todo_email_app.exe
   - 
 ´´´´
TO DO EMAIL App

What?
We create a TO DO App that sends the to do list to an email.

Why?
Most TO DO Apps, mostly focus on making a list with some form of alarm or notification, but not making an app where you make the list and that list is forwarded to your email for you to receive as apart of the work day mail.

Who?
Enthusiasts, Users, Programmers, interested in making their daily to do list easily accessible.

How? 

_Docs_

=======
>>>>>>> 787d5a68c01e9ec278f3cac2e8ca6f31317d178a
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
>>>>>>> bd563a7 (Chore:Add AUTHORS)
>>>>>>> d18c8f8 (Chore:Add AUTHORS.)
=======
=======
>>>>>>> 39affb5 (Add files via upload)
<<<<<<< HEAD
>>>>>>> 22cca21 (Add files via upload)
=======
=======
 ´´´´
>>>>>>> 818d650 (Add files via upload)
>>>>>>> a9f995b (Add files via upload)
