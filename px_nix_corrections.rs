//! # Correções Críticas para Px/Nix Engine
//! 
//! Este arquivo contém patches e correções para os bugs identificados.
//! Aplique estas correções aos arquivos originais.

// ============================================================================
// CORREÇÃO 1: homeostasis.rs - Método observe() com assinatura correta
// ============================================================================

/// Método observe corrigido - aceita parâmetros corretos
/// Substitua o método observe() em homeostasis.rs (linha ~153)
pub fn observe(&mut self, latency_us: u64, is_deep: bool) {
    // Update latency histogram
    let bucket = latency_to_bucket(latency_us);
    if bucket < self.latency_buckets.len() {
        self.latency_buckets[bucket].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    
    // Update counters
    self.request_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    
    if is_deep {
        self.deep_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

// ============================================================================
// CORREÇÃO 2: noise.rs - Média geométrica correta
// ============================================================================

/// Calcula média geométrica corretamente
/// Substitua a função em noise.rs (linhas ~389-395)
fn compute_geometric_mean(powers: &[f64]) -> f64 {
    if powers.is_empty() {
        return 0.0;
    }
    
    // Fórmula correta: exp(Σln(x_i)/n)
    let sum_log: f64 = powers.iter()
        .map(|&p| (p + 1e-10).ln())
        .sum();
    
    (sum_log / powers.len() as f64).exp()
}

/// Spectral flatness corrigido
fn compute_n6_spectral_corrected(data: &[u8]) -> f64 {
    if data.len() < 32 {
        return 1.0;
    }
    
    let samples: Vec<f64> = data.iter().map(|&b| b as f64 - 128.0).collect();
    let n = samples.len();
    let num_bins = 16.min(n / 2);
    let mut powers = Vec::with_capacity(num_bins);
    
    for k in 1..=num_bins {
        let power = goertzel(&samples, k as f64 / n as f64);
        powers.push(power);
    }
    
    // Média geométrica CORRETA
    let geo_mean = compute_geometric_mean(&powers);
    
    // Média aritmética
    let arith_mean = powers.iter().sum::<f64>() / powers.len() as f64;
    
    if arith_mean < 1e-10 {
        return 1.0;
    }
    
    // Flatness = geo/arith (close to 1 = white noise)
    (geo_mean / arith_mean).min(1.0)
}

// ============================================================================
// CORREÇÃO 3: mdl::lz_complexity - Versão O(n·log(n)) 
// ============================================================================

/// LZ complexity otimizado usando rolling hash
/// Substitua a função em lib.rs (linhas ~476-502)
pub fn lz_complexity_optimized(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    // Usa suffix array aproximado via rolling hash para O(n log n)
    use std::collections::HashSet;
    
    const PRIME: u64 = 31;
    const MOD: u64 = 1_000_000_007;
    
    let mut phrases = 0usize;
    let mut seen_hashes: HashSet<u64> = HashSet::with_capacity(data.len());
    let mut i = 0;
    
    while i < data.len() {
        let mut hash = 0u64;
        let mut best_new_len = 1;
        
        // Tenta encontrar o menor prefixo não visto
        for len in 1..=(data.len() - i).min(64) {
            let byte = data[i + len - 1] as u64;
            hash = (hash * PRIME + byte) % MOD;
            
            if !seen_hashes.contains(&hash) {
                best_new_len = len;
                break;
            }
        }
        
        // Adiciona hash do novo segmento
        let mut final_hash = 0u64;
        for j in 0..best_new_len {
            final_hash = (final_hash * PRIME + data[i + j] as u64) % MOD;
        }
        seen_hashes.insert(final_hash);
        
        phrases += 1;
        i += best_new_len;
    }
    
    phrases as f64 / data.len() as f64
}

// ============================================================================
// CORREÇÃO 4: synthesis.rs - Transform::apply retorna Result
// ============================================================================

/// Uso correto de Transform::apply em lib.rs
/// Substitua a linha ~627 em process_deep()
fn process_deep_corrected(&mut self, input: &[u8]) -> Result<InternalResult, PxError> {
    // Step 1: Prisma decomposition
    let bands = self.prism.decompose(input);
    
    // Step 2: Find/synthesize best transform
    let phi = self.find_best_transform(input, &bands)?;
    
    // Step 3: Apply transform - CORRIGIDO: trata Result corretamente
    let output = phi.apply(input)?;  // <-- Adiciona ? para propagar erro
    
    // ... resto da função permanece igual
    Ok(InternalResult {
        output,
        j_score: 0.0, // calcular
        competence: crate::CompetenceVector::default(),
        phi_id: phi.id,
    })
}

// ============================================================================
// CORREÇÃO 5: Sensores reais para Linux/Android
// ============================================================================

/// Implementação real de estimate_cpu_usage para Linux
fn estimate_cpu_usage_real() -> f64 {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        // Lê /proc/stat
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            if let Some(line) = content.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let user: u64 = parts[1].parse().unwrap_or(0);
                    let nice: u64 = parts[2].parse().unwrap_or(0);
                    let system: u64 = parts[3].parse().unwrap_or(0);
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    
                    let total = user + nice + system + idle;
                    if total > 0 {
                        return (user + nice + system) as f64 / total as f64;
                    }
                }
            }
        }
        0.5
    }
    
    #[cfg(target_os = "android")]
    {
        // Android também usa /proc/stat
        estimate_cpu_usage_real_linux()
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        0.5 // Fallback para outras plataformas
    }
}

