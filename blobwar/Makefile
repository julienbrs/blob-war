all: blobwar

blobwar: build-release     # Crée une cible nommée "blobwar" qui dépend de "build-release"
	cargo run --release --bin blobwar  # Exécute la commande pour construire et exécuter l'application "blobwar" en mode release

client: build-release     # Crée une cible nommée "client" qui dépend de "build-release"
	cargo run --release --bin client $(IP)  # Exécute la commande pour construire et exécuter le client de l'application en mode release, en passant l'adresse IP du serveur en argument

server: build-release     # Crée une cible nommée "server" qui dépend de "build-release"
	cargo run --release --bin server  # Exécute la commande pour construire et exécuter le serveur de l'application en mode release

bench: build-release     # Crée une cible nommée "bench" qui dépend de "build-release"
	cargo bench  # Exécute les tests de benchmark du projet en mode release

bench-time: build-release     # Crée une cible nommée "bench-time" qui dépend de "build-release"
	cargo bench time  # Exécute les tests de benchmark du projet en mode release et mesure le temps d'exécution

build-all:     # Crée une cible nommée "build-all"
	cargo build --all-targets  # Exécute la commande pour construire tous les binaires et bibliothèques du projet, en utilisant la configuration de compilation par défaut (debug)

build-release-all:     # Crée une cible nommée "build-release-all"
	cargo build --release --all-targets  # Exécute la commande pour construire tous les binaires et bibliothèques du projet en mode release
