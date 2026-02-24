#let sidew = 3cm

#let sidebar = [
  #place(
    left + top,
    dx: -1cm,
    dy: 2.5cm,
    [
      #rect(
        width: sidew,
        height: 100% - 2.5cm,
        fill: luma(95%),
      )
    ]
  )

  #place(
    left + top,
    dx: -1cm,
    dy: 0cm,
    image("lith.png", width: sidew)
  )

  #place(
    left + bottom,
    dx: -1cm,
    dy: 1cm,
    image("ida.png", width: sidew)
  )
]

#let conf(
  course: none,
  title: none,
  authors: (),
  version: none,
  date: none,
  semester: none,
  body
) = {
  set page(paper: "a4", margin: (x: 2.5cm, y: 2.5cm))
  set text(font: "New Computer Modern", size: 11pt)
  set heading(numbering: "1.1")

  sidebar
  place(center, dx: sidew/2, dy: 4cm)[
    #block(text(weight: 400, fill: rgb("#00008b"), 28pt, course))
  ]
  place(center, dx: sidew/2, dy: 10cm)[
    #set par(spacing: 0.5em)
    #block(text(weight: 700, 25pt, title))
    #v(0.5em)
    #block("Författare")
    #for author in authors [
      #block(author.name + ", " + text(font: "DejaVu Sans Mono", link("mailto:" + author.email)))
      #v(0.25em)
    ]
  ]

  place(center, dx: sidew/2, dy: 100%-1cm, {
    set par(spacing: 0.5em)

    block(semester)
    block("Version " + version)
    v(0.5em)
    block(date)
  })

  pagebreak()
  counter(page).update(1)
  set page(
    header: {
        date
        h(1fr)
        title
        h(1fr)
        for author in authors [
          #author.name
        ]
        move(dy: -1em, line(length: 100%, stroke: 0.5pt))
    },
    footer: {
        "Version " + version
        h(1fr)
        context(counter(page).display())
    },
  )


  body
}
