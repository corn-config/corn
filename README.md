# 🌽 Corn

A simple and pain-free configuration language.

Corn has been designed using inspiration from JSON and Nix to produce a language
that's easy and intuitive to write, good for config files, and has a feature-set
small enough you can learn it in minutes.

## Usage

### As a binary

Corn can be installed as an executable binary to convert files from the `.corn` format
into either `yaml` or `json`.

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

TODO: Link to docs

```rs
// TODO: Write example
```

## Writing Corn

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
- Variable

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

The next key starts as soon as the previous value ends,
meaning this is also valid without whitespace:

TODO: Test the limits of this!!

```nix
{foo="bar"hello="world"}
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

TODO: rename to 'inputs' in code

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

## Contributing

### Testing

You must have set `CORN_TEST=foobar` as this is required for the environment variable tests.