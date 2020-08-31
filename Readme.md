Pest Formatter
==============

Try Online: [PlayGround](https://sbeckeriv.github.io/pest_format/)


### Configs

#### pest_indent: usize

- 4(default)

```pest
rule = {
    token1
  | token2
}
rule2 = {
    token1 |
    token2
}
```

- 2

```pest
rule1 = {
  token1
| token2
}
rule2 = {
  token1 |
  token2
}
```

- 0

```pest
rule1 = {
  token1
| token2
}
rule2 = {
token1 |
token2
}
```

#### pest_choice_first: bool

- true(default)

```pest
rule = {
    token1
  | token2
}
```

- false

```pest
rule = {
    token1 |
    token2
}
```

#### pest_choice_hanging: bool

`pest_indent` & `pest_choice_first` will be disabled

- true

```pest
rule = !
       { token1
       | token2
       }
```

- false(default)

#### pest_braces_space: usize

- 0(default)

```pest
rule = {token}
```

- 1

```pest
rule = { token }
```

#### pest_parentheses_space: usize

- 0(default)

```pest
rule = {(token)}
```

- 1

```pest
rule = {( token )}
```


#### pest_choice_space: usize

- 0(default)

```pest
rule = {token1|token2}
```

- 1

```pest
rule = {token1 | token2}
```
