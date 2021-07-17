![GitHub contributors](https://img.shields.io/github/contributors/joaopedropp/git-management?style=flat-square)
[![GitHub issues](https://img.shields.io/github/issues/joaopedropp/git-management?style=flat-square)](https://github.com/joaopedropp/git-management/issues)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/JoaoPedroPP/Git-Management/blob/main/LICENSE)
<!-- [![GitHub stars](https://img.shields.io/github/stars/joaopedropp/git-management?style=flat-square)](https://github.com/joaopedropp/git-management/stargazers) -->

# Git Management CLI


## About The Project

Have you ever wondered how nice it would be if you could code your project and when you think it is time to publish it on a version control host like GitHub you could just type a command line and it sets up your repository? That is what Git Management CLI, gitmgt, gives to you. Now you do not need to leave your project's environment to manually create you repository on GitHub, you just need to type `gitmgt create -n awesome_repo` and then push your project.

### Built With

* [Rust](https://www.rust-lang.org/)
* [Clap](https://github.com/clap-rs/clap)


## Getting Started

### Prerequisites

To install gitmgt it is necessary that your machine has the following prerequisites:

* Rust
* Cargo

### Installation

#### Cargo
The easiest, and recommended way to install gitmgt is to download directly by [crates.io](https://crates.io/) typing `cargo install gitmgt` in a terminal.

Once it is finished, the tool is ready to be used and you can type `gitmgt --version` just to check.

#### Build from source

Other option is to download the code from this repository and build it manualy by typing `cargo build --release` in a terminal and when cargo process finishes just move the binary file from `target/release/` directory to where you think is better and add the binary path to the `$PATH`.

Once it is finished the the tool is ready to use and you can type `gitmgt --version` just to check.

After the installation process finishes, to start using gitmgt cli the user will have to insert the Github credentials to have access to all the set tools. The user just needs to create a new `personal access token` in Github portal and then just type in a terminal

```sh
gitmgt config -u github_username -t personal_access_token
```

After this all the commands are ready to be used.

## Usage

Update Github credentials
```sh
gitmgt github_config -u github_username -t github_token
```

Github Repository creation
```sh
gitmgt create -n awesome_repository_name
```

since Github is the most common version control service, it is the default option and its indication can be suppressed.
```sh
gitmgt github create -n awesome_repository_name
```

Org's repository creation
```sh
gitmgt github create -o orr_name -n awesome_repository_name
```

Private repository creation
```sh
gitmgt github create -n awesome_repository_name -p
```

Auto init repository creation
```sh
gitmgt github create -n awesome_repository_name -i
```

Repository deletion
```sh
gitmgt delete -n awesome_repository_name
```

Org's repository deletion
```sh
gitmgt delete -o org_name -n awesome_repository_name
```

Repository archiving
```sh
gitmgt archive -n awesome_repository_name
```

Org's repository archiving
```sh
gitmgt archive -o org_name -n awesome_repository_name
```

Repository's description update
```sh
gitmgt update -n awesome_repository_name -d "update awesome description od the repo"
```

Repository's visibility update public to private
```sh
gitmgt update -n awesome_repository_name -i
```

Repository's visibility update private to public
```sh
gitmgt update -n awesome_repository_name
```

Looking for help
```sh
gitmgt --help
```

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Copyright 2021 João Pedro Poloni Ponce

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
