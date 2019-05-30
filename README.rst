===========
Leabharlann
===========

Leabharlann_ is an a CLI tool to keep track of books I'm reading or want to read.
The purpose of this is to have a centralised point of doing this, rather than my usual
approach of scattering my reading lists across notebooks, post-its, and the occasional
draft email.

Leabharlann is the Irish word for library.

Install
=======

Prerequisites
-------------

- `Rust`_
- `Cargo`_

Install
-------

To install::

  $ git clone https://github.com/emilywoods/leabharlann.git
  $ cd leabharlann


Usage
=====

The following reading options are available to document:
- Current
- Finished
- Future

Each time a series of questions will be asked, and answers will be stored in a
`csv`_ file: ``reading.csv``.

Current reading
---------------

To document what you are reading now or have previously read::

  > $ cargo run now

A series of questions will be asked about current reading.

Or `as Gaeilge`_::

  > $ leabharlann anois

Finished
--------

To change a book from "current" to "finished"::

  > $ cargo run finish

Or::

  > $ cargo run criochnaigh

Future Reading
--------------

To input a book to read later::

  > $ cargo run future

Or::

  > $ cargo run sa-todhchai

Help
----

To see a list of available options::


  $ cargo run leabharlann --help


.. _Leabharlann: https://en.wiktionary.org/wiki/leabharlann
.. _as Gaeilge: https://en.wiktionary.org/wiki/as_Gaeilge
.. _Rust: https://doc.rust-lang.org/cargo/getting-started/installation.html
.. _Cargo: https://crates.io/
.. _csv: https://en.wikipedia.org/wiki/Comma-separated_values

