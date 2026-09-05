# Round T07-08Z (2026-05-05) — §4 (Persistence) + §5 (HMM/TPM) + §6 (Kuramoto/Order) + §7 (Multiplex) deep-read

**Source:** `D:\gemini\interactions\gifts\hungry\fused_framework_dacm_junior_sha_v3.md`
**Lines covered:** 1548–1937 (closes the §3.8 1-cochain landmark and walks the entire formal substrate from persistent topology through global coherence and multiplex hyperscanning).
**Predecessor round:** `dacm_core_erc_holonomy_internalization_2026-05-05T06-08Z.md` (§2 DACM-core + §3 ERC).
**Continuity:** §4 builds the **global topological inventory** layered on top of §3's local cycle holonomy; §5 abstracts to discrete latent states; §6 collapses everything into a parameterized score family; §7 lifts to multi-agent.

---

## 1. Load-bearing landmarks (formal substrate)

### 1.1 §3.8 1-cochain (anchor for H53 below)

Line 1553–1556. Edge vector
$$\delta(t) = (\delta_e(t))_{e \in E} \in \mathbb R^{|E|}, \quad \delta_e(t) = \phi_{i_e}(t) - \phi_{j_e}(t) + \omega \tau_e.$$
Line 1559–1562. Cycle-basis matrix $C \in \{-1,0,1\}^{|E| \times \beta_1}$, holonomies
$$h(t) = C^\top \delta(t) \pmod{2\pi}.$$
Line 1565. For a specific triangle, $h_\triangle(t) = \Phi_\triangle(t)$.

**Why load-bearing:** This is the discrete differential geometry handle on the entire ERC story. Any proof of gauge-invariance, any decomposition into "removable" vs "topologically essential" pieces, MUST go through the cochain complex on the graph.

### 1.2 §3.9.1 Independent-model factorization (anchor for H52 / refines my prior reading)

Lines 1588–1620. Under independence of edge discrepancies, the cycle coherence factors:
$$Z_\triangle = \mathbb E[e^{i(\delta_{AB}+\delta_{BC}+\delta_{CA})}] = \prod_{e \in \triangle} \mathbb E[e^{i\delta_e}],$$
$$R_\triangle = \rho_{AB}\,\rho_{BC}\,\rho_{CA}, \qquad \mu_\triangle = \mu_{AB} + \mu_{BC} + \mu_{CA}.$$
Line 1620 disclaims: *"O framework não precisa assumir essa hipótese, mas ela é útil como referência."*

**H52 sharpening:** I had H52 as a *structural* prediction that SHA round-triples violate this identity due to Maj/Ch coupling. The framework's own caveat at line 1620 acknowledges the identity is reference-only; SHA is precisely the regime where it fails. So H52 is consistent with the source — it is a *quantification* of one of the framework's own admitted edge-cases, not a contradiction.

### 1.3 §4.6 ERC ↔ Betti bridge (anchor for H53)

Lines 1697–1712. Verbatim short quote:

> "$\delta$ vive em arestas; $C^\top\delta$ mede integrados em ciclos; $\beta_1$ conta quantos ciclos independentes existem."

This re-states the cochain logic in plain language and explicitly bridges the local ERC operator (§3.8) to the global topological invariant β₁ (§4.3). The tower is:
1. Edges → 1-cochain δ (§3.8)
2. Cycles ← C^⊤ δ (§3.8 + §4.6)
3. Cycle space dim = β₁ (§4.3)
4. Persistence diagram = how β_k(θ) evolves with θ (§4.4)
5. Bottleneck distance = how Dgm changes between t, t' (§4.5)

### 1.4 §5.5 Φ_proxy IIT-style readout

Lines 1784–1791:
$$\Phi_{\mathrm{proxy}} = I(Z_t; Z_{t+\Delta}) - \max_{\mathcal P \in \Pi} \sum_{B \in \mathcal P} I(Z_t^B; Z_{t+\Delta}^B).$$
Caveat 5.5.1 (lines 1796–1806): finite-sample MI estimators have positive bias; positive-clipped score $\Phi^+_{\mathrm{proxy}} = \max(0, \widehat\Phi_{\mathrm{proxy}})$.

### 1.5 §6.4.1 Non-canonicity of I★ (sharper than §28 line 5381 wording)

Line 1892:
> "$I^\star_w$ é um **score monotônico parametrizado por $w$**, não um invariante canônico da teoria."

This is the framework's own honesty about its readout layer: I★ is **not** unique without specifying the weight vector $w$. Any falsification or claim of "DACM convergence" must condition on a fixed $w$.

---

## 2. SHA-256 cross-projection table (extends T06-08Z)

