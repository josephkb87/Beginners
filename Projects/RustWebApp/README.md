# Crustacean is a cross-platform desktop app made using Rust, SolidJS, and Tauri.

### Tauri
Tauri offers the ability to design your UI using web technologies like HTML, CSS, and JavaScript but allows you to use lower-level languages to write the application and backend logic. Tauri primarily supports Rust as the backend language, but its API can be implemented across multiple languages

### Rust
Rust as the backend is better for safety, speed, and preventing incorrect/unsafe code due to its statically-typed features and is great at preventing data races that can lead to undefined behavior, whereas C++ cannot do this work for you and opens up vulnerabilities.

### SolidJS
Contrary to the idea that using the main DOM slows down an application, it is part of what makes SolidJS so performant.The virtual DOM is a lightweight implementation of the main DOM.The way Solid achieves such performant speeds without the use of a virtual DOM is to compile its templates down to real DOM nodes and wrap updates in fine-grained reactions. This way, when your state updates, only the code that depends on it runs.

