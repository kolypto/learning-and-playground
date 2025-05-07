+++
title = 'My First Post'
date = '2025-05-07T15:14:55+03:00'
draft = false
#headless = true
keywords = [ "hashtag" ]
summary = "short text"

# Add it to menus
menus = ['main', 'footer']


# Custom page params
[params]
  author = 'John Smith'
+++

## Introduction

This is **bold** text, and this is *emphasized* text.

This is a paragraph with .class and #id attributes.
{.foo .bar #baz}

Visit the [Hugo](https://gohugo.io) website!

{{ .Site.Params }}

{{ .Page.Params }}

{{ .Page.Keywords }}

{{ $image := .Resources.Get "sunset.jpg" }}

# Images

{{ with .Resources.GetMatch "sunset.jpg" }}
  <img src="{{ .RelPermalink }}" width="{{ .Width }}" height="{{ .Height }}">
{{ end }}

