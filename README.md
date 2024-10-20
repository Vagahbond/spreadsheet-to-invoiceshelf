# spreadsheet-to-invoiceshelf

A simple CLI program that helps creating invoices on invoiceshelf out of a spreadsheet file

## Configuration

STI takes parameters from a configuration file to function

```
HOSTNAME=<Hostname of your invoiceshelf instance>
```

## Template

A template file takes an amount of inputs and an amout of outputs that helps it generate the items for the invoice.

```toml

template_name = "to use"

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
