//! # Módulo de Síntese Evolutiva (px_evolve)
//! 
//! Usa algoritmo genético para evoluir transforms ótimos com base em fitness = -J
//! 
//! ## Vantagens sobre síntese heurística:
//! - Explora espaço de soluções de forma mais ampla
//! - Encontra soluções não-óbvias
//! - Auto-melhora com o tempo
//! - Pressão por simplicidade via ℓ(Φ)

use std::collections::HashMap;
use rand::prelude::*;
use uuid::Uuid;

/// Configuração do algoritmo evolutivo
#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    /// Tamanho da população
    pub population_size: usize,
    /// Taxa de crossover [0,1]
    pub crossover_rate: f64,
    /// Taxa de mutação [0,1]
    pub mutation_rate: f64,
    /// Tamanho do torneio para seleção
    pub tournament_size: usize,
    /// Máximo de gerações
    pub max_generations: usize,
    /// Pressão por simplicidade (penaliza ℓ(Φ) alto)
    pub simplicity_pressure: f64,
    /// Threshold de convergência (para early stopping)
    pub convergence_threshold: f64,
    /// Elitismo: quantos melhores passam direto
    pub elite_count: usize,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            crossover_rate: 0.8,
            mutation_rate: 0.15,
            tournament_size: 5,
            max_generations: 100,
            simplicity_pressure: 0.001,
            convergence_threshold: 0.001,
            elite_count: 2,
        }
    }
}

/// Gene: unidade básica de um transform
#[derive(Debug, Clone, PartialEq)]
pub enum Gene {
    /// Identidade (no-op)
    Identity,
    /// Substituição de padrão
    Replace { pattern: Vec<u8>, replacement: Vec<u8> },
    /// Constante (bytes fixos)
    Constant(Vec<u8>),
    /// Operação aritmética por byte
    ByteOp(ByteOperation),
    /// Expressão simbólica
    Symbolic(SymbolicGene),
    /// Filtro (manter bytes que satisfazem predicado)
    Filter(FilterPredicate),
    /// Map (aplicar função a cada byte)
    Map(MapFunction),
}

/// Operação aritmética em byte
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteOperation {
    Add(i16),
    Mul(f32),
    Xor(u8),
    And(u8),
    Or(u8),
    Shr(u8),
    Shl(u8),
    Neg,
}

/// Gene simbólico
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolicGene {
    Const(f64),
    X,  // Variável (byte atual)
    I,  // Índice
    Add(Box<SymbolicGene>, Box<SymbolicGene>),
    Mul(Box<SymbolicGene>, Box<SymbolicGene>),
    Sub(Box<SymbolicGene>, Box<SymbolicGene>),
    Div(Box<SymbolicGene>, Box<SymbolicGene>),
    Mod(Box<SymbolicGene>, Box<SymbolicGene>),
    Sin(Box<SymbolicGene>),
    Cos(Box<SymbolicGene>),
}

/// Predicado de filtro
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FilterPredicate {
    IsAscii,
    IsPrintable,
    IsDigit,
    IsAlpha,
    GreaterThan(u8),
    LessThan(u8),
    Equals(u8),
    NotEquals(u8),
}

/// Função de map
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapFunction {
    ToUpper,
    ToLower,
    Increment,
    Decrement,
    Double,
    Half,
    Negate,
}

/// Cromossomo: sequência de genes que forma um transform
#[derive(Debug, Clone)]
pub struct Chromosome {
    pub id: Uuid,
    pub genes: Vec<Gene>,
    pub fitness: f64,
    pub j_score: f64,
    pub description_length: f64,
    pub generation: usize,
}

impl Chromosome {
    pub fn new(genes: Vec<Gene>) -> Self {
        Self {
            id: Uuid::new_v4(),
            genes,
            fitness: 0.0,
            j_score: f64::MAX,
            description_length: 0.0,
            generation: 0,
        }
    }
    
    /// Aplica o cromossomo a dados
    pub fn apply(&self, data: &[u8]) -> Vec<u8> {
        let mut result = data.to_vec();
        
        for gene in &self.genes {
            result = apply_gene(&result, gene);
        }
        
        result
    }
    
