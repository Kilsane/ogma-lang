# Spécification Technique : Gestionnaire de Malles Ogma

## 1. Introduction
Ce document définit l'organisation physique et logique des composants réutilisables du langage Ogma, appelés "Malles". L'objectif est de passer d'un système de build local "à plat" à un système modulaire capable de gérer des bibliothèques personnelles, de test et système, tout en garantissant la portabilité sur Windows, macOS et Linux.

## 2. Concepts Fondamentaux
Une **Malle** est une unité de code cohérente. Elle peut exister sous deux formes :
* **Le Dossier (`.ogma`) :** Utilisé durant la phase de développement (code "ouvert").
* **L'Archive (`.malle`) :** Un conteneur scellé (format tar/compressé) pour la distribution et le partage.

## 3. Hiérarchie et Priorités (Souveraineté Locale)
Afin d'éviter les conflits de versions et les "effets d'ombre" lors des mises à jour du langage, Ogma applique une règle de priorité stricte. En cas de présence d'une malle de même nom dans plusieurs emplacements, l'ordre de recherche est le suivant :

1.  **Configuration Manuelle :** Chemins explicitement définis dans le fichier `malle.toml`.
2.  **Dossier Local de Développement (`malle.test` / `malle.dev`) :** Priorité aux brouillons et tests en cours dans le projet.
3.  **Dossier Local Utilisateur (`malle.user`) :** Versions stables des outils propres au projet actuel.
4.  **Dossier Global Utilisateur :** Malles partagées entre tous les projets de l'utilisateur (stockées dans les dossiers d'application de l'OS).
5.  **Système (Natif) :** Bibliothèque standard fournie avec l'installation d'Ogma.



## 4. Le Fichier de Configuration : `malle.toml`
Le fichier `malle.toml` est le "cerveau" de l'organisation. Placé à la racine du projet, il utilise le format TOML pour sa lisibilité et sa rigueur.

### Structure type :
```toml
# Configuration des malles du projet
[projet]
nom = "MonApplication"
version = "1.0.0"

[priorites]
# Définit l'ordre de recherche personnalisé si besoin
ordre = ["malle.test", "malle.user", "systeme"]

[chemins]
# Pointer une malle spécifique hors de l'arborescence standard
logique = "./libs/ma_logique_v2"
```

## 5. Gestion des Conflits et Sécurité (La Règle du STOP)
Ogma privilégie la **Sécurité par l'Arrêt** plutôt que la **Supposition par l'Erreur**.
* **Ambiguïté :** Si deux malles du même nom sont détectées sans priorité définie dans `malle.toml`, le compilateur s'arrête immédiatement.
* **Message d'erreur :** L'utilisateur reçoit un message explicite indiquant les chemins en conflit et les solutions possibles (alias ou modification du `.toml`).

## 6. Portabilité et OS
Le système repose sur des chemins relatifs pour garantir que le projet fonctionne à l'identique après un déplacement.
* Les séparateurs de chemins sont normalisés par le compilateur pour assurer la compatibilité entre Windows (backslash) et les systèmes Unix et macOS (slash).


## 7. Évolutions Futures (v2.x)
* Introduction d'un fichier `malle.lock` pour figer les empreintes numériques des malles.
* Gestion des dépendances distantes (téléchargement automatique).
* Signature cryptographique des archives `.malle`.

