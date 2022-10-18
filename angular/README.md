Initialize
==========

```console
$ sudo npm install -g @angular/cli
$ ng new my-app --strict
? Would you like to add Angular routing? Yes
? Which stylesheet format would you like to use? SCSS
$ cd my-app
$ ng serve --open
```

Generate
========


Component
---------

By default, the Angular CLI command ng generate component generates components with a template file. You can override that by adding the "-t" (short for inlineTemplate=true) option:

```console
$ ng generate component --skip-tests -st hero/name
```

Directive
---------

```console
$ ng generate directive highlight
```

Service
-------

```console
$ ng generate service User
```


Module
------

```console
$ ng generate module CustomerDashboard
$ ng generate module hero-demo --route heroes --module app.module

$ ng generate component customer-dashboard/CustomerDashboard
```

Guard
-----

```console
$ ng generate guard app-routing/security
```


App Shell
---------

```console
$ ng generate app-shell
```

Run:

```console
$ ng run my-app:app-shell
    or 
$ ng run my-app:app-shell:production
```



Serve
=====

```console
$ ng serve
** Angular Live Development Server is listening on localhost:4200, open your browser on http://localhost:4200/ *
```

Localize
========

Install:

```console
$ ng add @angular/localize
```

Extract:

```console
$ ng xi18n
```

Copy and translate: `src/locale/messages.ru.xlf`

Build: 

```console
$ ng build --prod --localize
```

Service Workers
===============

Install: 

```console
$ ng add @angular/pwa --project *project-name*
```

Serve:

```console
$ ng build --prod
$ npm install http-server
$ http-server -p 8080 -c-1 dist/<project-name>
```