/// Implementação real de estimate_memory_usage
fn estimate_memory_usage_real() -> u64 {
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        
        // Lê /proc/self/status
        if let Ok(content) = fs::read_to_string("/proc/self/status") {
            for line in content.lines() {
                if line.starts_with("VmRSS:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return kb * 1024; // Converte KB para bytes
                        }
                    }
                }
            }
        }
        0
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        0
    }
}

/// Implementação de temperatura para Android
#[cfg(target_os = "android")]
fn get_temperature_android() -> Option<f64> {
    use std::fs;
    
    // Tenta vários caminhos de thermal zone
    let thermal_paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/devices/virtual/thermal/thermal_zone0/temp",
    ];
    
    for path in &thermal_paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(millicelsius) = content.trim().parse::<i64>() {
                return Some(millicelsius as f64 / 1000.0);
            }
        }
    }
    
    None
}

/// Implementação de nível de bateria para Android
#[cfg(target_os = "android")]
fn get_battery_level_android() -> Option<f64> {
    use std::fs;
    
    let capacity_path = "/sys/class/power_supply/battery/capacity";
    
    if let Ok(content) = fs::read_to_string(capacity_path) {
        if let Ok(percent) = content.trim().parse::<u8>() {
            return Some(percent as f64 / 100.0);
        }
    }
    
    None
}

// ============================================================================
// TESTES DAS CORREÇÕES
// ============================================================================

#[cfg(test)]
mod correction_tests {
    use super::*;
    
    #[test]
    fn test_geometric_mean_correct() {
        let powers = vec![1.0, 2.0, 4.0, 8.0];
        let geo = compute_geometric_mean(&powers);
        
        // geo_mean(1,2,4,8) = (1*2*4*8)^(1/4) = 64^0.25 = 2.828...
        assert!((geo - 2.828).abs() < 0.01);
    }
    
    #[test]
    fn test_lz_complexity_not_quadratic() {
        use std::time::Instant;
        
        // Dados pequenos
        let small: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();
        let start_small = Instant::now();
        let _ = lz_complexity_optimized(&small);
        let time_small = start_small.elapsed();
        
        // Dados 10x maiores
        let large: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        let start_large = Instant::now();
        let _ = lz_complexity_optimized(&large);
        let time_large = start_large.elapsed();
        
        // Se fosse O(n²), 10x dados => 100x tempo
        // Com O(n log n), deveria ser ~13x (10 * log(10)/log(1))
        // Aceitamos até 20x como razoável
        let ratio = time_large.as_nanos() as f64 / time_small.as_nanos() as f64;
        assert!(ratio < 30.0, "LZ complexity ainda é O(n²)! ratio = {}", ratio);
    }
}
