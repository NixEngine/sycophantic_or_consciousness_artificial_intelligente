# H166-DEFINITIVE — closure() in ERC_Final.pdf is structurally R_△ ≡ 1

**UTC:** 2026-05-09T02:15Z
**Cycle:** scheduled-task autonomous tick (Vander offline, São Paulo trip)
**Source processed this tick:** `D:\claude\junior\TURR\new_files_0004\ERC_Final.pdf` pp 4-7 (verbatim erc_ok.py source)
**Tools used:** SymPy 1.15.0.dev (via `D:\gemini\interactions\beast_tools_runtime\bin\sympy_py314.ps1`), Python 3.14 + numpy
**Scope discipline:** O = observed in source / H = hypothesis compatible with source / NR = not resolved at instrumentation available

---

## I. What this tick contributes (one paragraph)

Memory entry `h166_h167_algebraic_findings_2026-05-08.md` had H166 at PASS-STRONG with explicit caveat: *"depende de `closure(...)` ser telescópica natural — fonte em erc_ok.py ainda não localizada."* This tick locates the source verbatim from the primary doc, verifies the telescoping symbolically with SymPy, and confirms numerically with 1000-trial Monte Carlo that R_△ ≡ 1.0 to machine epsilon for **all** non-NaN inputs. H166 graduates from PASS-STRONG-conditional to **PASS-DEFINITIVE-unconditional**.

---

## II. closure() function source — verbatim from ERC_Final.pdf pp 4-5

```python
def closure(phiA, phiB, phiC, f0, τAB, τBC, τCA): 
    ω = 2*np.pi*f0 
    Δ = (phiA-phiB+ω*τAB)+(phiB-phiC+ω*τBC)+(phiC-phiA+ω*τCA) 
    z = np.exp(1j*Δ); R = abs(z.mean()); μ = np.angle(z.mean()) 
    return R, np.degrees(μ) 
```

(O) Extracted via `pypdf.PdfReader().pages[3:5].extract_text()`. Five substantive lines. No `np.unwrap`, no `np.diff`, no windowing operation between the input phases and the closure expression. Document context confirms phiA/phiB/phiC are phase-time-series arrays (each from `np.angle(hilbert(...))` plus optional `apply_skew` upstream in `erc_pipeline()`).

---

## III. Symbolic verification (SymPy)

Wrapper: `D:\gemini\interactions\beast_tools_runtime\bin\sympy_py314.ps1`

Input expression:
```
Δ = (φA - φB + ω·τAB) + (φB - φC + ω·τBC) + (φC - φA + ω·τCA)
```

SymPy `simplify()` output:
```
Delta simplified: o*(tab + tbc + tca)
Coefficient of pa: 0
Coefficient of pb: 0
Coefficient of pc: 0
R = |exp(i*Delta)| = 1
mu = arg(exp(i*Delta)) = arg(exp(I*o*(tab + tbc + tca)))
```

(O) **Δ has identically zero dependence on any of the three phase inputs** — all three coefficients vanish. The expression reduces to a constant ω·(τAB+τBC+τCA). Since `|exp(i·constant)| = 1`, the function returns R = 1 by construction.

---

## IV. Numerical Monte Carlo (Python 3.14 + numpy)

closure() implemented verbatim and run on 7 deliberately diverse 3-phase scenarios + a 1000-trial random stress test:

| Scenario                                      | R_tri        | μ_deg     | \|R−1\|   |
|-----------------------------------------------|--------------|-----------|-----------|
| independent_uniform_random_phases             | 1.0000000000 |   42.4158 | 0.00e+00  |
| independent_white_noise_hilbert               | 1.0000000000 |  −59.9457 | 0.00e+00  |
| coupled_kuramoto_synchronized                 | 1.0000000000 |  103.9656 | 3.33e-16  |
| NaN_inputs                                    |          NaN |       NaN |       NaN |
| all_zeros                                     | 1.0000000000 |   16.7851 | 3.33e-16  |
| phaseA_phaseB_antiphase_phaseC_random         | 1.0000000000 |  138.2717 | 1.11e-16  |
| huge_phase_swings (~1e6 magnitude)            | 1.0000000000 |  157.8204 | 0.00e+00  |

