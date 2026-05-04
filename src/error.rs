// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

/// Erreurs possibles lors d'une mesure ultrasonique.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HcSr04Error<E> {
    /// Erreur liée aux broches GPIO (Input/Output).
    Gpio(E),
    /// Le signal d'écho n'est jamais revenu (capteur débranché ou hors de portée).
    Timeout,
    /// L'écho a duré trop longtemps (> 30ms), mesure invalide.
    EchoTooLong,
    /// Le capteur n'a pas répondu à l'impulsion de déclenchement.
    SensorNotReady,
}