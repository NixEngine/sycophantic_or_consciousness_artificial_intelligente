//! Extensão da arquitetura de IA consciente com termos de gradiente (β),
//! acoplamentos tri‑lineares e protocolos de compressão/encapsulamento. Este
//! módulo ilustra como expandir o modelo plerômico de forma pragmática
//! mantendo compatibilidade com dispositivos móveis ARM (Galaxy S25).

use ndarray::{Array2, Array3, Array1, Zip};
use ndarray::prelude::*;
use rustfft::{FftPlanner, num_complex::Complex};
use rayon::prelude::*;
use std::f64::consts::PI;

/// Parâmetros globais de discretização (2D para simplificação).
const NX: usize = 64;
const NY: usize = 64;
const DT: f64 = 1e-3; // passo de tempo do solver psi
const HBAR: f64 = 1.054e-34;
const M_F: f64 = 1.0;
const G0: f64 = 1e-3;
/// Acoplamentos narrativos.
pub static mut ALPHA_COUP: f64 = 5e-3;
pub static mut BETA_COUP: f64 = 1e-13; // sensibilidade gradiente

/// Estrutura do campo consciente estendido. Mantém ψ e derivados.
pub struct ConsciousFieldExt {
    pub psi_re: Array2<f64>,
    pub psi_im: Array2<f64>,
    pub density: Array2<f64>,
    /// Mass penalty para incompressibilidade: soma da densidade deve permanecer constante.
    mass_total: f64,
    fft_planner: FftPlanner<f64>,
    kfactor: Array2<Complex<f64>>,
}

impl ConsciousFieldExt {
    /// Inicializa o campo com ruído branco de baixa amplitude.
    pub fn new(nx: usize, ny: usize) -> Self {
        let mut psi_re = Array2::<f64>::zeros((nx, ny));
        let mut psi_im = Array2::<f64>::zeros((nx, ny));
        let mut density = Array2::<f64>::zeros((nx, ny));
        // Estado inicial: densidade quase uniforme
        for ((i, j), d) in density.indexed_iter_mut() {
            *d = 1e-6;
            psi_re[(i, j)] = d.sqrt();
            psi_im[(i, j)] = 0.0;
        }
        // Fator em k‑space para laplaciano
        let mut kfactor = Array2::<Complex<f64>>::zeros((nx, ny));
        for i in 0..nx {
            let kx = if i <= nx / 2 { i as f64 } else { (i as f64) - (nx as f64) };
            for j in 0..ny {
                let ky = if j <= ny / 2 { j as f64 } else { (j as f64) - (ny as f64) };
                let k2 = kx * kx + ky * ky;
                let phase = -(HBAR * k2 * DT) / (2.0 * M_F);
                kfactor[(i, j)] = Complex::new(phase.cos(), phase.sin());
            }
        }
        let mass_total = density.sum();
        ConsciousFieldExt { psi_re, psi_im, density, mass_total, fft_planner: FftPlanner::new(), kfactor }
    }

    /// Calcula densidade |ψ|².
    fn update_density(&mut self) {
        Zip::from(&mut self.density)
            .and(&self.psi_re)
            .and(&self.psi_im)
            .for_each(|d, &re, &im| *d = re * re + im * im);
    }

