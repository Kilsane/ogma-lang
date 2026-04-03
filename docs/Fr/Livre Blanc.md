# 📘 OGMA : LE LIVRE BLANC

**Version 0.3.11 --- La Fondation Narrative** **Concepteur : Bruce Duchet**

**1. VISION ET PHILOSOPHIE PROFONDE**

Ogma n\'est pas un outil, c\'est un écosystème de pensée. Il est né du constat que la programmation moderne s\'est égarée dans la \"magie\" (comportements implicites) et la complexité inutile.

1.  **L\'Héritage Spirituel**

- **Rebol & Red** : Pour la structure par blocs et l\'aspect **linguistique**. Ogma traite le code comme une grammaire humaine.

- **Rust** : Pour la gestion de la mémoire par **Ownership**. Ogma simplifie ce concept pour le rendre accessible sans sacrifier la performance.

- **Zig & Go** : Pour le refus de la surcharge et des abstractions cachées. Une ligne de code Ogma ne fait **qu\'une seule chose**.

  1.  **Le Manifeste \"Zéro Magie\"**

1.  **Visibilité Totale** : Rien ne se passe en coulisse. Pas de Garbage Collector qui fige le programme, pas de conversion de type automatique.

2.  **Narrativité** : Un expert métier doit pouvoir lire un dialecte Ogma et comprendre l\'intention sans être développeur.

3.  **Stabilité Séculaire** : Ogma est conçu pour que le code écrit aujourd\'hui soit lisible et fonctionnel dans 50 ans.

**2. LE SYSTÈME DE TYPES (LA RIGUEUR NATIVE)**

Le système de types d\'Ogma est sa plus grande force. Il est statique, strict et déterministe.

**2.1 Les Types Scalaires (Noyau)**

- **int** : Entier signé 64 bits. C\'est le type par excellence pour les indices et les compteurs.

- **decimal** : Flottant 64 bits (IEEE754). Utilisé pour les simulations physiques ou les calculs où une approximation infinitésimale est acceptable.

- **string** : Chaîne UTF-8 immuable. Le runtime gère les chaînes comme des blocs de mémoire protégés.

- **bool** : true ou false.

- **char** : Point de code Unicode. Syntaxe : \'A\'.

**2.2 L\'Innovation Majeure : ognum(p, s)**

Le type ognum est le \"Type Institutionnel\". Il remplace les types \"Money\" ou \"Fixed\" souvent mal implémentés.

- **Précision (p)** : Capacité totale de chiffres (ex: 20).

- **Échelle (s)** : Chiffres après la virgule (ex: 2).

- **Comportement** : Il est stocké sous forme d\'entier large avec un facteur d\'échelle. Aucun arrondi binaire ne peut corrompre un calcul de TVA ou une trajectoire balistique.

**3. VARIABLES ET GESTION MÉMOIRE**

Ogma abandonne le let pour une syntaxe de \"déclaration d\'état\".

**3.1 Déclaration et Mutabilité**

- **Immuabilité (Défaut)** : valeur : int = 100. Une fois définie, elle est gravée dans la pile.

- **Mutabilité (Explicite)** : mut compteur : int = 0. Le mot-clé mut avertit le lecteur que cette zone mémoire est instable.

- **Constantes** : const OR_RATIO : decimal = 1.618. Portée globale, évaluée à la compilation.

**3.2 L\'Ownership (La Propriété)**

Inspiré de Rust, mais sans la complexité des \"lifetimes\" explicites pour l\'utilisateur 0.3.11.

1.  **Une valeur = Un propriétaire.**

2.  **Le Transfert (Move)** : Si j\'assigne un objet A à B, A meurt.

3.  **Libération Automatique** : À la fin d\'un bloc { }, le runtime nettoie tout ce qui a été créé. C\'est le \"Scope-Based Management\".

**4. LA GRAMMAIRE DES BLOCS**

Ogma est un langage de blocs. Il en existe quatre types, chacun avec son propre moteur d\'évaluation.

**4.1 Le Bloc d\'Exécution { \... }**

C\'est le moteur impératif.

- Exécute les instructions de haut en bas.

- La dernière expression est la \"valeur\" du bloc.

- Crée une nouvelle portée mémoire (Stack Frame).

**4.2 Le Bloc Objet object { \... }**

C\'est le constructeur de structure.

- Définit des champs (données).

- Définit des méthodes (fonctions liées).

- L\'objet est toujours créé sur le Tas (Heap).

**4.3 Le Bloc Explicite block { \... }**

C\'est du code \"mis en conserve\".

- Il n\'est **pas** exécuté à sa rencontre.

- Il peut être passé comme argument (Lazy Evaluation).

- Il sera exécuté plus tard par une commande comme system.run.

**4.4 Le Bloc Dialecte \[ \... \]**

C\'est la porte ouverte vers les DSL (Domain Specific Languages).

- Utilise le **kebab-case** (vitesse-maximale: 100).

- N\'est jamais exécuté par le moteur principal.

- Est lu par une fonction Ogma qui interprète cette structure de données.

**5. FONCTIONS ET NAVIGATION**

**5.1 Signatures Strictes**

Une fonction Ogma ne peut pas être ambiguë.

Extrait de code

appliquer_taxe: fn(montant: ognum(10,2), taux: decimal) -\> ognum(10,2) {

// Corps de la fonction

}

- **Pas de surcharge** : Une fonction = Un nom unique.

- **Parenthèses collées** : f() et non f ().

**5.2 Chemins (Paths)**

L\'accès aux données se fait par le point ., remplaçant le slash / des versions précédentes pour éviter la confusion avec la division.

- utilisateur.profil.nom

- math.calculer_pi()

**6. L\'ÉCOSYSTÈME : MODULES ET MALLES**

Ogma est conçu pour le travail en équipe et la réutilisation de code.

**6.1 Modules (0.3.11)**

- **Un fichier = Un module**. finance.ogma est le module finance.

- **Importation** : import finance.

- **Visibilité** : Tout est publique par défaut (encapsulation stricte).

**6.2 Les Malles (Vision 0.5)**

La **Malle** est le concept de bibliothèque externe. On \"embarque une malle\" dans son projet.

- Elles seront versionnées et stockées dans un dépôt central (Mallier).

- Le terme \"Malle\" souligne l\'aspect robuste et transportable de la ressource.

**7. RÈGLES DE SÉCURITÉ ET ERREURS**

- **Zéro Null** : Ogma ne connaît pas null. Une variable doit toujours avoir une valeur valide.

- **Panique Contrôlée** : En cas d\'erreur fatale (division par zéro, move violation), le programme s\'arrête avec une trace narrative complète.

- **Pas de conversions implicites** : On ne peut pas additionner un int et un decimal sans un cast explicite : decimal(mon_int) + mon_decimal.

**8. RÉSUMÉ TECHNIQUE POUR L\'IMPLÉMENTATION**

1.  **Lexer** : Reconnaissance des 4 blocs et des types numériques.

2.  **Parser** : Construction d\'un AST où chaque bloc a son propre nœud de type.

3.  **Runtime** : Gestion de la pile (Stack) pour les scalaires et du tas (Heap) pour les objets/chaînes.

4.  **Ownership** : Vérification sémantique des transferts de propriété lors des assignations.

**9. CONCLUSION**

Le Livre Blanc Ogma 0.3.11 définit un langage qui ne fait aucun compromis. Il est l\'outil parfait pour bâtir des systèmes où la **clarté du code** est la garantie de la **sécurité du système**.

**Ogma n\'est pas seulement du code, c\'est une architecture de la vérité.**
