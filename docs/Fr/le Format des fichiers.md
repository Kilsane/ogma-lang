# 📘 OGMA --- Spécification du Format des Fichiers

**Version 0.3.11** --- *Document de référence pour la structure des sources*

**1. Introduction**

Le format des fichiers Ogma définit l\'organisation du code source, la résolution des modules et la gestion des dépendances. Fidèle à la philosophie \"Zéro Magie\", chaque fichier correspond à une unité logique claire.

**Objectifs du format :**

- **Unicité** : Un fichier = Un module.

- **Prévisibilité** : Pas de chargement caché ou de fusion implicite.

- **Simplicité** : Une structure plate pour la version 0.3.11.

**2. Extension et Encodage**

- **Extension officielle** : .ogma (ex: main.ogma, logic.ogma).

- **Encodage** : **UTF-8 strict** sans BOM.

- **Fin de ligne** : LF (\n) fortement recommandé (CRLF accepté).

- **Indentation** : Non significative, mais les espaces sont préférés aux tabulations pour la lisibilité.

**3. Structure d\'un Fichier Ogma**

L\'ordre des éléments dans un fichier doit être strictement respecté pour garantir une lecture fluide par l\'humain et la machine :

1.  **Imports** (optionnels)

2.  **Déclaration de Module** (optionnelle mais recommandée)

3.  **Corps du programme** (constantes, types, fonctions)

**4. Le Système de Modules**

Dans Ogma, la hiérarchie est simple :

- **4.1 Unité de compilation** : Un fichier physique est égal à un module.

- **4.2 Nommage automatique** : Si le mot-clé module est omis, le module prend le nom du fichier (sans l\'extension).

- **4.3 Déclaration explicite** :

> Extrait de code
>
> module math {
>
> // Le contenu du module est encapsulé ici
>
> }

- **4.4 Interdiction** : Un fichier ne peut contenir qu\'un seul bloc module.

**5. Système d\'Imports**

Les imports permettent d\'accéder aux fonctions et types d\'autres fichiers .ogma situés dans le même répertoire.

- **5.1 Import simple** : import math

- **5.2 Import avec alias** : import utils as u

- **5.3 Règles strictes (v0.3.11)** :

  - Pas d\'imports sélectifs (import { add }).

  - Pas d\'imports \"wildcard\" (import math.\*).

  - L\'accès aux fonctions importées se fait obligatoirement par le **point** : math.add(x, y).

**6. Organisation du Projet (v0.3.11)**

Pour cette version, le système de build attend une structure **plate** :

Plaintext

mon_projet/

├── main.ogma (Point d\'entrée obligatoire)

├── moteur.ogma

└── data.ogma

- **Le point d\'entrée** : Le fichier main.ogma doit impérativement contenir la fonction main: fn().

**7. Règles de Propreté (Zéro Magie)**

- **7.1 Pas de multiligne sauvage** : Une seule déclaration par ligne (pas de x: int = 1 y: int = 2).

- **7.2 Pas de code \"orphelin\"** : Si un bloc module { \... } est déclaré, tout le code doit être à l\'intérieur. Rien ne doit traîner après l\'accolade fermante.

- **7.3 Pas de symboles invisibles** : Seuls les caractères UTF-8 imprimables et les sauts de ligne sont autorisés.

**8. Exemple Complet et Corrigé**

**📄 Fichier : math_utils.ogma**

Extrait de code

module math_utils {

pi: decimal = 3.14159

carre: fn(n: int) -\> int {

math.mul(n, n)

}

}

**📄 Fichier : main.ogma**

Extrait de code

import math_utils as mu

main: fn() {

rayon: int = 10

surface: decimal = math.mul(mu.pi, mu.carre(rayon))

print(string.concat(\"Surface : \", string(surface)))

}

**9. Tableau Comparatif des Interdictions**

+----------------------------+---------------------+------------------------------------+
| **Élément**                | **État en v0.3.11** | **Raison**                         |
+============================+=====================+====================================+
| **Sous-dossiers**          | ❌ Interdit         | Simplicité du parseur initial.     |
+----------------------------+---------------------+------------------------------------+
| **Imports circulaires**    | ❌ Interdit         | Éviter les boucles de dépendances. |
+----------------------------+---------------------+------------------------------------+
| **Code après }**           | ❌ Interdit         | Lisibilité et rigueur narrative.   |
+----------------------------+---------------------+------------------------------------+
| **Slash / dans les accès** | ❌ Interdit         | Réservé à la division (cf. Lexer). |
+----------------------------+---------------------+------------------------------------+
