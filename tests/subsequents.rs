test! {
  name: single_and_subsequent_success,
  justfile: "
    foo: && bar
      echo foo

    bar:
      echo bar
  ",
  stdout: "
    foo
    bar
  ",
  stderr: "
    echo foo
    echo bar
  ",
}

test! {
  name: single_and_subsequent_failure,
  justfile: "
    foo: && bar
      false

    bar:
      echo bar
  ",
  stdout: "",
  stderr: "
    false
  ",
}

test! {
  name: recipe_may_have_self_as_subsequent,
  justfile: r#"
    set positional-arguments

    @foo arg='x': && (foo arg + "x")
      echo {{arg}}
      if [{{arg}} == xxxxx]; then false; fi
  "#,
  stdout: "
    x
    xx
    xxx
    xxxx
    xxxxx
  ",
  stderr: "",
}