    /// Calcula comprimento de descrição do cromossomo
    pub fn compute_description_length(&self) -> f64 {
        let mut length = 0.0;
        
        for gene in &self.genes {
            length += gene_description_length(gene);
        }
        
        length
    }
    
    /// Cria cromossomo aleatório
    pub fn random(rng: &mut impl Rng, max_genes: usize) -> Self {
        let num_genes = rng.gen_range(1..=max_genes);
        let genes: Vec<Gene> = (0..num_genes)
            .map(|_| random_gene(rng))
            .collect();
        
        Self::new(genes)
    }
}

/// Motor de evolução
pub struct EvolutionEngine {
    config: EvolutionConfig,
    population: Vec<Chromosome>,
    best_ever: Option<Chromosome>,
    generation: usize,
    rng: rand::rngs::StdRng,
    /// Hall of fame: melhores soluções encontradas
    hall_of_fame: Vec<Chromosome>,
    /// Histórico de fitness por geração
    fitness_history: Vec<f64>,
}

impl EvolutionEngine {
    pub fn new(config: EvolutionConfig, seed: u64) -> Self {
        Self {
            config,
            population: Vec::new(),
            best_ever: None,
            generation: 0,
            rng: rand::rngs::StdRng::seed_from_u64(seed),
            hall_of_fame: Vec::new(),
            fitness_history: Vec::new(),
        }
    }
    
    /// Inicializa população
    pub fn initialize(&mut self, initial_transforms: Option<Vec<Chromosome>>) {
        self.population.clear();
        
        // Adiciona transforms iniciais (da memória, se houver)
        if let Some(transforms) = initial_transforms {
            for t in transforms.into_iter().take(self.config.population_size / 2) {
                self.population.push(t);
            }
        }
        
        // Completa com cromossomos aleatórios
        while self.population.len() < self.config.population_size {
            let chromosome = Chromosome::random(&mut self.rng, 5);
            self.population.push(chromosome);
        }
    }
    
    /// Executa evolução para encontrar melhor transform
    pub fn evolve(&mut self, data: &[u8], lambda: f64) -> Chromosome {
        // Inicializa se necessário
        if self.population.is_empty() {
            self.initialize(None);
        }
        
        let mut prev_best_fitness = f64::NEG_INFINITY;
        let mut stagnation_count = 0;
        
        for gen in 0..self.config.max_generations {
            self.generation = gen;
            
            // Avalia fitness de toda população
            self.evaluate_fitness(data, lambda);
            
            // Ordena por fitness (maior = melhor)
            self.population.sort_by(|a, b| 
                b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal)
            );
            
            // Atualiza melhor
            if let Some(best) = self.population.first() {
                self.fitness_history.push(best.fitness);
                
                if self.best_ever.is_none() || best.fitness > self.best_ever.as_ref().unwrap().fitness {
                    self.best_ever = Some(best.clone());
                    self.hall_of_fame.push(best.clone());
                    stagnation_count = 0;
                } else {
                    stagnation_count += 1;
                }
                
                // Early stopping por convergência
                if (best.fitness - prev_best_fitness).abs() < self.config.convergence_threshold {
                    stagnation_count += 1;
                    if stagnation_count > 10 {
                        break;
                    }
                }
                prev_best_fitness = best.fitness;
            }
            
            // Cria próxima geração
            self.create_next_generation();
        }
        
