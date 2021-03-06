# Haus

Here Another URL Shortener

# Install

I assume you already have [rust](https://www.rust-lang.org/) and [rustup](https://rustup.rs/) installed. This project is using rocket with nightly rust, so set
it accordingly.

```
git clone https://github.com/matematikaadit/haus
cd haus
rustup override set nightly
```

To run the project with the default setting in the dev environment, use this command:

```
cargo run
```

Now you can use the URL Shortener from http://localhost:8000/

# Initialize the Database

Exit previous `cargo run` command. To create the sqlite database, execute:

```
sqlite3 database_name.db '.read schema.sql'
```

Make sure that you have `sqlite3` installed.

# License

MIT License

Copyright (c) 2017 Adit Cahya Ramadhan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

