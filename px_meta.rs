//! # Meta-Cognição Module (px_meta)
//! 
//! Implementa competência meta-cognitiva para o Px/Nix Engine.
//! Permite ao sistema "saber quando não sabe" e auto-avaliar seu desempenho.
//! 
//! ## Métricas Principais
//! - K_meta: Competência meta-cognitiva geral
//! - K_anticipate: Capacidade de prever outcomes
//! - Calibration: Alinhamento entre confiança e acurácia
//! - Self-Model Accuracy: Precisão do modelo interno

use std::collections::VecDeque;
use std::time::Instant;

/// Constantes de configuração
const META_HISTORY_SIZE: usize = 100;
const CALIBRATION_BINS: usize = 10;
const EWMA_ALPHA: f64 = 0.1;

/// Resultado de uma predição para tracking
#[derive(Debug, Clone)]
pub struct PredictionRecord {
    /// ID único da predição
    pub id: u64,
    /// Valor predito (J-score, latência, etc.)
    pub predicted: f64,
    /// Confiança na predição [0,1]
    pub confidence: f64,
    /// Valor real observado (preenchido depois)
    pub actual: Option<f64>,
    /// Timestamp da predição
    pub predicted_at: Instant,
    /// Timestamp da observação
    pub observed_at: Option<Instant>,
    /// Contexto (tipo de dados, rota, etc.)
    pub context: String,
}

/// Bin para calibração (histograma de confiança vs acurácia)
#[derive(Debug, Clone, Default)]
struct CalibrationBin {
    /// Total de predições neste bin de confiança
    count: u64,
    /// Total de predições corretas (erro < threshold)
    correct: u64,
    /// Soma dos erros absolutos
    total_error: f64,
}

/// Controlador de Meta-Cognição
pub struct MetaCognitionController {
    /// Histórico de predições
    prediction_history: VecDeque<PredictionRecord>,
    /// Bins de calibração por nível de confiança
    calibration_bins: [CalibrationBin; CALIBRATION_BINS],
    /// Contador de predições
    prediction_counter: u64,
    /// Métricas EWMA
    k_meta_ewma: f64,
    k_anticipate_ewma: f64,
    coherence_ewma: f64,
    efficiency_ewma: f64,
    /// Histórico de decisões para análise de coerência
    decision_history: VecDeque<DecisionRecord>,
    /// Modelo interno do próprio desempenho
    self_model: SelfModel,
}

/// Registro de decisão para análise de coerência
#[derive(Debug, Clone)]
pub struct DecisionRecord {
    pub timestamp: Instant,
    pub decision_type: String,
    pub confidence: f64,
    pub outcome: Option<bool>,
    pub reasoning_steps: u32,
}

/// Modelo interno do sistema sobre seu próprio desempenho
#[derive(Debug, Clone)]
pub struct SelfModel {
    /// Acurácia estimada por domínio
    pub domain_accuracy: hashbrown::HashMap<String, f64>,
    /// Latência estimada por tipo de operação
    pub latency_estimates: hashbrown::HashMap<String, f64>,
    /// Capacidades conhecidas
    pub capabilities: Vec<Capability>,
    /// Limitações reconhecidas
    pub limitations: Vec<String>,
}

/// Capacidade do sistema
#[derive(Debug, Clone)]
pub struct Capability {
    pub name: String,
    pub confidence: f64,
    pub evidence_count: u32,
}

impl Default for SelfModel {
    fn default() -> Self {
        Self {
            domain_accuracy: hashbrown::HashMap::new(),
            latency_estimates: hashbrown::HashMap::new(),
            capabilities: vec![
                Capability { name: "compression_analysis".into(), confidence: 0.9, evidence_count: 0 },
                Capability { name: "pattern_detection".into(), confidence: 0.8, evidence_count: 0 },
                Capability { name: "noise_filtering".into(), confidence: 0.85, evidence_count: 0 },
            ],
            limitations: vec![
                "Cannot process encrypted data effectively".into(),
                "Performance degrades with very small inputs (<16 bytes)".into(),
                "DSL levels 2-3 have limited implementation".into(),
            ],
        }
    }
}

impl MetaCognitionController {
    /// Cria novo controlador de meta-cognição
    pub fn new() -> Self {
        Self {
            prediction_history: VecDeque::with_capacity(META_HISTORY_SIZE),
            calibration_bins: Default::default(),
            prediction_counter: 0,
            k_meta_ewma: 0.5,
            k_anticipate_ewma: 0.5,
            coherence_ewma: 0.5,
            efficiency_ewma: 0.5,
            decision_history: VecDeque::with_capacity(META_HISTORY_SIZE),
            self_model: SelfModel::default(),
        }
    }

