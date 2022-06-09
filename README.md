# ElrondFlip

This repository is a tutorial about building a smart contract on Elrond.

Links to tutorial :

🇫🇷 - https://twitter.com/gfusee33/status/1515011670732677132

🇺🇸 - COMING SOON


# Tutoriel en français

## SOMMAIRE

- [INTRODUCTION : Créer une dApp de flip sur Elrond](#introduction)
- [PARTIE 2A: Réflexions fonctionnelles](#partie2a)
- [PARTIE 2B: Initialisation du projet](#partie2b)
- [PARTIE 2C: Storage du contrat](#partie2c)
- [PARTIE 2D: Administration du contrat](#partie2d)
- [PARTIE 2E: Logique de la mise d’un joueur](#partie2e)
- [PARTIE 2F: Résultat du flip](#partie2f)
- [PARTIE 2G: Bounty](#partie2g)
- [PARTIE 2H: Tests](#partie2h)
- [PARTIE 2: Récapitulatif](#partie2_recap)

## <a name="introduction"></a>INTRODUCTION : Créer une dApp de flip sur Elrond

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

## <a name="partie1"></a>PARTIE 1 : Setup de l'environnement de dev

Tout d’abord il vous faudra installer **erdpy**, il s’agit d’un outil vous permettant de compiler, tester et debuguer vos smart contracts, pour l’installer suivez la doc Elrond : [Elrond doc installing erdpy](https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/)

Une fois l’IDE installé on va lui rajouter des plugins, pour **VSCode** il vous faudra l’extension officielle d’Elrond: [VScode Elrond extension](https://marketplace.visualstudio.com/items?itemName=Elrond.vscode-elrond-ide)

Pour **IntelliJ** on recommande le plugin Rust, pour l'installer vous allez au démarrage dans l'onglet "**Plugins**" et vous recherchez **Rust**.

On va tester l’installation rapidement en téléchargeant un contrat d’Elrond et en le compilant :

- Créez un nouveau dossier pour stocker les fichier du contract, puis ouvrez un terminal et positionnez-vous dans ce dossier avec la commande `cd <chemin du dossier>`

- Une fois dans le dossier lancez la commande `erdpy contract new adder --template adder`

- Si tout se passe bien 2 dossiers vont se créer : **adder** et **erdjs-snippets**

- Placez-vous dans le dossier adder (commande `cd adder`) et lancez la commande `erdpy contract build` qui va avoir pour effet de compiler le contrat.

- Laissez tourner, si à la fin vous voyez **WASM file generated: blablabla** alors votre installation tourne niquel et vous êtes prêts pour la partie 2 où nous allons coder le contrat.

## <a name="partie2a"></a>PARTIE 2A: Réflexions fonctionnelles

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

## <a name="partie2b"></a>PARTIE 2B: Initialisation du projet

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

```rust
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

## <a name="partie2c"></a>PARTIE 2C: Storage du contrat

On va créer un nouveau fichier `storage.rs` dans le dossier `src`, ouvrez le fichier et sur **Intellij** vous devriez avoir un avertissement **File is not included in module tree, [...]**, sélectionnez **Attach file to lib.rs**

On va déclarer un module `StorageModule` dans le fichier `storage.rs`, pour vulgariser on peut voir un module comme une collection de code, ça permet de ne pas avoir un fichier `lib`.rs qui fait 30000 lignes.

On écrit pour cela le code suivant dans `storage.rs`

```rust
elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait StorageModule {

}
```

Il faut maintenant dire à notre contrat que notre module existe (je vulgarise très fortement en disant ça), dans notre fichier `lib.rs` on le fait ainsi :

```rust
#![no_std]

mod storage;

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait FlipContract:// ContractBase +
    storage::StorageModule
{
    #[init]
    fn init(&self) {}
}
```

On va aussi avoir besoin de stocker des types plus complexes que des nombres ou des chaînes de caractères, comme l’ensemble des infos d’un flip (l’adresse du joueur, le block sur lequel le flip est initié, etc...)

On voit dans ce code par exemple qu’un Flip contient un **id, l’adresse du joueur, le token du flip**, etc... (ouais j’ai oublié de préciser mais le flip pourra se faire sur d’autres tokens que EGLD lol).

On va donc créer un fichier `struct.rs` dans lequel nous allons déclarer nos types personalisés (nos structures).
On y place le code suivant:

```rust
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct Flip<M : ManagedTypeApi> {
    pub id: u64,
    pub player_address: ManagedAddress<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
    pub bounty: BigUint<M>,
    pub block_nonce: u64,
    pub minimum_block_bounty: u64
}
```

On va maintenant placer nos variables qui vont être stockées dans la blockchain (le storage).

On se place à l’intérieur de `StorageModule` dans `storage.rs` et on va y ajouter le code suivant :

```rust
#[view(getOwnerPercentFees)]
#[storage_mapper("owner_percent_fees")]
fn owner_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;

#[view(getBountyAmount)]
#[storage_mapper("bounty_percent_fees")]
fn bounty_percent_fees(&self) -> SingleValueMapper<Self::Api, u64>;
```

On a ici rajouté 2 variables qui vont déterminer :

- le % que nous allons prendre sur chaque flip
- le % que va prendre la personne qui va générer l’aléatoire (cf partie 1a)

On va appeler “bounty” l’action de gagner un % du flip en contrepartie de la génération de l’aléatoire.

Pour donner un ordre de grandeur, 100000000 = 100%, ainsi par exemple mettre `owner_percent_fees` à 5000000 c’est prendre 5% de frais.

On fait ainsi car les nombres à virgule (flottants) n’existent pas quand on dev un smart contract, diviser par 100000000 au lieu de 100 permet de faire des frais plus précis comme 1.57% par exemple.

Maintenant nous allons rajouter les variables pour limiter la mise maximale :

```rust
#[view(getMaximumBet)]
#[storage_mapper("maximum_bet")]
fn maximum_bet(
    &self,
    token_identifier: &TokenIdentifier<Self::Api>,
    token_nonce: u64
) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;

#[view(getMaximumBetPercent)]
#[storage_mapper("maximum_bet_percent")]
fn maximum_bet_percent(
    &self,
    token_identifier: &TokenIdentifier<Self::Api>,
    token_nonce: u64
) -> SingleValueMapper<Self::Api, u64>;
```

`maximum_bet` représente la mise maximale possible pour un certain token, `maximum_bet_percent` aussi mais en % (même échelle qu’au-dessus)  du nombre du token que possède le contrat.

Par exemple on met `maximum_bet` à 1 $EGLD et `maximum_bet_percent` à 10%, la mise maximale autorisée sera le + petit d’un des deux nombres suivants :

- 1 $EGLD
- 10% du nombre d’EGLD que possède le contrat

On va rajouter une variable qui détermine le nombre de blocks à attendre avant de pouvoir bounty un flip :

```rust
#[view(getMinimumBlockBounty)]
#[storage_mapper("minimum_block_bounty")]
fn minimum_block_bounty(&self) -> SingleValueMapper<Self::Api, u64>;
```

Puis une variable qui nous indique la réserve d’un token disponible pour les flips:

```rust
#[view(getTokenReserve)]
#[storage_mapper("token_reserve")]
fn token_reserve(
    &self,
    token_identifier: &TokenIdentifier<Self::Api>,
    token_nonce: u64
) -> SingleValueMapper<Self::Api, BigUint<Self::Api>>;
```

Vous allez me dire qu’on pourrait juste récupérer la balance du token pour notre contrat, et bien ça ne marcherait pas vraiment.

En effet, un flip se fait en 2 temps, la mise puis l’execution via le bounty, mais entre ces deux moments il faut bloquer l’argent afin d’avoir de quoi payer en cas de victoire du joueur, c’est à ça que sert cette variable

Intuitivement, `token_reserve = balance - token bloqués`

Et pour finir on va rajouter 3 variables concernant notre flip :

```rust
#[view(flipForId)]
#[storage_mapper("flip_for_id")]
fn flip_for_id(&self, id: u64) -> SingleValueMapper<Self::Api, Flip<Self::Api>>;

#[view(getLastFlipId)]
#[storage_mapper("last_flip_id")]
fn last_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;

#[view(getLastBountyFlipId)]
#[storage_mapper("last_bounty_flip_id")]
fn last_bounty_flip_id(&self) -> SingleValueMapper<Self::Api, u64>;
```

`flip_for_id` contient les infos sur un flip (notre struct Flip déclarée un peu + haut)

Parlons rapidement des deux autres variables:

`last_flip_id` représente l’id du dernier flip fait, on fait +1 à chaque fois qu’un joueur place une mise.

`last_bounty_flip_id` représente le dernier flip pour lequel a déjà eu lieu l’exécution.

Lorsque quelqu’un va vouloir bounty, il ne va pas générer l’aléatoire pour un flip mais pour tous les flips entre `last_bounty_flip_id` et `last_flip_id` (en prenant en compte minimal_block_bounty) en one shot (et donc plusieurs rewards d’un coup).

## <a name="partie2d"></a>PARTIE 2D: Administration du contrat

On va créer un module `AdminModule` dans un nouveau fichier `admin.rs` dans le dossier `src`.

Notez bien que nous lui “**indiquons l’existence**” de `StorageModule` (je vulgarise ne me tombez pas dessus lol)

```rust
use crate::storage;
elrond_wasm::imports!();

#[elrond_wasm::derive::module]
pub trait AdminModule:// ContractBase +
    storage::StorageModule
{
    
}
}
```

Et on ajoute ce module à notre contrat dans `lib.rs`

```rust
#![no_std]

mod storage;
mod admin;
mod structs;

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait FlipContract:// ContractBase +
    storage::StorageModule + admin::AdminModule
{
    #[init]
    fn init(&self) {}
}
```

On se replace dans `AdminModule` puis on va tout d’abord créer un endpoint pour augmenter la réserve totale d’un token:

```rust
#[payable("*")]
    #[endpoint(increaseReserve)]
    fn increase_reserve(
        &self,
        #[payment_token] payment_token: TokenIdentifier<Self::Api>,
        #[payment_nonce] payment_nonce: u64,
        #[payment_amount] payment_amount: BigUint<Self::Api>
    ) {

        require!(
            payment_amount > 0u64,
            "zero payment"
        );

        self.token_reserve(
            &payment_token,
            payment_nonce
        ).update(|reserve| *reserve += payment_amount);

    }
```

Rien de fou mais notez qu’on ne sécurise pas cet endpoint par only_owner, si un utilisateur lambda veut gentillement nous donner de l’argent on accepte évidemment.

On rajoute un endpoint pour récupérer la réserve de token non utilisée:

```rust
#[only_owner]
#[endpoint(withdrawReserve)]
fn withdraw_reserve(
    &self,
    token_identifier: TokenIdentifier<Self::Api>,
    token_nonce: u64,
    amount: BigUint<Self::Api>
) {
    let token_reserve = self.token_reserve(
        &token_identifier,
        token_nonce
    ).get();

    require!(
        amount <= token_reserve,
        "amount too high"
    );

    self.send()
        .direct(
            &self.blockchain().get_caller(),
            &token_identifier,
            token_nonce,
            &amount,
            &[]
        );
}
```

Cet endpoint est évidemment sécurisé `only_owner` (seul l’owner peut l’utiliser) et le require est important, sans ce dernier on pourrait rug avec l’argent des flip en cours.

Dans ce monde faites confiance au code, pas aux humains.

Ensuite on va rajouter 3 endpoints 
- `only_owner` pour changer `maximum_bet`, 
- `maximum_bet_percent`
- `minimum_block_bounty`

```rust
#[only_owner]
#[endpoint(setMaximumBetPercent)]
fn set_maximum_bet_percent(
    &self,
    token_identifier: TokenIdentifier<Self::Api>,
    token_nonce: u64,
    percent: u64
) {

    require!(
        percent > 0u64,
        "percent zero"
    );

    self.maximum_bet_percent(
        &token_identifier,
        token_nonce
    ).set(percent);

}

#[only_owner]
#[endpoint(setMaximumBet)]
fn set_maximum_bet(
    &self,
    token_identifier: TokenIdentifier<Self::Api>,
    token_nonce: u64,
    amount: BigUint<Self::Api>
) {

    require!(
        amount > 0u64,
        "amount zero"
    );

    self.maximum_bet(
        &token_identifier,
        token_nonce
    ).set(amount);

}

#[only_owner]
#[endpoint(setMinimumBlockBounty)]
fn set_minimum_block_bounty(
    &self,
    minimum_block_bounty: u64
) {

    require!(
        minimum_block_bounty > 0u64,
        "minimum_block_bounty zero"
    );

    self.minimum_block_bounty().set(minimum_block_bounty);

}
```

C’est tout pour la partie administration qui est relativement simple et courte.

La prochaine étape va être de faire le code qui va gérer le flip et le bounty, ça va être un GROS morceau.

## <a name="partie2e"></a>PARTIE 2E: Logique de la mise d’un joueur

Tout est prêt pour coder le coeur du contrat : **la logique de la mise d’un joueur**.

On se place dans le fichier `lib.rs` (l’ancien `empty.rs` qu’on a renommé dans la partie 2b si vous vous rappelez) et on va y ajouter une constante `HUNDRED_PERCENT` qui représente l’échelle de nos % (cf partie 2b).

```rust
#![no_std]


const HUNDRED_PERCENT: u64 = 100_000_000;

mod storage;
mod structs;
mod admin;

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait FlipContract:// ContractBase +
    storage::StorageModule + admin::AdminModule
{
    #[init]
    fn init(&self) {}
}
```

On va se placer dans le trait `FlipContract` (et on va y rester qq temps) et on va remplacer la fonction d’initialisation du contrat:

```rust
#[init]
fn init(
    &self,
    owner_percent_fees: u64,
    bounty_percent_fees: u64,
    minimum_block_bounty: u64
) {
    self.owner_percent_fees().set(owner_percent_fees);
    self.bounty_percent_fees().set(bounty_percent_fees);

    require!(
        minimum_block_bounty > 0u64,
        "minimum_block_bounty is zero"
    );

    self.minimum_block_bounty().set(minimum_block_bounty)
}
```

La fonction annotée `#[init]` est une fonction qui va être appelée dans deux cas, au premier déploiement du contrat et à chaque mise à jour, dans cette fonction nous allons initialiser les valeurs d’administration.

Nous allons maintenant nous attaquer à l’endpoint `flip` qui va permettre à un joueur de placer sa mise, on commencer par le déclarer:

```rust
#[payable("*")]
#[endpoint]
fn flip(
    &self,
    #[payment_amount] payment_amount: BigUint<Self::Api>,
    #[payment_token] payment_token: TokenIdentifier<Self::Api>,
    #[payment_nonce] payment_nonce: u64
) {


}
```

On se place maintenant à l’intérieur de l’endpoint `flip` (et on y restera jusqu’à nouvel ordre), on va commencer par faire quelques vérifications:

```rust
let token_reserve = self.token_reserve(
    &payment_token,
    payment_nonce
).get();

require!(
    token_reserve > 0u64,
    "no token reserve"
);

require!(
    !self.maximum_bet(&payment_token, payment_nonce).is_empty(),
    "no maximum bet"
);

require!(
    !self.maximum_bet_percent(&payment_token, payment_nonce).is_empty(),
    "no maximum bet percent"
);
```

La première vérification consiste à vérifier que le contrat dispose bien de fonds pour effectuer un flip pour le token cible.

cette vérification sera un échec si par exemple on a pas appelé `increaseReserve` dans notre fichier `admin.rs` (cf partie 2d) ou si le contrat n’a plus de liquidité suite à une série de victoires.

Les deux vérifications suivantes vérifient que nous avons bien précisé les valeurs de mises maximales pour le token ciblé (cf parties 2c et 2d)

On va maintenant vérifier que l’utilisateur n’a pas mis une mise trop importante par rapport à la liquidité que possède notre contrat (via `maximum_bet_percent`) ou la mise maximale que nous avons autorisée (via `maximum_bet`)

On commence par calculer la mise maximale autorisée, on met à la suite du code précédent.

```rust
let maximum_bet = self.maximum_bet(
    &payment_token,
    payment_nonce
).get();

let maximum_bet_percent = self.maximum_bet_percent(
    &payment_token,
    payment_nonce
).get();

let max_allowed_bet = min(
    maximum_bet,
    token_reserve * &BigUint::from(maximum_bet_percent) / HUNDRED_PERCENT
);
```

Si min est en rouge c’est que la fonction n’est pas importée, pour ce faire passez le curseur dessus et sélectionnez “Import”.

Puis les mises qui iront à nous et au bounty via les frais, et la mise réelle du joueur (frais exclus), toujours à la suite du code précédent

```rust
let owner_profits = &payment_amount * &BigUint::from(self.owner_percent_fees().get()) / HUNDRED_PERCENT;
let bounty = &payment_amount * &BigUint::from(self.bounty_percent_fees().get()) / HUNDRED_PERCENT;
let amount = &payment_amount - &bounty - &owner_profits;
```

La première valeur ira directement à l’owner qq lignes plus tard (on y viendra), la deuxième valeur sera la récompense pour la personne qui va générer l’aléatoire  et la troisième valeur est la mise réelle du joueur (celle qui sera doublée en cas de victoire)

On peut maintenant vérifier que la mise ne dépasse pas le maximum autorisé, encore à la suite du code précédent

```rust
require!(
    amount <= max_allowed_bet,
    "too much bet"
);
```

A partir de maintenant toutes les vérifications sont faites, on a plus qu’à agir, on calcule tout d’abord le nouvel id de notre flip (qui vaut l’id du dernier flip + 1)

```rust
let last_flip_id = if self.last_flip_id().is_empty() {
    0u64
} else {
    self.last_flip_id().get()
};

let flip_id = last_flip_id + 1;
```

Et on instancie les infos du flip

```rust
let flip = Flip {
    id: flip_id,
    player_address: self.blockchain().get_caller(),
    token_identifier: payment_token.clone(),
    token_nonce: payment_nonce,
    amount: amount.clone(),
    bounty: bounty.clone(),
    block_nonce: self.blockchain().get_block_nonce(),
    minimum_block_bounty: self.minimum_block_bounty().get()
};
```

Si Flip est en rouge c’est qu’il n’est pas importé, pour ce faire passez le curseur dessus et sélectionnez “Import”

On bloque le montant de la mise en retirant le montant de `token_reserve`:

```rust
self.token_reserve(
    &payment_token,
    payment_nonce
).update(|reserve| *reserve -= &amount);
```

Et on s’envoie directement les frais du flip, on a pas besoin d’attendre que le flip soit effectué:

```rust
self.send()
    .direct(
        &self.blockchain().get_owner_address(),
        &payment_token,
        payment_nonce,
        &owner_profits,
        &[]
    );
```

Tout s’est bien passé, plus qu’à inscrire dans le storage l’existence du flip:

```rust
self.flip_for_id(flip_id).set(flip); 
self.last_flip_id().set(flip_id);
```

On a terminé l’endpoint “flip” ! A ce stade notre utilisateur peut placer sa mise et les frais sont calculés, la suite consiste à écrire le code qui permet de réaliser le flip.

## <a name="partie2f"></a>PARTIE 2F : Résultat du flip

Nous avons notre flip d’initialisé, nous allons dans cette partie le réaliser

On se place dans le trait `FlipContract` (fichier `lib.rs`) et nous allons créer une fonction `make_flip`

Cette partie sera donc consacré à l'écriture de cette fonction.

```rust
fn make_flip(
    &self,
    bounty_address: &ManagedAddress<Self::Api>,
    flip: &Flip<Self::Api>
) {
    
    }
}
```

Cette fonction, qui n’est pas un endpoint, aura comme objectif de réaliser un flip en faisant trois actions :

- Déterminer si le flip est gagnant ou perdant
- Envoyer le gain au joueur en cas de victoire ou augmenter `token_reserve` en cas de défaite
- Envoyer la récompense à celui qui génère l’aléatoire (`bounty_address`)

On commence par déterminer si le flip est gagnant:

```rust
let mut rand_source = RandomnessSource::<Self::Api>::new();
let random_number = rand_source.next_u8_in_range(0, 2);
let is_win = random_number == 1u8;
```

Le code ci-dessus va générer un nombre aléatoire entre 0 et 2 (exclus) donc soit 0 soit 1, on considère que le flip est gagné si ce nombre vaut 1, on a donc bien une chance sur deux de gagner.

On va maintenant gérer les transferts d’argent en fonction du résultat:

```rust
if is_win {
    self.send()
        .direct(
            &flip.player_address,
            &flip.token_identifier,
            flip.token_nonce,
            &profit_if_win,
            &[]
        );
} else {
    self.token_reserve(
        &flip.token_identifier,
        flip.token_nonce
    )
        .update(|reserve| *reserve += &profit_if_win);
}
```

Dans le cas d’une défaite nous ajoutons le profit (= double du montant du flip) à la réserve, pourquoi le montant doublé et pas juste le montant ?

Tout simplement car au moment où un joueur place sa mise (cf partie précédente), le smart contract reçoit dans sa balance le montant MAIS on ne l’ajoute pas à la réserve

Mais non seulement on ajoute pas le montant à la réserve mais en + on le retire, il faut donc en cas de défaite ajouter à la réserve deux fois le montant

Pour terminer on envoie le bounty à bounty_address (cf partie suivante où on créera l’endpoint qui appelle notre fonction `make_flip`)

Et on enlève du storage le flip qui vient d’être réalisé:

```rust
self.send()
    .direct(
        &bounty_address,
        &flip.token_identifier,
        flip.token_nonce,
        &flip.bounty,
        &[]
    );

self.flip_for_id(flip.id).clear();
```

Voilà qui termine cette petite partie sur le résultat du flip, prochaine partie nous allons gérer le bounty.

## <a name="partie2g"></a>PARTIE 2G : Bounty

On sait maintenant comment réaliser le flip, il reste juste à créer l’endpoint associé

Normalement il s’agit de l’avant dernier thread de la partie **smart contract** de cette longue série.

On va rajouter un endpoint, toujours dans `FlipContract` dans le fichier `lib.rs`, qu’on va appeler `flip_bounty`

```rust
#[endpoint(flipBounty)]
fn flip_bounty(
    &self
) {}
    
```

Cette endpoint va être appelé par n’importe quelle adresse et aura comme objectif de :

- réaliser tous les flips en attente d’un coup
- donner le bounty de chaque flip à la personne qui a appelé l’endpoint

On va commencer par interdire aux smarts contracts d’appeler cet endpoint afin de rendre la prédiction des nombres aléatoires plus difficile:

```rust
let caller = self.blockchain().get_caller();

require!(
    !self.blockchain().is_smart_contract(&caller),
    "caller is a smart contract"
);
```

On va ensuite récupérer l’id du dernier flip réalisé et l’id du dernier flip en attente.

Par exemple si l’id du dernier flip réalisé est 3 et l’id du dernier flip en attente est 8 on va tenter de réaliser les flips 4, 5, 6, 7 et 8.

Je dis tenter car supposons que les flips 7 et 8 ont été réalisés dans le même bloc que l’appel de `flip_bounty` on ne va PAS les réaliser, je vous renvoie à la partie 2A.

On rajoute donc le code suivant:

```rust
let last_bounty_flip_id = self.last_bounty_flip_id().get();
let last_flip_id = self.last_flip_id().get();

require!(
    last_bounty_flip_id < last_flip_id,
    "last bounty flip id >= last flip id"
);
```

On va initialiser l’itération sur tous les flips potentiels à réaliser:

```rust
let current_block_nonce = self.blockchain().get_block_nonce();

let mut bounty_flip_id = last_bounty_flip_id;

while bounty_flip_id < last_flip_id {
    
    // On va ecrire du code ici

    bounty_flip_id += 1u64;
}
```

A chaque itération on va :

- Récupérer les infos du flip
- Vérifier si le flip est sur un bloc antérieur au bloc actuel
- Réaliser le flip (via `make_flip` du thread précédent)


J’attire rapidement votre attention sur le deuxième point, si on doit réaliser les flips 5 à 30 et que le flip 10 est sur le même bloc que le bloc courant, alors les flips 11 à 30 aussi

On pourra donc casser la boucle immédiatement

Voilà à quoi ressemble notre boucle maintenant:

```rust
while bounty_flip_id < last_flip_id {
    let flip_id = bounty_flip_id + 1u64;

    if self.flip_for_id(flip_id).is_empty() {
        break;
    }

    let flip = self.flip_for_id(flip_id).get();

    if current_block_nonce < flip.block_nonce + flip.minimum_block_bounty {
        break;
    }

    self.make_flip(
        &caller,
        &flip
    );

    bounty_flip_id += 1u64;
}
```

On sort de notre boucle et il nous reste que deux choses à faire :

- Renvoyer une erreur si aucun flip n’a pu être réalisé
- Enregistrer le dernier flip réalisé dans le storage

On rajoute donc après la boucle les deux lignes suivantes:

```rust
if bounty_flip_id == last_bounty_flip_id {
    sc_panic!("no bounty")
}

self.last_bounty_flip_id().set(bounty_flip_id);
```

Voilà qui termine ce thread et le code du smart contract, il nous reste une dernière chose à voir : les tests.


## <a name="partie2h"></a>PARTIE 2H : Tests

Le code du contrat est terminé, on va se poser tranquillement et parler un peu de tests

Jusqu’à présent on a codé mais on a jamais vérifié si le contrat fonctionnait, nous n’avons pas non plus déployé le contrat donc à l’heure actuelle nous avons un contrat qui compile mais qui pourrait très bien faire des conneries

Il faut donc maintenant s’assurer que le contrat fait bien ce qu’on attend de lui, même dans des conditions particulières

On a donc plusieurs façon de procéder, tout d’abord la méthode naïve qui consiste à déployer le contrat sur un testnet et de tester à la main de faire des flips, des bounty, des increaseReserve en tant qu’owner, increaseReserve sans être owner pour check qu’il y a une erreur, ..

Avantages :

- On peut vérifier si ça fonctionne

Désavantages :

- C’est long
- C’est chiant
- C’est risqué car erreurs de manip qui peuvent fausser les tests
- Il faut tout refaire à chaque modification du code pour vérifier qu’on a rien cassé


- On maîtrise que dalle (entre deux tests le testnet aura changé)
- On ne peut pas jouer sur la temporalité (perso attendre plusieurs blocs après une mise pour tester le bounty non merci)

Conclusion : oubliez cette façon de faire

Ensuite on a la méthode “naïve mais y a de l’idée” qui consiste à écrire un script qui va déployer et faire tous les tests

C’est comme la méthode naïve mais automatisée, globalement on a le même unique avantage et les mêmes désavantages sauf les erreurs de manip

Vient donc la bonne méthode : faire des tests dans un environnement maîtrisé via un framwork

On a sur Elrond deux frameworks de tests : [Mandos](https://docs.elrond.com/developers/mandos-reference/overview/) ou [Rust Testing Framework](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)

Le concept est simple : une blockchain va être simulée en local et les tests vont être lancés dans cet environnement qui sera TOUJOURS le même

On peut agir sur le storage, faire des avances rapides dans le temps, modifier les balances des adresses, etc…

Je vais personnellement utiliser mandos, préférence personnelle, les deux frameworks font la même chose en bout de chaîne donc il n’y a pas de mauvais choix

Je ne vais pas vous mettre le code des tests il sera disponible dans le thread suivant où je publierai tout le code du contrat

Je vais plutôt faire une liste non exhaustif des tests que j’ai réalisé afin que vous ayez une idée :

- Mise d’un joueur
- Bounty d’un flip gagnant
- Bounty d’un flip perdant
- Mise de plusieurs personnes sur des blocs différents
- Bounty de plusieurs flips dont certains sur le même bloc que le bloc courant

A titre personnel les tests représentent 70% de mon temps et 30% c’est coder le smart contract.

## <a name="partie2_recap"></a>PARTIE 2 : Récapitulatif

On a fini la partie smart contract du flip

On a un contrat de flip fonctionnel qui possède une petite sécurité contre les attaques sur les nombres aléatoires.

Cette sécurité consiste en bref à payer des utilisateurs autres que les joueurs pour nous fournir l'aléatoire, pour que ça fonctionne il faut donc un minimum de trafic.

Notre flip peut se faire sur n'importe quel token du moment que le contrat possède une réserve de ce token, la mise maximale dépend de cette réserve

La suite consiste à créer le site web qui va se connecter au contrat, on pourra aussi regarder pour créer un script de déploiement automatisé du contrat.
