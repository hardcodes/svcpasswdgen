# svcpasswdgen -  service account password generator

Create deterministic passwords from input data.

Imagine using a service account in a computer with the same password in each machine. If an attacker breaks into one of them and brute forces the password, every machine is lost.

If you want to create an individual password for each machine

- without the need to store each password,
  - by setting up an infrastructure, like hashicorp vault or
  - by managing it manually in some sort of password manager,

`svcpasswdgen` enters the stage.

![](./docs/gfx/svcpasswdgen.gif)

**WARNING**: Protecting the seed password is imperative! Should an attacker get hold of this password, also every account using a generated password is lost! Think carefully before using this tool. It might not be the right choice for you.

Using it on dedicated machines that are exclusively used for provisioning and can only create outgoing network connections may be a good start.

Use a long seed password (as in >= 64 characters long) to protect yourself against attackers brute forcing it.


## How does it work?

1. `svcpassdgen` builds a hash (or digest) over the given arguments

  - `--machine`,
  - `--account`,
  - `--seed` and
  - optional as many `--extra` as you pass in.

    A sha512 sum is built over all argument values (`--machine`, `--account`, `--seed` and `--extra`) and stored as hex representation, like the `sha512sum` command does. Those hex converted sums are concatenated to a string and a final sha512 sum is build over the string and stored as hex representation again.
  
    The first 120 characters are used as input for the `argon2` hash algorithm.
2. `--machine`, `--account` are concatenated as a string and a sha512 sum is build over it and stored as hex representation. The first 32 characters are used to build a *salt* value for the `argon2` hash algorithm.

    The result is then base64 encoded with no padding to mimic the `argon2` cli tool.
3. The password results from the prefix, the `--length` characters of the base64 encoded final `argon2` hash and the suffix. The prefix and suffix do not add any security! How could they, beeing the same on each machine and account? They simply help to satisfy all of your possible password rules that need extra special characters.

**HINT**: The longer the resulting password, the better. But if you must enter it somewhere manually, there will be a trade-off between security and convenience.

The same can be done in a Unix shell:

```bash
PART1=$(echo -n "server001"|sha512sum|cut -d " " -f 1)
PART2=$(echo -n "superuser"|sha512sum|cut -d " " -f 1)
PART3=$(echo -n "passw0rd"|sha512sum|cut -d " " -f 1)
SHA512SUM=$(echo -n "${PART1}${PART2}${PART3}"|sha512sum|cut -d " " -f 1)
SALTHASH=$(echo -n "server001superuser"|sha512sum|cut -d " " -f 1)
SALT=$(echo -n "${SALTHASH:0:32}")
USEDPWD=$(echo -n "${SHA512SUM:0:120}")
echo "SALT         :  ${SALT}"
echo "SHA512SUM    :  ${SHA512SUM}"
echo "used password:  ${USEDPWD}\n"
ARGON2=$(echo -n "${USEDPWD}"|argon2 "${SALT}" -id -t 50 -m 16 -l 32 -v 13)
echo ${ARGON2}
ARGON2SHASH=$(echo -n "${ARGON2}"|grep "Hash:"|sed -E -e 's/\s//g'|cut -d ":" -f 2|xxd -r -p|base64 -w 0|sed -e 's/=//g'|sha512sum|cut -d " " -f 1)
PASSWORD=$(echo -n ${ARGON2SHASH}|base64 -w 0)
echo "Pr3${PASSWORD:0:20}\$1X"
```

**output**

```
SALT         :  cddab6bb2eaeaef88d39998fbd3aba5a
SHA512SUM    :  bc362aa50b489f0f4fc6594aca3a6b24093fb507d7813e15493ca791a2fe2e12fcefd91fa15a5149884d30e3b0a6aebd734d55a7a12559b66aa93f3a675fa71d
used password:  bc362aa50b489f0f4fc6594aca3a6b24093fb507d7813e15493ca791a2fe2e12fcefd91fa15a5149884d30e3b0a6aebd734d55a7a12559b66aa93f3a

Type:		Argon2id
Iterations:	50
Memory:		65536 KiB
Parallelism:	1
Hash:		8657c024054195e2ee62a9d72663bfdda754018187bccb6086d6189416f473b2
Encoded:	$argon2id$v=19$m=65536,t=50,p=1$Y2RkYWI2YmIyZWFlYWVmODhkMzk5OThmYmQzYWJhNWE$hlfAJAVBleLuYqnXJmO/3adUAYGHvMtghtYYlBb0c7I
1.615 seconds
Verification ok
Pr3YWQ0ZjE2ZDZlOWYxMjkw$1X
```


**Validation** 

```bash
# validate salt
echo -n "${SALT}"|base64 -w 0|sed -e 's/=//g'
# Y2RkYWI2YmIyZWFlYWVmODhkMzk5OThmYmQzYWJhNWE
# validate hash
echo -n "8657c024054195e2ee62a9d72663bfdda754018187bccb6086d6189416f473b2"|xxd -r -p|base64 -w 0|sed -e 's/=//g'
# hlfAJAVBleLuYqnXJmO/3adUAYGHvMtghtYYlBb0c7I
#
# OK, both values match encoded ouput
```


# Examples

- Passing the seed password as argument

    ```bash
    svcpasswdgen --machine server001 --account superuser --seed passw0rd
    Pr3SGFzaDoJCTA0ZGNhYTMw$1X
    ```
- Providing extra information that changes the resulting password.

    ```bash
    svcpasswdgen --machine server001 --account superuser --seed passw0rd --extra rack-042
    TODO
    ```
- Store the seed password in the environment variable `SEED_PASSWD`:

    ```bash
    export SEED_PASSWD="passw0rd"
    svcpasswdgen --machine server001 --account superuser
    Pr3SGFzaDoJCTA0ZGNhYTMw$1X
    ```
- Remove the seed password from the environment variable `SEED_PASSWD` and do not provide it at all:

    ```bash
    unset SEED_PASSWD
    svcpasswdgen --machine server001 --account superuser
    Enter seed password:
    Pr3SGFzaDoJCTA0ZGNhYTMw$1X
    ```
- Paste the generated password to the clipboard:

  ```bash
  unset SEED_PASSWD
  svcpasswdgen --machine server001 --account superuser --clip                
  Enter seed password: 

  Pasted password to clipboard.
  Waiting 20 seconds before clearing the clipboard.
  Cleared clipboard.
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

