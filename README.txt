# RustRecon - Scanner de ports TCP

## Description du projet

Ce projet a été réalisé dans le cadre de l'apprentissage du langage Rust et de la programmation système. Il s'agit d'un outil de reconnaissance réseau (scanner de ports) capable d'analyser les ports ouverts sur une machine cible.

L'objectif principal était de mettre en pratique la gestion de la concurrence en Rust, notamment l'utilisation des threads et la synchronisation de données partagées.

## Fonctionnalités

L'application permet de :
* Scanner une adresse IP (IPv4) ou un nom de domaine (ex: google.com).
* Définir une plage de ports spécifique (début et fin).
* Utiliser le multithreading pour accélérer le scan (nombre de threads configurable).
* Identifier les services courants associés aux ports ouverts (HTTP, SSH, SQL, etc.).
* Configurer un temps limite (timeout) pour chaque tentative de connexion.

## Choix techniques

* Langage : Rust (Edition 2021).
* Parallélisme : Utilisation de la bibliothèque standard `std::thread` pour lancer plusieurs scanners simultanément.
* Synchronisation : Utilisation de `Arc` (Atomic Reference Counting) et `Mutex` pour partager de manière sécurisée le compteur de ports entre les différents threads.
* Arguments CLI : Utilisation de la bibliothèque `clap` pour parser les arguments de la ligne de commande.
* Réseau : Utilisation de `std::net::TcpStream` pour tenter les connexions TCP.

## Prérequis

Pour compiler et exécuter ce projet, vous devez avoir l'environnement Rust installé sur votre machine (Rustc et Cargo).

## Installation et Compilation

1. Clonez le dépôt ou téléchargez les sources.
2. Placez-vous dans le dossier racine du projet.
3. Compilez le projet en mode "release" pour optimiser les performances des threads :

   cargo build --release

L'exécutable sera généré dans le dossier `./target/release/rustrecon`.

## Utilisation

Vous pouvez lancer le programme directement via `cargo run`. Voici les différentes options disponibles :

### Syntaxe de base
   cargo run -- <CIBLE> [OPTIONS]

### Exemples de commandes

1. Scan basique (ports 1 à 1024 par défaut)
   Scanner sa propre machine (localhost) :
   cargo run -- 127.0.0.1

2. Scan d'une plage spécifique
   Scanner les ports 20 à 100 sur un serveur distant :
   cargo run -- scanme.nmap.org --start 20 --end 100

3. Scan rapide (hautes performances)
   Scanner une large plage avec 500 threads simultanés :
   cargo run -- 192.168.1.15 --start 1 --end 5000 --threads 500

4. Aide
   Afficher toutes les options disponibles :
   cargo run -- --help

## Avertissement légal

Ce logiciel a été développé à des fins purement éducatives pour comprendre le fonctionnement des réseaux et du multithreading. L'utilisation de scanners de ports sur des réseaux ou des machines dont vous n'êtes pas propriétaire ou pour lesquels vous n'avez pas d'autorisation explicite est illégale.

## Auteur

Projet développé par Franck POKAM et Daniel ORDUY REY.