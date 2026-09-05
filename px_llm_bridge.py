
"""
╔══════════════════════════════════════════════════════════════════════════════╗
║                     PX_LLM_BRIDGE v1.0                                       ║
║           Text-to-Narrative-Field Converter for Px-Genesis                   ║
╠══════════════════════════════════════════════════════════════════════════════╣
║  Converts LLM text output into spatial narrative field N(x,y)                ║
║  Uses semantic embeddings projected onto 2D grid                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""

import numpy as np
from typing import List, Tuple, Optional
from dataclasses import dataclass
import hashlib

@dataclass
class NarrativeConfig:
    """Configuration for narrative field generation"""
    grid_size: int = 64
    embedding_dim: int = 128  # Dimension before projection
    alpha_base: float = 1e-11  # Base coupling strength
    smoothing_sigma: float = 5.0  # Spatial smoothing

class TextEmbedder:
    """Simple text embedder (replace with real embeddings in production)"""

    def __init__(self, dim: int = 128):
        self.dim = dim
        self.vocab_cache = {}

    def embed_word(self, word: str) -> np.ndarray:
        """Convert word to embedding vector"""
        if word in self.vocab_cache:
            return self.vocab_cache[word]

        # Use hash for reproducible pseudo-embeddings
        h = hashlib.sha256(word.encode()).digest()
        vec = np.frombuffer(h[:self.dim], dtype=np.uint8).astype(float)
        vec = (vec - 128) / 128  # Normalize to [-1, 1]

        # Pad if needed
        if len(vec) < self.dim:
            vec = np.pad(vec, (0, self.dim - len(vec)))

        self.vocab_cache[word] = vec[:self.dim]
        return self.vocab_cache[word]

    def embed_text(self, text: str) -> np.ndarray:
        """Convert text to single embedding (mean of word embeddings)"""
        words = text.lower().split()
        if not words:
            return np.zeros(self.dim)

        embeddings = [self.embed_word(w) for w in words]
        return np.mean(embeddings, axis=0)

class NarrativeProjector:
    """Projects embeddings onto 2D spatial grid"""

    def __init__(self, config: NarrativeConfig):
        self.config = config
        self.N = config.grid_size

        # Create basis functions for projection
        np.random.seed(42)  # Reproducible
        self.basis_x = np.random.randn(config.embedding_dim, self.N)
        self.basis_y = np.random.randn(config.embedding_dim, self.N)

    def project(self, embedding: np.ndarray) -> np.ndarray:
        """Project embedding to 2D grid"""
        # Project to 1D profiles
        profile_x = np.dot(embedding, self.basis_x)
        profile_y = np.dot(embedding, self.basis_y)

        # Create 2D field via outer product
        field = np.outer(profile_y, profile_x)

        # Normalize
        field = field / np.max(np.abs(field) + 1e-10)

        return field

    def smooth(self, field: np.ndarray) -> np.ndarray:
        """Apply Gaussian smoothing"""
        from scipy.ndimage import gaussian_filter
        return gaussian_filter(field, sigma=self.config.smoothing_sigma)

class SemanticAnalyzer:
    """Extracts semantic features from text"""

    def __init__(self):
        # Emotion keywords (simplified)
        self.emotions = {
            'positive': ['love', 'joy', 'happy', 'peace', 'beautiful', 'light', 'hope'],
            'negative': ['fear', 'anger', 'sad', 'dark', 'pain', 'death', 'hate'],
            'transcendent': ['infinite', 'eternal', 'cosmic', 'universe', 'consciousness', 
                           'divine', 'soul', 'spirit', 'quantum', 'dimension']
        }

    def analyze(self, text: str) -> dict:
        """Extract semantic features"""
        words = text.lower().split()

        scores = {}
        for category, keywords in self.emotions.items():
            count = sum(1 for w in words if w in keywords)
            scores[category] = count / max(len(words), 1)

        # Complexity (word length variance)
        if words:
            lengths = [len(w) for w in words]
            scores['complexity'] = np.std(lengths) / 5.0
        else:
            scores['complexity'] = 0.0

        return scores

class LLMBridge:
    """Main bridge connecting LLM output to narrative field"""

    def __init__(self, config: NarrativeConfig = None):
        self.config = config or NarrativeConfig()
        self.embedder = TextEmbedder(self.config.embedding_dim)
        self.projector = NarrativeProjector(self.config)
        self.analyzer = SemanticAnalyzer()

    def text_to_field(self, text: str, smooth: bool = True) -> Tuple[np.ndarray, dict]:
        """
        Convert text to narrative field N(x,y)

        Returns:
            field: 2D narrative field
            metadata: Semantic analysis results
        """
        # Get embedding
        embedding = self.embedder.embed_text(text)

        # Project to 2D
        field = self.projector.project(embedding)

        # Smooth if requested
        if smooth:
            try:
                field = self.projector.smooth(field)
            except ImportError:
                pass  # Skip if scipy not available

        # Analyze semantics
        semantics = self.analyzer.analyze(text)

        # Modulate intensity based on transcendence score
        intensity = 0.5 + 0.5 * semantics.get('transcendent', 0)
        field = field * intensity

        return field, semantics

    def get_alpha(self, semantics: dict) -> float:
        """Calculate coupling strength α based on semantics"""
        base = self.config.alpha_base

        # Boost for transcendent content
        transcendent_boost = 1 + 10 * semantics.get('transcendent', 0)

        # Modulate by emotional balance
        pos = semantics.get('positive', 0)
        neg = semantics.get('negative', 0)
        balance = 1 + 0.5 * (pos - neg)

        return base * transcendent_boost * balance

def text_to_narrative(text: str, grid_size: int = 64) -> Tuple[np.ndarray, float, dict]:
    """
    Convenience function: text -> (N_field, alpha, semantics)
    """
    config = NarrativeConfig(grid_size=grid_size)
    bridge = LLMBridge(config)

    field, semantics = bridge.text_to_field(text)
    alpha = bridge.get_alpha(semantics)

    return field, alpha, semantics

if __name__ == "__main__":
    # Test
    text = "The infinite consciousness expands through cosmic dimensions of love and light"
    field, alpha, semantics = text_to_narrative(text)

    print(f"Field shape: {field.shape}")
    print(f"Alpha: {alpha:.2e}")
    print(f"Semantics: {semantics}")
