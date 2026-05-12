# Självbedömning - Otto Roming
Projektet bedöms nå betyg: 5

## Köra projektet:
Projektet kompileras först med kommandot make, sedan kör man 
kod genom att ge filnamnet som argument. Ex: ./a.out loop_example.xyz

Motivation för betyg 3:
Språket har stöd för loopar och fungerande aritmetrik.

### Kodexempel:
```oeno
each i : range(5) {
  puts("inside each: " + i_to_s(i));
}

var i = 0
while i < 5 {
  puts("inisde while: " + i_to_s(i));

  i++;
}

if 6/2*(1+2) == 9 {
  puts("it works!");
} else {
  throw "something is wrong"
}

fun foo() {
  puts("inside foo");
}
foo();

```

## Motivation för betyg 4:
En lista kan innehålla flera listor, de är alltså rekursiva.
Dessa listor går också att appenda till med `<-` operatorn

### Kodexempel:
```oeno
var matrix: List<List<String>> = [
  ["a", "b", "c"],
  ["1", "2", "3"]
]

matrix <- ["!", "@", "#"];

each row : matrix {
  each col : row {
    puts(col);
  }
  puts("reached end of row");
}
```


## Motivation för betyg 5:
Parsern är egen och skriven i Rust
