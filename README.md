# [![Coursera Course](./assets/banner.svg)](https://insight.paiml.com/58m "Coursera Specialization")

This repository is part of the Rust Fundamentals Coursera course and Systems Engineering specialization by Duke University

# Applied Rust

_Rust Bootcamp week 4: Applying Rust to build a library_

This week, you will learn how to build a Rust library with Cargo and apply debugging, testing, and documentation patterns that are applicable in a professional setting. You'll use new Cargo commands to work with the library and you'll get an idea on how to work with Rust libraries in other projects. Finally, you'll work closely with public and private code and adding tests to it. The tests will help you verify your work and increase confidence for a solid release.

This is the last wee of the Rust Bootcamp. There are 4 weeks in total:

- [week 1](https://github.com/alfredodeza/rust-setup) 
- [week 2](https://github.com/alfredodeza/rust-fundamentals) 
- [week 3](https://github.com/alfredodeza/rust-structs-types-enums/)
- [week 4](https://github.com/alfredodeza/applied-rust) 👈 You are here!

💡 Are you just looking for a 👉 [Rust template](https://github.com/alfredodeza/rust-template) to get started easily with a project? The [template](https://github.com/alfredodeza/rust-template) has everything you need!


## Contents
This week uses an example use-case to create a Rust library from scratch. Instead of separate projects as examples, it uses one single example that you can use as a reference. All the code is in the [./examples](./examples) directory. Make sure you have Rust installed and you are using [Visual Studio Code](https://code.visualstudio.com/?WT.mc_id=academic-0000-alfredodeza).

This repository is *Codespaces ready*, and it is set as a template repository. You can create a new repository from this template and start working on your own with Codespaces. This means that Rust, Copilot, and all the extensions are already installed and configured for you.

### Lesson 1: Building a real-world library
- [Adding code to lib.rs](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/lib.rs#L24)
- [Documenting your code](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/lib.rs#L1-L23)
- [Using a Makefile](./examples/cli-utils/Makefile)

### Lesson 2:
- [Using Cargo for dependencies](./examples/cli-utils/Cargo.toml)
- [Extending with modules](./examples/cli-utils/src/config.rs)
- [Defining Public and Private modules](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/lib.rs#L13-L14)
- [Public and Private fields](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/config.rs#L15-L36)

### Lesson 3: Testing Rust Code
- [Organizing test files](./examples/cli-utils/tests)
- [Understanding testing private code](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/lib.rs#L38-L61)
- [Writing your first test](./examples/cli-utils/tests/test_simple.rs)
- [Other assert macros](https://github.com/alfredodeza/applied-rust/blob/main/examples/cli-utils/src/lib.rs#L49)

### Lesson 4: Practice lab
Use the included [practice lab](./lab.md) to apply the content you've learned in this week. Follow the steps to create your own repository and apply the requirements to complete the lab so that you end up creating a library in Rust.

## Resources

**Coursera Courses**

- [MLOps Machine Learning Operations Specialization](https://www.coursera.org/specializations/mlops-machine-learning-duke)
- [Linux and Bash for Data Engineering](https://www.coursera.org/learn/linux-and-bash-for-data-engineering-duke)
- [Open Source Platforms for MLOps](https://www.coursera.org/learn/open-source-platforms-duke)
- [Python Essentials for MLOps](https://www.coursera.org/learn/python-essentials-mlops-duke)
- [Web Applications and Command-Line tools for Data Engineering](https://www.coursera.org/learn/web-app-command-line-tools-for-data-engineering-duke)
- [Python and Pandas for Data Engineering](https://www.coursera.org/learn/python-and-pandas-for-data-engineering-duke)
- [Scripting with Python and SQL for Data Engineering](https://www.coursera.org/learn/scripting-with-python-sql-for-data-engineering-duke)

**Readings**
- [Rust Book](https://doc.rust-lang.org/book/)
- [First steps with Rust Learning Path](https://learn.microsoft.com/training/paths/rust-first-steps/?WT.mc_id=academic-0000-alfredodeza)

**Other courses**
- [DevOps command-line tools in Python and Rust](https://learning.oreilly.com/videos/devops-command-line-tools/28037639VIDEOPAIML/)