        self.best_ever.clone().unwrap_or_else(|| Chromosome::new(vec![Gene::Identity]))
    }
    
    /// Avalia fitness de cada cromossomo
    fn evaluate_fitness(&mut self, data: &[u8], lambda: f64) {
        for chromosome in &mut self.population {
            // Aplica transform
            let transformed = chromosome.apply(data);
            
            // Calcula J-score
            let l_phi = chromosome.compute_description_length();
            let residual = compute_residual(data, &transformed);
            let j_score = l_phi + lambda * residual;
            
            chromosome.j_score = j_score;
            chromosome.description_length = l_phi;
            
            // Fitness = -J (queremos minimizar J, então maximizamos -J)
            // Com penalidade por complexidade
            chromosome.fitness = -j_score - self.config.simplicity_pressure * l_phi;
            chromosome.generation = self.generation;
        }
    }
    
    /// Cria próxima geração
    fn create_next_generation(&mut self) {
        let mut new_population = Vec::with_capacity(self.config.population_size);
        
        // Elitismo: mantém os melhores
        for elite in self.population.iter().take(self.config.elite_count) {
            new_population.push(elite.clone());
        }
        
        // Preenche resto com crossover e mutação
        while new_population.len() < self.config.population_size {
            // Seleção por torneio
            let parent1 = self.tournament_select();
            let parent2 = self.tournament_select();
            
            // Crossover
            let mut child = if self.rng.gen::<f64>() < self.config.crossover_rate {
                self.crossover(&parent1, &parent2)
            } else {
                parent1.clone()
            };
            
            // Mutação
            if self.rng.gen::<f64>() < self.config.mutation_rate {
                self.mutate(&mut child);
            }
            
            child.id = Uuid::new_v4();
            new_population.push(child);
        }
        
        self.population = new_population;
    }
    
    /// Seleção por torneio
    fn tournament_select(&mut self) -> Chromosome {
        let mut best: Option<&Chromosome> = None;
        
        for _ in 0..self.config.tournament_size {
            let idx = self.rng.gen_range(0..self.population.len());
            let candidate = &self.population[idx];
            
            if best.is_none() || candidate.fitness > best.unwrap().fitness {
                best = Some(candidate);
            }
        }
        
        best.unwrap().clone()
    }
    
    /// Crossover de dois cromossomos
    fn crossover(&mut self, parent1: &Chromosome, parent2: &Chromosome) -> Chromosome {
        // Single-point crossover
        if parent1.genes.is_empty() || parent2.genes.is_empty() {
            return parent1.clone();
        }
        
        let point1 = self.rng.gen_range(0..parent1.genes.len());
        let point2 = self.rng.gen_range(0..parent2.genes.len());
        
        let mut child_genes = Vec::new();
        child_genes.extend_from_slice(&parent1.genes[..point1]);
        child_genes.extend_from_slice(&parent2.genes[point2..]);
        
        // Limita tamanho
        child_genes.truncate(10);
        
        Chromosome::new(child_genes)
    }
    
    /// Mutação de cromossomo
    fn mutate(&mut self, chromosome: &mut Chromosome) {
        if chromosome.genes.is_empty() {
            return;
        }
        
        let mutation_type = self.rng.gen_range(0..5);
        
        match mutation_type {
            0 => {
                // Muta um gene existente
                let idx = self.rng.gen_range(0..chromosome.genes.len());
                chromosome.genes[idx] = random_gene(&mut self.rng);
            }
            1 => {
                // Adiciona um gene
                if chromosome.genes.len() < 10 {
                    let idx = self.rng.gen_range(0..=chromosome.genes.len());
                    chromosome.genes.insert(idx, random_gene(&mut self.rng));
                }
            }
            2 => {
                // Remove um gene
                if chromosome.genes.len() > 1 {
                    let idx = self.rng.gen_range(0..chromosome.genes.len());
                    chromosome.genes.remove(idx);
                }
            }
            3 => {
                // Troca ordem de dois genes
                if chromosome.genes.len() > 1 {
                    let idx1 = self.rng.gen_range(0..chromosome.genes.len());
                    let idx2 = self.rng.gen_range(0..chromosome.genes.len());
                    chromosome.genes.swap(idx1, idx2);
                }
            }
            _ => {
                // Mutação pontual em um gene
                if let Some(gene) = chromosome.genes.get_mut(0) {
                    *gene = mutate_gene(gene, &mut self.rng);
                }
            }
        }
    }
    
    /// Retorna estatísticas da evolução
    pub fn statistics(&self) -> EvolutionStats {
        let fitness_values: Vec<f64> = self.population.iter().map(|c| c.fitness).collect();
        
        let avg_fitness = fitness_values.iter().sum::<f64>() / fitness_values.len() as f64;
        let min_fitness = fitness_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_fitness = fitness_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        
        EvolutionStats {
            generation: self.generation,
            population_size: self.population.len(),
            avg_fitness,
            min_fitness,
            max_fitness,
            best_j_score: self.best_ever.as_ref().map(|c| c.j_score).unwrap_or(f64::MAX),
            hall_of_fame_size: self.hall_of_fame.len(),
        }
    }
}

