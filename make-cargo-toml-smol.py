#!/usr/bin/env python3

# Convert a Cargo.toml into a Cargo.toml.smol,
# leaving only the [[bin]] section corresponding to the "dummy" bin.
# This Cargo.toml.smol is then used by Docker builds.

# This should be run from the root of the repository.
# Consider making this a pre-commit hook by copying it to .git/hooks/pre-commit.

import toml

old_toml = toml.load("Cargo.toml")
old_toml["bin"] = [i for i in old_toml["bin"] if i["name"] == "dummy"]
toml.dump(old_toml, open("Cargo.toml.smol", "w"))