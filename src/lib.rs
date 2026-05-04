// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

//! # embassy-hcsr04
//!
//! Driver asynchrone `no_std` pour le télémètre ultrasonique HC-SR04 / HC-SR04P.
//!
//! ## Exemple minimal
//! ```rust,ignore
//! let mut sonar = HcSr04::new(p.PIN_TRIG, p.PIN_ECHO);
//! let dist = sonar.measure_mm().await?; 
//! ```

#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub use error::HcSr04Error;

use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::{InputPin, OutputPin};

/// Driver pour le HC-SR04P.
pub struct HcSr04<TRIG, ECHO> {
    trig: TRIG,
    echo: ECHO,
}

impl<TRIG, ECHO, E> HcSr04<TRIG, ECHO>
where
    TRIG: OutputPin<Error = E>,
    ECHO: InputPin<Error = E>,
{
    /// Crée une nouvelle instance du driver.
    pub fn new(mut trig: TRIG, echo: ECHO) -> Self {
        let _ = trig.set_low();
        Self { trig, echo }
    }

    /// Mesure la distance en millimètres (vitesse du son à 20°C).
    pub async fn measure_mm(&mut self) -> Result<u32, HcSr04Error<E>> {
        self.measure_mm_compensated(2000).await // 20.00 °C par défaut
    }

    /// Mesure la distance en mm avec compensation de température.
    ///
    /// `temp_cdeg` : température en centidegrés (ex: `bmp.read().await?.temperature_cdeg`).
    pub async fn measure_mm_compensated(&mut self, temp_cdeg: i32) -> Result<u32, HcSr04Error<E>> {
        // 1. Impulsion Trigger de 10µs
        self.trig.set_high().map_err(HcSr04Error::Gpio)?;
        Timer::after_micros(10).await;
        self.trig.set_low().map_err(HcSr04Error::Gpio)?;

        // 2. Attente du front montant de l'Echo (Timeout 5ms)
        let mut retry = 0;
        while self.echo.is_low().map_err(HcSr04Error::Gpio)? {
            Timer::after_micros(10).await;
            retry += 1;
            if retry > 500 {
                return Err(HcSr04Error::Timeout);
            }
        }

        let start = Instant::now();

        // 3. Attente du front descendant de l'Echo (Max 30ms pour ~5m)
        while self.echo.is_high().map_err(HcSr04Error::Gpio)? {
            if start.elapsed() > Duration::from_millis(30) {
                return Err(HcSr04Error::EchoTooLong);
            }
            // On cède la main pour laisser respirer l'exécuteur
            Timer::after_micros(20).await;
        }

        let duration_us = start.elapsed().as_micros() as u32;

        // 4. Calcul de la vitesse du son (v ≈ 331.3 + 0.606 * T)
        // On calcule la vitesse en (mm/s) * 1024 pour garder de la précision en entier
        // 331300 mm/s à 0°C. 606 mm/s par degré.
        let speed_mm_s = 331300 + (606 * temp_cdeg) / 100;
        
        // Distance = (Temps_s * Vitesse) / 2
        // distance_mm = (duration_us / 1_000_000 * speed_mm_s) / 2
        let distance_mm = ((duration_us as u64 * speed_mm_s as u64) / 2_000_000) as u32;

        Ok(distance_mm)
    }
}