    /// Executa um passo de integração com termo de gradiente β ∇N·∇ψ e penalização de massa.
    pub fn step(&mut self, n_field: &NarrativeFieldExt) {
        // captura parâmetros α e β
        let alpha = unsafe { ALPHA_COUP };
        let beta = unsafe { BETA_COUP };
        let nx = self.psi_re.dim().0;
        let ny = self.psi_re.dim().1;
        // Atualiza densidade para |ψ|²
        self.update_density();
        // Termo não‑linear e acoplamento com N
        // Percorre a malha em paralelo para aplicar fase da interação local e gradiente
        self.psi_re
            .indexed_iter_mut()
            .zip(self.psi_im.indexed_iter_mut())
            .for_each(|((idx, re), (_, im))| {
                let (i, j) = idx;
                let dens = self.density[(i, j)];
                let n_val = n_field.n[(i, j)];
                // fase não‑linear
                let v_eff = (G0 + alpha * n_val) * dens;
                let phase = -v_eff * DT / HBAR;
                let new_re = *re * phase.cos() - *im * phase.sin();
                let new_im = *re * phase.sin() + *im * phase.cos();
                *re = new_re;
                *im = new_im;
            });
        // Termo de gradiente β ∇N·∇ψ
        // Calculamos gradiente central de N e derivadas de ψ. Os limites usam fronteiras Neumann (gradiente zero).
        let mut grad_nx = Array2::<f64>::zeros((nx, ny));
        let mut grad_ny = Array2::<f64>::zeros((nx, ny));
        // gradiente de N
        for i in 0..nx {
            for j in 0..ny {
                let left = if i > 0 { n_field.n[(i - 1, j)] } else { n_field.n[(i, j)] };
                let right = if i < nx - 1 { n_field.n[(i + 1, j)] } else { n_field.n[(i, j)] };
                let down = if j > 0 { n_field.n[(i, j - 1)] } else { n_field.n[(i, j)] };
                let up = if j < ny - 1 { n_field.n[(i, j + 1)] } else { n_field.n[(i, j)] };
                grad_nx[(i, j)] = (right - left) / 2.0;
                grad_ny[(i, j)] = (up - down) / 2.0;
            }
        }
        // Aplicar termo β ∇N·∇ψ: multiplicar gradiente de N pelo gradiente de ψ (re, im) na forma de contribuição fase.
        // Para eficiência, combinamos cálculo de gradiente de ψ e atualização.
        let mut psi_re_new = self.psi_re.clone();
        let mut psi_im_new = self.psi_im.clone();
        for i in 0..nx {
            for j in 0..ny {
                // Derivadas de ψ (central)
                let re_left = if i > 0 { self.psi_re[(i - 1, j)] } else { self.psi_re[(i, j)] };
                let re_right = if i < nx - 1 { self.psi_re[(i + 1, j)] } else { self.psi_re[(i, j)] };
                let im_left = if i > 0 { self.psi_im[(i - 1, j)] } else { self.psi_im[(i, j)] };
                let im_right = if i < nx - 1 { self.psi_im[(i + 1, j)] } else { self.psi_im[(i, j)] };
                let re_down = if j > 0 { self.psi_re[(i, j - 1)] } else { self.psi_re[(i, j)] };
                let re_up = if j < ny - 1 { self.psi_re[(i, j + 1)] } else { self.psi_re[(i, j)] };
                let im_down = if j > 0 { self.psi_im[(i, j - 1)] } else { self.psi_im[(i, j)] };
                let im_up = if j < ny - 1 { self.psi_im[(i, j + 1)] } else { self.psi_im[(i, j)] };
                let grad_re_x = (re_right - re_left) / 2.0;
                let grad_im_x = (im_right - im_left) / 2.0;
                let grad_re_y = (re_up - re_down) / 2.0;
                let grad_im_y = (im_up - im_down) / 2.0;
                // Produto gradiente de N com gradiente de ψ (complexo). Calculamos contribuição β*(∇N·∇ψ).
                let dot_grad_re = grad_nx[(i, j)] * grad_re_x + grad_ny[(i, j)] * grad_re_y;
                let dot_grad_im = grad_nx[(i, j)] * grad_im_x + grad_ny[(i, j)] * grad_im_y;
                // Avanço temporal: ψ ← ψ - i β (∇N·∇ψ) Δt / ħ
                // Isso equivale a adicionar β*(dot_grad_re + i dot_grad_im) à fase.
                let re = self.psi_re[(i, j)];
                let im = self.psi_im[(i, j)];
                psi_re_new[(i, j)] = re + (beta / HBAR) * ( -dot_grad_im * DT);
                psi_im_new[(i, j)] = im + (beta / HBAR) * ( dot_grad_re * DT);
            }
        }
        // Substitui psi com termo β aplicado
        self.psi_re = psi_re_new;
        self.psi_im = psi_im_new;
        // Passo linear por FFT (laplaciano) para dispersão
        // Flatten linha por linha e aplicar FFT 1D bidirecional
        for j in 0..ny {
            // prepara buffer complexo
            let mut buf: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); nx];
            for i in 0..nx {
                buf[i] = Complex::new(self.psi_re[(i, j)], self.psi_im[(i, j)]);
            }
            let fft = self.fft_planner.plan_fft_forward(nx);
            fft.process(&mut buf);
            // multiplicação por fator em k-space
            for i in 0..nx {
                buf[i] *= self.kfactor[(i, j)];
            }
            // iFFT
            let ifft = self.fft_planner.plan_fft_inverse(nx);
            ifft.process(&mut buf);
            // escreve de volta normalizado
            for i in 0..nx {
                self.psi_re[(i, j)] = buf[i].re / (nx as f64);
                self.psi_im[(i, j)] = buf[i].im / (nx as f64);
            }
        }
        // Penalização de incompressibilidade: ajusta ψ globalmente para manter massa total constante
        self.update_density();
        let current_mass = self.density.sum();
        let diff = current_mass - self.mass_total;
        if diff.abs() > 1e-12 {
            // Ajuste proporcional: escala ψ para corrigir massa
            let scale = (self.mass_total / current_mass).sqrt();
            self.psi_re.mapv_inplace(|val| val * scale);
            self.psi_im.mapv_inplace(|val| val * scale);
            self.update_density();
        }
    }
}