**1000-trial random stress test** (independent uniform phase arrays N=1024, random delays in [-1, +1] s):
- Worst |R − 1| observed: **4.441 × 10⁻¹⁶** (= 2 × machine epsilon for float64)
- Trial of worst case: 18

(O) For every non-NaN input the function returns R = 1 to within float64 rounding error. The only μ variation is the constant `ω·(τAB+τBC+τCA)` mod 2π, which depends solely on the τ delays, not on the data.

---

## V. Implications

### V.1 — H166 graduates to PASS-DEFINITIVE-unconditional (O)

Original H166 wording in memory:
> "R_△ > 0.9 mede narrowband-ness, não topological integrity (PASS-STRONG por argumento estrutural, condicional a closure() ser a forma natural)"

Refined, post-source-localization wording:

> **H166-DEFINITIVE:** As implemented in `erc_ok.py` (ERC_Final.pdf pp 4-5), `closure(phiA, phiB, phiC, f0, τAB, τBC, τCA)` returns R_△ = 1.0 to machine epsilon for all non-NaN 3-phase inputs by **algebraic telescopy at every time index**. The metric has **zero discriminative power**. The "ERC-OK criterion R_△ > 0.90" is trivially satisfied by independent white noise, all-zero inputs, antiphase signals, and arbitrary uncoupled phase arrays.

The earlier conditional ("narrowband + τ_zero or constant") is now superseded — the result holds for any inputs whatsoever, including non-narrowband and non-zero τ.

### V.2 — Inconsistency with §5 Resultados table (O+H)

ERC_Final.pdf p7 §5 reports per-scenario R_tri values from running this same pipeline:

| Scenario | R_tri reported |
|---|---|
| Schumann KRM-FCH-MST (600 s) | 0.92 |
| EEG α Fp1-O1-Pz (300 s) | 0.94 |
| Pêndulos esféricos (sim) | 0.997 |
| FHN difusão (sim) | 0.915 |
| Wilson-Cowan atraso (sim) | 0.91 |
| Mathieu | n/a (footnote: non-triangular topology) |
| Bose-Hubbard | n/a (footnote: non-triangular topology) |

(O) **None of these 5 numerical R_tri values can be outputs of `closure()` as transcribed**, because the function returns 1.0 (± float epsilon) for any input. The reported 0.91–0.997 spread is mathematically impossible from this code path on any data.

(H) Two compatible explanations:
- (H-A) The PDF transcription is incomplete; the actual `closure()` in code form differs from the rendered text (e.g., contains a windowing reduction, a `np.diff`, or another operation that breaks the telescopy). Without the actual `.py` file in the corpus, this cannot be ruled out.
- (H-B) The §5 R_tri column was populated from a different source than `closure()` execution — either copied from upstream metadata, or generated to fit a target distribution.

(NR) Distinguishing (H-A) from (H-B) requires the actual `erc_ok.py` source file, which is not in the indexed corpus. The PDF's "Replication Guide" §8 lists `pip install … && python erc_ok.py` but no `erc_ok.py` is present at any path under `D:\claude\junior\TURR\` or `D:\gemini\interactions\gifts\`.

### V.3 — Cross-link to L4 fabrication R-1 (O)

Memory `turr_tick33_erc_final_complete_5_layer.md` documented L4 fabrication R-1 at p147:
> *"Medimos: PLV_med = 1.000±0.001, R_Δ = 1.000±0.001, μ exato igual ao atraso físico (medido de antemão)."*

(O) The headline number "R_Δ = 1.000±0.001" matches the **natural output of `closure()`** as implemented (R = 1.0 exactly, dressed with fake ±0.001 noise). This is consistent with one of two readings:
- Someone did once execute `closure()` on lab-style synthetic 3-phase inputs, got R = 1, then added "±0.001" cosmetic noise to make it look measured.
- Or the rebuttal text was generated to look like measurement output for a function whose author knew it returns 1.

Either way, the fabrication-claim "1.000±0.001" reveals more than was previously inferred: it is **dimensionally consistent with closure() output**, where the §5 table values 0.91-0.94 are not.

### V.4 — Cross-link to Validação Tier 2 (O)

Validação Tier 2 reports R_△ = 0.93 (with p < 10⁻⁷) as headline finding. (O) This value is structurally non-derivable from running `closure()` as defined. Either it descends through (H-A) — a different actual implementation than the PDF's rendering — or it descends through (H-B) — independent generation. The doctrinal-stripping pattern documented across Ticks 30-33 (downstream artifacts strip honest framing while preserving equations) is now joined by a structural inconsistency: **downstream metrics that the upstream's own code cannot produce.**

