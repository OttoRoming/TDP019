#let conf(
  title: none,
  authors: (),
  body
) = {
  set page(paper: "a4", margin: (x: 3cm, y: 2cm))
  set text(font: "Computer Modern", size: 11pt)
  set heading(numbering: "1.1")

  align(center)[
    #block(text(weight: 700, 1.75em, title))
    #v(1em)
    #authors.join(", ", last: " & ")
  ]

  body
}
