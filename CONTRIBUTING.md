# Contributing to tonic-music

Thank you for your interest in contributing to `tonic-music`! We are excited to welcome help from the community. Whether you're reporting a bug, suggesting a new feature, or writing code, your input is valuable.

## ❤️ Our Pledge (Code of Conduct)

We are committed to providing a welcoming and respectful environment for everyone. By participating in this project, you agree to:

* **Be respectful:** Treat all other contributors, users, and maintainers with respect and kindness.
* **Be constructive:** Post feedback, issues, and pull requests that are constructive and helpful.
* **Be open:** Be open to different opinions and perspectives.
* **Be considerate:** Be mindful of your language. Harassment, discrimination, or any exclusionary behavior will not be tolerated.

We want this to be a positive space for everyone.

## 💡 How You Can Help

There are many ways to contribute, and many of them don't involve writing code.

### 🐞 Reporting Bugs
If you find a bug, please help us by submitting an issue. A good bug report includes:

1.  **Check existing issues:** See if someone has already reported the bug.
2.  **Title:** A clear and descriptive title.
3.  **Details:**
    * What operating system you are using.
    * Steps to reproduce the bug.
    * What you expected to happen.
    * What actually happened (including any error messages).

### ⭐ Suggesting Features
Have an idea for a new feature? We'd love to hear it.

1.  **Check existing issues:** See if the feature has already been suggested.
2.  **Open an issue** with a "Feature Request" label.
3.  **Explain your idea:**
    * What is the problem you're trying to solve?
    * How do you imagine the new feature working?
    * Are there any examples from other tools?

### 📖 Improving Documentation
Noticed a typo in the `README.md`? Could an example be clearer? Found a comment in the code that is confusing?

These contributions are incredibly helpful! Feel free to open an issue or a Pull Request with your suggested changes.

## 💻 Contributing Code
If you're ready to write some code, here is the basic workflow:

### Development Setup
To ensure code quality and consistency, this project uses `pre-commit` hooks. These hooks run automatically before each commit to format, lint, and test the code.

1.  **Install pre-commit:** Follow the official [installation guide](https://pre-commit.com/#installation). A common method is using pip:
    ```bash
    pip install pre-commit
    ```
2.  **Set up the hooks:** In the root of the project, run:
    ```bash
    pre-commit install
    ```
    Now, the checks will run automatically every time you run `git commit`.

### Contribution Workflow
1.  **Fork** the repository.
2.  **Create a new branch** for your feature or bugfix (e.g., `git checkout -b feat/add-pentatonic-scale`).
3.  **Make your changes.** Please follow the existing code style and add comments where necessary.
4.  **Test your changes.** The `pre-commit` hooks will automatically run format (`cargo fmt`), lint (`cargo clippy`), and test (`cargo test`) checks for you. Our CI pipeline will also run a full suite of tests, so ensuring they pass locally is a great first step.
    *   If you are adding a new feature or fixing a bug, please add new tests to cover your changes.
5.  **Commit** your changes with a clear commit message.
6.  **Push** your branch to your fork.
7.  **Open a Pull Request** (PR) against the `main` branch of the `airvzxf/music-theory-calculator` repository.

## 📜 License Agreement
By contributing to this project, you agree that your contributions will be licensed under the **GNU AGPLv3 License** that covers this project.
