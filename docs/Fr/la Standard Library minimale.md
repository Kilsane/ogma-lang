# 📘 OGMA --- Standard Library Minimale (std‑min)

**Version 0.3.11** --- *Document de référence des fonctions intégrées*

**1. Introduction**

La Standard Library minimale d'Ogma fournit l\'ensemble essentiel de briques permettant de construire des programmes, de valider le runtime et de garantir une base stable avant la version 0.6.

**Principes de la std‑min :**

- **Zéro Magie** : Pas de comportement implicite.

- **Narrativité** : Les noms de fonctions expliquent l\'action.

- **Déterminisme** : Même appel = même résultat.

- **Explicite** : Pas de surcharge (overloading). Une fonction par type.

**2. Structure de la std‑min**

La bibliothèque est organisée en quatre modules natifs, disponibles sans import :

1.  **core** : Primitives fondamentales.

2.  **string** : Manipulation de texte.

3.  **math** : Calculs numériques stricts.

4.  **error** : Gestion explicite des pannes.

**3. Module core**

Le module core contient les fonctions universelles du langage.

**3.1 print(value)**

Affiche une valeur sur la sortie standard.

- Convertit la valeur en chaîne de caractères pour l\'affichage.

- Exemple : print(\"Hello Ogma\")

**3.2 type_of(value) -\> string**

Retourne le type d'une valeur sous forme de chaîne.

- Exemple : t: string = type_of(10) // \"int\"

**3.3 clone(value)**

Effectue une copie explicite d'une valeur.

- Indispensable pour la gestion de la possession (ownership).

- Exemple :

> a: object = { x: int = 1 }
>
> b: object = clone(a)

**4. Module string**

Opérations déterministes sur les chaînes de caractères UTF-8.

- **string.length(s: string) -\> int** : Retourne la longueur de la chaîne.

- **string.slice(s: string, start: int, end: int) -\> string** : Extrait une sous-chaîne.

- **string.concat(a: string, b: string) -\> string** : Fusionne deux chaînes.

- **string.contains(s: string, sub: string) -\> bool** : Recherche une sous-chaîne.

- **string.to_int(s: string) -\> int** : Conversion explicite vers entier.

- **string.to_decimal(s: string) -\> decimal** : Conversion vers décimal.

**5. Module math**

Ogma refuse la surcharge. Chaque type a ses propres fonctions mathématiques.

**5.1 Opérations sur les entiers (int)**

- **math.add(a: int, b: int) -\> int**

- **math.sub(a: int, b: int) -\> int**

- **math.mul(a: int, b: int) -\> int**

- **math.div(a: int, b: int) -\> int** (Division entière)

**5.2 Opérations sur les décimaux (decimal)**

- **math.decimal_add(a: decimal, b: decimal) -\> decimal**

- **math.decimal_div(a: decimal, b: decimal) -\> decimal**

**Note :** Le modulo (%) est exclu de la std-min pour favoriser des implémentations explicites.

**6. Module error**

Gestion des erreurs sans interruptions magiques du flux.

- **error(message: string)** : Lève une erreur fatale avec un message.

- **try { \... } catch(e) { \... }** : Capture une erreur dans un bloc.

- **error.message(e) -\> string** : Extrait le message d\'une erreur capturée.

**7. Règles d\'or de la std‑min**

- **7.1 Pas de surcharge** : math.add est pour les int. Pour les décimaux, utilisez math.decimal_add.

- **7.2 Pas de conversion implicite** : \"10\" + 2 est une erreur. Utilisez math.add(string.to_int(\"10\"), 2).

- **7.3 Accès par point** : On utilise toujours module.fonction (jamais de slash /).

**8. Exemple complet (v0.3.11)**

Voici un programme utilisant la Standard Library avec la syntaxe corrigée :

Extrait de code

main: fn() {

// Manipulation de texte

name: string = \"Ogma\"

greeting: string = string.concat(\"Hello \", name)

print(greeting)

// Calculs mathématiques explicites

x: int = 10

y: int = 20

sum: int = math.add(x, y)

print(string.concat(\"La somme est : \", string(sum)))

}

**9. Ce que la std‑min ne contient pas**

Pour rester légère, la std-min ignore volontairement : les listes complexes, le JSON, le réseau, les fichiers et le graphisme. Ces fonctions seront ajoutées via des **Malles** (0.5) ou la **std-complète** (0.6).