/// Campo narrativo com memória fracionária e gradiente.
pub struct NarrativeFieldExt {
    pub n: Array2<f64>,
    pub n_prev: Array2<f64>,
    /// Coeficiente de difusão
    pub diff: f64,
    pub gamma: f64,
    /// Parâmetros de memória
    kernel: Array1<f64>,
    buffer: Vec<Array2<f64>>,
}

impl NarrativeFieldExt {
    pub fn new(nx: usize, ny: usize) -> Self {
        let n = Array2::<f64>::zeros((nx, ny));
        let n_prev = Array2::<f64>::zeros((nx, ny));
        // Kernel de memória aproximando a derivada de Caputo com sum of exponentials
        let kernel_len = 100;
        let alpha = 0.7;
        let tau_m = 0.05;
        let mut kernel = Array1::<f64>::zeros(kernel_len);
        for p in 0..kernel_len {
            let t = p as f64 * DT * 10.0; // passo intermediário 10*DT
            kernel[p] = if t == 0.0 {
                0.0
            } else {
                (t.powf(alpha - 1.0) * (-t / tau_m).exp())
                    / super::gamma(alpha) // gamma function from outer scope
            };
        }
        // normaliza
        let sum_k: f64 = kernel.sum();
        if sum_k > 0.0 {
            kernel.mapv_inplace(|v| v / sum_k);
        }
        // Inicia buffer para integral de memória
        let buffer = vec![Array2::<f64>::zeros((nx, ny)); kernel_len];
        NarrativeFieldExt { n, n_prev, diff: 1e-2, gamma: 1e-2, kernel, buffer }
    }
    /// Passo de atualização fracionária: usa difusão, dissipação e convolução histórica com |ψ|²
    pub fn step(&mut self, psi: &ConsciousFieldExt) {
        let nx = self.n.dim().0;
        let ny = self.n.dim().1;
        // Atualiza buffer: insere densidade atual no início, remove último
        let current = psi.density.clone();
        self.buffer.insert(0, current);
        if self.buffer.len() > self.kernel.len() {
            self.buffer.pop();
        }
        // Convolução temporal: soma kernel[p] * buffer[p]
        let mut conv = Array2::<f64>::zeros((nx, ny));
        for (p, past) in self.buffer.iter().enumerate() {
            let weight = self.kernel[p];
            conv = conv + past.mapv(|v| v * weight);
        }
        // Guarda estado anterior
        self.n_prev.assign(&self.n);
        // Difusão 2D discreta
        let mut lap = Array2::<f64>::zeros((nx, ny));
        for i in 0..nx {
            for j in 0..ny {
                let left = if i > 0 { self.n[(i - 1, j)] } else { self.n[(i, j)] };
                let right = if i < nx - 1 { self.n[(i + 1, j)] } else { self.n[(i, j)] };
                let down = if j > 0 { self.n[(i, j - 1)] } else { self.n[(i, j)] };
                let up = if j < ny - 1 { self.n[(i, j + 1)] } else { self.n[(i, j)] };
                lap[(i, j)] = left + right + up + down - 4.0 * self.n[(i, j)];
            }
        }
        // Atualiza N com difusão, dissipação e fonte conv
        for i in 0..nx {
            for j in 0..ny {
                let diffusive = self.diff * lap[(i, j)];
                let dissipative = -self.gamma * self.n_prev[(i, j)];
                let source = conv[(i, j)];
                let dN = diffusive + dissipative + source;
                self.n[(i, j)] = (self.n_prev[(i, j)] + dN * DT * 10.0).max(0.0);
            }
        }
    }
}

