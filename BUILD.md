\#\# Building

We use [Asciidoctor](https://asciidoctor.org) for slides and
assignments.

To compile `.adoc` to HTML, you need to have Ruby and
[bundler](https://rubygems.org/gems/bundler). Using
[`rbenv`](https://github.com/rbenv/rbenv) for project-local ruby package
installation is recommended.

We use [Mermaid.js](https://mermaid-js.github.io/) for diagrams in
slides. It requres Node installed. You can use
[`fnm`](https://github.com/Schniz/fnm) to manage Node versions.

**Prerequisites.**

$ eval "$(rbenv init -)" \# when using rbenv $ gem install bundler $
bundle --path=.bundle/gems --binstubs=.bundle/.bin $ npm
ci&lt;/programlisting&gt;

**Building.**

$ eval "$(rbenv init -)" \# when using rbenv $ ./rake $ firefox
./target/index.html&lt;/programlisting&gt;

\#\# Links

-   [Rendered
    Presentations](https://ferrous-systems.github.io/teaching-material/index.html)

-   [AsciiDoc Syntax Quick
    Reference](https://asciidoctor.org/docs/asciidoc-syntax-quick-reference/)

-   [Asciidoctor User Manual](https://asciidoctor.org/docs/user-manual/)

\#\# Translations

The materials are in English. Translation to over languages is allowed
and encouraged, if you would like to maintain a translation, send us a
pull request with a link.

Partial translations to German, French and Spanish are available in the
[old
repository](https://github.com/ferrous-systems/rust-three-days-course/tree/master/presentation/chapters).