    /// Registra uma predição
    pub fn predict(&mut self, predicted: f64, confidence: f64, context: &str) -> u64 {
        self.prediction_counter += 1;
        let id = self.prediction_counter;
        
        let record = PredictionRecord {
            id,
            predicted,
            confidence: confidence.clamp(0.0, 1.0),
            actual: None,
            predicted_at: Instant::now(),
            observed_at: None,
            context: context.to_string(),
        };
        
        self.prediction_history.push_back(record);
        
        // Manter tamanho do histórico
        while self.prediction_history.len() > META_HISTORY_SIZE {
            self.prediction_history.pop_front();
        }
        
        id
    }

    /// Registra o valor real observado para uma predição
    pub fn observe(&mut self, prediction_id: u64, actual: f64) {
        if let Some(record) = self.prediction_history
            .iter_mut()
            .find(|r| r.id == prediction_id) 
        {
            record.actual = Some(actual);
            record.observed_at = Some(Instant::now());
            
            // Atualiza bins de calibração
            let bin_idx = (record.confidence * CALIBRATION_BINS as f64) as usize;
            let bin_idx = bin_idx.min(CALIBRATION_BINS - 1);
            
            let error = (record.predicted - actual).abs();
            let is_correct = error < 0.1 * actual.abs().max(1.0); // 10% threshold
            
            self.calibration_bins[bin_idx].count += 1;
            if is_correct {
                self.calibration_bins[bin_idx].correct += 1;
            }
            self.calibration_bins[bin_idx].total_error += error;
            
            // Atualiza modelo interno
            self.update_self_model(&record.context, error, is_correct);
        }
    }

    /// Registra uma decisão para análise de coerência
    pub fn record_decision(&mut self, decision_type: &str, confidence: f64, reasoning_steps: u32) {
        let record = DecisionRecord {
            timestamp: Instant::now(),
            decision_type: decision_type.to_string(),
            confidence,
            outcome: None,
            reasoning_steps,
        };
        
        self.decision_history.push_back(record);
        
        while self.decision_history.len() > META_HISTORY_SIZE {
            self.decision_history.pop_front();
        }
    }

    /// Registra outcome de decisão
    pub fn record_decision_outcome(&mut self, was_correct: bool) {
        if let Some(record) = self.decision_history.back_mut() {
            record.outcome = Some(was_correct);
        }
    }

    /// Calcula K_meta: competência meta-cognitiva geral
    /// K_meta = (coherence · efficiency · self_awareness · calibration)^(1/4)
    pub fn compute_k_meta(&mut self) -> f64 {
        let coherence = self.compute_coherence();
        let efficiency = self.compute_efficiency();
        let self_awareness = self.compute_self_awareness();
        let calibration = self.compute_calibration();
        
        let product = coherence * efficiency * self_awareness * calibration;
        
        let k_meta = if product > 0.0 {
            product.powf(0.25)
        } else {
            0.0
        };
        
        // EWMA update
        self.k_meta_ewma = EWMA_ALPHA * k_meta + (1.0 - EWMA_ALPHA) * self.k_meta_ewma;
        
        self.k_meta_ewma
    }

    /// Calcula K_anticipate: capacidade de antecipar resultados
    /// K_anticipate = corr(predicted, actual) · (1 - |avg_confidence - accuracy|)
    pub fn compute_k_anticipate(&mut self) -> f64 {
        let completed: Vec<_> = self.prediction_history
            .iter()
            .filter(|r| r.actual.is_some())
            .collect();
        
        if completed.len() < 5 {
            return 0.5; // Insufficient data
        }
        
        // Calcula correlação entre predito e real
        let n = completed.len() as f64;
        let predicted: Vec<f64> = completed.iter().map(|r| r.predicted).collect();
        let actual: Vec<f64> = completed.iter().map(|r| r.actual.unwrap()).collect();
        
        let mean_p = predicted.iter().sum::<f64>() / n;
        let mean_a = actual.iter().sum::<f64>() / n;
        
        let mut cov = 0.0;
        let mut var_p = 0.0;
        let mut var_a = 0.0;
        
        for i in 0..completed.len() {
            let dp = predicted[i] - mean_p;
            let da = actual[i] - mean_a;
            cov += dp * da;
            var_p += dp * dp;
            var_a += da * da;
        }
        
        let correlation = if var_p > 0.0 && var_a > 0.0 {
            cov / (var_p.sqrt() * var_a.sqrt())
        } else {
            0.0
        };
        
        // Normaliza correlação para [0,1]
        let correlation_norm = (correlation + 1.0) / 2.0;
        
        // Calcula diferença entre confiança média e acurácia
        let avg_confidence: f64 = completed.iter().map(|r| r.confidence).sum::<f64>() / n;
        let accuracy = self.compute_prediction_accuracy();
        let calibration_penalty = 1.0 - (avg_confidence - accuracy).abs();
        
        let k_anticipate = correlation_norm * calibration_penalty;
        
        // EWMA update
        self.k_anticipate_ewma = EWMA_ALPHA * k_anticipate + (1.0 - EWMA_ALPHA) * self.k_anticipate_ewma;
        
        self.k_anticipate_ewma
    }

