# 📘 OGMA --- Futures Fonctionnalités (Roadmap Technique)

**Versions 0.4 → 1.0** --- *Document de Vision Stratégique*

**1. Introduction**

Ogma 0.3.11 est la fondation. Ce document trace la route pour transformer ce noyau en un système complet capable de gérer des infrastructures critiques, tout en restant fidèle au dogme : **Explicite, Narratif, Déterministe.**

**2. Principes Directeurs**

- **Noyau Pur vs Malles Spécialisées** : Le langage (Core) reste agnostique aux protocoles techniques. IPv4, JSON, ou SQL vivent dans des **Malles**.

- **Évolution par l\'Explicite** : Chaque ajout doit rendre le code plus clair, pas plus court.

- **Zéro Magie** : Pas de sucre syntaxique qui cache une logique complexe.

**3. Ogma 0.4 --- Sûreté et Contrôle d\'Accès**

La version 0.4 marque l\'arrivée du contrôle de visibilité et des types énumérés.

**3.1 Introduction du mot-clé private**

Conformément à la philosophie Ogma, le langage est **public par défaut** (gain de lisibilité de 85%). Cependant, la 0.4 introduit private pour restreindre l\'accès à certains membres :

- **Usage** : private secret_key: string = \"\...\".

- **Philosophie** : La restriction est une exception intentionnelle. Si ce n\'est pas marqué private, c\'est ouvert.

- **Avantage** : On élimine le bruit visuel du pub tout en offrant une sécurité réelle pour l\'encapsulation.

**3.2 Types enum et union**

Pour structurer les données à choix multiples sans pattern matching opaque.

Extrait de code

// Exemple 0.4

Status: enum {

Actif

EnPause

Termine

}

**3.3 Type Result et Gestion d\'Erreur**

Remplacement des paniques système par un type retourné (Success / Error). La gestion d\'erreur devient une branche narrative du code : if result.is_error { \... }.

**4. Ogma 0.5 --- L\'Ère des Malles et de l\'I/O**

La 0.5 permet à Ogma de communiquer avec son environnement.

**4.1 Le Système de Malles (Gestionnaire de Dépendances)**

- Création du \"Mallier\" : outil de résolution et de versionnage des malles.

- Isolation totale des dépendances pour éviter l\' \"Enfer des DLL\".

**4.2 Net I/O & File I/O**

- **Malle Network** : Inclusion officielle des types techniques **IPv4 / IPv6** et des sockets.

- **Malle File** : Lecture/Écriture synchrone et explicite.

**4.3 Génériques (Generics)**

Possibilité de créer des fonctions universelles avec une syntaxe \<T\> strictement typée.

inverser: fn\<T\>(liste: list\<T\>) -\> list\<T\> { \... }

**5. Ogma 0.6 --- Optimisation et Standard Library**

**5.1 Runtime de Haute Performance**

- Optimisation de la gestion de l\'Ownership pour réduire les copies mémoires.

- **Standard Library** : Finalisation des modules math, string, datetime, et collections.

**6. Ogma 0.7 à 1.0 --- Stabilisation et Self-Hosting**

**6.1 Self-Hosting (Le Grand Test)**

Réécriture du compilateur Ogma en Ogma. C\'est l\'étape de certification finale de la puissance du langage.

**6.2 Documentation Institutionnelle**

Production de la référence complète pour les banques, les industries et les développeurs système.

**6.3 VID (Visual Interface Dialect)\***

Premier moteur graphique basé sur un dialecte. Permet de décrire une interface utilisateur comme on décrit une configuration.

\*Après la v1.0x

**7. Résumé de la Roadmap**

+-------------+---------------+-------------------------------------------------------------------------+
| **Version** | **Focus**     | **Fonctionnalités Clés**                                                |
+=============+===============+=========================================================================+
| **0.4**     | **Contrôle**  | **private**, enum, Result, Closures explicites.                         |
+-------------+---------------+-------------------------------------------------------------------------+
| **0.5**     | **Système**   | Malles, I/O, **IPv4/IPv6 (en malle)**, Génériques.                      |
+-------------+---------------+-------------------------------------------------------------------------+
| **0.6**     | **Maturité**  | Runtime optimisé, Std Library.                                          |
+-------------+---------------+-------------------------------------------------------------------------+
| **1.0**     | **Stabilité** | Self-hosting, Gel de la syntaxe, VID (Graphique), Usage Institutionnel. |
+-------------+---------------+-------------------------------------------------------------------------+

**8. Conclusion**

Ogma 0.4 → 1.0 est une évolution maîtrisée. En gardant le partage par défaut mais en introduisant private pour la sécurité, Ogma offre le meilleur des deux mondes : la rapidité d\'écriture et la rigueur de l\'encapsulation.
