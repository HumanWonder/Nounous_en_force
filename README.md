# Projet de mise en relation Crèches / Intervenants

Ce projet vise à faciliter la mise en relation entre des responsables de crèches et des intervenants/remplaçants via une plateforme web sécurisée.

---

## Sommaire

- [Stack Technique](#stack-technique)
- [Dépendances](#dépendances)
- [Installation et lancement en local](#installation-et-lancement-en-local)
- [Configuration des variables d'environnement](#configuration-des-variables-denvironnement)
- [Envoi d'e-mails](#envoi-de-mails)
- [Déploiement](#déploiement)
- [Tests à prévoir](#tests-à-prévoir)
- [Notes sur la structure du projet](#notes-sur-la-structure-du-projet)

---

## Stack Technique

### Frontend

- Next.js (TypeScript)
- Tailwind CSS
- Formulaires multi-étapes avec stockage local
- Authentification via cookies / localStorage (en test, switch prévu dès le déploiement en httpS)

### Backend

- Rust avec Actix-Web
- PostgreSQL avec Diesel ORM
- Authentification JWT + validation d'e-mail
- Cryptage avec Argon2
- Envoi de mails via SMTP (lettre)

---

## Dépendances

### Frontend (extrait de `package.json`)

```json
"dependencies" 
    "@hookform/resolvers": "^5.0.1",
    "@next/env": "^15.2.2",
    "@radix-ui/react-label": "^2.1.3",
    "@radix-ui/react-select": "^2.1.7",
    "@radix-ui/react-slot": "^1.2.0",
    "@tailwindcss/postcss": "^4.1.3",
    "class-variance-authority": "^0.7.1",
    "clsx": "^2.1.1",
    "js-cookie": "^3.0.5",
    "lucide-react": "^0.487.0",
    "next": "^15.1.7",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "react-hook-form": "^7.55.0",
    "shadcn": "2.4.0",
    "tailwind-merge": "^3.2.0",
    "zod": "^3.24.2"

"devDependencies"
    "@tailwindcss/forms": "^0.5.10",
    "@tailwindcss/typography": "^0.5.16",
    "@types/node": "22.14.0",
    "@types/react": "^19",
    "@types/react-dom": "^19",
    "autoprefixer": "^10.4.21",
    "postcss": "^8.5.3",
    "tailwindcss": "^4.1.3",
    "tailwindcss-animate": "^1.0.7",
    "typescript": "^5"
```

### Backend (extrait de `Cargo.toml`)

```toml
actix-cors = "0.7.0"
actix-rt = "2.10.0"
actix-service = "2.0.3"
# Framework web
actix-web = "4.9.0"
# Hashage des mots de passe
bcrypt = "0.17.0"
#Typage DateTime
chrono = { version = "0.4.39", features = ["serde"] }
# ORM (Mapping Object-Relationnel) pour PostgreSQL
diesel = { version = "2.2.7", features = ["postgres", "r2d2", "uuid", "chrono"] }
diesel_migrations = "2.2.0"
# Gestion des variables d'environnement
dotenv = "0.15.0"
# Gestion des JWT
jsonwebtoken = "9.3.1"
#Envoi de mail pour valider email
lettre = "0.11.14"
# Serialisation des données
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
# Async runtime
tokio = { version = "1.43.0", features = ["full"] }
uuid = {version = "1.15.0", features = [
    "v4",                # Lets you generate random UUIDs
    "fast-rng",          # Use a faster (but still sufficiently random) RNG
    "macro-diagnostics", # Enable better diagnostics for compile-time UUIDs
    "serde", 
]}
```

---

## Installation et lancement en local

### Prérequis

- Node.js ≥ 18
- Cargo (Rust)
- PostgreSQL
- Docker (facultatif pour les mails)

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Backend

```bash
cd backend
cp .env.example .env
diesel setup
cargo run
```

---

## Configuration des variables d'environnement

Créer un fichier `.env.local` à la racine du projet **frontend**, et un fichier `.env` dans le projet **backend**.

### Frontend (`.env.local`)

```env
NEXT_PUBLIC_API_URL=http://localhost:8080
```

### Backend (`.env`)

```env
DATABASE_URL=postgres://user:password@localhost:5432/creche_db
JWT_SECRET=ton_secret
SMTP_USER=adresse@tondomaine.fr
SMTP_PASSWORD=motdepasse
FRONTEND_URL=http://localhost:3000
```

---

## Envoi de mails

Pour tester localement les mails de validation/refus :

```bash
docker run -d -p 1025:1025 -p 8025:8025 mailhog/mailhog
```

Configurer dans `.env` :

```env
SMTP_HOST=localhost
SMTP_PORT=1025
```

---

## Déploiement

Ce projet est actuellement en phase de développement. Le déploiement n'est pas encore actif mais plusieurs options sont envisagées :

- **Frontend (Next.js)** : hébergé sur [Vercel](https://vercel.com) pour sa simplicité d'intégration avec GitHub et son support natif de Next.js.
- **Backend (Rust - Actix Web)** : déployé avec [Docker](https://www.docker.com/) sur un VPS (comme Hetzner, OVH ou Scaleway), avec HTTPS via Let's Encrypt (Certbot).
- **Base de données** : PostgreSQL auto-hébergé ou via un service managé (ex: Supabase, Railway).
- **Emailing SMTP** : SMTP professionnel envisagé via Mailjet ou Sendinblue. (Utilisation personnelle de Gmail pendant le développement)

---

## Tests à prévoir

- Connexion utilisateur
- Accès conditionnels selon le rôle et la validation
- Upload et accessibilité des documents
- Validation/refus par l'admin
- Persistance et restauration du formulaire depuis `localStorage`

---

## Notes sur la structure du projet

- Deux types d'utilisateurs : `intervenant` et `gérant`
- Formulaires multi-étapes liés à des objets structurés en TypeScript/Rust
- Validation manuelle obligatoire pour l'accès complet par un admin (rôle attribué manuellement)
- Possibilité à terme de lier plusieurs utilisateurs à une même crèche (rôle "entreprise")

---

**Auteur :** Axelle Fouquemberg
**Dernier jour de stage :** 18/04/2025

Projet en cours, base propre sur la branche `main`.