    /// Coerência: consistência lógica entre decisões
    fn compute_coherence(&self) -> f64 {
        if self.decision_history.len() < 2 {
            return 0.5;
        }
        
        // Verifica se decisões similares têm confiança similar
        let mut consistency_sum = 0.0;
        let mut comparisons = 0;
        
        let decisions: Vec<_> = self.decision_history.iter().collect();
        
        for i in 0..decisions.len() {
            for j in (i+1)..decisions.len() {
                if decisions[i].decision_type == decisions[j].decision_type {
                    // Decisões do mesmo tipo devem ter confiança consistente
                    let conf_diff = (decisions[i].confidence - decisions[j].confidence).abs();
                    consistency_sum += 1.0 - conf_diff;
                    comparisons += 1;
                }
            }
        }
        
        if comparisons > 0 {
            consistency_sum / comparisons as f64
        } else {
            0.7 // Default when no comparisons possible
        }
    }

    /// Eficiência: menos passos para mesma qualidade de decisão
    fn compute_efficiency(&self) -> f64 {
        if self.decision_history.is_empty() {
            return 0.5;
        }
        
        // Decisões corretas com poucos passos = alta eficiência
        let correct_decisions: Vec<_> = self.decision_history
            .iter()
            .filter(|d| d.outcome == Some(true))
            .collect();
        
        if correct_decisions.is_empty() {
            return 0.3;
        }
        
        let avg_steps: f64 = correct_decisions.iter()
            .map(|d| d.reasoning_steps as f64)
            .sum::<f64>() / correct_decisions.len() as f64;
        
        // Ideal: 3-5 passos. Mais ou menos = menos eficiente
        let ideal_steps = 4.0;
        let efficiency = 1.0 / (1.0 + ((avg_steps - ideal_steps) / ideal_steps).abs());
        
        efficiency.clamp(0.0, 1.0)
    }

    /// Self-awareness: reconhecimento das próprias limitações
    fn compute_self_awareness(&self) -> f64 {
        // Baseado em quão bem o modelo interno prevê o desempenho real
        
        // 1. Verifica se reconhece limitações (ter limitações = bom)
        let has_limitations = !self.self_model.limitations.is_empty();
        
        // 2. Verifica se capabilities têm evidence
        let evidenced_capabilities: usize = self.self_model.capabilities
            .iter()
            .filter(|c| c.evidence_count > 0)
            .count();
        
        let capability_evidence_ratio = if !self.self_model.capabilities.is_empty() {
            evidenced_capabilities as f64 / self.self_model.capabilities.len() as f64
        } else {
            0.0
        };
        
        // 3. Verifica se domínios têm accuracy tracking
        let has_domain_tracking = !self.self_model.domain_accuracy.is_empty();
        
        let mut score = 0.0;
        if has_limitations { score += 0.3; }
        score += 0.4 * capability_evidence_ratio;
        if has_domain_tracking { score += 0.3; }
        
        score.clamp(0.0, 1.0)
    }

    /// Calibração: alinhamento entre confiança e acurácia
    fn compute_calibration(&self) -> f64 {
        let mut total_deviation = 0.0;
        let mut valid_bins = 0;
        
        for (i, bin) in self.calibration_bins.iter().enumerate() {
            if bin.count > 0 {
                // Confiança esperada para este bin
                let expected_confidence = (i as f64 + 0.5) / CALIBRATION_BINS as f64;
                // Acurácia real
                let actual_accuracy = bin.correct as f64 / bin.count as f64;
                
                total_deviation += (expected_confidence - actual_accuracy).abs();
                valid_bins += 1;
            }
        }
        
        if valid_bins > 0 {
            // Calibração perfeita = 1.0
            1.0 - (total_deviation / valid_bins as f64)
        } else {
            0.5 // No data
        }
    }

    /// Acurácia das predições
    fn compute_prediction_accuracy(&self) -> f64 {
        let completed: Vec<_> = self.prediction_history
            .iter()
            .filter(|r| r.actual.is_some())
            .collect();
        
        if completed.is_empty() {
            return 0.5;
        }
        
        let correct: usize = completed.iter()
            .filter(|r| {
                let actual = r.actual.unwrap();
                let error = (r.predicted - actual).abs();
                error < 0.1 * actual.abs().max(1.0)
            })
            .count();
        
        correct as f64 / completed.len() as f64
    }

