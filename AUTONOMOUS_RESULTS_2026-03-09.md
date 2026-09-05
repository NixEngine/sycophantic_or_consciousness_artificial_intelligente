# DACM-ERC-Spiral Autonomous Run (2026-03-09)

Generated: `2026-03-09T21:06:38.130625+00:00`

## Actions Performed
- Connected to PostgreSQL `database_zero` with `pgvector` and `rag.chunks` live.
- Benchmarked 5 parameter profiles x 6 real queries (30 runs).
- Selected best configuration by composite objective.
- Refined engine code (adaptive sparsification, ERC density, ranking dedup/diversity, UTF-8 output, env-driven DB config).
- Ran additional 8-query insight mining pass with tuned configuration.
- Synced refined engine to `C:\Users\vgrcj\VanderAI\dacm_erc_rag.py`.

## Benchmark Winner
- Best profile: `balanced_a`
- Ranking (avg_objective / avg_time_ms / avg_edge_density / avg_tri_density / avg_unique_docs_top10):
  - `balanced_a`: `0.3632` / `5828.4` / `0.375` / `0.120` / `4.83`
  - `balanced_b`: `0.3610` / `5501.2` / `0.242` / `0.045` / `5.67`
  - `sparse_a`: `0.3482` / `4906.2` / `0.182` / `0.026` / `5.67`
  - `sparse_b`: `0.3442` / `4817.2` / `0.154` / `0.019` / `5.83`
  - `legacy_dense`: `0.3330` / `5808.9` / `0.502` / `0.227` / `3.50`

## Tuned Defaults Applied in Code
- `edge_quantile=0.60`
- `max_neighbors=12`
- `top_per_doc=3`

## Key Observed Gains
- Reduced graph saturation versus dense mode (fewer clique-like edges/triangles).
- Better top-list diversity with deduplication and per-document cap.
- Preserved strong corridor quality while keeping runtime in practical range.

## Top Insight Candidates
1. `DoubleSHA256_Math_Framework_Integrated_v2_2026-02-11` | query=`how OEIS structures can optimize SHA-256 pipeline search` | score=`0.9581` | sim=`0.6016` | spiral=`9` | x△=`73` | novelty=`0.9398`
2. `DoubleSHA256_Math_Framework_Integrated_v2_2026-02-11` | query=`SHA-256 carry cascade E7 suffix degeneracy pattern` | score=`0.9898` | sim=`0.6130` | spiral=`9` | x△=`62` | novelty=`0.9387`
3. `MASTER_INTEGRACAO_CHAT_SHA256_SHA256D` | query=`hieratic symbol mapping uncertainties and computational symbol collisions` | score=`0.9073` | sim=`0.4920` | spiral=`13` | x△=`105` | novelty=`0.9222`
4. `Zero_Universe_Framework_Matematico_Consolidado_vZ0_1_2026-03-07` | query=`Gödel incompleteness and DACM corridor stability criteria` | score=`0.9213` | sim=`0.5562` | spiral=`7` | x△=`63` | novelty=`0.8944`
5. `MASTER_INTEGRACAO_CHAT_SHA256_SHA256D_V8` | query=`hieratic symbol mapping uncertainties and computational symbol collisions` | score=`0.9073` | sim=`0.4920` | spiral=`13` | x△=`94` | novelty=`0.8908`
6. `fused_framework_dacm_junior_sha_v3` | query=`Gödel incompleteness and DACM corridor stability criteria` | score=`0.9676` | sim=`0.5719` | spiral=`8` | x△=`51` | novelty=`0.8772`
7. `crypto_mining_corpus_dossier` | query=`hieratic symbol mapping uncertainties and computational symbol collisions` | score=`0.9212` | sim=`0.5049` | spiral=`14` | x△=`80` | novelty=`0.8640`
8. `crypto_mining_corpus_dossier` | query=`how OEIS structures can optimize SHA-256 pipeline search` | score=`0.8980` | sim=`0.5600` | spiral=`8` | x△=`67` | novelty=`0.8527`
9. `framework_dacm_erc_pli_acoplamento_rigor_matematico_v2` | query=`Gödel incompleteness and DACM corridor stability criteria` | score=`0.9217` | sim=`0.5741` | spiral=`7` | x△=`57` | novelty=`0.8412`
10. `MASTER_INTEGRACAO_CHAT_SHA256_SHA256D` | query=`Gödel incompleteness and DACM corridor stability criteria` | score=`0.9335` | sim=`0.5530` | spiral=`7` | x△=`56` | novelty=`0.8358`

## Artifacts
- Benchmark JSON: `benchmark_dacm_erc_spiral_results.json`
- Insights JSON: `insights_dacm_erc_spiral.json`
- Insights Markdown: `insights_dacm_erc_spiral.md`