![Logo](https://github.com/MarcosFlavioGS/PassGen/assets/95108526/a2399c32-10b7-40ac-be17-9507f744f186)

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
