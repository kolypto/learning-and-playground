# Cobra

Install:

```console
$ go install github.com/spf13/cobra-cli@latest
$ go get -u github.com/spf13/cobra@latest
```

Import it:

```go
import "github.com/spf13/cobra"
```

Initialize the application:

```console
$ cobra-cli init
$ cobra-cli add serve
```

Configure Cobra in your `~/.cobra.yaml` file:

```yaml
author: Mark Vartanyan <kolypto@gmail.com>
#license: MIT
license: ~
useViper: true
```

Customize code:

* Use `RunE` to return errors from your command funcs
