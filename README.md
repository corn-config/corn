# 🌽 Corn

A simple and pain-free configuration language.

Corn has been designed using inspiration from JSON and Nix to produce a language
that's easy and intuitive to write, good for config files, and has a feature-set
small enough you can learn it in minutes. It was born out of the following frustrations:

- JSON is not a config language, despite how often people use it as one.
- TOML is good for flat structures but gets ugly quickly with deeper objects.
- YAML is far too complex and its whitespace rules make it error-prone.
- Nix gets much closer to what I want, but it's not easy to integrate.

## What Corn Is Not

Corn is not a full programming language and does not try to be. 
There are no dynamic variables, there are no operations and there are no statement blocks.

Likewise, Corn is not a data exchange format. 
Unlike JSON or YAML or TOML, you cannot serialize code back into Corn. 

## Usage

### As a binary

You can install the official Corn CLI to convert files from the `.corn` format
into any supported output format. Currently, these are:

- JSON
- YAML
- TOML

Windows, Linux and macOS are currently supported.

Install it using cargo:

```shell
cargo install corn-cli
```

Then simply:

```shell
corn file.corn
corn file.corn -t yaml
```

### As a library

Corn can be used as a Rust library to deserialize config files directly
without needing to convert to other file formats.

[crate](https://crates.io/crates/libcorn) | [docs](https://docs.rs/libcorn)

The recommended way to do this is using `serde` into a struct:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    foo: u8
}

fn main() {
    let corn = "{foo = 42}";
    let config = corn::from_str::<Config>(corn).unwrap();
    assert_eq!(config.foo, 42);
}
```

You can also use `libcorn::parse` directly to get an AST representation of the config. 
This can be serialized directly, which potentially offers greater control or a faster route when converting to other formats.

### JavaScript

A WASM version for use in Node.js and browsers is also available,
which can parse Corn into valid JavaScript objects.

> ⚠ Note when running under Node.js you will require `--experimental-modules` for versions <= 16. 
> On all versions you require `--experimental-wasm-modules`.

[npm](https://www.npmjs.com/package/libcorn)

```js
import * as corn from 'libcorn';

const parsed = corn.parse('{foo = "bar"}');
console.log(parsed) // Map(foo -> "bar")
```

### Lua

Lua support can be built into the library using __one of__ the feature flags,
allowing you to bind directly to `libcorn.so`:

- `lua51`
- `lua52`
- `lua53`
- `lua54`
- `luajit`
- `luajit52`

So long as `libcorn.so` is in Lua's module path, it can be then be used as below:

```lua
local libcorn = require("libcorn")
local success, res = pcall(libcorn.parse, '{foo = "bar"}')

if success then
    print(res.foo) -- lua table
else
    print(res) -- pretty printed error
end
```

Thanks to [A-Cloud-Ninja](https://github.com/A-Cloud-Ninja) for adding Lua support!

## Writing Corn

> This section gives all the outputs in JSON format. Remember you can output in any supported format!

All Corn files must contain a top-level object that contains keys/values.

The first character in the key cannot be whitespace, 
a number or any of the following characters: `. - " $ { [ =`.
The remaining characters can be any unicode character except whitespace and the following:  `. =`.

Keys do not require quotes around them, although you can optionally use `single quotes` to avoid the above limitations and use any character in any position.
Within quoted keys, you can use `\'` to escape a quote.

Values must be one of the following:

- String
- Integer
- Float
- Boolean
- Object
- Array
- Null
- Input

(More on these types below)

Keys and values are separated by an equals `=`.

A very basic example therefore would be:

```corn
{
    hello = "world"
}
```

Which of course maps to the following in JSON:

```json
{
  "hello": "world"
}
```

A more complex example would be something like a `package.json`. 
This makes use of inputs, various other data types and key chaining:

```corn
let {
    $entry = "dist/index.js"
    $author = { name = "John Smith" email = "mail@example.com" }
} in {
    name = "example-package"
    version = "1.0.0"
    main = $entry
    bin.executable = $entry
    private = false
    
    author = $author
    author.url = "https://example.com" 
    
    contributors = [ $author ]
    
    scripts.build = "tsc"
    scripts.run = "node dist"
    
    dependencies = {
        dotenv = "^8.2.0"
        // put the rest of your deps here...
    }
    
    devDependencies.typescript = "^4.5"
    
    config.port = 8080
    config.hostname = null
}
```

<details>
<summary>This output's a bit longer than the others, so click here to expand it and have a look.</summary>

