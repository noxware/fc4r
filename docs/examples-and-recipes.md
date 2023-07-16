# Recipes and examples

## Files tagged samples

```
l1 l2 long_label_name fn filename.txt
implies_l1_and_l2 l3 fn filename.ext
```

> ` fn ` separates the tag list from the rest of the filename.
> A tagged file must include it as a delimiter.

## Command samples

### Initialize the system config for the current folder

```
fcinit
```

### View the result of a query by generating file system links.

```
fcwalk | fcq label1 label2 | fclink
```

### Extract unlabeled files moving them into a different folder

```
fcwalk | fcq system:unlabeled | fcmv unlabeled
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

> Warning: This is not yet implemented.

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


### Get a summary of all labels known and unknown in the current folder

```
fcwalk | fclabels
```

### Get only the known labels

```
echo "" | fclabels
```

### Get only the unknown labels

Currently, not possible.

## Config sample

### On `labels.toml`

```toml
[art]
description = "Anything artistic, landscapes, music, etc."

[book]

  [comic]
  implies = ["book", "art"]

    [manga]
    implies = ["comic"]

    [webtoon]
    implies = ["comic"]

[creature]
description = "Creatures, animals, etc."

  [dragon]
  implies = ["creature"]

[cute]
description = "Kawaii stuff for the heart."
aliases = ["kawaii", "sweet"]

  [pink]
  implies = ["cute"]

  [cat]
  implies = ["cute", "creature"]

  [love]
  implies = ["cute"]
  description = "Love related, romantic, stuff."
  aliases = ["romantic"]

[learning]
description = "Learning exclusive material."

  [drawing_learning]
  implies = ["learning"]
  aliases = ["drawing_tutorial"]

[meme]
description = "Memes and fun stuff."
aliases = ["fun", "joke"]

[tech]
description = "Technology related stuff."

  [programming]
  implies = ["tech"]

    [programming_tutorial]
    implies = ["programming", "learning"]

  [electronics]
  implies = ["tech"]

[news]

[screenshot]
```

### On `settings.toml`

```toml
link_dir="/home/user/volumes/fake/fileclass/links"
```