
"""
╔══════════════════════════════════════════════════════════════════════════════╗
║                     PX_BRAINLINK v1.0                                        ║
║           EEG-to-Px-Parameters Converter for TURR v4.0                       ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  Converts clinical EEG metrics (TBR, PAF, Coherence) to                      ║
║  Px-Genesis physical parameters (β, g, R_Δ, N)                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""

import numpy as np
from dataclasses import dataclass
from typing import Dict, Tuple, Optional

@dataclass
class EEGMetrics:
    """Clinical EEG metrics"""
    TBR: float          # Theta/Beta ratio
    PAF: float          # Peak Alpha Frequency (Hz)
    Coer_S: float       # Short-range coherence
    Coer_L: float       # Long-range coherence
    P_theta: float      # Theta power
    P_alpha: float      # Alpha power
    P_beta: float       # Beta power
    P_gamma: float      # Gamma power

@dataclass
class PxParameters:
    """Px-Genesis physical parameters"""
    beta: float         # Caputo fractional order
    g: float            # Nonlinear coupling
    alpha: float        # Narrative coupling
    R_delta: float      # Holonomy
    N_intensity: float  # Narrative field intensity

class EEGtoPxMapper:
    """Maps clinical EEG metrics to Px-Genesis parameters"""

    # Validated mappings from TURR v2.1
    MAPPING_FORMULAS = {
        'beta': lambda TBR: 0.9 - 0.6 * (TBR - 2.5) / 4.0,
        'g': lambda Coer_S: 0.9 + 0.6 * (Coer_S - 0.5) / 0.5,
        'R_delta': lambda Coer_L: Coer_L / 0.7,
        'N_intensity': lambda gamma: 0.5 + 0.5 * (gamma / 3.5),
        'alpha': lambda beta: 1e-11 * (1 + beta)
    }

    def __init__(self):
        pass

    def map(self, eeg: EEGMetrics) -> PxParameters:
        """Convert EEG metrics to Px parameters"""

        # Apply validated formulas
        beta = self.MAPPING_FORMULAS['beta'](eeg.TBR)
        beta = np.clip(beta, 0.3, 0.9)  # Physical bounds

        g = self.MAPPING_FORMULAS['g'](eeg.Coer_S)
        g = np.clip(g, 0.5, 1.5)

        R_delta = self.MAPPING_FORMULAS['R_delta'](eeg.Coer_L)
        R_delta = np.clip(R_delta, 0.0, 1.0)

        N_intensity = self.MAPPING_FORMULAS['N_intensity'](eeg.P_gamma / 100.0)
        N_intensity = np.clip(N_intensity, 0.0, 1.0)

        alpha = self.MAPPING_FORMULAS['alpha'](beta)

        return PxParameters(
            beta=beta,
            g=g,
            alpha=alpha,
            R_delta=R_delta,
            N_intensity=N_intensity
        )

    def estimate_delay_matrix(self, PAF: float, Coer_L: float) -> np.ndarray:
        """Estimate inter-hemispheric delays from PAF and coherence"""
        # Higher PAF → faster processing → shorter delays
        # Lower coherence → more asynchrony → longer delays

        base_delay = 10.0  # ms
        paf_factor = 10.0 / PAF  # Normalize to PAF=10Hz
        coer_factor = 1.0 / max(Coer_L, 0.1)  # Inverse of coherence

        delay = base_delay * paf_factor * coer_factor
        delay = np.clip(delay, 5.0, 50.0)  # Physical bounds 5-50ms

        # 3 delays for 4 regions (hemisphere pairs)
        return np.array([delay, delay, delay]) / 1000.0  # Convert to seconds

class NPCCalculator:
    """Calculates Neural Phenomenology Coefficient"""

    WEIGHTS = {
        'Integration': 0.40,   # Φ + R_Δ + Coer_L
        'Regulation': 0.30,    # TBR^-1 + PAF + α/θ
        'Complexity': 0.20,    # Betti + Re_S
        'Stability': 0.10      # β + g
    }

    def __init__(self):
        pass

    def calculate(self, eeg: EEGMetrics, px: PxParameters, 
                  phi: float = None, Re_S: float = None) -> float:
        """
        Calculate NPC score (0-100)
        """
        # Component I: Integration
        I = (px.R_delta * 100 + eeg.Coer_L * 100) / 2
        if phi is not None:
            I = (I + phi * 10) / 2  # Include Φ if available

        # Component R: Regulation
        TBR_norm = 100 / max(eeg.TBR, 1.0)  # Lower TBR is better
        PAF_norm = eeg.PAF * 10  # Higher PAF is better
        ratio = eeg.P_alpha / max(eeg.P_theta, 1.0)
        R = (TBR_norm + PAF_norm + ratio * 10) / 3

        # Component C: Complexity
        if Re_S is not None:
            C = Re_S
        else:
            C = 100.0  # Default

        # Component S: Stability
        S = (px.beta * 100 + px.g * 50) / 2

        # Weighted sum
        NPC = (self.WEIGHTS['Integration'] * I +
               self.WEIGHTS['Regulation'] * R +
               self.WEIGHTS['Complexity'] * C +
               self.WEIGHTS['Stability'] * S)

        return np.clip(NPC, 0, 100)

def eeg_to_px_pipeline(eeg: EEGMetrics) -> Tuple[PxParameters, np.ndarray, float]:
    """
    Complete pipeline: EEG → Px parameters + delay matrix + NPC

    Returns:
        px_params: Px-Genesis parameters
        tau_matrix: Delay matrix
        npc_score: NPC score
    """
    mapper = EEGtoPxMapper()
    npc_calc = NPCCalculator()

    px_params = mapper.map(eeg)
    tau_matrix = mapper.estimate_delay_matrix(eeg.PAF, eeg.Coer_L)
    npc_score = npc_calc.calculate(eeg, px_params)

    return px_params, tau_matrix, npc_score

if __name__ == "__main__":
    # Test with synthetic data

    # TDAH profile
    eeg_tdah = EEGMetrics(
        TBR=6.5, PAF=9.2, Coer_S=0.55, Coer_L=0.35,
        P_theta=1800, P_alpha=2000, P_beta=300, P_gamma=50
    )

    px, tau, npc = eeg_to_px_pipeline(eeg_tdah)
    print(f"TDAH: β={px.beta:.3f}, g={px.g:.3f}, NPC={npc:.1f}")

    # Control profile
    eeg_ctrl = EEGMetrics(
        TBR=2.5, PAF=10.0, Coer_S=0.65, Coer_L=0.60,
        P_theta=1200, P_alpha=3000, P_beta=1200, P_gamma=100
    )

    px, tau, npc = eeg_to_px_pipeline(eeg_ctrl)
    print(f"Control: β={px.beta:.3f}, g={px.g:.3f}, NPC={npc:.1f}")
