# FC4R

## Introduction

A flexible file tagging system based on filenames with support for aliases and configurable relationships between tags.


## Recipes and examples

### Initialize the system config for the current folder

```
fcinit
```

### Basic usage to preview results into a folder of links

```
fcwalk | fcq label1 label2 | fclink
```

### Use cached list of files instead of traversing the file system

```
fcwalk --save
```

```
fcwalk --load | fcq label1 label2
```

> Note: If a cache is present but not used, a warn in stderr will appear
> notifying that.

### Search for unlabeled files

```
fcwalk | fcq system:unlabeled
```

### Search for labeled files

```
fcwalk | fcq system:labeled
```

### Search for files that contain a label "x" explicitly written in the name (not implied)

```
fcwalk | fcq explicit:x
```

### Search for files containing a label "x" implicitly.

```
fcwalk | fcq x not:explicit:x
```

## License

All code in this repo is MIT licensed unless said otherwise. See the license
file for details.