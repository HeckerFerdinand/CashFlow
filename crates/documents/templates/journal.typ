// Yearly journal (Lohnjournal / Zeitjournal), A4 landscape: one column per month.
#import "common.typ": *

#let d = json("data.json")
#show: page-setup.with(title: d.title, flipped: true)

#title-line(d.title, d.date)

#section[Arbeitnehmer]
#attribute-table(
  ("Name", "Anschrift", "SV-Nummer", "Personal-Nummer"),
  (d.employee.name, d.employee.street_line, d.employee.ssn, d.employee.personnel_number),
  ([], d.employee.city_line, [], []),
)

#section[Arbeitgeber]
#attribute-table(
  ("Name", "Anschrift", "Betriebsnummer", "Steuernummer"),
  (d.employer.name, d.employer.street_line, d.employer.company_number, d.employer.tax_number),
  (if d.employer.representative == "" { [] } else { d.employer.representative }, d.employer.city_line, [], []),
)

#v(6mm)
#{
  set text(size: 9pt)
  let columns = (2.2fr,) + (1fr,) * 12 + (1.3fr,)
  let cells = ()
  // Header row
  cells.push(text(weight: "bold", d.unit))
  for m in d.months { cells.push(align(right, text(weight: "bold", m))) }
  cells.push(align(right, text(weight: "bold")[Gesamt]))
  cells.push(table.hline(stroke: rule))
  // Row groups, separated by rules
  for (index, group) in d.groups.enumerate() {
    if index > 0 { cells.push(table.hline(stroke: rule)) }
    for row in group {
      let style(body) = if row.bold { text(weight: "bold", body) } else { body }
      cells.push(style(row.label))
      for v in row.values { cells.push(align(right, v)) }
      cells.push(align(right, style(row.total)))
    }
  }
  table(columns: columns, stroke: none, inset: (x: 3pt, y: 5pt), ..cells)
}

#if d.note != "" {
  v(3mm)
  text(size: 8.5pt, d.note)
}
