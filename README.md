# Password Generator

This is a password generator written in Rust. It generates secure passwords with a specified length and saves them in separate files for easy retrieval. The generated passwords can also be copied to the clipboard for convenient use.

## Features

The password generator provides the following features:

- Generate secure passwords with a specified length
- Save generated passwords in separate files
- Retrieve passwords from saved files
- Copy passwords to the clipboard for quick access

## Requirements

- Rust programming language

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/password-generator.git
   ```
 2. Cd do project directory and run:
  ```bash
  cargo build --release
  ```
## Usage

The password generator can be used with the following command-line arguments:

```bash
$ ./password-generator [command] [dirname]
```
- [command] can be any of the following:
- -generate
