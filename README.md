# Rust Based Mortgage Calculator


## Goals
* Replicate the mortgage formulas used by the online calculators
* Create a command line interface for adding in mortgage details
* Integrate a relational database using sqlite and store the mortgage
  information between uses of the application
* Add a terminal UI for displaying and entering information


## Formulae

### Effective interest
**EAR** = Effective Interest Rate\
**i**   = interest rate\
**n**   = number of periods
$EAR = (1+\frac{i}{n})^n-1$
### Periodic Rate
**pr** = Periodic Rate\
**i**  = interest rate\
**m**  = compounding periods per year\
**n**  = number of payments in a year
$pr = (1+i/m)^{\frac{m}{n}}-1$

### Monthly Payment
P = Principle\
r = effective periodic interest rate\
n = total number of payments
$M = P * \frac{r(1+r)^n}{(1+r)^n-1}$

### Amortization

**Interest Payment:**\
$InterestPayment = Balance * PeriodicRate$

**Principle Paid:**\
$Principle Paid = Monthly Payment - Interest Payment$

**Remaining Balance:**\
$Balance = Principle - Principle Paid$
