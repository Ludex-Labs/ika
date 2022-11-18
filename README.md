<div align="center">
  <img height="200x" src="./ika.png" />

  <h1>Ika</h1>
</div>

## What is it?
A starter kit for building a [**Sui**](https://sui.io/) applications. It includes a basic structure for a **Sui** application with a e2e local testing envrionment as a cli.

## Installation
Firstly you need to go through the process of installing **Sui**.
You can find this [here](https://docs.sui.io/build/install)

Currently, the only way to install the starter is to clone the repo and install 

```cargo install --path .```

## Create a new project
To create a new project run the following command

```ika new <project-name>```

## Run tests against the project
To run tests against the project run the following command

```ika test```

This will run the test entry in Move.toml under ika (which acts as a e2e test) and the **Sui** move tests.
```
[package]
...

[dependencies]
Sui = { git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework", rev = "devnet" }

[addresses]
...

[ika]
test = "npm run test"
```

Flags can be provided to skip the **Sui** tests or the e2e tests

```ika test --skip-contract --skip-e2e```


## Prior Art
[Anchor](https://github.com/coral-xyz/anchor)

[Hardhat](https://hardhat.org/)
