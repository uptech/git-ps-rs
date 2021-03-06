## Development

### Filesystem Hierarchy

The following is a breakdown of the filesystem hierarchy and what each of the
module's is responsible for. Hopefully this gives you a spring board into
understanding where to find certain things or where to put things if you are
contributing to the project.

* `src`
	* `main` - command line entry point, parsing & sub command handoff
	* `commands` - module collecting command functions with cli concerns
	* `lib` - library entry point and definition of public interface
	* `ps` - parenting module collecting Patch Stack specific modules 
		* `private` - internal cmds & shared functionality & utilities
		* `public` - library public interfaces for the commands

### System Dependencies

In order to build git-ps-rs it needs to have the [gpgme][] library installed
including the headers, etc.

On macOS this can be installed by `brew install gpgme`.

On Ubunto this can be installed by `apt-get install libgpgme-dev`.

As part of the Cargo build process it will try and get the correct
configuration informating using `gpgme-config`. This dependency is what enables
us to support GPG commit signing.

### Build

To build the application for local development and debug simply run the
following.

```text
cargo build
```

To build the application for release run the following.

```text
cargo build --release
```

### Test

To run the test suite simply run the following.

```text
cargo test
```

[gpgme]: https://gnupg.org/software/gpgme/index.html
