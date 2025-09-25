git-wire
========

A git subcommand
which wires part of other repositoriy's source code
into the repository in a declarative manner.

installation
------------

- If your environment have rust installed:

    ```
    $ cargo install git-wire
    ```

    Or you can build from source if you clone [this repository](https://github.com/msr1k/git-wire).

- In other cases:

    Currently not supported.


Preparation
-----------

Create a file named `.gitwire` at the root of the repository with following JSON format.

```json
[
  {
    "url": "url-of-the-repository",
    "rev": "revision (commit hash or branch name or tag name)",
    "src": "source directory of the target repository",
    "dst": "directory where to put the `src` on this repositry"
  },
  ...
]
```

### name: Optional key to identify the item
Optionally, you can define `"name"` which is the key to identify the item.

It can be used to narrow down the scope of the command into particular item.

```json
[
  {
    "name": "any string can be a name, it must be unique within a .gitwire file",
    "url": "url-of-the-repository",
    "rev": "revision (commit hash or branch name or tag name)",
    "src": "source directory of the target repository",
    "dst": "directory where to put the `src` on this repositry"
  },
  ...
]
```

### dsc: Optional key to describe the item
Optionally, you can also define `"dsc"` to describe the item.

```json
[
  {
    "dsc": "a description of the item can be written here if you want",
    "url": "url-of-the-repository",
    "rev": "revision (commit hash or branch name or tag name)",
    "src": "source directory of the target repository",
    "dst": "directory where to put the `src` on this repositry"
  },
  ...
]
```

### mtd: Optional key to specify checkout method
Optionally, you can select a method, `"shallow"`, `"shallow_no_sparse"`, and `"partial"` to checkout src like below.

```json
[
  {
    "url": "url-of-the-repository",
    "rev": "revision (commit hash or branch name or tag name)",
    "src": "source directory of the target repository",
    "dst": "directory where to put the `src` on this repositry",
    "mtd": "shallow/shallow_no_sparse/partial"
  },
  ...
]
```

### About checkout methods

#### shallow (default)
This method gets all the files under a specified `src` at once from specified `rev`.
In almost all cases, it will be the best choice.

If you omit `"mtd"` key, `"shallow"` method will be used automatically.


#### shallow_no_sparse
`"shallow_no_sparse"` gets all the files managed by that repository at once from specified `rev`,
it inherently requires more memory and temporary storage than `"shallow"` and `"partial"`,
but it must be faster than `"partial"` if there are many files to `sync`.  
(It must not faster than `"shallow"`.)

I assume that it is the alternative method when you face some problems by using `"shallow"`.

#### partial (not recommended)
`"partial"` gets all the files under a specified directory ONE BY ONE.
Since it performs downloading for each file respectively (it is mere git command behavior),
it could be significantly slow as the number of files grows.
(In the worst case, you might get an error)

There is a faint chance that it might be sperior than `"shallow"` in terms of memory consumption.
But basically it seems no motivation to use this method.

Main Commands
--------

### sync

Sync sources depending on the definition of the `.gitwire`.

Please note that it always clears the destination before sync started for each item.

    $ git wire sync

### check

Check sources depending on the definition of the `.gitwire`.

If there are some differences, this command reports each of them all,
and returns with exit code 1, otherwise returns with 0.

    $ git wire check

Options for main commands
----

### name

`-n <name>` or `--name <name>` can be added for both command sync and check.

When you add this option, command will be executed only for an item that has specified name.

### target

`-t <name>` or `--target <name>` can be added for both command sync and check.

This option is mere alias of `--name` and `-n`.

### singlethread

If you set this, `-s` or `--singlethread`, commands work on single thread.

Unless you specify this option, commands work on multiple threads.

Other Commands
--------

### direct-sync

Almost same as `sync` but it is performed based on arguments given instead `.gitwire` definition.

    $ git wire direct-sync --help

The meaning of arguments are equivalent to the conrresponding `.gitwire` JSON key-value.

### direct-check

Almost same as `check` but it is performed based on arguments given instead `.gitwire` definition.

    $ git wire direct-check --help

The meaning of arguments are equivalent to the conrresponding `.gitwire` JSON key-value.


A sample .gitwire
-----------------

This `.gitwire` sample wires this repository's `src/common`
at revision v1.0.0 and v1.1.0 into `src_common_v1.0.0`, `src_common_v1.1.0` directory respectively.

https://github.com/msr1k/git-wire/blob/main/.gitwire

## Changelog

- v1.5.0 (2024/09/6)

    - Added `direct-sync` and `direct-check` commands

- v1.4.0 (2024/06/15)

    - Make command execution multi-threaded.
    - `-s`, `--singlethread` option added to forcibly execute a command with single thread.
    - `-t`, `--target` option added which is same as existing `-n` and `--name`.

- v1.3.1 (2024/06/09)

    - Some document and output enahancements. (README, `--help` and console output format)
    - Color output support.

- v1.3.0 (2024/06/08)

    Added optional `name` and `dsc` key to the .gitwire json object.

    - `name` can be used to narrow down the scope of the commands.
    - `dsc` can be used to describe the item

- v1.2.1 (2024/03/19)

    Updated dependent crates, no functional changes included.

- v1.2.0 (2023/09/22)

    Added sparse-checkout feature into `"shallow"` checkout method.
    In almost all cases it is the best in terms of memory & time consumption.

    And previous `"shallow"` checkout method renamed as `"shallow_no_sparse"`,
    it could be an alternative to `"shallow"` if you encount some problems with `"shallow"`.

    In addition, several tiny improvements have been done.

- v1.1.3 (2023/09/21)

    Fixed: if target repository placed somewhere including `.git`,
    `git wire check` wrongly ignores all the files in this repository,
    and no changes detected if it exists.

- v1.1.2

    Update all dependent crate's versions.  
    Fix a typo.

- v1.1.1

    Change default checkout method from `"partial"` to `"shallow"`,
    since it seems that, in most cases, `"shallow"` is faster and stabler.

- v1.1.0

    Added optional `"mtd"` (method) setting which can control the way to chekcout target source code.

- v1.0.1

    Replace one dependent crate with one of others to reduce unwanted dependencies.  
    (No functionality changes have been made from v1.0.0.)

- v1.0.0

    Initial Version.


License
-------

The MIT License