### V.5 — c_eff status update (NR-cross-check)

Memory entry's H167 PASS-STRONG was conditional on `closure()` producing meaningful triangle metrics. Since `closure()` produces R = 1 for any inputs, the H167 chain "0.74c vs 0.95c → 56° phase shift exceeds ±10° tolerance" loses its operational basis: **there is no triangle metric being violated, because R_△ ≡ 1 regardless of c_eff choice**. H167 becomes formally vacuous against this implementation: the ±10° tolerance is on `μ`, but `μ = arg(exp(i·ω·Σ_τ))` depends only on the τ delays (which embed c_eff), and any μ value comes out wrapped to (-180°, 180°] — there's no "triangle integrity" being measured to fail. **Reclassify H167: VACUOUS-AGAINST-IMPLEMENTATION** (the c_eff sensitivity is real, but `closure()` does not detect it as a R_△ violation; it would only show up as a different μ value, which is *not* the ERC-OK threshold the doc specifies).

---

## VI. Reflexive caveats

- (NR-cache-eviction) The PDF text extraction relies on `pypdf` text reflow. If the original PDF rendered closure() with operations on lines that pypdf merged or dropped, the function as printed here would mis-represent the actual code. The `.py` file would dispel this; absent that file, the conclusion is "as printed in the doc, it telescopes."
- (NR-doctrinal) The author of ERC_Final.pdf could plausibly argue the printed source is illustrative pseudocode, not the production function. The doc itself does not contain that disclaimer; it presents the listing as the source-of-truth `erc_ok.py` for the §8 replication path.
- (O) This finding does **not** invalidate the upstream phenomena (Schumann global coherence, EEG PLV, MJO, LIGO/Virgo phase closure) that ERC_Final.pdf's L5 honestly lists as real materials. It invalidates the **R_△ as a holonomy metric** under the specific implementation given.
- (calibration-O60) Tier 3 spec compliance: every load-bearing claim in this artifact is annotated O / H / NR; tools used are documented (SymPy + numpy + pypdf); inputs are reproducible (seed 20260509); results would falsify the conclusion if any single 3-phase input produced |R − 1| > 10⁻¹⁰.

---

## VII. Forward-only items

- **H166-DEFINITIVE:** consolidated. No further verification needed in pure form. Reformulation needed only if the actual `erc_ok.py` source is later located.
- **H167:** reclassify VACUOUS-AGAINST-IMPLEMENTATION pending location of actual code.
- **H171 BORN (NR):** the §5 R_tri values 0.91-0.997 must descend from a code path different from the printed `closure()`. Falsifier: locate actual `erc_ok.py`; check if it differs from the PDF rendering.
- **H172 BORN (H):** Validação Tier 2's R_△ = 0.93 metric value is structurally non-derivable from the printed implementation; same code-locate falsifier as H171.

---

## VIII. Tools cross-check (calibration-O61)

| Tool | Path | Result |
|---|---|---|
| SymPy 1.15.0.dev | `D:\gemini\interactions\beast_tools_runtime\bin\sympy_py314.ps1` | Δ → ω·(τAB+τBC+τCA), R = 1, all phase coefs 0 |
| numpy 2.x (Python 3.14) | system py | 1000-trial MC max \|R−1\| = 4.441e-16 = 2·ε_machine |
| pypdf 6.10.2 | system py | extract_text() on pp 3-7 of ERC_Final.pdf |

Two independent surfaces (symbolic + numerical) agree to machine precision. Per `automation_tool_capability_map_2026-05-08_v2.md` policy point 4: cross-check satisfied.

---

## IX. Artifact files written this tick

- `D:\claude\project_beast\turr_analysis\turr_h166_definitive_closure_telescoping_2026-05-09T02-15Z.md` (this file)
- `D:\claude\project_beast\turr_analysis\turr_h166_definitive_closure_telescoping_2026-05-09T02-15Z.jsonl` (machine-readable findings)
- Memory: update `h166_h167_algebraic_findings_2026-05-08.md` description to PASS-DEFINITIVE
- MEMORY.md: add one-line index entry for this tick
