# spreadsheet-to-invoiceshelf

A simple CLI program that helps creating invoices on invoiceshelf out of a spreadsheet file

TO DO:

- [x] Parse ODS file
- [x] Configuration
  - [x] Define default config file place and template
  - [x] Parse config file
- [x] Invoice templates
  - [x] Define template format
  - [x] Parse template
  - [x] Map ODS extracted data to template
- [x] CLI
  - [x] Find a suitable lib for displaying menus
  - [x] Establish UX path
  - [x] Code UX
- [ ] InvoiceShelf
  - [x] Login
  - [ ] Securely store authentication token
  - [ ] Renew Auth token ?
  - [ ] Send Invoice creation Query

## Configuration

STI takes parameters from a configuration file to function

```
hostname=<Hostname of your invoiceshelf instance>
```

## Template

A template file takes an amount of inputs and an amout of outputs that helps it generate the items for the invoice.

```toml

invoice_name = "Some base invoice name"
template_name = "PDF template to use"

[items-inputs]
columns = [
    "Tâche"
    "Durée"
    "Coût"
]

[items-outputs]
name = "${Tâche} : ${Durée}",
quantity =  1,
price = "${Coût}",
description = "${Tâche}",
sub_total = "${Coût}",
total = "${Coût}",
unit_name = "Tâche",
```
