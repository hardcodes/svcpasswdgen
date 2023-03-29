# svcpasswdgen

Create deterministic passwords from input data.

Imagine using a service account in a computer with the same password in each machine. If an attacker breaks into one of them and brute forces the password, every machine is lost.

If you want to create an individual password for each machine without the need to store each password, `svcpasswdgen` enters the stage.

**WARNING**: protecting the seed password is imperative! Should an attacker get hold of this password, also every machine is lost! Thin carefully before using this tool. It might not be the right choice for you.

## How does it work?

`svcpassdgen` builds a sha512 sum over the given arguments

- `--machine`,
- `--account`,
- `--seed` and
- optional as many `--extra` as you pass in.

Those are concatenated and a sha512 sum is build over this value.

The password results from the prefix the `--length` characters of the sha512 sum and the suffix. The prefix and suffix do not add any security, they simply help to satisfy password rules that need extra special characters.
------

# License

The code is dual licensed under the [MIT License](./LICENSE-MIT) **or** the [APACHE 2.0 License](http://www.apache.org/licenses/LICENSE-2.0), which ever suits you better.