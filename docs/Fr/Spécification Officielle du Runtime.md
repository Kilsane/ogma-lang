# 📘 OGMA --- Spécification Officielle du Runtime

**Version 0.3.11** --- *Document de référence pour l\'exécution mémoire*

**1. Introduction**

Le Runtime d'Ogma régit l\'existence des données en mémoire et leur cycle de vie. Il est conçu pour être **prévisible** et **déterministe**. Contrairement à Java ou Python, il n\'y a pas de \"Ramasse-miettes\" (Garbage Collector) ; la mémoire est gérée par un système d\'**Ownership strict** (similaire à Rust, mais simplifié).

**2. Représentation des Valeurs en Mémoire**

Toute donnée dans Ogma est encapsulée dans une structure Value.

Rust

// Vision conceptuelle du stockage (Rust-like)

enum Value {

Int(i64),

Decimal(f64),

Bool(bool),

Char(char),

String(String),

Ognum(Precision, Scale, Box\<InternalData\>),

Object(ObjectInstance),

Block(ExecutionBlock),

ExplicitBlock(ExplicitBlock),

}

**3. Gestion de la Mémoire : L\'Ownership**

C\'est le pilier de la performance d\'Ogma.

- **Propriétaire unique** : Chaque objet (object) a un seul propriétaire à la fois.

- **Le \"Move\" (Déplacement)** : Quand on assigne un objet à une nouvelle variable, l\'ancienne ne peut plus l\'utiliser.

> Extrait de code
>
> a: Person = Person(name: \"Alice\")
>
> b = a // \'a\' est maintenant vide (état Moved).
>
> print(a.name) // ERREUR RUNTIME : Accès à une valeur déplacée.

- **Destruction Déterministe** : Lorsqu\'une variable sort de sa portée (fin de bloc { }), la mémoire est libérée **instantanément**.

**4. Pile (Stack) et Tas (Heap)**

- **La Pile (Stack)** : Stocke les types simples (int, decimal, bool) et les pointeurs vers les objets. Très rapide.

- **Le Tas (Heap)** : Stocke les données lourdes (string, object, block). Géré par l\'ownership.

**5. Modèle d\'Évaluation des Blocs**

**5.1 Blocs d\'exécution { \... }**

Le runtime crée une **Stack Frame** (cadre de pile) :

1.  Évalue chaque ligne de haut en bas.

2.  Stocke la valeur de la dernière ligne.

3.  Détruit toutes les variables locales créées dans le bloc.

4.  Rend la valeur finale au parent.

**5.2 Blocs Explicites block { \... }**

Ils sont stockés dans le Tas comme du **code inerte**. Le runtime ne les touche pas tant qu\'une fonction de déclenchement (ex: system.run) n\'est pas appelée.

**6. Appel de Fonction et \"Self\"**

Lorsqu\'une méthode d\'un objet est appelée :

1.  Le runtime injecte l\'objet lui-même dans le paramètre self.

2.  Une nouvelle portée est créée.

3.  Les arguments sont passés par **Move** (pour les objets) ou par **Copie** (pour les types simples).

**7. Gestion des Erreurs Runtime**

Ogma ne \"plante\" pas de manière désordonnée. Toute erreur (division par zéro, objet déplacé, type incorrect) déclenche une **Panique Contrôlée**.

- **Propagation** : L\'erreur remonte les blocs jusqu\'à trouver un intercepteur ou arrêter le programme proprement avec un message narratif.

- **Zéro Magie** : Le runtime ne tentera jamais de \"réparer\" une erreur (ex: transformer un null en 0).

**8. Résumé Technique du Runtime**

+---------------------------+----------------------------------------------+
| **Action**                | **Comportement Runtime**                     |
+===========================+==============================================+
| **Assignation d\'objet**  | Déplacement de propriété (Move).             |
+---------------------------+----------------------------------------------+
| **Assignation d\'entier** | Copie de la valeur.                          |
+---------------------------+----------------------------------------------+
| **Fin de bloc**           | Libération immédiate de la mémoire locale.   |
+---------------------------+----------------------------------------------+
| **Accès membre**          | Vérification de l\'existence via le point .. |
+---------------------------+----------------------------------------------+
| **Conversion**            | Appel explicite d\'une fonction de cast.     |
+---------------------------+----------------------------------------------+

**9. Conclusion**

Le Runtime d'Ogma 0.3.11 est le socle de ta mise en œuvre en Rust. Sa simplicité (pas de GC, pas de pointeurs complexes) permet d\'obtenir des performances proches du C tout en gardant une sécurité totale.
