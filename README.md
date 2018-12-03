find-and-replace
================

A small tool that allow the user to find recursively in a directory all the
files matching a glob (`src/**/*.html`) and substitute text in the content
using a regular expression.

This tool can easily be built using the musl target.

Usage
-----

```
find-and-replace 1.0.0
Cecile Tonglet <cecile.tonglet@collibra.com>
Tool to find and replace, recursively in a directory using globs, file content using a regular expression

USAGE:
    find-and-replace <glob> <regex> <replacement>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <glob>           Glob (Unix shell style pattern)
    <regex>          Regular expression to search
    <replacement>    Replacement expression
```

Example:

```
./find-and-replace /path/to/src/**/*.html '<li id="\\d+">' '<li>'
```