```json
{
  "author": {
    "email": "mail@example.com",
    "name": "John Smith",
    "url": "https://example.com"
  },
  "bin": {
    "filebrowser": "dist/index.js"
  },
  "config": {
    "hostname": null,
    "port": 8080
  },
  "contributors": [
    {
      "email": "mail@example.com",
      "name": "John smith"
    }
  ],
  "dependencies": {
    "dotenv": "^8.2.0"
  },
  "devDependencies": {
    "typescript": "^4.5"
  },
  "main": "dist/index.js",
  "name": "example-package",
  "private": false,
  "scripts": {
    "build": "tsc",
    "run": "node dist"
  },
  "version": "1.0.0"
}
```
</details>

### Types

#### String

Strings must be surrounded by double quotes. All unicode is supported.

The following escape codes are supported: `\\`, `\"`, `\n`, `\r`, `\t`.
You can also add any unicode character using a `\uXXXX` escape.

```corn
foo = "bar"
two_lines = "hello\nworld"
```

#### Integer

Integers are signed and 64 bit, meaning you can use any value
between `-9223372036854775808` and `9223372036854775807`.

You can use a single underscore `_` separator between digits in decimal values to break up larger numbers.

Hexadecimal values are also supported.

```corn
answer = 42
big_value = 1_000_000
color = 0xfafafa
```

#### Float

Double precision (64-bit) floats are used.

Very large or very small values can be represented with an exponent `e`.

```corn
pi = 3.14159
very_big = 1.01e+10
very_small = 1.01e-10
```

#### Boolean

Either `true` or `false`.

```corn
not_false = true
```

#### Object

Objects use braces to mark the start and end. 
They contain key/value pairs.

There is no restriction on nesting objects within objects or arrays.

```corn
{
  foo = "bar"
  hello = "world"
}
```

#### Array

Arrays use square brackets to mark the start and end. 
Values are space-separated. 
Like objects, there is no restriction on nesting arrays or objects inside.

```corn
    array = [ 1 2 3 ]
```

You can also include whitespace as you please and mix element types:

```corn
{
    array = [ 1 2 3 ]
    array2 = [
        "one"
        2
        true
        { three = 3 }
    ]
}
```

The above converts to the following JSON:

```json
{
  "array": [
    1,
    2,
    3
  ],
  "array2": [
    "one",
    2,
    true,
    {
      "three": 3
    }
  ]
}
```

#### Null

Null values simply use the `null` keyword:

```corn
foo = null
```

#### Inputs

Sometimes it may be useful to store some values at the top
of the config file, and use or re-use them later on,
or even use them to compose more complex values. Corn supports config inputs, akin to variables but constant.

All input names start with a dollar sign `$` followed by an alphabetic ASCII character or an underscore `_`.
This can then be followed by any amount of alphanumeric ASCII characters or underscores.

Inputs can be used to store any value type, or inside strings.

To declare inputs, you must include a `let { } in` block
at the start of your config.

```corn
let { 
    $firstName = "John"
    $lastName = "Smith" 
    
    $birthday = {
        day = 1
        month = 1
        year = 1970
    }
    
} in {
    name = {
        first = $firstName
        last = $lastName
    }
    
    dob = $birthday
}
```

As well as declaring your own inputs, you can also access any environment variables by prefixing input names with `$env_`.
For example, to use the system `PATH`:

```corn
{
    path = $env_PATH
}
```

Will output something similar to:

```json
{
  "path": "/home/jake/.cargo/bin:/home/jake/.local/bin:/usr/local/bin:/usr/bin"
}
```

If a referenced environment variable is not defined, 
the parser will fall back to a standard input of the same name (including the `env_` prefix). 
This allows you to create overridable defaults.

```corn
let { 
    // will fall back to this if `FOO` is not set
    $env_FOO = 42
} in { 
    foo = $env_FOO
    bar = $env_BAR
}
```

Inputs are intentionally quite limited as to what they can do -
if you need more power you should use a full language. 

#### String Interpolation

String inputs can be put inside string values to interpolate their values, like so:

```corn
let {
    $subject = "world"
} in {
    greeting = "hello, $subject"
}
```

Evaluates to:

```json
{
  "greeting": "hello, world"
}
```

To use a literal dollar, you can escape with `\$`. For example, `\$subject`. 

#### Merging

Somtimes you want to re-use an object or array to compose a larger object/array. 
It is possible to achieve this by merging two together using the `..$input` spread operator.
This allows you to spread object inputs into other objects, and array inputs into other arrays.

