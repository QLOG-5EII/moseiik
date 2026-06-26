# Moseiik

Projet de génération de mosaïque d’images en Rust.

## Fonctionnalités

- Génération d’une image mosaïque à partir d’une image cible et d’un ensemble de tuiles
- Support du SIMD (x86 / ARM)
- Support multi-thread

---

##  Tests

Le projet contient :

### Tests unitaires
- Vérification des fonctions de base (L1 distance, préparation des tuiles, préparation de l’image cible)
- Tests spécifiques selon l’architecture (x86 / ARM)

### Tests d’intégration
- Génération complète d’une mosaïque
- Comparaison via hash SHA256 de l’image produite avec une image de référence

---

##  Docker

Une image Docker est utilisée pour exécuter le projet et les tests.

- L’image contient le code source Rust et Cargo
- Les tests sont exécutés automatiquement via un `ENTRYPOINT`

Exécution :

```bash
docker build -t moseiik .
docker run --rm moseiik



