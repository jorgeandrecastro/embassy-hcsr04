# embassy-hcsr04

[![Crates.io](https://img.shields.io/crates/v/embassy-hcsr04.svg)](https://crates.io/crates/embassy-hcsr04)
[![Documentation](https://img.shields.io/docsrs/embassy-hcsr04/latest.svg)](https://docs.rs/embassy-hcsr04)
[![License](https://img.shields.io/crates/l/embassy-hcsr04.svg)](LICENSE)

**Driver asynchrone `no_std` pour le télémètre ultrasonique HC-SR04 et HC-SR04P.**

Conçu pour l'écosystème [Embassy](https://embassy.dev), ce driver permet de mesurer des distances sans bloquer l'exécuteur asynchrone, tout en offrant une précision accrue via la compensation de température.

---

## ⚡ Caractéristiques

- **Async natif** : Utilise `embassy-time` pour les mesures de durée d'écho sans attente active bloquante.
- **Sécurité garantie** : `#![forbid(unsafe_code)]`, aucune allocation dynamique.
- **Compensation thermique** : Intègre nativement le calcul de la vitesse du son ajustable.
- **Calculs optimisés** : Algorithmes entièrement basés sur des entiers (idéal pour les cibles sans FPU).
- **Compatible `embedded-hal`** : Fonctionne avec n'importe quel GPIO implémentant `InputPin` et `OutputPin`.

---

## 📦 Installation

Ajoutez ceci à votre `Cargo.toml` :
```toml
[dependencies]
embassy-hcsr04 = "0.1.1"
embassy-time = "0.5"
embedded-hal = "1.0"
```
-------

# 🚀 Utilisation

**Mesure standard (20°C par défaut)**

````rust
use embassy_hcsr04::HcSr04;

// Initialisation avec TRIG sur GP2 et ECHO sur GP3
let mut sonar = HcSr04::new(p.PIN_2, p.PIN_3);

loop {
    match sonar.measure_mm().await {
        Ok(dist) => defmt::info!("Distance : {} mm", dist),
        Err(e) => defmt::error!("Erreur : {:?}", e),
    }
    Timer::after_millis(100).await;
}

````

----

## Mesure précise avec compensation (via BMP280)

Le HC-SR04 est sensible à la température ambiante. Vous pouvez coupler ce driver avec embassy-bmp280 pour une précision maximale :

````rust 
let temp = bmp.read().await?.temperature_cdeg; // Récupère la temp du BMP280
let dist = sonar.measure_mm_compensated(temp).await?;

````

----

# 🔴 Câblage (Exemple RP2350 )

| Broche HC-SR04P | Connexion    | Note                                       |
| :-------------- | :----------- | :----------------------------------------- |
| **VCC**         | 3.3V         | Utilisez la version "P" pour le 3.3V natif |
| **GND**         | GND          | Masse commune                              |
| **TRIG**        | GP2 (Output) | Signal de déclenchement                    |
| **ECHO**        | GP3 (Input)  | Signal de retour                           |

-----

# 🛠️ Gestion des erreurs


*Le driver retourne une enum HcSr04Error<E> pour identifier précisément les pannes :*

  - Gpio(E) : Erreur matérielle sur les broches.

  - Timeout : Aucun signal d'écho reçu (obstacle trop loin ou capteur débranché).

  - EchoTooLong : Signal incohérent (supérieur à 30ms).

# 📜 Licence
GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro.