/// Estatísticas da evolução
#[derive(Debug, Clone)]
pub struct EvolutionStats {
    pub generation: usize,
    pub population_size: usize,
    pub avg_fitness: f64,
    pub min_fitness: f64,
    pub max_fitness: f64,
    pub best_j_score: f64,
    pub hall_of_fame_size: usize,
}

// ============================================================================
// Funções auxiliares
// ============================================================================

fn apply_gene(data: &[u8], gene: &Gene) -> Vec<u8> {
    match gene {
        Gene::Identity => data.to_vec(),
        
        Gene::Replace { pattern, replacement } => {
            if pattern.is_empty() {
                return data.to_vec();
            }
            
            let mut result = Vec::new();
            let mut i = 0;
            
            while i < data.len() {
                if data[i..].starts_with(pattern) {
                    result.extend_from_slice(replacement);
                    i += pattern.len();
                } else {
                    result.push(data[i]);
                    i += 1;
                }
            }
            
            result
        }
        
        Gene::Constant(bytes) => bytes.clone(),
        
        Gene::ByteOp(op) => {
            data.iter().map(|&b| apply_byte_op(b, *op)).collect()
        }
        
        Gene::Symbolic(expr) => {
            data.iter()
                .enumerate()
                .map(|(i, &b)| {
                    let val = eval_symbolic(expr, b as f64, i as f64);
                    val.clamp(0.0, 255.0) as u8
                })
                .collect()
        }
        
        Gene::Filter(pred) => {
            data.iter()
                .filter(|&&b| apply_filter_predicate(b, *pred))
                .copied()
                .collect()
        }
        
        Gene::Map(func) => {
            data.iter().map(|&b| apply_map_function(b, *func)).collect()
        }
    }
}

fn apply_byte_op(b: u8, op: ByteOperation) -> u8 {
    match op {
        ByteOperation::Add(n) => ((b as i16 + n) % 256) as u8,
        ByteOperation::Mul(n) => ((b as f32 * n) % 256.0) as u8,
        ByteOperation::Xor(n) => b ^ n,
        ByteOperation::And(n) => b & n,
        ByteOperation::Or(n) => b | n,
        ByteOperation::Shr(n) => b >> n.min(7),
        ByteOperation::Shl(n) => b << n.min(7),
        ByteOperation::Neg => (!b).wrapping_add(1),
    }
}

fn eval_symbolic(expr: &SymbolicGene, x: f64, i: f64) -> f64 {
    match expr {
        SymbolicGene::Const(c) => *c,
        SymbolicGene::X => x,
        SymbolicGene::I => i,
        SymbolicGene::Add(a, b) => eval_symbolic(a, x, i) + eval_symbolic(b, x, i),
        SymbolicGene::Mul(a, b) => eval_symbolic(a, x, i) * eval_symbolic(b, x, i),
        SymbolicGene::Sub(a, b) => eval_symbolic(a, x, i) - eval_symbolic(b, x, i),
        SymbolicGene::Div(a, b) => {
            let denom = eval_symbolic(b, x, i);
            if denom.abs() < 1e-10 { 0.0 } else { eval_symbolic(a, x, i) / denom }
        }
        SymbolicGene::Mod(a, b) => {
            let denom = eval_symbolic(b, x, i);
            if denom.abs() < 1e-10 { 0.0 } else { eval_symbolic(a, x, i) % denom }
        }
        SymbolicGene::Sin(a) => eval_symbolic(a, x, i).sin(),
        SymbolicGene::Cos(a) => eval_symbolic(a, x, i).cos(),
    }
}

fn apply_filter_predicate(b: u8, pred: FilterPredicate) -> bool {
    match pred {
        FilterPredicate::IsAscii => b < 128,
        FilterPredicate::IsPrintable => b >= 32 && b < 127,
        FilterPredicate::IsDigit => b >= b'0' && b <= b'9',
        FilterPredicate::IsAlpha => (b >= b'A' && b <= b'Z') || (b >= b'a' && b <= b'z'),
        FilterPredicate::GreaterThan(t) => b > t,
        FilterPredicate::LessThan(t) => b < t,
        FilterPredicate::Equals(t) => b == t,
        FilterPredicate::NotEquals(t) => b != t,
    }
}

