# 📘 OGMA --- Documentation Publique

# Version 0.3.11 --- Structure officielle

**1. Page d'accueil (courte)**

🟦 **Bienvenue dans Ogma**

Ogma est un langage moderne, narratif et explicite, conçu pour être lisible, déterministe et durable.

Il privilégie la clarté, l'absence de magie, et une syntaxe cohérente qui reste compréhensible même après des années.

Ogma est pensé pour :

- les développeurs qui aiment les langages propres et structurés,

- les institutions qui ont besoin de stabilité,

- les créateurs qui veulent des mini‑langages internes (dialectes),

- les projets où la lisibilité est aussi importante que la performance.

👉 Cette documentation vous guide pas à pas pour découvrir Ogma, écrire vos premiers programmes, et comprendre les bases du langage.

**2. Guide utilisateur**

**2.1 Installation**

Ogma 0.3.11 fournit deux commandes :

ogma run fichier.ogma

ogma build fichier.ogma

L'installation dépendra de votre plateforme (instructions à venir dans Ogma 0.4).

**2.2 Votre premier fichier Ogma**

Créez un fichier main.ogma :

Extrait de code

main: fn() {

print(\"Hello Ogma\")

}

Exécutez‑le :

ogma run main.ogma

**2.3 Structure d'un fichier Ogma**

Un fichier Ogma contient :

- des imports

- un module (optionnel)

- des déclarations

- des expressions

Exemple :

Extrait de code

import math

main: fn() {

x: int = 10

y: int = 20

print(math.add(x, y))

}

**2.4 Syntaxe de base**

Déclaration de variable

x: int = 10

mut y: int = 20

Fonction

Extrait de code

add: fn(a: int, b: int) -\> int {

a + b

}

Objet

Extrait de code

person: object {

name: string = \"Alice\"

greet: fn(self) {

print(\"Hello \" + self.name)

}

}

Bloc d'exécution

Extrait de code

{

x: int = 10

x + 2

}

**2.5 Types de base**

- int

- decimal

- string

- bool

- char

- ognum(p,s) (type numérique exact)

**2.6 Modules et imports**

Import simple

import math

Import avec alias

import utils as u

**2.7 Erreurs**

Ogma utilise :

Extrait de code

try { \... }

catch(e) { \... }

Et la fonction :

error(\"message\")

**2.8 Standard Library minimale**

Ogma fournit :

print : Affiche une valeur.

string : length, slice, concat, contains

math : add, sub, mul, div

**3. Tutoriel Ogma**

**3.1 Hello World**

Extrait de code

main: fn() {

print(\"Hello Ogma\")

}

**3.2 Variables et expressions**

Extrait de code

x: int = 10

y: int = x + 5

print(y)

**3.3 Fonctions**

Extrait de code

add: fn(a: int, b: int) -\> int {

a + b

}

main: fn() {

print(add(3, 4))

}

**3.4 Objets**

Extrait de code

person: object {

name: string = \"Alice\"

greet: fn(self) {

print(\"Hello \" + self.name)

}

}

main: fn() {

person.greet()

}

**3.5 Blocs**

Extrait de code

result: int = {

x: int = 10

x + 2

}

print(result)

**3.6 Modules**

math.ogma

Extrait de code

add: fn(a: int, b: int) -\> int {

a + b

}

main.ogma

Extrait de code

import math

main: fn() {

print(math.add(2, 3))

}

**3.7 Mini‑projet complet**

utils.ogma

Extrait de code

double: fn(x: int) -\> int {

x \* 2

}

main.ogma

Extrait de code

import utils

main: fn() {

x: int = 21

print(utils.double(x))

}

**4. Référence officielle Ogma 0.3.11**

**4.1 Mots‑clés**

Structure

- module

- import

- as

- object

- block

- fn

- mut

- const

- type

Contrôle

- if

- else

- try

- catch

- error

Littéraux

- true

- false

**4.2 Types**

Scalaires

- int

- decimal

- string

- bool

- char

Conceptuels

- ognum(p,s)

Composés

- object

- block

- function

**4.3 Blocs**

Bloc d'exécution : { \... }

Bloc dialecte : \[ \... \]

Bloc objet : object { \... }

Bloc explicite : block { \... }

**4.4 Opérateurs**

Arithmétique : +, -, \*, /

Comparaison : ==, !=, \<, \>, \<=, \>=

Logique : and, or, not

**4.5 Modules**

Déclaration

module nom { \... }

Import

import nom

import nom as alias

**4.6 Erreurs**

Lever une erreur : error(\"message\")

Capturer une erreur :

Extrait de code

try { \... }

catch(e) { \... }

**5. Structure de la documentation publique**

Voici l'arborescence recommandée pour GitHub Pages :

docs/

\... (structure conservée)
