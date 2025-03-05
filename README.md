# 🌽 Corn

A simple and pain-free configuration language.

[Corn](https://cornlang.dev) has been designed using inspiration from JSON and
Nix to produce a language that's easy and intuitive to write, good for config
files, and has a feature-set small enough you can learn it in minutes. It was
born out of the following frustrations:

- JSON is not a config language, despite how often people use it as one.
- TOML is good for flat structures but gets ugly quickly with deeper objects.
- YAML is far too complex and its whitespace rules make it error-prone.
- Nix is a full-sized language and not easy to integrate.

---

<div align="center">

## Documentation

[📖User guide](https://cornlang.dev/user-guide) |
[📝 Full specification](https://cornlang.dev/spec)

</div>

---

```corn
let {
    $entry = "dist/index.js"
    $author = { name = "John Smith" email = "mail@example.com" }
} in {
    name = "example-package"
    version = "1.0.0"
    main = $entry
    bin.filebrowser = $entry
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

---

Corn is available as libraries for Rust, Go, Lua, and JavaScript (via WASM). A
CLI and web API are also available.

Editor plugins are available for JetBrains IDEs, VS Code and Neovim.
