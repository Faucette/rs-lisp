## The Package Manager

Lambda does not use semver because all semver can tell your code will break or not. Instead, we use time based versioning or `timever`.

The scheme looks like this: `<package-version>:<timestamp>.`

The way it works is that you change the package version whenever you have a breaking change and otherwise you just update the timestamp everytime you push new code.

We think this provides all the functionality of semver plus you also know when the last time was the library got updated.

Its also much easier to do automatic updates and resolve dependencies since the library with the newest timestamp can always be used.


### Internal Workings

Internally the package manager works tightly with Lambda to do provide detailed analysis of package changes.

The `timever` references the root of a tree of hash values where the `root hash` is a hash of the `namespaces' hashes` and each `namespace hash` is computed by hashing all the `expression hashes` in its scope.

This allows the package manager to track code changes and dependencies over time starting from the expression level and expanding to the package level.

Eventually, we would like to build code analysis/testing/search tooling on top of this technology in order to improve the software engineering practice.

For instance, a fast expression search would be possible where you could find all other identical expressions in the public packages repository. Also much more tightly grained and efficient dependency resolution could be built into the package manager.

## Security

Downloads are checksumed to prevent MITM attacks and all packages are required to be cryptographically signed and verified.

## Inspiration

Overall we've been inspired by the user-friendliness of the cargo and mix and are shooting for their ease and simplicity in our implementation.
