# svcpasswdgen

Create deterministic passwords from input data.

Imagine using a service account in a computer with the same password in each machine. If an attacker breaks into one of them and brute forces the password, every machine is lost.

If you want to create an individual password for each machine without the need to store each password, `svcpasswdgen` enters the stage.

![](./docs/gfx/svcpasswdgen.gif)

**WARNING**: Protecting the seed password is imperative! Should an attacker get hold of this password, also every machine is lost! Think carefully before using this tool. It might not be the right choice for you.

## How does it work?

`svcpassdgen` builds a sha512 sum over the given arguments

- `--machine`,
- `--account`,
- `--seed` and
- optional as many `--extra` as you pass in.

Those are concatenated and a sha512 sum is build over the string representation of this value.

The password results from the prefix the `--length` characters of the sha512 sum and the suffix. The prefix and suffix do not add any security, they simply help to satisfy password rules that need extra special characters.

The same can be done in a Unix shell:

```bash
PART1=$(echo -n "server001"|sha512sum|cut -d " " -f 1)
PART2=$(echo -n "superuser"|sha512sum|cut -d " " -f 1)
PART3=$(echo -n "passw0rd"|sha512sum|cut -d " " -f 1)
PASSWORD=$(echo -n "${PART1}${PART2}${PART3}"|sha512sum|cut -d " " -f 1)
echo "Pr3${PASSWORD:0:20}\$1X"
#Pr3bc362aa50b489f0f4fc6$1X
```

------

# License

The code is dual licensed under the [MIT License](./LICENSE-MIT) **or** the [APACHE 2.0 License](http://www.apache.org/licenses/LICENSE-2.0), which ever suits you better.