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
