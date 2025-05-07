# Hugo

# Install

Install:

```console
# apt install hugo
# snap install hugo
$ go install github.com/gohugoio/hugo@latest
$ CGO_ENABLED=1 go install -tags extended github.com/gohugoio/hugo@latest
```

# Commands

New project:

```console
$ hugo new site quickstart
```

Theme:
* Create new: "hugo new theme <THEMENAME>"
* Install: https://themes.gohugo.io/

```console
$ git submodule add https://github.com/theNewDynamic/gohugo-theme-ananke.git themes/ananke
$ git submodule add https://github.com/alex-shpak/hugo-book themes/hugo-book
$ echo "theme = 'ananke'" >> hugo.toml
```

Start Hugo server:

```console
$ hugo server --navigateToChanged
$ hugo server --buildDrafts
$ hugo server --minify --theme hugo-book
```

Add post:

```console
$ hugo new content content/posts/my-first-post.md
$ hugo new content content/posts/2025/04/my-first-post/index.md

```

Publish: Hugo will create static files in `/public/`:

```console
$ hugo
```


# Front Matter

Posts are in `content/posts`:

By default, Hugo will not publish content when:

* The draft value is true
* The date is in the future
* The publishDate is in the future
* The expiryDate is in the past

# Directory Structure

Config:

* `/hugo.toml`
* `/config/_default/hugo.toml` (when using multiple sites)
* `/archetypes`: templates for new content

Data:

* `/content`: Markdown posts and their resources
* `/data`: JSON, TOML, YAML, or XML

Resources:

* `/assets`: global resources
* `/layouts`: templates to transform content, data, and resources
* `/themes`: themes
* `/static`: files to copy to `/public` on build

Other:

* `/i18n`: multilingual tables

Hugo uses union filesystem: your project, laid over theme. You can override this way.

You can also mount shared content onto your project (i.e. from another "project").


# Config

All settings: <https://gohugo.io/configuration/all/>

You can split the configuration into multiple files:

* `/config/_default/hugo.toml`: `[params] foo = bar`
* `/config/_default/params.toml`: `foo = bar`  (can omit `[params]`)

Hugo will recursively parse all files.
It will also parse theme configs (`/themes/theme/hugo.toml`) — but override it with project values.

To override some of them based on environment in `/config/staging/`:

```console
$ hugo --environment staging
$ HUGO_ENVIRONMENT=staging hugo
```

You can use "merge" to control merge:

```toml
[languages]
  _merge = 'none'
  [languages.en]
    _merge = 'none'
    [languages.en.menus]
      _merge = 'shallow'
    [languages.en.params]
      _merge = 'deep'
```


Merge configs, with left-to-right precedence:

```console
$ hugo --config a.toml,b.yaml,c.json
```

Review config:

```console
$ hugo config
```


# Content

## Formats

Use extensions: `*.md`, `*.adoc`, `*.html`, ...

