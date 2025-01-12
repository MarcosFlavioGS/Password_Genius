<div align="center">
  <img width=100% src="https://github.com/MarcosFlavioGS/Password_Genius/assets/95108526/8939958b-8dd5-46e8-8a9c-12e175157604" alt="Capturar">
</div>

# PassGen

This is a password generator written in Rust. It generates secure passwords with a specified length and saves them in separate files for easy retrieval. The generated passwords can also be copied to the clipboard for convenient use.

## Features

The password generator provides the following features:

- Generate secure passwords with a specified length
- Save generated passwords in separate files
- Retrieve passwords from saved files
- Copy passwords to the clipboard for quick access

## Requirements

- Rust programming language

## Encryptio

PassGen encrypts your passwords for better safety. But for that it expects that you have a secret key in your system as an environment variable, To set this variable, go to your shell config file(eg. .bashrc, .zshrc...) and to this line:

``` bash
export PASSGEN_KEY="MySecretKey123"
```

This was just an example. Put whatever text you want insede the double quotes **" "**

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/MarcosFlavioGS/Password_generator.git
   ```
 2. Cd do project directory and run:
  ```bash
  cargo build --release
  ```
## Usage

The password generator can be used with the following command-line arguments:

```bash
$ ./passgen [command] [dirname]
```
- [command] can be any of the following:
   - *Generate*:  Generates a new password and saves it in a file in the specified directory name.
   - *Insert*: Inserts a password into the specified dir file.
   - *get*: Retrieves the password from the specified file.
- [dirname] is the path to the directory where the password will be stored or retrieved.

## Examples

Creates a new directory named *github* that will contain a file with the new password:
```bash
$ ./passgen generate github
```
Passgen will then create a new github folder inside a *password* folder in your Home directory.

You could create multiple passwords for the same source passing the entire path:
```bash
./passgen generator github/profile1
```
Then, to retrieve the password, just type get followed profile1:
```bash
./passgen get profile1
```
or
```bash
./passgen get github/profile1
```

## File Structure
The generated passwords are stored in separate files within the passwords directory. Each file represents a specific password source or category.

The file structure will look like:
```
passwords/
├── source1/
│ └── pass
├── source2/
│ └── pass
└── source3/
└── pass
```
