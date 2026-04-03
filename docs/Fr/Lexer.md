# 📘 OGMA --- Spécification Officielle du Lexer

**Version 0.3.11** --- *Document de référence pour l\'analyse lexicale*

**1. Introduction**

Le Lexer est la porte d\'entrée du compilateur Ogma. Sa mission est de transformer le texte source en une suite de **Tokens** (unités atomiques) sans aucune interprétation sémantique.

**Principes de conception :**

- **Zéro Ambiguïté** : Un symbole ne peut pas avoir deux sens incompatibles.

- **Déterminisme** : L\'analyse est linéaire et prévisible.

- **Pureté** : Aucun mot-clé n\'est présent \"juste pour faire joli\".

**2. Catégories de Tokens**

Le Lexer d'Ogma reconnaît huit familles de tokens :

1.  **Mots‑clés** (Structure et Contrôle)

2.  **Identifiants** (Variables, fonctions, types)

3.  **Identifiants Dialecte** (kebab-case)

4.  **Littéraux** (Valeurs brutes)

5.  **Opérateurs** (Arithmétique, Logique, Affectation)

6.  **Délimiteurs** (Blocs et parenthèses)

7.  **Symboles structurants** (Le point, les deux-points, la virgule)

8.  **Commentaires** (Ignorés après l\'analyse)

**3. Mots‑clés Réservés**

Ces mots sont la propriété du langage. Ils ne peuvent pas servir de noms de variables.

**3.1 Mots‑clés réservés (futurs)** :

- private

- enum

- union

**3.2 Mots‑clés actifs 0.3.11**

- block

**3.3 Structure et Déclaration**

- module / import / as

- fn (Définition de fonction)

- object (Définition de structure)

- type (Alias ou définition de type)

- mut (Modificateur de mutabilité)

**3.4 Contrôle de flux**

- if / else

- try / catch

- error (Signalement d\'erreur)

**3.5 Littéraux Booléens**

- true / false

**4. Identifiants et Dialectes**

**4.1 Identifiants Standards (Snake Case)**

Utilisés pour tout le code Ogma standard.

- **Règle :** \[a-z\]\[a-z0-9\_\]\*

- **Exemples :** prix_total, calculer_tva, user_1

**4.2 Identifiants Dialecte (Kebab Case)**

Uniquement valides à l\'intérieur de blocs \[ \... \].

- **Règle :** \[a-z\]\[a-z0-9-\]\*

- **Exemples :** avance-rapide, set-color, tourne-droite

**5. Littéraux (Valeurs)**

**5.1 Numériques**

- **Entiers (int)** : 42, -10

- **Décimaux (decimal)** : 3.14, -0.5 (La présence du point définit le type decimal).

**5.2 Texte**

- **Chaînes (string)** : \"Texte entre guillemets\" (Gère l\'échappement \n, \\).

- **Caractères (char)** : \'A\', \'\n\' (Un seul caractère entre apostrophes).

**5.3 Types Spéciaux**

- **ognum** : ognum(p,s) (Syntaxe de définition de précision).

- **IPv4** : Reconnu comme un littéral spécifique via le mot-clé IPv4 \"192.168.1.1\".

**6. Opérateurs et Symboles**

**6.1 Opérateurs Arithmétiques**

- \+ (Addition), - (Soustraction), \* (Multiplication), / (Division).

**6.2 Opérateurs de Comparaison et Logique**

- ==, !=, \<, \>, \<=, \>=

- &, or, not

**6.3 Symboles de Structure (Crucial)**

- . (**Le Point**) : Séparateur de membres et de modules (math.add, person.name).

- : (**Deux-points**) : Liaison de type (x: int).

- = (**Égal**) : Affectation d\'une valeur.

- , (**Virgule**) : Séparateur d\'arguments.

**6.4 Délimiteurs de Blocs**

- { } : Blocs de code, objets, exécution.

- ( ) : Appels de fonctions, priorité mathématique.

- \[ \] : Blocs de dialectes (change le mode du Lexer pour autoriser le kebab-case).

**7. Gestion des Chemins et Accès**

Contrairement aux versions précédentes, Ogma 0.3.11 utilise exclusivement le **point** pour l\'accès aux données et aux modules.

- **Valide :** math.add(x, y)

- **Valide :** utilisateur.profil.nom

- **Invalide :** math/add (Le slash est réservé à la division mathématique).

**8. Commentaires**

1.  **Ligne unique** : // (Ignore tout jusqu\'à la fin de la ligne).

2.  **Bloc** : /\* \... \*/ (Ignore tout ce qui se trouve entre les balises).

**9. Erreurs Lexicales Immédiates**

Le Lexer doit s\'arrêter et signaler une erreur si :

- **Symbole inconnu** : Présence de @, #, \$, %, ?.

- **Identifiant mal formé** : Un nom commençant par un chiffre ou contenant un caractère interdit.

- **Chaîne non fermée** : Un guillemet ouvrant sans guillemet fermant avant la fin de ligne.

- **Confusion de délimiteur** : Utilisation d\'un symbole de dialecte hors d\'un bloc dialecte.

**10. Résumé des Interdictions (Zéro Magie)**

- **Pas de let ou const** : La déclaration se fait par nom: type.

- **Pas d\'incrémentation auto** : ++ et \-- sont interdits (utiliser x = x + 1).

- **Pas d\'opérateurs combinés** : +=, \*= sont interdits pour favoriser la lecture explicite.
