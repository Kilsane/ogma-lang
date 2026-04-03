# 📘 OGMA --- Spécification du Build System Minimal

**Version 0.3.11** --- *Document de référence pour l\'exécution et la compilation*

**1. Introduction**

Le Build System d\'Ogma 0.3.11 a pour mission d\'offrir une interface simple et robuste pour transformer le code source en action ou en binaire. À ce stade, il se concentre sur la gestion de projets locaux à plat, sans dépendances externes (malles).

**Principes de conception :**

- **Zéro Configuration** : Pas de fichier .toml ou .json requis en 0.3.11.

- **Rapidité** : Une commande unique pour tout faire.

- **Transparence** : Les erreurs de build doivent pointer précisément vers la ligne du code source.

**2. Commandes Officielles**

L\'outil en ligne de commande ogma propose deux modes principaux.

**2.1 ogma run \<fichier.ogma\>**

Exécute le code immédiatement via l\'interpréteur.

- **Usage** : Développement rapide, tests, scripts.

- **Processus** : Lecture → Lexing/Parsing → Analyse de type → Exécution directe dans le runtime.

**2.2 ogma build \<fichier.ogma\>**

Compile le code vers un exécutable natif.

- **Usage** : Distribution, performance maximale.

- **Sortie** :

  - Windows : nom_du_fichier.exe

  - Linux/macOS : nom_du_fichier (exécutable)

- **Processus** : Analyse complète → Optimisation de l\'AST → Génération de code machine.

**3. Structure d\'un Projet 0.3.11**

Le système de build impose une structure \"à plat\" pour garantir la simplicité de la résolution des noms.

Plaintext

mon_projet/

├── main.ogma (Contient la fonction main)

├── math.ogma (Module mathématique local)

└── utils.ogma (Module utilitaire local)

**Règles de résolution :**

1.  **Un fichier = Un module**. math.ogma devient automatiquement le module math.

2.  **Localité** : Tous les fichiers doivent être dans le même dossier.

3.  **Point d\'entrée** : Le build commence toujours par le fichier qui contient main: fn().

**4. Gestion des Imports (Build Local)**

Le Build System résout les imports de manière statique.

- **Valide** : import math (cherche math.ogma dans le dossier courant).

- **Valide** : import utils as u (crée un alias pour le module local).

- **Interdit** : import malles.json (les malles externes arrivent en 0.5).

- **Interdit** : import ./dossier/math (les sous-dossiers arrivent en 0.6).

**5. Flux de Compilation (Pipeline)**

Le build suit un chemin strict et s\'arrête à la première erreur rencontrée.

1.  **Analyse Lexicale** : Transformation en tokens.

2.  **Analyse Syntaxique** : Construction de l\'AST (Arbre de Syntaxe Abstraite).

3.  **Résolution de noms** : Vérification que les modules importés existent.

4.  **Vérification de Types** : S\'assurer que math.add(10, \"texte\") provoque une erreur avant l\'exécution.

5.  **Génération** : Production de l\'exécutable (ou exécution directe pour run).

**6. Exemple de Projet Cohérent (v0.3.11)**

**📄 Fichier : calcul.ogma**

Extrait de code

module calcul {

multiplier: fn(a: int, b: int) -\> int {

math.mul(a, b)

}

}

**📄 Fichier : main.ogma**

Extrait de code

import calcul

main: fn() {

x: int = 5

resultat: int = calcul.multiplier(x, 10)

print(string.concat(\"Résultat : \", string(resultat)))

}

**Commandes de build :**

- ogma run main.ogma → Affiche \"Résultat : 50\".

- ogma build main.ogma → Génère l\'exécutable main.

6.  **Limitations Volontaires et Évolutions**

+-------------------------------+-----------------+-------------+
| **Fonctionnalité**            | **État 0.3.11** | **Horizon** |
+===============================+=================+=============+
| **Malles (Packages)**         | ❌ Absent       | Ogma 0.5    |
+-------------------------------+-----------------+-------------+
| **Gestion des versions**      | ❌ Absent       | Ogma 0.5    |
+-------------------------------+-----------------+-------------+
| **Compilation incrémentale**  | ❌ Absent       | Ogma 0.6    |
+-------------------------------+-----------------+-------------+
| **Fichier de config (.toml)** | ❌ Absent       | Ogma 0.5    |
+-------------------------------+-----------------+-------------+

**8. Conclusion**

Le Build System Minimal est le moteur de confiance du développeur Ogma. En restant limité aux fichiers locaux en 0.3.11, il garantit une stabilité totale pour les premiers utilisateurs et simplifie drastiquement ton implémentation en Rust.

**Pourquoi c\'est important ?**

En interdisant les malles et les configurations complexes maintenant, on se concentres sur l\'essentiel : **faire en sorte que le code s\'exécute parfaitement**.