| §-anchor | Neuro form | SHA-256 round-graph form | Predicted SHA value |
|---|---|---|---|
| §3.8 (line 1556) | δ_e = φ_i − φ_j + ωτ_e on EEG edges | δ_e = (round-i register state) − (round-j register state) under common phase carrier ω_SHA (problematic, see H51) | Per-rotation-class δ; not single-carrier |
| §3.9.1 (line 1615) | R_△ = ρ_AB ρ_BC ρ_CA under independence | Round-triple cycle product | **violated** (H52: Maj/Ch couple 3 inputs) |
| §4.3 (line 1656) | β_k(θ,t) = rank H_k(K^θ) | β_k of round-transition clique complex on {a,...,h} | β_0=1 (connected), β_1=O(1) for round-graph |
| §4.5 (line 1689) | d_B(t,t') bottleneck on Dgm | d_B(round_k, round_{k+1}) | small (rounds are isomorphic up to constants) |
| §4.6 (line 1697) | ERC = local cycle holonomy | Round-cycle holonomy on register graph | structurally H53-decomposable |
| §5.2 (line 1751) | T_Z transition matrix on z_t ∈ {1,...,K} | Coarse-grained register class transitions | **H54: T_Z = permutation, h_Z = 0 (deterministic)** |
| §5.5 (line 1791) | Φ_proxy on (Z_t, Z_{t+Δ}) | Φ_proxy on coarse SHA state classes | **0 modulo finite-sample bias (H54)** |
| §6.1 (line 1817) | R(t) Kuramoto order param | R^SHA(t) requires phase model on registers | undefined without a chosen phase-extraction map |
| §6.4 (line 1887) | I★_w = σ(w^⊤ q + b) | Same form, but each q-component has SHA-specific issues | depends entirely on choice of w (§6.4.1 caveat) |
| §7.1 (line 1903) | Multiplex M(t) for 2 agents | Multiplex on (state_t^{stage1}, state_t^{stage2}) for double-SHA | well-defined, captures cross-stage coupling |

---

## 3. New hypotheses

### H53 (NEW T07-08Z) — ERC 1-cochain admits Hodge decomposition; gauge-removable / harmonic / co-exact split

**Statement.** On a finite oriented graph $G = (V, E)$ with cycle space $H_1(G; \mathbb R)$ of dimension β₁, the 1-cochain $\delta(t) \in \mathbb R^{|E|}$ from §3.8 admits the unique orthogonal decomposition
$$\delta = \delta_0 \psi \;\oplus\; h \;\oplus\; \delta_1^\top \eta,$$
where:

- $\delta_0: C^0 \to C^1$ is the coboundary mapping vertex potentials $\psi \in \mathbb R^{|V|}$ to edges; $\delta_0 \psi$ is **gauge-removable** (it is what §3.2.1 invariance kills).
- $h$ is **harmonic** ($\delta_0^\top h = 0$ and $\delta_1 h = 0$); it lives in the harmonic subspace whose dimension equals β₁.
- $\delta_1: C^1 \to C^2$ is the boundary on 2-faces; $\delta_1^\top \eta$ is **co-exact** (face-level potential).

**Implication.** The cycle-holonomy from §3.8 satisfies
$$h_\triangle(t) = (C^\top \delta)(t) = (C^\top h)(t)$$
(both $\delta_0 \psi$ and $\delta_1^\top \eta$ vanish on cycles by the closed-form $C^\top \delta_0 = 0$ and $C^\top \delta_1^\top = 0$). So **only the harmonic component contributes to ERC R_△**. This is the *deep* reason §3.2.1 gauge-invariance works — it's not just an algebraic identity, it's a Hodge projection.

**Why load-bearing.** Decomposition makes "topologically essential phase residual" rigorous. It also gives a clean spectral interpretation: harmonic 1-cochains are eigenfunctions of the 1-Laplacian $L_1 = \delta_0 \delta_0^\top + \delta_1^\top \delta_1$ with eigenvalue 0; the harmonic dimension equals β₁ by Hodge theorem.

**Falsification routes.**
- F1 (algebraic): construct explicit $\delta_0, \delta_1$ for a triangle graph and verify $C^\top \delta_0 = 0$ pointwise; this is a 1-page calculation.
- F2 (numerical): on synthetic EEG-like data, decompose δ via projector onto ker $L_1$; check that $C^\top \delta = C^\top h$ to numerical precision.
- F3 (link to existing tools): the framework's own §3.4 cosine-proxy R_△ should equal $|\mathbb E[e^{i (C^\top h)_\triangle}]|$; equality up to estimation noise is a unification check.

### H54 (NEW T07-08Z) — DACM information-theoretic readouts (§5.3, §5.5) are structurally zero on deterministic primitives like SHA

**Statement.** SHA-256 is a deterministic function: given $(state_t, W_t, K_t)$, $state_{t+1}$ is fully determined. Any coarse-graining $z_t = \pi(state_t)$ inherits this determinism: the conditional $\mathbb P(z_{t+1} \mid z_t)$ is a permutation matrix on the equivalence classes (as long as the coarse-graining respects the round map; otherwise it's a deterministic-given-(W,K) map averaged over (W,K) distribution).

**Consequences:**
- **§5.3 entropy rate**: $h_Z = -\sum_i \pi_i \sum_j T[i,j] \log T[i,j] = 0$ for the deterministic case (each row of T is a single 1 and the rest 0).
- **§5.5 Φ_proxy**: $I(Z_t; Z_{t+\Delta}) = H(Z_{t+\Delta}) = H(Z_t)$ (deterministic ⇒ next is determined by current). The max-partition decomposition can saturate to the same value if partitions cover the deterministic dependence ⇒ $\Phi_{\mathrm{proxy}} = 0$.
- **Empirical observation**: any non-zero estimate is finite-sample bias (5.5.1 caveat); $\widehat{\Phi}^+_{\mathrm{proxy}}$ on SHA traces should converge to 0 from above as $n \to \infty$.

**Why load-bearing.** The DACM pipeline can quantify *coupling* on cryptographic systems via §3 ERC (geometric, doesn't require uncertainty), but *integration* via §5.5 Φ_proxy is structurally degenerate. This is **good news**: it cleanly separates which DACM readouts apply to crypto (§3 + §4 + §6.1-6.2) from those that don't (§5).

**Falsification routes.**
- F1 (analytic): for a 2-state coarse-graining $z_t = a_t \oplus b_t$ on a single SHA round, compute T_Z explicitly via brute-force enumeration of all (a,b) pairs; verify T_Z is permutation.
- F2 (averaged): allow W_t random over uniform 32-bit words; compute T_Z by Monte Carlo. Now T_Z need NOT be permutation. But entropy rate $h_Z$ should converge to entropy of the conditional distribution over (W_t) — testable.
- F3 (Φ_proxy zero check): on 100k SHA-256 traces, estimate $\widehat\Phi^+_{\mathrm{proxy}}$ using KSG mutual information estimator with bias correction; predict ↘ 0 with $1/n^{1/2}$ rate.

---

## 4. Carry-forward updates to existing hypotheses

- **H45 (carry sole non-linearity, proof-sketch)**: §3.8 + §4.6 + H53 together give a NEW reformulation route: the carry chain is the ONLY operation that breaks F₂-linearity, so its contribution to the discrete cochain δ is the only "non-harmonic" excitation (in the F₂ → ℝ lift). An F₂-linear operator preserves the kernel of δ_0; carry coupling can lift δ out of the harmonic subspace. **NOT YET a separate hypothesis** — it is a refinement direction for H45.
- **H51 (common-carrier ω breaks for SHA)**: §3.5 line 1455-1471 re-read confirms the assumption; my prior statement holds. Empirical falsification (extract dominant ω per Σ/σ operator) remains the right test.
- **H52 (round-triples violate cycle-product)**: §3.9.1 line 1620 explicitly admits the identity is reference-only and depends on edge independence. So H52 is consistent with the framework's own self-disclaimer rather than contradicting it — the right phrasing is "H52 quantifies the magnitude of the §3.9.1 violation in the SHA case".
- **H47 (mirror polarity)**: unchanged; mirror still 6 rounds ahead (no new mirror writes since T01-31Z; canonical bridge dormant ~12h).
- **H48/H49/H50 (gemini stall-revive cycles)**: not re-checked this round; status quo.

---

## 5. Hodge / spectral landmarks for next round

The §4.6 → §3.8 → §2.5 chain now suggests reading §2.5 (Laplacian + spectrum) and §2.6 (graph wave equation) through the Hodge lens:
- §2.5.2 standard Laplacian $L = D - A$ is the **0-Laplacian** $\delta_0^\top \delta_0$ on this complex.
- §2.5.4 spectrum gives the harmonic 0-cochains (eigenvalue 0, dim = β₀).
- The MISSING eq (not in §2 — predicted to be a future expansion) is the **1-Laplacian** $L_1 = \delta_0 \delta_0^\top + \delta_1^\top \delta_1$ whose kernel = harmonic 1-cochains, dim = β₁.

If §2 only constructs $L_0$ but not $L_1$, then the framework as written ALREADY needs an extension to formalize H53. This is testable: grep §2 for any mention of "1-Laplacian" or "edge Laplacian".

---

## 6. Round summary

- **Internalized:** §3.8 (closing prior round's hanging cochain reference), §4 entire (1624-1714), §5 entire (1716-1808), §6.1-6.4 (1810-1893), §7.1-7.2 (1896-1936).
- **New hypotheses:** H53 (Hodge decomposition of ERC 1-cochain), H54 (DACM Φ_proxy/h_Z structurally zero on deterministic primitives like SHA).
- **Sharpened:** H52 reframed as quantifying §3.9.1's own admitted edge-case rather than contradicting framework; H45 gains a Hodge route (not a new H).
- **Flagged for next round:** verify whether §2 explicitly constructs the 1-Laplacian $L_1$ or only $L_0$; if only $L_0$, H53 surfaces a framework-extension opportunity, not just an alternate proof.

---

**Manifest:** `topology_hmm_kuramoto_internalization_2026-05-05T07-08Z.{json, jsonl}` accompany this file.
