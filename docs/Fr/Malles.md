# 📦 OGMA --- La Malle (Écosystème et Dépendances)

**Version 0.3.11** --- *Note conceptuelle sur l\'extension du langage*

**1. Introduction : L\'esprit de la \"Malle\"**

Dans Ogma, nous n\'utilisons pas les termes *package*, *library* ou *crate*. Nous utilisons la **Malle**.

Une malle est un contenant robuste que l\'on ouvre pour y puiser des outils spécifiques. Ce choix de mot renforce le caractère **narratif** et **humain** du langage.

- **Concret** : On imagine un objet physique, pas un concept abstrait.

- **Ordonné** : Une malle est rangée, chaque outil y a sa place.

- **Français** : Un choix assumé pour marquer l\'identité unique d\'Ogma dans un monde de langages anglophones.

**2. Distinction Technique : Module vs Malle**

Il est crucial de ne pas confondre l\'unité de code et l\'unité de distribution.

+-------------+-------------------------------+-----------------------------------------------------+
| **Concept** | **Nature**                    | **Portée**                                          |
+=============+===============================+=====================================================+
| **Module**  | Unité de code interne         | Un fichier .ogma local.                             |
+-------------+-------------------------------+-----------------------------------------------------+
| **Malle**   | Unité de distribution externe | Un ensemble de modules versionnés et réutilisables. |
+-------------+-------------------------------+-----------------------------------------------------+

**3. Intégration dans l\'Écosystème (Vision 0.5+)**

Bien que les malles soient interdites en 0.3.11 pour garantir la stabilité du noyau, leur intégration suivra les principes de \"Zéro Magie\".

**3.1 Gestion Déclarative**

L\'ajout d\'une malle se fera via un bloc dialecte dans un fichier de configuration (ex: ogma.toml ou malle.ogma) :

Extrait de code

// Configuration des dépendances

malles: \[

json version \"1.0.2\"

crypto version \"2.1.0\"

ui_kit version \"0.5.0\"

\]

**3.2 Chargement dans le Code**

L\'importation utilisera la syntaxe par point, cohérente avec le reste du langage :

Extrait de code

import malle.json

import malle.crypto as cr

main: fn() {

donnees: string = \"{\'id\': 1}\"

objet: object = json.parse(donnees)

}

**4. Pourquoi ce choix est stratégique ?**

- **Identité visuelle forte** : \"Ouvrir une malle\" est une métaphore que n\'importe qui comprend instantanément.

- **Différenciation** : Ogma ne cherche pas à imiter Rust ou Python. Il impose son propre vocabulaire, plus proche de l\'artisanat que de l\'industrie lourde.

- **Narrativité** : Le code devient une histoire : *\"J\'importe la malle de dessin, je prépare mon canevas, et je trace un cercle.\"*

**5. Résumé de la Terminologie Ogma**

- **Module** : Le fichier (l\'outil).

- **Malle** : La boîte (la collection d\'outils).

- **Dialecte** : La notice (comment utiliser les outils).

- **Objet** : La matière (ce que l\'on manipule).

**Conclusion sur les Malles**

La Malle n\'est pas qu\'un mot, c\'est une promesse de **simplicité**. En 0.5, quand tu ouvriras une malle Ogma, tu sauras exactement ce qu\'il y a dedans, sans dépendances cachées et sans complexité inutile.
