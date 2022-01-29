# 🌽 Corn

A simple and pain-free configuration language.

Corn has been designed using inspiration from JSON and Nix to produce a language
that's easy and intuitive to write, good for config files, and has a feature-set
small enough you can learn it in minutes. It was born out of the following frustrations:

- JSON is not a config language, despite how often people use it as one
- TOML is good for flat structures but gets ugly quickly with deeper objects
- YAML is far too complex and its whitespace rules make it error-prone

## What Corn Is Not

Corn is not a full programming language and does not try to be. 
There are no variables, there is no interpolation and there are no statement blocks.

Likewise, Corn is not a data exchange format. 
Unlike JSON or YAML or TOML, you cannot serialize code back into Corn. 

## Usage

### As a binary

Corn can be installed as an executable binary to convert files from the `.corn` format
into any supported output format. Currently these are:

- JSON
- YAML
- TOML

Windows, Linux and macOS are currently supported.

Install it using cargo:

```shell
cargo install cornfig
```

Then simply:

```shell
cornfig file.corn
cornfig file.corn -t yaml
```

### As a library

Corn can be used as a Rust library to deserialize config files directly
without needing to convert to other file formats.

Rust docs can be found [here](https://docs.rs/cornfig/latest/cornfig/).

```rust
use cornfig::parse;

fn main() {
    let corn = "{foo = 42}";

    let config = parse(corn).unwrap();
    let json = serde_json::to_string_pretty(&config.value).unwrap();

    assert_eq!(json, "{\"foo\": 42}");
}
```

## Writing Corn

> This section gives all the outputs in JSON format. Remember you can output in any supported format!

All Corn files must contain a top-level object that contains keys/values.
Keys must be alphanumeric and do not need quotes around them.
Values must be one of the following:

- String
- Integer
- Float
- Boolean
- Object
- Array
- Null
- Input

A very basic example therefore would be:

```nix
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

```nix
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

```nix
foo = "bar"
```

#### Integer

Integers are signed and 64 bit, meaning you can use any value
between `-9223372036854775808` and `9223372036854775807`.

```nix
answer = 42
```

#### Float

Double precision (64-bit) floats are used.

```nix
pi = 3.14159
```

#### Boolean

As you'd expect.

```nix
not_false = true
```

#### Object

Objects use braces to mark the start and end. 
They contain key/value pairs.

There is no restriction on nesting objects within objects or arrays.

```nix
{
  foo = "bar"
  hello = "world"
}
```

#### Array

Arrays use square brackets to mark the start and end. 
Values are space-separated. 
Like objects, there is no restriction on nesting arrays or objects inside.

```nix
    array = [ 1 2 3 ]
```

You can also include whitespace as you please and mix element types:

```nix
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

```nix
foo = null
```

#### Inputs

Sometimes it may be useful to store some values at the top
of the config file, and use or re-use them later on,
or even use them to compose more complex values. Corn supports config inputs, akin to variables but they don't change.

All input names start with a dollar sign `$` followed by an alphabetic ASCII character or an underscore `_`.
This can then be followed by any amount of alphanumeric ASCII characters or underscores.

Inputs can be used to store any value type, or inside strings.

To declare inputs, you must include a `let { } in` block
at the start of your config.

```nix
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

```nix
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

Environment variable inputs will fall back to regular inputs of the same name, allowing you to create defaults.

Inputs are intentionally quite limited as to what they can do -
if you need more power you should use a full language. 
That said, they hopefully provide a way of quickly viewing/changing values
without needing to trawl through the whole file.

### Comments

At any point you can start a comment using `//`. A comment is terminated by a newline `\n` character.
Comments are entirely ignored and not included in the output.

```nix
{
    // this is a single-line comment
    foo = bar // this is a mixed-line comment
}
```

### Nesting Keys

Throughout the page, we've created objects within objects
using a syntax like this:

```nix
{
    foo = {
        bar = "baz"
    }
}
```

While this is okay for short cases, it can get tiresome very fast
and the braces add a lot of noise when reading.

To solve this, keys can be chained to create deep objects instantly:

```nix
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

```nix
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

This means the below is perfectly valid (although for general consistency and readability this is strongly not recommended):

```nix
{
    one={foo="bar"bar="foo"}
    two={foo=1bar=2}
    three={foo=1.0bar=2.0}
    four={foo=truebar=false}
    five={foo=nullbar=null}
    six={foo={}bar={}}
    seven={foo=[]bar=[]}

    eight=["foo""bar"]
    nine=[truefalse]
    ten=[nullnull]
    eleven=[[][]]
    twelve=[{}{}]
}
```

And in fact, we could even go as far as to reduce that to a single line:

```nix
{one={foo="bar"bar="foo"}two={foo=1bar=2}three={foo=1.0bar=2.0}four={foo=truebar=false}five={foo=nullbar=null}six={foo={}bar={}}seven={foo=[]bar=[]}eight=["foo""bar"]nine=[truefalse]ten=[nullnull]eleven=[[][]]twelve=[{}{}]}
```

## Syntax Highlighting

- VSCode: [extension](https://marketplace.visualstudio.com/items?itemName=JakeStanger.corn&ssr=false) | [repo](https://github.com/JakeStanger/corn-vscode)


## Contributing

Contributions are very welcome, although please do open an issue first as not every potential feature will get merged.

At the moment Corn is in very early stages. If you'd like to help out, I'd absolutely love to see the following:

- More output formats
- Improvements and fixes to existing features
- More tests
- Better documentation

### Running Tests

You must set `CORN_TEST=foobar` as this is required for the environment variable tests.