```corn
let {
    $base = { foo = "bar"}
} in {
    ..$base
}
```

Evaluates to:

```json
{
    "foo": "bar"
}
```

And with arrays:

```corn
let {
    $low = [ 1 2 ]
    $high = [ 3 4 ]
} in {
    nums = [ ..$low ..$high ]
}
```

Evaluates to:

```json
{
     "nums": ["1", "2", "3", "4"]
}
```

Object keys and spreads are evaulated in the order they are written, 
which allows you to spread a base object and then manually overwrite specific keys:

```corn
let {
    $base = { 
        greeting = "hello"
        subject = "world"
    }
} in {
  ..$base
  subject = "github"
}
```

JSON:

```json
{
    "greeting": "hello",
    "subject": "github"
}
```

### Comments

At any point you can start a comment using `//`. A comment is terminated by a newline `\n` character.
Comments are entirely ignored and not included in the output.

```corn
{
    // this is a single-line comment
    foo = bar // this is a mixed-line comment
}
```

### Nesting Keys

Throughout the page, we've created objects within objects
using a syntax like this:

```corn
{
    foo = {
        bar = "baz"
    }
}
```

While this is okay for short cases, it can get tiresome very fast
and the braces add a lot of noise when reading.

To solve this, keys can be chained to create deep objects instantly:

```corn
{
    foo.bar = "baz"
}
```

Which in JSON is:

```json
{
  "foo": {
    "bar": "baz"
  }
}
```

You can mix and match chained keys with nested objects at any time:

```corn
{
    foo = {
        bar.baz = 42
        qux = true
    }
    
    foo.quux = [ "green eggs" "ham" ]
}
```

JSON:

```json
{
  "foo": {
    "bar": {
      "baz": 42
    },
    "qux": true,
    "quux": ["green eggs", "ham"]
  }
}
```

### Whitespace

Almost all whitespace in Corn is optional, since keywords and types end as soon as they end. 
There are only a few exceptions to this:

- An integer or float following an integer or float must be whitespace separated to tell where one ends and the next starts.
- References to inputs must terminate with whitespace as otherwise the parser cannot tell where the name ends.
- There must be whitespace between `key=value` assignments (ie after each value and before the next key).

This means the below is perfectly valid (although for general consistency and readability this is strongly not recommended):

```corn
{
    one={foo="bar" bar="foo"}
    two={foo=1 bar=2}
    three={foo=1.0 bar=2.0}
    four={foo=true bar=false}
    five={foo=null bar=null}
    six={foo={} bar={}}
    seven={foo=[] bar=[]}

    eight=["foo""bar"]
    nine=[truefalse]
    ten=[nullnull]
    eleven=[[][]]
    twelve=[{}{}]
}
```

And in fact, we could even go as far as to reduce that to a single line:

```corn
{one={foo="bar" bar="foo"} two={foo=1 bar=2} three={foo=1.0 bar=2.0} four={foo=true bar=false} five={foo=null bar=null} six={foo={} bar={}} seven={foo=[] bar=[]} eight=["foo""bar"] nine=[truefalse] ten=[nullnull] eleven=[[][]] twelve=[{}{}]}
```

## Editor Support

### VSCode

- Basic syntax highlighting
- Support for bracket matching and commenting

[extension](https://marketplace.visualstudio.com/items?itemName=JakeStanger.corn&ssr=false) | [repo](https://github.com/JakeStanger/corn-vscode)

### IntelliJ

- Syntax highlighting
- Live error checking
- Support for bracket matching and commenting
- Basic completion support
- Basic refactoring support
- Basic Formatting and code style options

[extension](https://plugins.jetbrains.com/plugin/18519-corn) | [repo](https://github.com/JakeStanger/corn-intellij)

### Neovim

> Via [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter)

- Syntax highlighting
- Parsing and live syntax error checking
- Support for bracket matching and commenting

[parser](https://github.com/jakestanger/tree-sitter-corn)

## Contributing

Contributions are very welcome, although please do open an issue first as not every potential feature will get merged.

At the moment Corn is in very early stages. If you'd like to help out, I'd absolutely love to see the following:

- More output formats
- Improvements and fixes to existing features
- More tests
- Better documentation

### Running Tests

You must set `CORN_TEST=bar` as this is required for the environment variable tests.

### WASM

WASM support is a feature called `wasm` which is disabled by default. 
Make sure to enable it when building:

```sh
wasm-pack build -- --features wasm
```
