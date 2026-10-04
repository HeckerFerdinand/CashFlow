// Monthly payslip (Lohnabrechnung), A4 portrait.
#import "common.typ": *

#let d = json("data.json")
#show: page-setup.with(title: d.title + " " + d.period)

#title-line(d.title, d.date)
#text(size: 14pt, d.period)

#v(5mm)
#attribute-table(
  ("Personal-Nr.", "Geburtsdatum", "SV-Nummer", "Krankenkasse"),
  (d.employee.personnel_number, d.employee.birth_date, d.employee.ssn, d.employee.health_insurer),
)
#v(3mm)
#attribute-table(
  ("PGRS", "BGRS", "Steuer-ID", "Eintritt"),
  (d.employee.person_group, d.employee.contribution_group, d.employee.tax_id, d.employee.employment_start),
)

// Sender line (employer) and recipient (employee)
#v(12mm)
#d.employer.name_line \
#d.employer.address_line

#v(6mm)
#text(size: 12pt, weight: "bold")[
  #d.employee.name \
  #d.employee.street_line \
  #d.employee.city_line
]

#v(10mm)
#let amount-table(..rows) = table(
  columns: (1fr, 1fr, 1fr, 1fr),
  inset: (x: 5pt, y: 6pt),
  stroke: (x, y) => (right: if x == 2 { 0.6pt + black } else { none }),
  align: (x, y) => if x == 3 { right } else { left },
  ..rows.pos().flatten(),
)

#section[Brutto-Bezüge]
#line(length: 100%, stroke: rule)
#amount-table(
  ([], [Bezeichnung], [], [Betrag]),
  ([], d.gross.base_label, [], d.gross.base),
  ([], d.gross.extra_label, [], d.gross.extra),
)
#line(length: 100%, stroke: rule)
#amount-table(
  ([], [], [], text(weight: "bold")[Gesamt-Brutto]),
  ([], [], [], d.gross.total),
)

#section[Sozialversicherung]
#line(length: 100%, stroke: rule)
#amount-table(
  ([], [KV-Brutto], [RV-Brutto], [SV-rechtliche Abzüge]),
  ([], d.social.kv_gross, d.social.rv_gross, d.social.deductions),
)
#line(length: 100%, stroke: rule)
#amount-table(
  ([], [], [], text(weight: "bold")[Netto-Verdienst]),
  ([], [], [], d.net),
)
#align(right, line(length: 25%, stroke: rule))
#amount-table(
  ([], [], [], text(weight: "bold")[Auszahlungsbetrag]),
  ([], [], [], text(weight: "bold", d.payout)),
)
#align(right, line(length: 25%, stroke: rule))
