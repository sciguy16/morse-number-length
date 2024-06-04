# Morse number length

Follow-up to a question posed in Ayliean's EMF Camp talk
[*The Best Word In Morse Code*](https://www.emfcamp.org/schedule/2024/294-the-best-word-in-morse-code),
this program searches for numbers whose Morse code representation contains
more symbols (where each dot or dash counts as a symbol here) than the
number's English written form (counting letters).

## How to run
* Install the Rust compiler via [Rustup](https://rustup.rs/)
* Clone this repo
* Run with `cargo run --release`

## Results
The output provides a selection of additional numbers which are more efficient
to send in words than in digit form, for instance these numbers below 200
million:

```
number: 10 symbols: 10, string: 4, ten
number: 1000000 symbols: 35, string: 25, one million
number: 2000000 symbols: 35, string: 26, two million
number: 3000000 symbols: 35, string: 29, three million
number: 4000000 symbols: 35, string: 32, four million
number: 5000000 symbols: 35, string: 30, five million
number: 6000000 symbols: 35, string: 28, six million
number: 7000000 symbols: 35, string: 30, seven million
number: 8000000 symbols: 35, string: 31, eight million
number: 9000000 symbols: 35, string: 26, nine million
number: 10000000 symbols: 40, string: 23, ten million
number: 10000001 symbols: 40, string: 36, ten million and one
number: 10000002 symbols: 40, string: 37, ten million and two
number: 10000006 symbols: 40, string: 39, ten million and six
number: 10000009 symbols: 40, string: 37, ten million and nine
number: 10000010 symbols: 40, string: 34, ten million and ten
number: 11000000 symbols: 40, string: 32, eleven million
number: 12000000 symbols: 40, string: 33, twelve million
number: 13000000 symbols: 40, string: 34, thirteen million
number: 14000000 symbols: 40, string: 37, fourteen million
number: 15000000 symbols: 40, string: 34, fifteen million
number: 16000000 symbols: 40, string: 33, sixteen million
number: 17000000 symbols: 40, string: 35, seventeen million
number: 18000000 symbols: 40, string: 35, eighteen million
number: 19000000 symbols: 40, string: 31, nineteen million
number: 20000000 symbols: 40, string: 31, twenty million
number: 21000000 symbols: 40, string: 37, twenty-one million
number: 22000000 symbols: 40, string: 38, twenty-two million
number: 29000000 symbols: 40, string: 38, twenty-nine million
number: 30000000 symbols: 40, string: 34, thirty million
number: 40000000 symbols: 40, string: 34, forty million
number: 50000000 symbols: 40, string: 34, fifty million
number: 60000000 symbols: 40, string: 33, sixty million
number: 61000000 symbols: 40, string: 39, sixty-one million
number: 70000000 symbols: 40, string: 35, seventy million
number: 80000000 symbols: 40, string: 35, eighty million
number: 90000000 symbols: 40, string: 31, ninety million
number: 91000000 symbols: 40, string: 37, ninety-one million
number: 92000000 symbols: 40, string: 38, ninety-two million
number: 99000000 symbols: 40, string: 38, ninety-nine million
Found 40 winners in 73 seconds
```
