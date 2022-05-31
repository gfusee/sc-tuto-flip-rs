# ElrondFlip

This repository is a tutorial about building a smart contract on Elrond.

Links to tutorial :

🇫🇷 - https://twitter.com/gfusee33/status/1515011670732677132

🇺🇸 - COMING SOON


# Tutoriel en français

## INTRODUCTION : Créer une dApp de flip sur Elrond

L'application que l'on va développer ensemble est un flip : un "**quitte ou double**" où un joueur a une chance sur deux de doubler sa mise.

Ce tutoriel sera divisé en 3 parties : 
- installation des outils de dev
- développement du smart contract
- développement de l'interface web

Ce que nous allons faire :

- Coder un smart contract
- Déployer le smart contract
- Coder l'interface web et intéragir avec le contract
- Parler de quelques bonnes pratiques

Ce que nous n'allons PAS faire :

- Coder de facon optimisée, on va préférer la lisibilité pour toucher les débutants
- Faire une interface jolie, le but de ce thread est de dev
- Déployer de facon propre & sécurisée le tout (s'il y a bcp de demande on peut se faire ca en bonus)

## PARTIE 1 : Setup de l'environnement de dev

Tout d’abord il vous faudra installer **erdpy**, il s’agit d’un outil vous permettant de compiler, tester et debuguer vos smart contracts, pour l’installer suivez la doc Elrond : [Elrond doc installing erdpy](https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/)

Une fois l’IDE installé on va lui rajouter des plugins, pour **VSCode** il vous faudra l’extension officielle d’Elrond: [VScode Elrond extension](https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide)

Pour **IntelliJ** on recommande le plugin Rust, pour l'installer vous allez au démarrage dans l'onglet "**Plugins**" et vous recherchez **Rust**.

On va tester l’installation rapidement en téléchargeant un contrat d’Elrond et en le compilant :

- Créez un nouveau dossier pour stocker les fichier du contract, puis ouvrez un terminal et positionnez-vous dans ce dossier avec la commande `cd <chemin du dossier>`

- Une fois dans le dossier lancez la commande `erdpy contract new adder --template adder`

- Si tout se passe bien 2 dossiers vont se créer : **adder** et **erdjs-snippets**

- Placez-vous dans le dossier adder (commande `cd adder`) et lancez la commande `erdpy contract build` qui va avoir pour effet de compiler le contrat.

- Laissez tourner, si à la fin vous voyez **WASM file generated: blablabla** alors votre installation tourne niquel et vous êtes prêts pour la partie 2 où nous allons coder le contrat.

## PARTIE 2A: Réflexions fonctionnelles

Vous avez votre environnement de prêt? Parfait car nous n'allons pas encore coder.
On va se poser calmement et faire un petit cahier des charges de ce que notre contrat fera, comment et avec quelles précautions.

On va donc développer un contrat de flip, on aimerait plusieurs choses, tout d'abord que lorsqu'un joueur mise il ait une chance sur deux de doubler (la base du jeu donc).

On souhaite aussi prendre des frais (en % de la mise) sur le montant doublé en cas de victoire.

L'argent qui va être remporté par les joueurs gagnants n'apparaît pas par magie, il faudra que nous l'alimentions nous-même

Imaginons que nous alimentions avec 5 EGLD, avec 5 flips gagnants consécutifs de 1 EGLD le contrat serait à court de liquidité.

5 flips sur 5 gagnants = 3.13% de chances (loi binomiale)

Il faut donc faire jouer la loi faible des grands nombres en jouant sur deux leviers : la quantité d'EGLD que nous allons donner au contrat et la mise maximale autorisée (on va ici mettre 10% et max 1 EGLD).

Autre chose importante on doit éviter à tout prix d'effectuer la mise d'un joueur ET la génération de l'aléatoire pour le flip dans le même bloc afin de ne pas se prendre une attaque dans la gueule.

Imaginez que le flip se fasse dans la même tx que celle où le joueur mise, il suffirait à un joueur malveillant d'avoir un clone identique de la blockchain, de tester sa tx sur ce clone et de soumettre la transaction à la vraie blockchain uniquement si le résultat est gagnant.

Pour faire simple sur Elrond les "nombres aléatoires" sont possibles contrairement à Ethereum, si on est au bloc N les nombres aléatoires des blocs N+1, N+2, etc... sont imprévisibles car ils dépendent de la signature des validateurs des blocs précédents.

Mais lorsque nous sommes au bloc N les nombres aléatoires de ce même bloc N sont prévisibles et calculables et heureusement, sinon comment prouver qu'un noeud n'est pas malveillant si on peut pas recalculer le résultat d'une transaction?

Un oracle permettrait aussi évidemment de contourner le problème mais cette solution rendrait ce thread bien trop compliqué.

On va résoudre ce problème en faisant le flip en deux transactions, la première où le joueur place sa mise et la deuxième où le flip sera réalisé.

Mais petite subtilité, n'importe qui pourra faire la deuxième transaction qui générera l'aléatoire du flip
Pour inciter d'autres utilisateurs (joueur ou non) à faire cette transaction on va les rémunérer avec % de la mise!

En clair si joueur A place 1 EGLD au bloc N, dès le bloc N-1 un utilisateur B pourra générer son flip et touchera un % de la mise de 1 EGLD
Évidemment premier arrivé premier servi afin de ne pas laisser le temps de tester sur une blockchain clonée.

## PARTIE 2B: Initialisation du projet

On va commencer par se placer avec le terminal dans le dossier où vous aller créer le projet, dans mon cas `~/Documents/Elrond` puis on va lancer la commande `erdpy contract new flip –template empty`, un nouveau dossier “**flip**” va apparaître.

Ouvrir le projet avec votre IDE.

Renommer `/flip/src/empty.rs` → `/flip/src/lib.rs` en utlisant l’outil **Refactor**

![IDE refactor](tutorial/partie2b_1.jpg)

Ensuite dans notre fichier `lib.rs`, on va renommer le **EmptyContract** en **FlipContract**, pareil on va pas le faire à la main mais utiliser l’outil de refactor de notre IDE:

![IDE rename](tutorial/partie2b_2.jpg)

Maintenant on va changer la version du compilateur Elrond, on va se fixer une version afin que personne ne soit perdu, imaginez si quelqu’un lit ce thread dans 3 mois et que des mises à jour du framework ont changé la façon de coder cette personne sera complètement perdue.

J’ai choisi la version `0.30.0.` car c’est la version la plus à jour au moment où je code ce contrat

Petit update de dernière minute: la version `0.31.1` est sortie, on ne va pas l’utiliser pour ce thread mais je vous encourage évidemment à l’utiliser dans vos projets.

Pour changer la version on va dans le fichier `Cargo.toml` et on change les version d’`elrond-wasm-XXX` pour mettre la `0.30.0`.

```aidl
[package]
name = "flip"
version = "0.0.0"
authors = [ "you",]
edition = "2018"
publish = false

[lib]
path = "src/lib.rs"

[dev-dependencies]
num-bigint = "0.4.2"

[dependencies.elrond-wasm]
version = "0.30.0"

[dev-dependencies.elrond-wasm-debug]
version = "0.30.0"
```

Et on fait la même modification dans les `Cargo.toml` des dossiers `wasm` et `meta`, à ce stade là si on compile avec la commande `erdpy contract build` tout devrait bien se passer.

Le projet est setup! 