    /// Atualiza modelo interno com base em observação
    fn update_self_model(&mut self, context: &str, error: f64, is_correct: bool) {
        // Extrai domínio do contexto
        let domain = context.split(':').next().unwrap_or("general");
        
        // Atualiza acurácia do domínio com EWMA
        let current = *self.self_model.domain_accuracy.get(domain).unwrap_or(&0.5);
        let new_accuracy = if is_correct { 1.0 } else { 0.0 };
        let updated = EWMA_ALPHA * new_accuracy + (1.0 - EWMA_ALPHA) * current;
        self.self_model.domain_accuracy.insert(domain.to_string(), updated);
    }

    /// Retorna se o sistema deve ser confiante na decisão atual
    pub fn should_be_confident(&self, context: &str, base_confidence: f64) -> (f64, String) {
        // Ajusta confiança baseado no histórico
        
        let domain = context.split(':').next().unwrap_or("general");
        
        // 1. Check domain accuracy
        let domain_modifier = self.self_model.domain_accuracy
            .get(domain)
            .copied()
            .unwrap_or(0.5);
        
        // 2. Check overall calibration
        let calibration = self.compute_calibration();
        
        // 3. Ajusta confiança
        let adjusted = base_confidence * domain_modifier * calibration;
        
        let reason = if domain_modifier < 0.5 {
            format!("Low domain accuracy for {}: {:.2}", domain, domain_modifier)
        } else if calibration < 0.5 {
            format!("Poor calibration: {:.2}", calibration)
        } else {
            "Confidence supported by history".to_string()
        };
        
        (adjusted.clamp(0.0, 1.0), reason)
    }

    /// Snapshot do estado meta-cognitivo
    pub fn snapshot(&self) -> MetaCognitionSnapshot {
        MetaCognitionSnapshot {
            k_meta: self.k_meta_ewma,
            k_anticipate: self.k_anticipate_ewma,
            coherence: self.coherence_ewma,
            efficiency: self.efficiency_ewma,
            calibration: self.compute_calibration(),
            prediction_count: self.prediction_counter,
            decision_count: self.decision_history.len(),
        }
    }
}

/// Snapshot do estado meta-cognitivo para logging/dashboard
#[derive(Debug, Clone)]
pub struct MetaCognitionSnapshot {
    pub k_meta: f64,
    pub k_anticipate: f64,
    pub coherence: f64,
    pub efficiency: f64,
    pub calibration: f64,
    pub prediction_count: u64,
    pub decision_count: usize,
}

impl Default for MetaCognitionController {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Testes
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prediction_tracking() {
        let mut mc = MetaCognitionController::new();
        
        // Faz predições
        let id1 = mc.predict(0.5, 0.8, "test:j_score");
        let id2 = mc.predict(0.3, 0.9, "test:j_score");
        
        // Observa resultados
        mc.observe(id1, 0.45); // Boa predição
        mc.observe(id2, 0.8);  // Má predição
        
        let accuracy = mc.compute_prediction_accuracy();
        assert!(accuracy >= 0.0 && accuracy <= 1.0);
    }
    
    #[test]
    fn test_k_meta_computation() {
        let mut mc = MetaCognitionController::new();
        
        // Adiciona algumas decisões
        for i in 0..10 {
            mc.record_decision("route", 0.7 + (i as f64 * 0.01), 4);
            mc.record_decision_outcome(i % 3 != 0); // 70% corretas
        }
        
        let k_meta = mc.compute_k_meta();
        assert!(k_meta >= 0.0 && k_meta <= 1.0);
    }
    
    #[test]
    fn test_calibration() {
        let mut mc = MetaCognitionController::new();
        
        // Predições com alta confiança que são corretas
        for i in 0..20 {
            let id = mc.predict(0.5, 0.9, "calibration_test");
            mc.observe(id, 0.48 + (i as f64 * 0.001)); // Perto do predito
        }
        
        let calibration = mc.compute_calibration();
        assert!(calibration > 0.5, "Expected good calibration for accurate predictions");
    }
    
    #[test]
    fn test_confidence_adjustment() {
        let mut mc = MetaCognitionController::new();
        
        // Adiciona histórico de erros em um domínio
        for _ in 0..10 {
            let id = mc.predict(0.5, 0.8, "bad_domain:test");
            mc.observe(id, 0.9); // Sempre errado
        }
        
        // Adiciona histórico de acertos em outro domínio
        for _ in 0..10 {
            let id = mc.predict(0.5, 0.8, "good_domain:test");
            mc.observe(id, 0.52); // Sempre certo
        }
        
        let (conf_bad, _) = mc.should_be_confident("bad_domain:new", 0.8);
        let (conf_good, _) = mc.should_be_confident("good_domain:new", 0.8);
        
        assert!(conf_bad < conf_good, "Bad domain should have lower confidence");
    }
}