fn apply_map_function(b: u8, func: MapFunction) -> u8 {
    match func {
        MapFunction::ToUpper => {
            if b >= b'a' && b <= b'z' { b - 32 } else { b }
        }
        MapFunction::ToLower => {
            if b >= b'A' && b <= b'Z' { b + 32 } else { b }
        }
        MapFunction::Increment => b.wrapping_add(1),
        MapFunction::Decrement => b.wrapping_sub(1),
        MapFunction::Double => b.wrapping_mul(2),
        MapFunction::Half => b / 2,
        MapFunction::Negate => (!b).wrapping_add(1),
    }
}

fn gene_description_length(gene: &Gene) -> f64 {
    match gene {
        Gene::Identity => 1.0,
        Gene::Replace { pattern, replacement } => {
            (pattern.len() + replacement.len() + 2) as f64 * 8.0
        }
        Gene::Constant(bytes) => (bytes.len() + 1) as f64 * 8.0,
        Gene::ByteOp(_) => 16.0,
        Gene::Symbolic(expr) => symbolic_length(expr) * 8.0,
        Gene::Filter(_) => 16.0,
        Gene::Map(_) => 8.0,
    }
}

fn symbolic_length(expr: &SymbolicGene) -> f64 {
    match expr {
        SymbolicGene::Const(_) | SymbolicGene::X | SymbolicGene::I => 1.0,
        SymbolicGene::Add(a, b) | SymbolicGene::Mul(a, b) |
        SymbolicGene::Sub(a, b) | SymbolicGene::Div(a, b) |
        SymbolicGene::Mod(a, b) => 1.0 + symbolic_length(a) + symbolic_length(b),
        SymbolicGene::Sin(a) | SymbolicGene::Cos(a) => 1.0 + symbolic_length(a),
    }
}

fn compute_residual(original: &[u8], transformed: &[u8]) -> f64 {
    if original.is_empty() && transformed.is_empty() {
        return 0.0;
    }
    
    let max_len = original.len().max(transformed.len());
    let min_len = original.len().min(transformed.len());
    
    let mut diff = 0usize;
    for i in 0..min_len {
        diff += (original[i] as i32 - transformed[i] as i32).unsigned_abs() as usize;
    }
    
    // Penaliza diferença de tamanho
    diff += (max_len - min_len) * 255;
    
    diff as f64 / (max_len * 255).max(1) as f64 * (original.len() as f64 * 8.0)
}

fn random_gene(rng: &mut impl Rng) -> Gene {
    match rng.gen_range(0..7) {
        0 => Gene::Identity,
        1 => Gene::ByteOp(random_byte_op(rng)),
        2 => Gene::Filter(random_filter_predicate(rng)),
        3 => Gene::Map(random_map_function(rng)),
        4 => Gene::Symbolic(random_symbolic(rng, 3)),
        5 => {
            let pattern: Vec<u8> = (0..rng.gen_range(1..5))
                .map(|_| rng.gen())
                .collect();
            let replacement: Vec<u8> = (0..rng.gen_range(0..5))
                .map(|_| rng.gen())
                .collect();
            Gene::Replace { pattern, replacement }
        }
        _ => {
            let bytes: Vec<u8> = (0..rng.gen_range(1..10))
                .map(|_| rng.gen())
                .collect();
            Gene::Constant(bytes)
        }
    }
}

fn random_byte_op(rng: &mut impl Rng) -> ByteOperation {
    match rng.gen_range(0..8) {
        0 => ByteOperation::Add(rng.gen_range(-128..128)),
        1 => ByteOperation::Mul(rng.gen_range(0.1..2.0)),
        2 => ByteOperation::Xor(rng.gen()),
        3 => ByteOperation::And(rng.gen()),
        4 => ByteOperation::Or(rng.gen()),
        5 => ByteOperation::Shr(rng.gen_range(0..4)),
        6 => ByteOperation::Shl(rng.gen_range(0..4)),
        _ => ByteOperation::Neg,
    }
}

fn random_filter_predicate(rng: &mut impl Rng) -> FilterPredicate {
    match rng.gen_range(0..8) {
        0 => FilterPredicate::IsAscii,
        1 => FilterPredicate::IsPrintable,
        2 => FilterPredicate::IsDigit,
        3 => FilterPredicate::IsAlpha,
        4 => FilterPredicate::GreaterThan(rng.gen()),
        5 => FilterPredicate::LessThan(rng.gen()),
        6 => FilterPredicate::Equals(rng.gen()),
        _ => FilterPredicate::NotEquals(rng.gen()),
    }
}

