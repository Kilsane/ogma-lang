**🟦 Ogma --- Lisez‑moi**

Ogma est un langage de programmation narratif, strict et déterministe, conçu pour être **aussi clair pour les humains que pour les machines**.\
Il repose sur une idée simple : un programme doit pouvoir être lu, compris et audité sans effort, comme un document technique ou une recette.

Ogma n'est pas un langage ésotérique.\
Ogma n'est pas un langage "magique".\
Ogma est un langage **explicite**, **lisible**, **prévisible**.

**🟩 Human Ready**

Ogma est pensé pour être **immédiatement compréhensible par un humain**, même sans expertise technique.

- Syntaxe narrative et lisible

- Pas de symboles obscurs

- Pas de comportements implicites

- Pas de conversions automatiques

- Pas de surcharge qui brouille l'intention

- Chaque opération est nommée, claire, explicite

Un développeur, un analyste, un auditeur ou un régulateur peut lire un fichier Ogma et comprendre exactement ce qu'il fait, sans interprétation.

Ogma est un langage qui **se lit comme un texte**.

**🟧 AI Ready**

Ogma est également conçu pour être **parfaitement manipulable par une IA**, grâce à sa rigueur structurelle.

- Syntaxe sans ambiguïté

- Règles strictes et déterministes

- Pas de magie, pas d'implicite

- Types conceptuels explicites

- Arbre syntaxique simple et stable

- Pas de surcharge, pas de polymorphisme caché

Une IA peut écrire, analyser, transformer ou vérifier du code Ogma **sans heuristique**, sans deviner, sans interpréter.

Ogma est un langage que les IA peuvent **comprendre sans approximation**.

**🟨 Pourquoi Ogma ?**

Ogma apporte une combinaison rare :

+--------------------+-----------------------------------------+
| **Besoin**         | **Réponse d'Ogma**                      |
+====================+=========================================+
| Lisibilité humaine | Syntaxe narrative, explicite            |
+--------------------+-----------------------------------------+
| Auditabilité       | Pas de magie, pas d'implicite           |
+--------------------+-----------------------------------------+
| Précision          | Types conceptuels comme ognum           |
+--------------------+-----------------------------------------+
| Sécurité           | Typage strict, déterministe             |
+--------------------+-----------------------------------------+
| Simplicité         | Pas de surcharge, pas de syntaxe cachée |
+--------------------+-----------------------------------------+
| Automatisation     | Structure parfaite pour les IA          |
+--------------------+-----------------------------------------+

Ogma vise les domaines où **la clarté est essentielle** :\
finance, régulation, industrie, simulation, automatisation, documentation vivante.

**🟦 Exemple Ogma**

Exemple 1

// Définition d\'un type ognum pour les prix

type Price: ognum(20,2)

// Calcul du prix TTC

calculate_ttc: fn(ht: Price, tax_rate: Price) -\> Price {

one: Price = Price(\"1.00\")

multiplier: Price = math.add(one, tax_rate)

math.mul(ht, multiplier)

}

main: fn() {

price_ht: Price = Price(\"100.00\")

tva: Price = Price(\"0.20\")

price_ttc: Price = calculate_ttc(price_ht, tva)

print(string.concat(\"Prix TTC : \", string(price_ttc)))

}

Exemple 2

// Conversion Celsius vers Fahrenheit (F = C \* 1.8 + 32)

celsius_to_fahrenheit: fn(celsius: decimal) -\> decimal {

ratio: decimal = 1.8

part1: decimal = math.mul(celsius, ratio)

math.add(part1, 32.0) // La dernière ligne est la valeur de retour

}

main: fn() {

temp_c: decimal = 25.0

temp_f: decimal = celsius_to_fahrenheit(temp_c)

print(string.concat(\"25°C = \", string(temp_f), \"°F\"))

}

Exemple 3

// Vérifie si un mot de passe respecte les critères de sécurité

validate_password: fn(password: string) -\> bool {

// Règle 1 : Longueur minimale

is_long_enough: bool = string.length(password) \>= 8

// Règle 2 : Recherche de chiffres (Version narrative)

// On utilise mut car \'i\' et \'found\' vont évoluer

mut has_digit: bool = false

mut i: int = 0

limit: int = string.length(password)

while (i \< limit) and (not has_digit) {

char: string = string.char_at(password, i)

if string.is_digit(char) {

has_digit = true

}

i = i + 1

}

is_long_enough and has_digit

}

main: fn() {

pwd: string = \"Abcdef12\"

if validate_password(pwd) {

print(\"Mot de passe sécurisé.\")

} else {

print(\"Sécurité insuffisante.\")

}

}

Lisible par un humain.\
Compréhensible par une IA.\
Exécutable sans ambiguïté.

**🟫 Ogma en une phrase**

**Ogma est un langage narratif, strict et déterministe, conçu pour être Human Ready et AI Ready.**
