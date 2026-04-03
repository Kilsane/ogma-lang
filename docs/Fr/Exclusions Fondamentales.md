# 📘 OGMA --- Liste des Fonctionnalités Volontairement Exclues

**Version 0.3.11** --- *Le Manifeste de la Soustraction Réfléchie*

Ogma se définit par ses choix radicaux. Pour garantir un langage **narratif, explicite et déterministe**, nous avons choisi d\'exclure des mécanismes pourtant standards afin d\'éliminer toute trace de \"magie\" ou d\'ambiguïté.

**1. La Surcharge d'Opérateurs et de Fonctions**

**Pourquoi c'est exclu :**

- **Comportements implicites** : Le compilateur ne doit pas décider du sens d\'un symbole selon le contexte.

- **Lisibilité** : Un humain doit savoir exactement quelle fonction est appelée sans connaître par cœur la hiérarchie des types.

**Position d'Ogma :**

Privilégier des fonctions nommées. math.add pour les nombres, string.concat pour le texte. **Une fonction = Un nom = Un comportement.**

**2. L'Héritage de Classes (OOP Classique)**

**Pourquoi c'est exclu :**

- **Fragilité** : Les hiérarchies rigides créent des dépendances invisibles et des effets de bord complexes.

- **Complexité** : L\'héritage obscurcit le flux narratif naturel du code.

**Position d'Ogma :**

Favoriser la **composition**. Si un objet a besoin d\'une capacité, il possède un composant qui la détient. C\'est plat, traçable et robuste.

**3. L\'Exclusion du mot-clé pub (Le dogme de la visibilité)**

**Pourquoi c'est exclu :**

- **Le Bruit Visuel** : Dans les langages \"privés par défaut\" (Rust, Java), les développeurs écrivent pub ou public dans **85% des cas**. Cela devient un signal inutile qui pollue la lecture.

- **L\'Inversion de la philosophie** : En Ogma, on partage par défaut. On restreint seulement si c\'est nécessaire.

**Position d'Ogma :**

**Tout est public.**

- Pour signaler qu\'une donnée est interne ou sensible, on utilise la **convention visuelle du préfixe \_** (ex: \_secret_key).

- L\'IA et l\'humain comprennent immédiatement l\'intention (\"Ne pas toucher\") sans verrou syntaxique lourd.

- Cela simplifie radicalement le compilateur (moins de règles de portée à vérifier).

**4. L'Inférence de Type Agressive**

**Pourquoi c'est exclu :**

- **Invisibilité de l\'intention** : Si tout est automatique, le lecteur perd le contrat passé par le programmeur.

- **Risque d\'erreur** : Une erreur de type peut se propager sur des centaines de lignes avant d\'être détectée.

**Position d'Ogma :**

Le typage est manifeste pour les déclarations : nom: type = valeur. L\'inférence n\'est qu\'un outil d\'aide secondaire pour les expressions complexes.

**5. Les Exceptions Implicites (try/catch)**

**Pourquoi c'est exclu :**

- **Brisure du flux** : Une exception est un \"GOTO\" caché qui saute par-dessus la logique du programme.

- **Coût Runtime** : La gestion des piles d\'exception est lourde et imprévisible.

**Position d'Ogma :**

La gestion d\'erreur est **explicite**. L\'erreur est une donnée (un retour de fonction) que l\'on traite dans le flux narratif normal.

**6. L'Opérateur Modulo %**

**Pourquoi c'est exclu :**

- **Non-déterminisme** : Le résultat de -5 % 3 varie selon les processeurs et les langages.

- **Cryptique** : Le symbole % n\'est pas narratif.

**Position d'Ogma :**

Utiliser math.remainder(x, y). C\'est une instruction nommée, stable et dont le comportement est identique partout.

**7. Le mot-clé switch**

**Pourquoi c'est exclu :**

- **Pièges historiques** : Le fallthrough (passage au cas suivant) est une source de bugs constante.

- **Structure lourde** : Introduit des règles de saut qui brisent la lecture linéaire.

**Position d'Ogma :**

Utiliser l\'enchaînement if / else if / else. C\'est la forme pure de la narration logique : \"Si ceci, sinon si cela, sinon le reste.\"

**8. Les Tuples (a, b)**

**Pourquoi c'est exclu :**

- **Anonymat destructeur** : data.0 ne raconte rien à l\'humain.

- **Confusion syntaxique** : Les parenthèses sont réservées aux fonctions et aux mathématiques.

**Position d'Ogma :**

Utiliser un object. Nommer les champs (point.x, point.y) rend le code auditable et pérenne.

**9. Les Macros et la Métaprogrammation**

**Pourquoi c'est exclu :**

- **Code Fantôme** : Les macros créent du code invisible. Ce que vous lisez n\'est pas ce qui s\'exécute.

- **Instabilité** : Menace la stabilité syntaxique du langage.

**Position d'Ogma :**

Utiliser les **Dialectes \[ \]**. Ils permettent d\'étendre le langage de façon structurée, visible et sans magie syntaxique.

**10. Les Valeurs Globales Implicites**

**Pourquoi c'est exclu :**

- **Effets de bord** : N\'importe quel module peut altérer l\'état du programme sans prévenir.

- **Parallélisation impossible** : Les globales empêchent une gestion saine du multithreading.

**Position d'Ogma :**

Privilégier les **Constantes** (const) pour le partage immuable. Les données mutables doivent être passées explicitement.

**11. Les Closures (Captures implicites)**

**Pourquoi c'est exclu :**

- **Mémoire opaque** : Les captures rendent l\'ownership difficile à suivre et nécessitent souvent un Garbage Collector.

**Position d'Ogma :**

Les fonctions sont pures. Elles reçoivent leurs données par paramètres. Pour

conserver un état, on utilise un object.

**12. Les Syntaxes Alternatives (\"Sucre Syntaxique\")**

**Pourquoi c'est exclu :**

- **Fragmentation** : Évite les débats de \"style\" (ex: 5 façons de faire une boucle).

- **Lisibilité** : Uniformise le code produit par toute la communauté.

**Position d'Ogma :**

**Une seule façon claire de faire chaque chose.**

**Conclusion : La Clarté par le Refus**

Ogma se construit par soustraction. En retirant les Switch, les Modulo, les Macros et les mots-clés de visibilité comme pub, nous garantissons un langage **AI-Ready** et **Human-Ready**. Un code Ogma ne contient aucune règle cachée : **ce que vous voyez est exactement ce qui est exécuté.**
