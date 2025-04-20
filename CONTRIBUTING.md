# Contributing to the **Core shell**

Thank you for considering contributing to the **Core shell**! We welcome contributions to improve the project, fix bugs, and enhance functionality. Before getting started, please read this guide to ensure your contributions align with the project's goals.

## How to Contribute

### 1. Fork the Repository

Start by forking the repository to your own GitHub account. You can do this by clicking the "Fork" button at the top right of the repository page.

### 2. Clone Your Fork

Once you have forked the repository, clone it to your local machine:

```sh
git clone https://github.com/your-username/core-shell.git
cd core-shell
```

### 3. Create a New Branch

Create a new branch for your changes:

```sh
git checkout -b feature/your-feature-name
```

### 4. Make Changes

Now you can make your changes! Please ensure that your code follows the style guidelines of the project:

- Keep the code clean, modular, and well-structured.
- Ensure your changes are covered by tests, or add tests where applicable.
- Update the documentation if you add new features or change behavior.

### 5. Run Tests
If you’re adding or modifying code that interacts with other parts of the system, it’s important to run all tests to ensure your changes don’t break anything.

### 6. Format and Lint Code

To maintain code quality, format and lint your code using the Rust tools:

```sh
cargo fmt
cargo clippy
```

### 7. Commit Your Changes

Once your changes are ready, commit them with a clear and concise message:

```sh
git add .
git commit -m "Add new feature or fix bug"
```

### 8. Push Changes

Push your changes to your forked repository:

```sh
git push origin feature/your-feature-name
```

### 9. Submit a Pull Request

Go to the GitHub page for your forked repository and click "Compare & pull request." Provide a description of your changes and submit the pull request to the main repository.

---

## Code Guidelines

- **Modular Code:** Organize the code into logical modules and keep the codebase maintainable.
- **Error Handling:** Use Rust's error handling mechanisms, such as `Result` and `Option`, appropriately.
- **Documentation:** Ensure that functions and modules are well-documented. If your change introduces new functionality, update or add relevant documentation.

---

## Code of Conduct

By participating in this project, you agree create a welcoming environment for all contributors.

---

## Reporting Issues

If you encounter any bugs or have suggestions for improvements, please open an issue in the GitHub repository. Include as much detail as possible to help us understand and address the problem.

---

## Thank You!

Your contributions help make **Core Shell** better, and we appreciate your time and effort. Happy coding!
