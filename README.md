# Comb
This is the repository for Comb, a CLI for the Handlebars templating engine. Currently it just provides way to access environment variables from inside a Handlebars template.

[![Latest deployment status](https://img.shields.io/drone/build/hwittenborn/comb?logo=drone&server=https%3A%2F%2Fdrone.hunterwittenborn.com)](https://drone.hunterwittenborn.com/hwittenborn/comb/latest)

## Example
```handlebars
The user's home directory is {{ env.HOME }}
```

## Installation
### MPR (Debian/Ubuntu)
Comb is available in the [MPR](https://mpr.makedeb.org/packages/comb).

To install, run the following:

```sh
git clone 'https://mpr.makedeb.org/comb'
cd comb/
makedeb -si
```