fn random_map_function(rng: &mut impl Rng) -> MapFunction {
    match rng.gen_range(0..7) {
        0 => MapFunction::ToUpper,
        1 => MapFunction::ToLower,
        2 => MapFunction::Increment,
        3 => MapFunction::Decrement,
        4 => MapFunction::Double,
        5 => MapFunction::Half,
        _ => MapFunction::Negate,
    }
}

fn random_symbolic(rng: &mut impl Rng, max_depth: usize) -> SymbolicGene {
    if max_depth == 0 || rng.gen::<f64>() < 0.3 {
        // Terminal
        match rng.gen_range(0..3) {
            0 => SymbolicGene::Const(rng.gen_range(-100.0..100.0)),
            1 => SymbolicGene::X,
            _ => SymbolicGene::I,
        }
    } else {
        // Non-terminal
        match rng.gen_range(0..7) {
            0 => SymbolicGene::Add(
                Box::new(random_symbolic(rng, max_depth - 1)),
                Box::new(random_symbolic(rng, max_depth - 1))
            ),
            1 => SymbolicGene::Mul(
                Box::new(random_symbolic(rng, max_depth - 1)),
                Box::new(random_symbolic(rng, max_depth - 1))
            ),
            2 => SymbolicGene::Sub(
                Box::new(random_symbolic(rng, max_depth - 1)),
                Box::new(random_symbolic(rng, max_depth - 1))
            ),
            3 => SymbolicGene::Div(
                Box::new(random_symbolic(rng, max_depth - 1)),
                Box::new(random_symbolic(rng, max_depth - 1))
            ),
            4 => SymbolicGene::Mod(
                Box::new(random_symbolic(rng, max_depth - 1)),
                Box::new(random_symbolic(rng, max_depth - 1))
            ),
            5 => SymbolicGene::Sin(Box::new(random_symbolic(rng, max_depth - 1))),
            _ => SymbolicGene::Cos(Box::new(random_symbolic(rng, max_depth - 1))),
        }
    }
}

fn mutate_gene(gene: &Gene, rng: &mut impl Rng) -> Gene {
    match gene {
        Gene::ByteOp(op) => {
            // Pequena mudança no operador
            match op {
                ByteOperation::Add(n) => Gene::ByteOp(ByteOperation::Add(n + rng.gen_range(-10..10))),
                ByteOperation::Mul(n) => Gene::ByteOp(ByteOperation::Mul(n * (1.0 + rng.gen_range(-0.2..0.2)))),
                _ => Gene::ByteOp(random_byte_op(rng)),
            }
        }
        _ => random_gene(rng),
    }
}

// ============================================================================
// Testes
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evolution_basic() {
        let config = EvolutionConfig {
            population_size: 20,
            max_generations: 10,
            ..Default::default()
        };
        
        let mut engine = EvolutionEngine::new(config, 42);
        engine.initialize(None);
        
        let data = b"Hello World Hello World Hello";
        let best = engine.evolve(data, 0.001);
        
        assert!(best.j_score < f64::MAX);
        
        let stats = engine.statistics();
        assert!(stats.generation > 0);
    }
    
    #[test]
    fn test_chromosome_apply() {
        let chromosome = Chromosome::new(vec![
            Gene::Map(MapFunction::ToUpper),
        ]);
        
        let data = b"hello";
        let result = chromosome.apply(data);
        
        assert_eq!(result, b"HELLO");
    }
    
    #[test]
    fn test_evolution_improves() {
        let config = EvolutionConfig {
            population_size: 30,
            max_generations: 20,
            ..Default::default()
        };
        
        let mut engine = EvolutionEngine::new(config, 12345);
        engine.initialize(None);
        
        // Dados repetitivos devem ter bom J-score com bom transform
        let data = vec![42u8; 100];
        let best = engine.evolve(&data, 0.001);
        
        // Deve encontrar algo melhor que identidade aleatória
        assert!(best.j_score < 1000.0, "Evolution should find reasonable solution");
    }
}
