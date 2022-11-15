# Sui Starter

## What is it?
A starter kit for building a Sui applications. It includes a basic structure for a Sui application with a e2e local testing envrionment as a cli.

## Installation
Currently, the only way to install the starter is to clone the repo and install 

```cargo install --path .```

## Create a new project
To create a new project run the following command

```sui-starter init <project-name>```

## Run tests against the project
To run tests against the project run the following command

```sui-starter test```

this will run the test entry in Move.toml under extra (which acts as a e2e test) and the sui move tests.
```
[package]
...

[dependencies]
Sui = { git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework", rev = "devnet" }

[addresses]
...

[extra]
test = "npm run test"
```

Flags can be provided to skip the sui tests or the e2e tests

```sui-starter test --contract --full```


## Prior Art
[Anchor](https://github.com/coral-xyz/anchor)
