# 📘 OGMA --- Spécification Officielle du Système de Types

**Version 0.3.11** --- *Document de référence sur la sûreté des données*

**1. Introduction**

Le système de types d'Ogma est **strict, statique et explicite**. Il est conçu pour éliminer toute forme d\'ambiguïté (\"Zéro Magie\").

**Principes directeurs :**

- **Aucune conversion implicite** : Le compilateur ne devine jamais.

- **Typage manifeste** : Le type d\'un littéral doit être clair dès sa déclaration.

- **Déterminisme** : Un type int restera un int durant toute sa vie.

**2. Les Types Scalaires (Primitifs)**

Ce sont les briques de base, stockées efficacement en mémoire.

- **int** : Entier signé 64 bits.

  - *Exemple :* age: int = 25

- **decimal** : Flottant 64 bits (IEEE754).

  - *Exemple :* prix: decimal = 19.99

- **bool** : Valeur logique (true ou false).

- **string** : Chaîne UTF-8 immuable.

- **char** : Caractère Unicode unique (apostrophes simples \'A\').

**3. Le Type Institutionnel : ognum(p, s)**

Le ognum est le type de précision d\'Ogma. Contrairement au decimal, il est **exact** et ne souffre d\'aucune erreur d\'arrondi binaire.

- **Syntaxe** : ognum(précision, échelle)

- **Usage** : Finance, mesures scientifiques, données critiques.

- **Exemple** :

> Extrait de code
>
> type Prix = ognum(10, 2)
>
> total: Prix = 120.50

**4. Types Composés et Blocs**

- **object** : Structure regroupant données et méthodes.

- **block** : Code encapsulé (type de la valeur de retour du bloc).

- **fn** : Signature d\'une fonction (arguments et retour).

**5. Règles de Déclaration et d\'Inférence**

Ogma refuse l\'inférence sauvage pour garantir que le code reste lisible par une IA ou un humain sans contexte.

**5.1 Littéraux**

Un littéral doit être assigné à une variable typée.

- **Interdit** : x = 10

- **Correct** : x: int = 10

**5.2 Inférence d\'Expression**

L\'inférence n\'est autorisée que lorsque le type est déjà connu par une opération précédente.

- **Correct** :

> Extrait de code
>
> a: int = 10
>
> b: int = 20
>
> c = math.add(a, b) // c est déduit comme \'int\' car math.add retourne un int.

**6. Conversions Explicites (Casting)**

Aucune valeur ne change de type sans un appel de fonction de conversion.

- **Valide** : d: decimal = decimal(10) (convertit l\'entier 10 en 10.0).

- **Valide** : s: string = string(true) (convertit en \"true\").

- **Interdit** : x: decimal = 10 (Erreur de type : attendu decimal, reçu int).

**7. Typage des Chemins d\'Accès**

L\'accès aux données se fait par le **point .**. Le système de type vérifie la validité de chaque segment au moment du build.

- **Valide** : utilisateur.nom

- **Valide** : math.pi

- **Interdit** : 10.nom (Le type int n\'a pas de membre nom).

**8. Résumé des Interdictions du Système de Type**

+-----------------------------+-------------+--------------------------------------+
| **Concept**                 | **Statut**  | **Alternative / Raison**             |
+=============================+=============+======================================+
| **Conversion Implicite**    | ❌ Interdit | Utiliser type(valeur) explicitement. |
+-----------------------------+-------------+--------------------------------------+
| **Surcharge (Overloading)** | ❌ Interdit | math.add vs math.decimal_add.        |
+-----------------------------+-------------+--------------------------------------+
| **Types dynamiques (any)**  | ❌ Interdit | Ogma est 100% statique en 0.3.11.    |
+-----------------------------+-------------+--------------------------------------+
| **IPv4 / IPv6 en dur**      | ❌ Interdit | Seront dans une **Malle** (0.5).     |
+-----------------------------+-------------+--------------------------------------+

**9. Conclusion**

Le système de types d'Ogma 0.3.11 est le garant de la **\"Zéro Magie\"**. En forçant l\'utilisateur à être explicite, on élimine 90% des bugs de runtime avant même que le programme ne soit lancé.
