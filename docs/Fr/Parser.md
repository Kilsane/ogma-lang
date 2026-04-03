# 📘 OGMA --- Spécification Officielle du Parser

**Version 0.3.11** --- *Document de référence pour l\'Analyse Syntaxique*

**1. Introduction**

Le Parser d'Ogma transforme le flux de tokens en un **Arbre de Syntaxe Abstraite (AST)**. Contrairement à d\'autres langages, le parser d\'Ogma est \"intolérant\" : toute déviation de la structure narrative provoque une erreur immédiate.

**Principes de parsing :**

- **Structure Fixe** : Pas de sucre syntaxique. Une seule façon d\'écrire une structure.

- **Priorité aux Blocs** : Les blocs sont les unités reines du langage.

- **Zéro Ambiguïté** : Aucun token ne doit avoir deux interprétations possibles.

**2. Grammaire Officielle (EBNF Simplifié)**

**2.1 Déclarations (Le sommet de l\'arbre)**

EBNF

declaration = variable_decl \| function_decl \| object_decl \| module_decl ;

variable_decl = identifier , \":\" , type_hint , \"=\" , expression ;

type_hint = identifier ;

function_decl = identifier , \":\" , \"fn\" , \"(\" , \[ params \] , \")\" , \[ \"-\>\" , type_hint \] , execution_block ;

module_decl = \"module\" , identifier , \"{\" , { declaration } , \"}\" ;

**2.2 Expressions et Chemins**

EBNF

expression = literal \| identifier \| path \| function_call \| block_type ;

path = identifier , { \".\" , ( identifier \| method_call ) } ;

function_call = identifier , \"(\" , \[ args \] , \")\" ;

method_call = identifier , \"(\" , \[ args \] , \")\" ;

**2.3 Les 4 Types de Blocs**

EBNF

block_type = execution_block \| object_block \| explicit_block \| dialect_block ;

execution_block = \"{\" , { statement } , expression , \"}\" ;

object_block = \"object\" , \"{\" , { field_decl } , \"}\" ;

explicit_block = \"block\" , execution_block ;

dialect_block = \"\[\" , { dialect_line } , \"\]\" ;

**3. Priorités des Opérateurs**

Le Parser applique les règles mathématiques et logiques standard, sans exception.

+--------------+----------------+--------------------------------------------+
| **Priorité** | **Opérateur**  | **Description**                            |
+==============+================+============================================+
| 1            | .              | Accès membre / Chemin (le plus fort)       |
+--------------+----------------+--------------------------------------------+
| 2            | ()             | Appels de fonction et parenthèses groupées |
+--------------+----------------+--------------------------------------------+
| 3            | \*, /          | Multiplication et Division                 |
+--------------+----------------+--------------------------------------------+
| 4            | +, -           | Addition et Soustraction                   |
+--------------+----------------+--------------------------------------------+
| 5            | ==, !=, \<, \> | Comparaisons                               |
+--------------+----------------+--------------------------------------------+
| 6            | not, and, or   | Logique (par ordre de force)               |
+--------------+----------------+--------------------------------------------+
| 7            | =              | Assignation (le plus faible)               |
+--------------+----------------+--------------------------------------------+

**4. Règles de Propreté (Strict Parser)**

**4.1 L\'interdiction de l\'espace (Call-Gap)**

Pour éviter toute confusion, aucun espace n\'est autorisé entre le nom d\'une fonction et sa parenthèse.

- ✅ print(\"Hello\")

- ❌ print (\"Hello\") (Erreur de parsing)

**4.2 La règle du point final**

Dans un chemin, le dernier segment ne peut pas être \"vide\".

- ✅ personne.nom

- ❌ personne. (Erreur de parsing)

**4.3 La structure des blocs**

Un bloc d\'exécution {} doit toujours se terminer par une expression qui représente sa valeur de retour. S\'il ne retourne rien, il doit se terminer par un type void (prévu pour 0.4).

**5. Architecture de l\'AST (Sortie du Parser)**

Le Parser produit une structure de données arborescente que le Runtime pourra parcourir.

**Nœuds principaux de l\'AST :**

- **Stmt::Decl** : Contient le nom, le type et la valeur initiale.

- **Expr::Call** : Contient le nom de la fonction et une liste de nœuds d\'arguments.

- **Expr::Path** : Une liste chaînée de segments (identifiants ou appels).

- **Block::Dialect** : Une liste brute de lignes textuelles pour le traitement différé.

**6. Gestion des Erreurs de Parsing**

Le Parser d\'Ogma est conçu pour être pédagogique.

- **Erreur :** Attendu \':\' après l\'identifiant \'age\', trouvé \'=\'.

- **Erreur :** Bloc d\'exécution non fermé. \'{\' ouvert à la ligne 12.

- **Erreur :** Symbole interdit \'/\' utilisé pour un chemin. Utilisez \'.\' à la place.

**7. Conclusion**

Le Parser de la version 0.3.11 est le gardien du temple. En refusant toute syntaxe ambiguë ou \"raccourcie\", il garantit que le code source Ogma est une représentation parfaite de l\'intention du programmeur.

