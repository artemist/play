#{
  set page(width: 3.5in, height: 1.125in, margin: 0em)
  set par(leading: 0.2em)

  place(
    top + left,
    dx: 12mm,
    dy: 0in,
    block(width: 3.5in - 12mm, height: 1in, align(center + horizon, text(font: "P22UndergroundCYBook", size: 20pt)[
      Artemis Tosini \
      me\@artem.ist \
      hostname
    ]))
  )
}
