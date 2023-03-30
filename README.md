# svcpasswdgen

Create deterministic passwords from input data.

Imagine using a service account in a computer with the same password in each machine. If an attacker breaks into one of them and brute forces the password, every machine is lost.

If you want to create an individual password for each machine

- without the need to store each password,
  - by setting up an infrastructure, like hashicorp vault or
  - by managing it manually in some sort of password manager,

`svcpasswdgen` enters the stage.

![](./docs/gfx/svcpasswdgen.gif)

**WARNING**: Protecting the seed password is imperative! Should an attacker get hold of this password, also every machine is lost! Think carefully before using this tool. It might not be the right choice for you.

Using it on dedicated machines that are exclusively used for provisioning and can only create outgoing network connections may be a good start.

Use a long seed password (as in >= 64 characters long).


## How does it work?

`svcpassdgen` builds a sha512 sum (or digest) over the given arguments

- `--machine`,
- `--account`,
- `--seed` and
- optional as many `--extra` as you pass in.

Those are concatenated and a sha512 sum is build over the string representation of this value.

The password results from the prefix the `--length` characters of the final sha512 sum and the suffix. The prefix and suffix do not add any security! How could they, beeing the same on each machine and account? They simply help to satisfy password rules that need extra special characters.

**HINT**: The longer the resulting password, the better. But if you must enter it somewhere manually, there will be a trade-off between security and convenience.

The same can be done in a Unix shell:

```bash
PART1=$(echo -n "server001"|sha512sum|cut -d " " -f 1)
PART2=$(echo -n "superuser"|sha512sum|cut -d " " -f 1)
PART3=$(echo -n "passw0rd"|sha512sum|cut -d " " -f 1)
PASSWORD=$(echo -n "${PART1}${PART2}${PART3}"|sha512sum|cut -d " " -f 1)
echo "Pr3${PASSWORD:0:20}\$1X"
#Pr3bc362aa50b489f0f4fc6$1X
```


# Examples

- Passing the seed password as argument

    ```bash
    svcpasswdgen --machine server001 --account superuser --seed passw0rd
    Pr3bc362aa50b489f0f4fc6$1X
    ```
- Providing extra information that changes the resulting password.

    ```bash
    svcpasswdgen --machine server001 --account superuser --seed passw0rd --extra rack-042
    Pr3fc88df44a89c3202cf8b$1X
    ```
- Store the seed password in the environment variable `SEED_PASSWD`:

    ```bash
    export SEED_PASSWD="passw0rd"
    svcpasswdgen --machine server001 --account superuser
    Pr3bc362aa50b489f0f4fc6$1X
    ```
- Remove the seed password from the environment variable `SEED_PASSWD` and do not provide it at all:

    ```bash
    unset SEED_PASSWD
    svcpasswdgen --machine server001 --account superuser
    Enter seed password:
    Pr3bc362aa50b489f0f4fc6$1X
    ```


# Compile and install

Compiling probably works on any system that has a Rust compiler with standard library available.

Head over to [www.rust-lang.org](https://www.rust-lang.org/tools/install) and follow the instructions if you don't have a Rust compiler installed yet.

```bash
# clone and compile the code
git clone https://github.com/hardcodes/svcpasswdgen.git
cd svcpasswdgen
cargo build --release
```

After that copy the file `target/release/svcpasswdgen` to a directory of your liking.

------

# License

The code is dual licensed under the [MIT License](./LICENSE-MIT) **or** the [APACHE 2.0 License](http://www.apache.org/licenses/LICENSE-2.0), which ever suits you better.