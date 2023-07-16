# FC4R
![example workflow](https://github.com/noxware/fc4r/actions/workflows/checks.yaml/badge.svg)

## Introduction

A flexible file tagging system based on filenames with support for aliases and
configurable relationships between tags JIT evaluated.

## Project status

The project reached an MVP state and is usable and working. But with my little
time I have to dedicate to this project, I prefer to focus on adding features
instead of polishing the existing ones. So at the moment, if used without
care, you may encounter some rough edges.

For the moment the cargo version means nothing.

## Features

- Tag files directly in the filename.
- Tagging syntax tries to use only alphanumeric characters making it easy to
  type even when writting the filename from a mobile device virtual keyboard.
- The same tag can be written in different ways thanks to aliases.
- A tag can be defined and configured to imply another tag, allowing
  helpful relationships between them.
- The system is composed of individual executables/commands that can be
  composed between them and also with your own software if necessary.
- Contains a good amount of unit tests.

## Limitations

- Can only query files inside a folder where a `fileclass` config directory has
  been initialized by the `fcinit` command.
- Involves walking the whole directory at least once with the `fcwalk` command
  before doing a query. This works fine for personal usage but in the future
  more efficient escape hatches will be added for more demanding use cases like
  integration with databases instead of filesystems.
- At the moment, tagged directories are not well handled by the system.
- To view the results of a query you will need to generate filesystem links
  with `fclink` or move the files using `fcmv`.
- Although the project is tested, it doesn't handle unexpected errors or weird
  cases very well. This is because it is still in development and I have limited
  time to dedicate to this project so I prefer to focus on adding features for
  the moment.

## Guides and tutorials

I will write some in depth guide when more features are added, but for the
moment you will need to figure it out using the examples in the `docs`
directory.

## License

All code in this repo is MIT licensed unless said otherwise. See the license
file for details.
