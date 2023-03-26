let Foo = {
  foo | optional,
  bar | optional
} in
{
  foo = 5,
  bar = null,
} | Foo
