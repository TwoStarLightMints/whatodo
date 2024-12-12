# Whatodo

## Install
```
$ git clone https://github.com/TwoStarLightMints/whatodo.git
$ cargo install --path ./whatodo
```

## todo.todos file

### Format

One todo with no nested todos:

Complete | Contents |
0|Something|

One todo with nested todos:

Complete | Contents | [ Complete | Contents | *Separator* Complete | Contents | ]

0|Something|[1|Another|%0|One more|]

Bars are used to separate fields
If there is no list of sub todos for any given todo, the ending bar must still be supplied
Sub todos are separated by the percent sign
White space is allowed within the **contents** field, no where else
