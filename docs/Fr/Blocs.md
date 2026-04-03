# 📘 OGMA --- Spécification Officielle des Blocs

**Version 0.3.11** --- *Document de référence sur la structure et la portée*

**1. Introduction**

Les blocs définissent la structure, la portée et l\'évaluation du code Ogma. Ils garantissent que le langage reste déterministe et narratif.

Ogma distingue quatre formes de blocs :

1.  **Bloc d'exécution { \... }** : Logique séquentielle.

2.  **Bloc dialecte \[ \... \]** : Données déclaratives (DSL).

3.  **Bloc objet object { \... }** : Structures de données et méthodes.

4.  **Bloc explicite block { \... }** : Code encapsulé stocké comme valeur.

**2. Bloc d'exécution { \... }**

C\'est l\'unité de base pour l\'exécution de code.

- **Syntaxe :**

> Extrait de code
>
> {
>
> x: int = 10
>
> y: int = math.add(x, 2)
>
> y // Dernière expression = valeur de retour du bloc
>
> }

- **Règles :**

  - Crée une portée lexicale (les variables meurent à la fermeture de }).

  - Pas de ; en fin de ligne.

  - Retourne toujours la valeur de la dernière ligne.

**3. Bloc dialecte \[ \... \]**

Utilisé pour créer des langages dédiés (comme Logo ou des interfaces graphiques).

- **Syntaxe :**

> Extrait de code
>
> canvas.draw(\[
>
> circle center player.pos radius 20
>
> line color blue from 0.0 to 100.0
>
> \])

- **Règles :**

  - **Kebab-case** autorisé pour les commandes (line-to).

  - Jamais exécuté directement par le runtime, mais interprété par une fonction.

  - **Immuable** : On ne peut pas modifier un dialecte après sa construction.

**4. Bloc objet object { \... }**

Définit une structure de données complexe.

- **Syntaxe :**

> Extrait de code
>
> person: object {
>
> name: string = \"Alice\"
>
> mut age: int = 20
>
> greet: fn(self) {
>
> print(string.concat(\"Hello \", self.name))
>
> }
>
> }

- **Règles :**

  - Syntaxe nom: type = valeur.

  - Les méthodes reçoivent self explicitement.

  - Utilise le point . pour accéder aux membres (self.name).

**5. Bloc explicite block { \... }**

Permet de stocker du code sans l\'exécuter. Utile pour les scripts internes ou les comportements différés.

- **Syntaxe :**

> Extrait de code
>
> npc: object {
>
> name: string = \"Garde\"
>
> on_alert: block {
>
> print(\"Intrus détecté !\")
>
> }
>
> }

- **Règles :**

  - C\'est une **valeur**. Il peut être passé en argument.

  - Ne possède pas de paramètres (différent d\'une fn).

  - S\'exécute via une fonction de déclenchement (ex: system.run).

**6. Grammaire Formelle (EBNF Simplifié)**

EBNF

block = execution_block \| dialect_block \| object_block \| explicit_block ;

execution_block = \"{\" , { statement } , expression , \"}\" ;

statement = declaration \| assignment \| expression ;

declaration = \[ \"mut\" \] , identifier , \":\" , type , \"=\" , expression ;

dialect_block = \"\[\" , { dialect_command } , \"\]\" ;

dialect_command = kebab_identifier , { argument } ;

object_block = \"object\" , \"{\" , { field_definition } , \"}\" ;

field_definition = \[ \"mut\" \] , identifier , \":\" , type , \"=\" , expression ;

explicit_block = \"block\" , execution_block ;

**7. Sémantique d'Évaluation**

+------------------+--------------------+----------------------+--------------------------+
| **Type de Bloc** | **Exécution auto** | **Valeur de retour** | **Usage principal**      |
+==================+====================+======================+==========================+
| **{ }**          | Oui                | Dernière ligne       | Logique et portée locale |
+------------------+--------------------+----------------------+--------------------------+
| **\[ \]**        | Non                | Structure de données | DSL / Logo / UI          |
+------------------+--------------------+----------------------+--------------------------+
| **object { }**   | Non                | Instance d\'objet    | Data & Méthodes          |
+------------------+--------------------+----------------------+--------------------------+
| **block { }**    | Non                | Le bloc lui-même     | Callbacks / Scripts      |
+------------------+--------------------+----------------------+--------------------------+

**8. Représentation AST (Rust-Ready)**

Pour implémentation en Rust, voici la structure de données recommandée :

Rust

enum Expr {

Block(ExecutionBlock),

Dialect(DialectBlock),

Object(ObjectLiteral),

Explicit(ExplicitBlock),

// \...

}

struct ExecutionBlock {

statements: Vec\<Stmt\>,

final_expr: Box\<Expr\>,

}

struct ObjectField {

name: String,

is_mutable: bool,

value: Box\<Expr\>,

}
