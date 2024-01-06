# Frakt

1. [Introduction](#introduction)
2. [Lancement](#lancement)
3. [Objectif](#objectif)
4. [Modalités](#modalites)

## Introduction
- **Classe** : ESGI 4AL2
- **Groupe 4** :
  - LAHMADI Zakarya
  - MALHA Marc
  - ZERGUINE Mohammed Mazene
  - ZHU Loïc

## Lancement
- Compiler et lancer les tests unitaires & d'intégration  :
  ```
  cargo test
  ```
- Lancer un client avec ou sans serveur :
  ```
  cargo run -- worker localhost
  ```
- Voir la _rustdoc_ :
  ```
  cargo doc --open
  ```
- Générer / mettre à jour la _rustdoc_ :
  ```
  ./generate_rustdoc.sh
  ```
  
## Objectif
- [x] Définir un "_crate_" pour:
  - [x] Le travailleur
  - [x] Le serveur
  - [x] Les éléments communs au travailleur et au serveur
  - [x] Les opérations mathématiques sur nombres complexes

- [ ] Réaliser un petit serveur pour tester un travailleur.
- [ ] Travailleur :
  - [x] Lancement d'un travailleur avec `worker [server_address]`
  - [ ] Gérer plusieurs définitions de fractales (en commençant par les ensemble de Julia)
  - [ ] Avoir un résultat :
    - [ ] effectuer un rendu en local
    - [ ] sauvegarder le résultat dans une image

## Modalites
- [x] Code régie par `rustfmt`
- [ ] Documentation README
- [x] Code fonctionnel sous:
  - [x] Linux
  - [x] Windows
  - [x] MacOS
- [ ] Code testé :
  - [ ] Tests unitaires
  - [ ] Tests d'intégration

## Bonus
- [ ] 💥 Réussir à faire crasher le serveur de référence
- [ ] Utilisation d'un fichier externe pour recharger des configurations ou pour sauvegarder celle courante.
- [ ] Optimisation spécifique (parallélisme ou vectorisation ou exploitation de méthodes avancées de calcul comme la méthode des perturbations)
- [ ] Réaliser une interface pour :
  - [ ] le travailleur
  - [ ] et/ou le serveur.
- [ ] Ajouter une intégration continue testant :
  - [ ] travailleur
  - [ ] serveur
- [ ] Réduire au maximum :
  - _warnings_ de compilation
  - `mut`
  - `unwrap()`, `expect()`, `panic!()`

## Organisation du travail
TODO

## Demarche des composants du projet
TODO