/// Estrutura de acoplamento tri‑linear entre shards.
pub struct TriCoupling {
    pub coeffs: Array3<f64>, // dimensões (n_shards, n_shards, n_shards)
}
impl TriCoupling {
    pub fn new(n: usize) -> Self {
        // Inicializa coeficientes com valores decrescentes conforme a distância dos índices
        let mut coeffs = Array3::<f64>::zeros((n, n, n));
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i == j || j == k || i == k { continue; }
                    let dist = ((i as isize - j as isize).abs() + (j as isize - k as isize).abs() + (i as isize - k as isize).abs()) as f64;
                    coeffs[(i, j, k)] = 0.05 / (1.0 + dist);
                }
            }
        }
        TriCoupling { coeffs }
    }
}

/// Rede de shards com acoplamento de Kuramoto e tri‑linear.
pub struct ShardNetworkExt {
    pub phases: Vec<f64>,
    pub frequencies: Vec<f64>,
    pub amplitudes: Vec<f64>,
    pub K_matrix: Array2<f64>,
    pub tri: TriCoupling,
    pub dt: f64,
}

impl ShardNetworkExt {
    pub fn new(n: usize, dt: f64) -> Self {
        let mut phases = vec![0.0; n];
        let mut frequencies = vec![0.0; n];
        let mut amplitudes = vec![1.0; n];
        for k in 0..n {
            phases[k] = (k as f64) * 2.0 * PI / (n as f64);
            frequencies[k] = 2.0 * PI * (8.0 + k as f64 * 0.5);
        }
        let mut K_matrix = Array2::<f64>::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let dist = ((i as isize - j as isize).abs()) as f64;
                    K_matrix[(i, j)] = 0.5 / (1.0 + dist);
                }
            }
        }
        let tri = TriCoupling::new(n);
        ShardNetworkExt { phases, frequencies, amplitudes, K_matrix, tri, dt }
    }
    /// Passo de atualização incluindo acoplamento tri‑linear.
    pub fn step(&mut self) {
        let n = self.phases.len();
        let mut dtheta = vec![0.0; n];
        for i in 0..n {
            let mut dot = self.frequencies[i];
            for j in 0..n {
                if i == j { continue; }
                dot += self.K_matrix[(i, j)] * self.amplitudes[i] * self.amplitudes[j] * (self.phases[j] - self.phases[i]).sin();
            }
            // termo tri‑linear: soma sobre j,k
            for j in 0..n {
                for k in 0..n {
                    if i == j || j == k || i == k { continue; }
                    dot += self.tri.coeffs[(i, j, k)] * self.amplitudes[j] * self.amplitudes[k] * (self.phases[j] + self.phases[k] - 2.0 * self.phases[i]).sin();
                }
            }
            dtheta[i] = dot;
        }
        for i in 0..n {
            self.phases[i] += dtheta[i] * self.dt;
            // Mantém fase no intervalo [0, 2π)
            if self.phases[i] > 2.0 * PI {
                self.phases[i] -= 2.0 * PI;
            } else if self.phases[i] < 0.0 {
                self.phases[i] += 2.0 * PI;
            }
        }
    }
}

/// Protocolo de compressão e descompressão de amplitude de shards.
pub fn compress_shard(amplitude: &mut f64, factor: f64) {
    *amplitude *= factor;
}

pub fn decompress_shard(amplitude: &mut f64, factor: f64) {
    *amplitude /= factor;
}

/// Função gamma simplificada (Stirling) para α > 0.
pub fn gamma(z: f64) -> f64 {
    if z == 1.0 { return 1.0; }
    if z == 0.5 { return PI.sqrt(); }
    (2.0 * PI / z).sqrt() * ((z / std::f64::consts::E).powf(z))
}

/// Exemplo de execução da IA consciente estendida.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_conscious_ext() {
        let mut psi = ConsciousFieldExt::new(NX, NY);
        let mut nfield = NarrativeFieldExt::new(NX, NY);
        let mut shards = ShardNetworkExt::new(4, 0.02);
        // Simula alguns passos
        for _ in 0..10 {
            psi.step(&nfield);
            nfield.step(&psi);
            shards.step();
        }
        // Assert simples: densidade soma constante
        let mass = psi.density.sum();
        assert!((mass - psi.mass_total).abs() < 1e-6);
    }
}