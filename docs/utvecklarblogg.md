# Utvecklarblogg

## 2026-02-17

Påbörjade funktionssyntax

## 2026-02-18

Utvecklade språkidén och skrev om den till LaTeX

## 2026-02-19

Fixade katalogstrukturen

## 2026-02-25

Skrev version 2 av språkspecen och utkast av BNF-grammatiken

## 2026-02-26

Påbörjade programmering av implementationen i Rust. Började skriva på lexern och datastrukturer för felmedelanden, tokens med mer

## 2026-02-27

Fick hjälp med att utveckla grammatiken på handledningstillfälle 2 och började arbeta för att färdigställa den

## 2026-03-01

Arbetade på lexern till ett stadium då det kändes färdigt. Förbättrade hur felmeddelanden skrevs ut till terminalen

## 2026-03-03

Skapade structs och enums i Rust för att representera AST-strukturen som jag hade utvecklat under handledningstillfällena. Gjorde så att lexern kan hantera string escape sequences som \n

## 2026-03-05

Skapade en Rust-modul för parsern och började arbeta på parserkoden.

## 2026-03-06

Flyttade semicolon till att vara efter expression_statement för att likna andra språk som använder sig av semicolon. Därför behövs nu inte semicolon efter alla typer av statements.

## 2026-03-08

Löste parsing av binära uttryck genom att läsa igenom mycket av craftinginterpreters.com

## 2026-03-09

Lade till parsing för null, string, list, funktionsanrop, identifierare och floats

## 2026-03-10

Fixade parsing för variabeldeklarationer

## 2026-03-11

Implementerade parsing för if statements och dess grenar. Skapade Rust-modulen för typechecking.

## 2026-03-13

Fortsatt arbete på typechecking.

## 2026-03-17

Arbetade på att försöka rengöra den mycket långa koden för alla tester till att parsa de olika AST noderna

## 2026-03-24

Implementerade en WIP-version av typcheckingen av binära uttryck

## 2026-03-30

Försökte optimisera koden i parsern och organisera om hur AST trädet är representerat. Detta gjorde så att några tester började misslyckas. Jag behöver fixa detta någon gång inom kort

## 2026-03-31

Fixade de tidigare misstagen jag gjorde i parsern så alla test fungerar nu. Nu använder också testerna pretty_assertions-biblioteket för att göra tester som misslyckas lättare att tolka

## 2026-04-02

Kopplade in evaluator-modulen och lade till mer typkontroll i checker.

## 2026-04-03

Implementerade parsing för return.

## 2026-04-05

Påbörjade typkontroll av return i funktioner.

## 2026-04-06

Uppdaterade TODO-planen i README.

## 2026-04-07

Fortsatte TODO-städning i kod och planering.

## 2026-04-09

Tog bort null och gjorde stora uppdateringar i parser, checker och evaluator.

## 2026-04-12

Implementerade Type::is_matching för kompatibla typer.

## 2026-04-13

Gjorde typmatchning rekursiv, förbjöd == mellan funktioner och lade till builtin-funktionen puts.

## 2026-04-15

Utökade evaluatorn med stöd för fler primära uttryck.
