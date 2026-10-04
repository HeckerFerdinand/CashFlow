// Shared look of all CashFlow documents.
// The data of each document comes from the virtual file "data.json"; all
// numbers and dates are already formatted (German notation) by the Rust code.

#let header-fill = luma(222)
#let rule = 0.7pt + black

// Page setup, fonts and footer.
#let page-setup(title: "", flipped: false, body) = {
  set document(title: title, author: "CashFlow")
  set page(
    paper: "a4",
    flipped: flipped,
    margin: (top: 14mm, bottom: 24mm, x: 15mm),
    footer: context {
      set text(size: 8pt)
      grid(
        columns: (1fr, auto),
        align: (left + bottom, right + bottom),
        [Dieses Dokument wurde von CashFlow erstellt.
          #if counter(page).final().first() > 1 [ · Seite #counter(page).display("1 von 1", both: true)]],
        stack(spacing: 2pt, align(right, image("logo.png", height: 9mm)), text(size: 10pt, weight: "bold")[CashFlow]),
      )
    },
  )
  set text(font: "Carlito", size: 10pt, lang: "de", region: "DE")
  set par(leading: 0.5em)
  body
}

// Title line with the date on the right.
#let title-line(title, date) = grid(
  columns: (1fr, auto),
  align: (left + bottom, right + bottom),
  text(size: 14pt, weight: "bold", title), date,
)

// Shows "–" for empty values.
#let value(v) = if v == none or v == "" { [–] } else { v }

// Grey header row + value rows, separated by vertical rules (like the old payslip).
#let attribute-table(headers, ..rows) = {
  let count = headers.len()
  table(
    columns: (1fr,) * count,
    inset: (x: 5pt, y: 4pt),
    fill: (x, y) => if y == 0 { header-fill },
    stroke: (x, y) => (
      right: if x < count - 1 { 0.6pt + black } else { none },
      bottom: if y == rows.pos().len() { 0.6pt + black } else { none },
    ),
    ..headers.map(h => text(weight: "bold", h)),
    ..rows.pos().flatten().map(value),
  )
}

// Bold section label.
#let section(label) = block(above: 6mm, below: 2mm, text(weight: "bold", label))
