Newsboat Archiver
=================

The purpose of this program is to download and archive all the links of every entry that exists in a Newsboat SQLite Database file.
The following is a list of the currently supported software to archive web pages:

- [Monolith](https://github.com/Y2Z/monolith)
- [Lynx](https://lynx.invisible-island.net/)

Usage
=======

```
newsboat-archiver 1.0
Romeu Vieira <romeu.bizz@gmail.com>
Archive Newsboat DB information

USAGE:
    newsboat-archiver [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --blacklist <FILE>         Blacklist file
    -f, --file <FILE>              Database file
    -d, --directory <DIRECTORY>    Output directory
    -s, --settings <FILE>          Settings file
```

Example
=======

```
./newsboat-archiver -f $HOME/.local/share/newsboat/cache.db -d outputfolder/ -s settings.conf
```

Config Example
==============

The config example should have the following format:

```
EXEC|HOST|ARGS
```

Just like this:

```
monolith|devblogs.microsoft.com|kIFfMj
lynx|fluentcpp.com|none
```

LICENSE
=======

Copyright 2021 Romeu Gomes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
