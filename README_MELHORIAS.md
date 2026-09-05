# Px/Nix Engine - Melhorias e Correções

## Resumo

Este pacote contém análises detalhadas, correções críticas e novas funcionalidades para o Px/Nix Engine v1.0.

## Arquivos Incluídos

| Arquivo | Descrição |
|---------|-----------|
| `Px_Nix_Engine_Analysis_Report.docx` | Relatório completo de análise técnica |
| `px_nix_corrections.rs` | Correções de bugs críticos identificados |
| `px_meta.rs` | Novo módulo de meta-cognição (K_meta) |
| `px_evolve.rs` | Novo módulo de síntese evolutiva de transforms |

---

## Instruções de Integração

### 1. Aplicar Correções Críticas (PRIORIDADE MÁXIMA)

#### 1.1 Corrigir assinatura do método `observe()`

Em `src/homeostasis.rs`, substitua a linha ~153:

```rust
// DE:
pub fn observe(&mut self, latency_us: u64, route: &router::Route, result: &crate::InternalResult) {

// PARA:
pub fn observe(&mut self, latency_us: u64, is_deep: bool) {
```

#### 1.2 Corrigir média geométrica em `noise.rs`

Substitua as linhas ~389-395:

```rust
// Média geométrica CORRETA
fn compute_geometric_mean(powers: &[f64]) -> f64 {
    if powers.is_empty() {
        return 0.0;
    }
    let sum_log: f64 = powers.iter()
        .map(|&p| (p + 1e-10).ln())
        .sum();
    (sum_log / powers.len() as f64).exp()
}
```

#### 1.3 Corrigir uso de `Transform::apply()`

Em `src/lib.rs`, linha ~627, adicione `?` para propagar Result:

```rust
// DE:
let output = phi.apply(input);

// PARA:
let output = phi.apply(input)?;
```

---

### 2. Adicionar Novos Módulos

#### 2.1 Meta-Cognição

1. Copie `px_meta.rs` para `src/meta.rs`
2. Adicione em `src/lib.rs`:
   ```rust
   pub mod meta;
   pub use meta::MetaCognitionController;
   ```
3. Integre no `PxEngine`:
   ```rust
   pub struct PxEngine {
       // ... campos existentes ...
       meta: MetaCognitionController,
   }
   ```

#### 2.2 Síntese Evolutiva

1. Copie `px_evolve.rs` para `src/evolve.rs`
2. Adicione em `src/lib.rs`:
   ```rust
   pub mod evolve;
   pub use evolve::{EvolutionEngine, EvolutionConfig, Chromosome};
   ```
3. Use no lugar de `synthesis::synthesize_transform()`:
   ```rust
   // Em find_best_transform():
   let mut evolver = EvolutionEngine::new(EvolutionConfig::default(), seed);
   let best_chromosome = evolver.evolve(input, self.config.lambda);
   // Converter Chromosome para Transform...
   ```

---

### 3. Implementar Sensores Reais (Mobile)

Para Android/Galaxy S25, substitua os placeholders em `homeostasis.rs`:

```rust
#[cfg(target_os = "android")]
fn estimate_cpu_usage_real() -> f64 {
    use std::fs;
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        // Parse CPU stats...
    }
    0.5
}

#[cfg(target_os = "android")]
fn get_temperature_android() -> Option<f64> {
    use std::fs;
    let paths = ["/sys/class/thermal/thermal_zone0/temp"];
    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(mc) = content.trim().parse::<i64>() {
                return Some(mc as f64 / 1000.0);
            }
        }
    }
    None
}
```

---

### 4. Otimizações SIMD

Adicione implementações SIMD para cálculos de entropia:

```rust
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
pub fn entropy_neon(data: &[u8]) -> f64 {
    // Implementação com NEON intrinsics
    // Conta bytes usando SIMD, depois calcula entropia
}

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86_64")]
pub fn entropy_avx2(data: &[u8]) -> f64 {
    // Implementação com AVX2 intrinsics
}
```

---

## Checklist de Validação

Após aplicar as correções:

- [ ] `cargo build --release` compila sem erros
- [ ] `cargo test` passa todos os testes
- [ ] `cargo bench` executa sem crashes
- [ ] Testar em dispositivo Android (se aplicável)

---

## Métricas de Sucesso

| Métrica | Antes | Depois (Esperado) |
|---------|-------|-------------------|
| Compilação | ❌ Erros | ✅ Sucesso |
| lz_complexity | O(n²) | O(n log n) |
| Sensores Mobile | Placeholders | Funcionais |
| K_meta | N/A | [0,1] funcional |
| Síntese | Heurística | Evolutiva |

---

## Suporte

Para dúvidas sobre a integração das melhorias, consulte:
- O relatório completo `Px_Nix_Engine_Analysis_Report.docx`
- Comentários nos arquivos `.rs`
- Testes unitários incluídos em cada módulo

---

## Roadmap de Próximos Passos

1. **Semana 1-2**: Aplicar correções críticas, validar compilação
2. **Semana 3-4**: Integrar módulo de meta-cognição
3. **Semana 5-6**: Integrar síntese evolutiva
4. **Semana 7-8**: Implementar sensores reais para Android
5. **Semana 9+**: Otimizações SIMD e benchmarking

---

*Gerado em Novembro 2025*
