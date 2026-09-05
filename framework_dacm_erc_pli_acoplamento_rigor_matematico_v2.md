# Framework Matemático Refinado — DACM, ERC, PLI, Acoplamento e Ponte SHA-256

**Versão:** rigor matemático pessoal  
**Base usada:** `bridge.zip` + framework consolidado anterior + análise textual fornecida pelo usuário  
**Objetivo:** reorganizar o material em uma forma matematicamente mais rigorosa, separando o que é definição, o que é aproximação, o que é proxy e o que é apenas parametrização fenomenológica.

---

## 0. Escopo e princípio organizador

Este documento preserva a arquitetura central do material anterior, mas introduz cinco correções estruturais:

1. **Separação formal entre *coherency* e *coherence*.**
2. **Correção da forma debiased de wPLI/dwPLI.**
3. **Separação explícita entre**
   - $R_\triangle \in [0,1]$ = módulo do fechamento holonômico,
   - $Q_\triangle \in [-1,1]$ = proxy cossenoidal estrutural.
4. **Tratamento do DoubleSHA256 do header Bitcoin como objeto de 3 estágios / 192 rounds**, e não 128 rounds.
5. **Normalizações e caveats matemáticos** para Laplacianos, TDA, HMM, estabilidade, leading zeros e derivadas logísticas em índice discreto.

A leitura recomendada é:

- **DACM-core (neuro / sinais / grafos / topologia / latentes)** = teoria principal;
- **DACM-operator / ERC-crypto (SHA-256)** = ramo formal separado, útil como álgebra de propagação, mas sem colapsar os significados da sigla DACM.

---

## 1. Convenções, notação e hipóteses mínimas

### 1.1 Convenções de índice

- $i,j,k$ indexam **nós / canais / ROIs**.
- $e=(i,j)$ indexa **arestas orientadas**.
- $\triangle=(A\to B\to C\to A)$ indexa **triângulos orientados**.
- $m$ indexa **janelas temporais**.
- $r$ indexa **rounds** de SHA-256 dentro de um estágio.
- $s$ indexa **estágios** de SHA256d.

### 1.2 Domínios

- Tempo contínuo: $t\in\mathbb R$.
- Amostras discretas: $t_n=n\Delta t$.
- Frequência: $f$ em Hz; $\omega=2\pi f$ em rad/s.
- Fase: sempre considerada **módulo $2\pi$**, salvo quando explicitamente “unwrap”.

### 1.3 Operadores básicos

- Transformada de Hilbert: $\mathcal H$.
- Parte real / imaginária: $\Re,\Im$.
- Conjugado complexo: $\overline{z}$.
- Esperança / média temporal / média por ensaios: $\mathbb E[\cdot]$ ou $\langle \cdot \rangle$, conforme o contexto.
- Função logística:
  $$
  \sigma(x)=\frac{1}{1+e^{-x}}.
  $$

### 1.4 Hipótese mínima para fase instantânea

A fase instantânea via Hilbert **não é universalmente bem definida** para qualquer sinal bruto. Ela faz sentido matemático e operacional quando, após filtragem, o sinal é **suficientemente narrowband / monocomponente** no intervalo estudado. Em prática teórica, assume-se:

$$
x_i^{(f)}(t) = a_i^{(f)}(t)\cos\theta_i^{(f)}(t) + \eta_i^{(f)}(t),
\qquad
\theta_i'(t)=\omega_i(t),
$$

com $a_i(t)$ e $\omega_i(t)$ variando mais lentamente que a portadora.

### 1.5 Hipótese de mascaramento de baixa amplitude

Se $A_i(t)\approx 0$, a fase $\phi_i(t)=\arg z_i(t)$ torna-se numericamente instável. Para uso rigoroso, convém trabalhar com um conjunto de tempos válidos

$$
\mathcal T_i=\{t:\, A_i(t)\ge \varepsilon_A\},
$$

e calcular métricas de fase apenas sobre $\mathcal T_i\cap\mathcal T_j$.

---

## 2. Núcleo matemático do DACM-core (sinais, fase, conectividade e grafos)

## 2.1 Modelo oscilatório local

Uma forma geral e suficientemente rica é

$$
x_i(t)=A_i(t)\cos\phi_i(t)+\eta_i(t),
\qquad
\dot\phi_i(t)=\omega_i(t).
$$

Versão com componente exógena explícita:

$$
x_i(t)=A_i(t)\cos\phi_i(t)+u_i(t)+\eta_i(t).
$$

Se houver necessidade de separar escala rápida e lenta:

$$
\phi_i(t)=\omega_{0,i}t+\vartheta_i(t),
\qquad
|\dot\vartheta_i(t)|\ll \omega_{0,i}.
$$

### 2.1.1 Sinal analítico

Após filtragem na banda de interesse:

$$
z_i(t)=x_i(t)+i\mathcal H\{x_i(t)\}=A_i(t)e^{i\phi_i(t)}.
$$

Logo,

$$
A_i(t)=|z_i(t)|,
\qquad
\phi_i(t)=\arg z_i(t),
\qquad
f_i(t)=\frac{1}{2\pi}\frac{d}{dt}\,\mathrm{unwrap}(\phi_i(t)).
$$

### 2.1.2 Observação importante

A fase instantânea é uma **coordenada derivada do observável filtrado**, não uma variável ontológica bruta. Em notação explícita:

$$
x_i^{\mathrm{raw}}(t)\xrightarrow{\text{filtro banda }f}x_i^{(f)}(t)\xrightarrow{\text{Hilbert}}z_i^{(f)}(t)\xrightarrow{\arg}\phi_i^{(f)}(t).
$$

---

## 2.2 Espectro cruzado, *coherency* e *coherence*

Se $X_i(f)$ é a transformada complexa do sinal do nó $i$, define-se o espectro cruzado:

$$
S_{ij}(f)=\mathbb E\!\left[X_i(f)\overline{X_j(f)}\right].
$$

### 2.2.1 Distinção crucial

A razão

$$
\gamma_{ij}(f):=\frac{S_{ij}(f)}{\sqrt{S_{ii}(f)S_{jj}(f)}}
$$

é a **coherency complexa**, não a coherence escalar.

A **coherence escalar** é

$$
\mathrm{Coh}_{ij}(f)=|\gamma_{ij}(f)|^2\in[0,1].
$$

Logo, o uso mais rigoroso é:

- $\gamma_{ij}(f)$ = coherency complexa;
- $\mathrm{Coh}_{ij}(f)$ = magnitude quadrática da coherency.

Essa distinção resolve uma das ambiguidades mais importantes do material anterior.

### 2.2.2 Parte imaginária

Duas convenções aparecem na prática:

1. **Parte imaginária da coherency**
   $$
   \mathrm{iCoh}_{ij}(f)=\Im\gamma_{ij}(f).
   $$

2. **Módulo da parte imaginária**
   $$
   \mathrm{aiCoh}_{ij}(f)=|\Im\gamma_{ij}(f)|.
   $$

A primeira conserva sinal; a segunda é não negativa. O sinal pode ser útil para orientação, mas a interpretação exige convenção temporal consistente.

---

## 2.3 Métricas de aresta por fase

## 2.3.1 Diferença de fase

Para cada par $(i,j)$:

$$
\Delta\phi_{ij}(t)=\phi_i(t)-\phi_j(t).
$$

Em janela $m$, com amostras $n=1,\dots,N_m$:

$$
\Delta\phi_{ij}^{(m)}(n)=\phi_i(t_n)-\phi_j(t_n).
$$

## 2.3.2 PLV — *Phase Locking Value*

Definição amostral:

$$
\widehat{\mathrm{PLV}}_{ij}^{(m)}
=
\left|
\frac{1}{N_m}
\sum_{n=1}^{N_m}
e^{i\Delta\phi_{ij}^{(m)}(n)}
\right|.
$$

Forma populacional:

$$
\mathrm{PLV}_{ij}
=
\left|
\mathbb E\left[e^{i\Delta\phi_{ij}}\right]
\right|.
$$

Propriedades:

- $\mathrm{PLV}_{ij}\in[0,1]$;
- $\mathrm{PLV}=1$ se $\Delta\phi$ é quase constante;
- $\mathrm{PLV}=0$ se $\Delta\phi$ é uniforme em $[0,2\pi)$.

### 2.3.2.1 Viés de amostra finita

Mesmo sob fase uniforme, $\widehat{\mathrm{PLV}}$ não é exatamente zero para $N_m$ finito. Um corretivo útil é o PPC (*pairwise phase consistency*):

$$
\mathrm{PPC}_{ij}
=
\frac{N_m}{N_m-1}
\left(
\widehat{\mathrm{PLV}}_{ij}^2-\frac{1}{N_m}
\right).
$$

PPC não é obrigatório para o framework, mas é uma forma rigorosa de lembrar que PLV sofre viés positivo finito-amostral.

## 2.3.3 PLI — *Phase Lag Index*

Forma canônica:

$$
\widehat{\mathrm{PLI}}_{ij}^{(m)}
=
\left|
\frac{1}{N_m}
\sum_{n=1}^{N_m}
\operatorname{sgn}\!\big(\sin \Delta\phi_{ij}^{(m)}(n)\big)
\right|.
$$

Forma equivalente:

$$
\operatorname{sgn}\!\big(\sin\Delta\phi\big)
=
\operatorname{sgn}\!\big(\Im(e^{i\Delta\phi})\big).
$$

Propriedades:

- $\mathrm{PLI}\in[0,1]$;
- anula contribuição de atrasos centrados simetricamente em zero;
- ignora magnitude do desfasamento: usa apenas o sinal do atraso.

## 2.3.4 wPLI — *weighted Phase Lag Index*

Se $Y_n=\Im\{X_{ij,n}\}$ é a parte imaginária do espectro cruzado instantâneo / por taper / por segmento, então

$$
\widehat{\mathrm{wPLI}}_{ij}
=
\frac{\left|\sum_{n=1}^{N} Y_n\right|}{\sum_{n=1}^{N}|Y_n|}.
$$

Forma equivalente:

$$
\widehat{\mathrm{wPLI}}_{ij}
=
\frac{\left|\sum_n |Y_n|\operatorname{sgn}(Y_n)\right|}{\sum_n |Y_n|}.
$$

Interpretação:

- pondera o sinal do atraso pela magnitude do conteúdo imaginário;
- reduz o peso de observações com atraso quase nulo e alta instabilidade de sinal.

## 2.3.5 dwPLI — forma debiased correta

A forma que deve ser tratada como referência matemática é a **estimativa debiased do quadrado de wPLI**:

$$
\widehat{\mathrm{dwPLI}}_{ij}^{\,2}
=
\frac{
\left(\sum_{n=1}^{N}Y_n\right)^2-\sum_{n=1}^{N}Y_n^2
}{
\left(\sum_{n=1}^{N}|Y_n|\right)^2-\sum_{n=1}^{N}Y_n^2
}.
$$

Forma equivalente em dupla soma:

$$
\widehat{\mathrm{dwPLI}}_{ij}^{\,2}
=
\frac{\sum_{n\neq m}Y_nY_m}{\sum_{n\neq m}|Y_nY_m|}.
$$

### 2.3.5.1 Observação muito importante

Em muitas implementações, o que é chamado de “dwPLI” é na verdade essa quantidade **debiased para $wPLI^2$**. Portanto:

- o estimador pode assumir valores ligeiramente negativos por não-viesamento;
- em uso operacional, às vezes se aplica
  $$
  \widehat{\mathrm{dwPLI}}_{+}=\sqrt{\max(0,\widehat{\mathrm{dwPLI}}^{\,2})}.
  $$

Isso precisa ficar explícito para evitar confusão entre intervalo $[0,1]$ e pequenas negatividades finito-amostrais.

## 2.3.6 iCoh — versão rigorosa

A versão mais limpa é

$$
\mathrm{iCoh}_{ij}(f)=\Im\gamma_{ij}(f)
=
\Im\left(
\frac{S_{ij}(f)}{\sqrt{S_{ii}(f)S_{jj}(f)}}
\right).
$$

Se for preferível trabalhar com uma quantidade não negativa, usar

$$
\mathrm{aiCoh}_{ij}(f)=|\Im\gamma_{ij}(f)|.
$$

### 2.3.6.1 Caveat sobre Laplaciano

Como $\mathrm{iCoh}_{ij}(f)$ pode ser **negativa**, não se deve alimentar diretamente um Laplaciano padrão $L=D-A$ sem escolher uma convenção:

- usar $A=|\mathrm{iCoh}|$;
- ou usar $A=\max(\mathrm{iCoh},0)$;
- ou trabalhar com **Laplaciano assinado**.

Esse ponto corrige outra fragilidade do framework anterior.

---

## 2.4 DACM como campo temporal de grafos ponderados

### 2.4.1 Janela móvel

Se a janela tem centro $t_m$, comprimento $L$ e passo $\Delta$, define-se

$$
W^{(f)}(t_m)=\big[w_{ij}^{(f)}(t_m)\big]_{i,j=1}^n,
$$

com

$$
w_{ij}^{(f)}(t_m)
=
\mathcal M_f\!\left(
x_i|_{[t_m-L/2,t_m+L/2]},
x_j|_{[t_m-L/2,t_m+L/2]}
\right),
$$

onde $\mathcal M_f$ pode ser PLV, PLI, wPLI, dwPLI, aiCoh etc.

### 2.4.2 Formulação compacta

$$
t_m\longmapsto W^{(f)}(t_m)
$$

é o objeto matemático mínimo da DACM: um **campo temporal de matrizes de conectividade**.

### 2.4.3 Hipótese de simetria

Para métricas de fase não direcionais:

$$
W^{(f)}(t_m)=W^{(f)}(t_m)^\top,
\qquad
w_{ii}^{(f)}(t_m)=0.
$$

Para métricas dirigidas, a matriz pode ser não simétrica; nesse caso, o operador espectral também deve mudar.

### 2.4.4 Condição de comprimento de janela

A regra “200–400 ms” não é universal. O critério matematicamente melhor é em número de ciclos:

$$
L \gtrsim \frac{n_c}{f_0},
\qquad
n_c\in[4,8]
$$

como regra de trabalho.

Consequências:

- para $f_0=33.3$ Hz, $L=0.2$ s contém $\approx 6.7$ ciclos, o que é aceitável;
- para $f_0=10$ Hz, $L=0.2$ s contém apenas 2 ciclos, o que já é curto para métricas estáveis de fase.

Portanto, o comprimento da janela deve ser **banda-dependente**.

### 2.4.5 Dependência serial entre janelas

Se janelas sobrepõem, então

$$
W(t_m)\not\perp W(t_{m+1}).
$$

Isso importa para:

- inferência estatística,
- HMM,
- estimação de TPM,
- cálculo de erro padrão.

A sobreposição aumenta resolução temporal, mas reduz tamanho amostral efetivo.

---

## 2.5 Camada de grafo: adjacência, Laplaciano e espectro

## 2.5.1 Adjacência

Se $W(t)$ é não negativa e simétrica, pode-se tomar

$$
A(t)=W(t).
$$

Se for necessário thresholding / normalização:

$$
A(t)=\mathcal N\big(W(t)\big),
$$

com $\mathcal N$ explicitando recorte, escala e simetrização.

## 2.5.2 Laplaciano padrão

$$
D_{ii}(t)=\sum_j A_{ij}(t),
\qquad
L(t)=D(t)-A(t).
$$

Versão normalizada:

$$
L_{\mathrm{sym}}(t)=I-D(t)^{-1/2}A(t)D(t)^{-1/2}.
$$

### 2.5.2.1 Quando o Laplaciano padrão vale

O formalismo espectral clássico exige, idealmente:

- $A_{ij}\ge 0$;
- $A=A^\top$.

Se isso falha, o uso de $\lambda_2$ como “Fiedler” precisa de reformulação.

## 2.5.3 Laplaciano assinado

Se $A=A^+-A^-$ com $A^\pm\ge 0$, uma opção rigorosa é

$$
L^\pm = D^+ + D^- - (A^+-A^-),
$$

com

$$
D^\pm_{ii}=\sum_j A^\pm_{ij}.
$$

Isso permite tratar conectividade com sinal.

## 2.5.4 Espectro

Para $L=L^\top$:

$$
L=U\Lambda U^\top,
\qquad
0=\lambda_1\le \lambda_2\le\cdots\le \lambda_n.
$$

Aqui:

- $\lambda_2$ é o **valor de Fiedler** em grafos conectados e não negativos;
- $\lambda_2$ mede a conectividade algébrica / custo de corte mínimo relaxado.

## 2.5.5 Transformada de Fourier no grafo (GFT)

Para um sinal nodal $x(t)\in\mathbb R^n$:

$$
\widehat x(t)=U^\top x(t).
$$

Energia total:

$$
\|x(t)\|_2^2=\sum_{k=1}^n |\widehat x_k(t)|^2.
$$

### 2.5.5.1 Energia em modos baixos

Se $\mathcal K_{\mathrm{low}}=\{1,\dots,K\}$:

$$
GSE_{\mathrm{low}}(t)
=
\frac{\sum_{k\in\mathcal K_{\mathrm{low}}}|\widehat x_k(t)|^2}
{\sum_{k=1}^{n}|\widehat x_k(t)|^2+\varepsilon}.
$$

A forma com $\widehat x_k^2$ sem módulo só é rigorosa se todos os coeficientes forem reais; $|\widehat x_k|^2$ é a forma geral correta.

### 2.5.5.2 Interpretação conjunta

- $\lambda_2$ = capacidade estrutural de integração;
- $GSE_{\mathrm{low}}$ = ocupação efetiva dos modos integrativos.

---

## 2.6 Equação de onda no grafo

Forma linear amortecida:

$$
\ddot x(t)+2\zeta\omega_0\dot x(t)+(\omega_0^2I+c_g^2L)x(t)=u(t)+\varepsilon(t).
$$

Se $L=L(t)$ varia no tempo:

$$
\ddot x(t)+2\zeta\omega_0\dot x(t)+(\omega_0^2I+c_g^2L(t))x(t)=u(t)+\varepsilon(t).
$$

### 2.6.1 Caveat de tempo variável

Se $L(t)$ muda de janela para janela, o sistema deixa de ser autônomo e várias propriedades espectrais simples deixam de ser estacionárias. Em teoria, convém tratar $L(t)$ como:

- **piecewise constant** por janela,
- ou **slowly varying** em relação à dinâmica de $x(t)$.

---

## 3. ERC-neuro como holonomia discreta em ciclos

## 3.1 Definição de discrepância de fase corrigida por atraso

Para uma aresta orientada $e=(i\to j)$ com atraso $\tau_{ij}\ge 0$, define-se

$$
\delta_{ij}(t)
=
\phi_i(t)-\phi_j(t-\tau_{ij}).
$$

Sob hipótese narrowband / frequência local lenta,

$$
\phi_j(t-\tau_{ij})
\approx
\phi_j(t)-\omega_j(t)\tau_{ij},
$$

logo

$$
\delta_{ij}(t)
\approx
\phi_i(t)-\phi_j(t)+\omega_j(t)\tau_{ij}.
$$

Se houver portadora comum $\omega$ na banda analisada:

$$
\delta_{ij}(t)
\approx
\phi_i(t)-\phi_j(t)+\omega\tau_{ij}.
$$

Essa é a forma mais rigorosa da “correção de atraso” usada no ERC.

## 3.2 Holonomia triangular

Para o ciclo orientado $\triangle=(A\to B\to C\to A)$:

$$
\Phi_\triangle(t)
=
\delta_{AB}(t)+\delta_{BC}(t)+\delta_{CA}(t).
$$

Na aproximação de portadora comum:

$$
\Phi_\triangle(t)
=
(\phi_A-\phi_B+\omega\tau_{AB})
+
(\phi_B-\phi_C+\omega\tau_{BC})
+
(\phi_C-\phi_A+\omega\tau_{CA}).
$$

Como as fases telescopam,

$$
(\phi_A-\phi_B)+(\phi_B-\phi_C)+(\phi_C-\phi_A)=0,
$$

logo, no caso idealizado,

$$
\Phi_\triangle(t)\approx \omega(\tau_{AB}+\tau_{BC}+\tau_{CA}).
$$

### 3.2.1 Invariância de gauge

Se todas as fases são deslocadas por uma fase comum $\chi(t)$:

$$
\phi_i(t)\mapsto \phi_i(t)+\chi(t),
$$

então $\Phi_\triangle(t)$ é invariante, pois o termo comum cancela no ciclo. Isso reforça a interpretação geométrica do ERC.

## 3.3 Fechamento circular completo

Define-se

$$
Z_\triangle
=
\left\langle e^{i\Phi_\triangle(t)}\right\rangle,
\qquad
R_\triangle=|Z_\triangle|,
\qquad
\mu_\triangle=\arg Z_\triangle.
$$

Propriedades:

- $R_\triangle\in[0,1]$;
- $\mu_\triangle\in(-\pi,\pi]$.

Interpretação:

- $R_\triangle$ = quão concentrada está a holonomia;
- $\mu_\triangle$ = em que ângulo médio ela se concentra.

### 3.3.1 Critério nominal

Um critério coerente com o material é

$$
R_\triangle>0.90,
\qquad
|\mu_\triangle|\le 10^\circ.
$$

Mas isso deve ser lido como **regra operacional**, não como teorema universal.

## 3.4 Proxy estrutural cossenoidal

Para reconciliar a forma circular com a forma reduzida usada em alguns relatórios, define-se

$$
Q_\triangle:=\Re(Z_\triangle)=R_\triangle\cos\mu_\triangle.
$$

Logo,

$$
Q_\triangle\in[-1,1].
$$

No caso ideal determinístico $R_\triangle\approx 1$:

$$
Q_\triangle
\approx
\cos\mu_\triangle
\approx
\cos(\omega\Sigma_\triangle\tau),
\qquad
\Sigma_\triangle\tau:=\tau_{AB}+\tau_{BC}+\tau_{CA}.
$$

Assim, a forma “cosseno do atraso total” não é o mesmo objeto que $R_\triangle$; ela é um **proxy da parte real** do fechamento.

### 3.4.1 Convenção recomendada

- $R_\triangle$ = módulo circular completo;
- $\mu_\triangle$ = offset angular;
- $Q_\triangle$ = proxy estrutural real.

Isso elimina a inconsistência entre faixas $[0,1]$ e $[-1,1]$.

## 3.5 Frequências de ressonância

A condição de fechamento estrutural é

$$
\mu_\triangle(\omega)\approx 2\pi m,
\qquad
m\in\mathbb Z.
$$

Na idealização com portadora comum:

$$
\omega\Sigma_\triangle\tau \approx 2\pi m.
$$

Portanto,

$$
f_m \approx \frac{m}{\Sigma_\triangle\tau}.
$$

### 3.5.1 Exemplo de 30 ms

Se

$$
\Sigma_\triangle\tau\approx 30\text{ ms}=0.03\text{ s},
$$

então

$$
f_1\approx \frac{1}{0.03}=33.3\text{ Hz}.
$$

Isso está matematicamente correto.

A forma
$$
f_1=\frac{1}{4\times 30\text{ ms}}=33.3\text{ Hz}
$$
é aritmeticamente incorreta; ela daria

$$
\frac{1}{0.12\text{ s}}=8.33\text{ Hz}.
$$

Portanto, a leitura consistente é mesmo $\Sigma_\triangle\tau\approx 30$ ms, não $4\times 30$ ms.

## 3.6 Largura de banda de ressonância

Se o critério angular admissível é

$$
|\mu_\triangle-2\pi m|\le \mu_0,
$$

então, usando $\mu_\triangle\approx 2\pi f\Sigma_\triangle\tau$,

$$
|f-f_m|
\le
\frac{\mu_0}{2\pi\Sigma_\triangle\tau}.
$$

Para $\mu_0=10^\circ=\pi/18$ e $\Sigma_\triangle\tau=0.03$ s:

$$
|f-f_m|
\le
\frac{\pi/18}{2\pi\cdot 0.03}
=
\frac{1}{36\cdot 0.03}
\approx 0.93\text{ Hz}.
$$

Essa é uma expansão útil do framework: a regra angular induz uma **janela de ressonância em frequência**.

## 3.7 Propagação de incerteza em atraso

Se $\Sigma_\triangle\tau$ é estimado com desvio padrão $\sigma_{\Sigma\tau}$, então

$$
f_m=\frac{m}{\Sigma_\triangle\tau}
\quad\Longrightarrow\quad
\sigma_{f_m}
\approx
\frac{m}{(\Sigma_\triangle\tau)^2}\,\sigma_{\Sigma\tau}.
$$

Se os atrasos de aresta são independentes:

$$
\sigma_{\Sigma\tau}^2
=
\sigma_{\tau_{AB}}^2+\sigma_{\tau_{BC}}^2+\sigma_{\tau_{CA}}^2.
$$

## 3.8 ERC como 1-cochain em grafo

Escolha uma orientação global das arestas. Defina o vetor de aresta

$$
\delta(t)=\big(\delta_e(t)\big)_{e\in E}
\in\mathbb R^{|E|},
\qquad
\delta_e(t)=\phi_{i_e}(t)-\phi_{j_e}(t)+\omega\tau_e.
$$

Se $C\in\{-1,0,1\}^{|E|\times \beta_1}$ é uma matriz de base de ciclos, então as holonomias de ciclo são

$$
h(t)=C^\top\delta(t)\quad (\mathrm{mod}\ 2\pi).
$$

Para um triângulo específico, $h_\triangle(t)=\Phi_\triangle(t)$.

### 3.8.1 Leitura geométrica

- $\delta$ é uma **1-cochain de fase-atraso** nas arestas;
- $C^\top\delta$ mede a integral ao longo dos ciclos;
- ERC é, portanto, uma teoria de **holonomia discreta** em complexos simpliciais / grafos.

Essa é uma das formulações matematicamente mais fortes de todo o framework.

## 3.9 Relação com PLV

PLV e ERC não são o mesmo funcional:

- PLV atua em **arestas**;
- ERC atua em **ciclos**.

Não existe teorema geral dizendo que eles são independentes ou ortogonais. O que se pode afirmar com rigor é:

1. eles medem **objetos diferentes**;
2. podem ser pouco correlacionados empiricamente;
3. essa baixa correlação é **resultado de modelo / dados**, não identidade matemática universal.

### 3.9.1 Modelo independente idealizado

Se, por hipótese simplificada, as discrepâncias de aresta $\delta_{AB},\delta_{BC},\delta_{CA}$ forem independentes e

$$
\mathbb E[e^{i\delta_{AB}}]=\rho_{AB}e^{i\mu_{AB}},
\quad
\mathbb E[e^{i\delta_{BC}}]=\rho_{BC}e^{i\mu_{BC}},
\quad
\mathbb E[e^{i\delta_{CA}}]=\rho_{CA}e^{i\mu_{CA}},
$$

então

$$
Z_\triangle
=
\mathbb E[e^{i(\delta_{AB}+\delta_{BC}+\delta_{CA})}]
=
\prod_{e\in\triangle}\mathbb E[e^{i\delta_e}],
$$

e portanto

$$
R_\triangle
=
\rho_{AB}\rho_{BC}\rho_{CA},
\qquad
\mu_\triangle=\mu_{AB}+\mu_{BC}+\mu_{CA}.
$$

Esse resultado mostra que, num modelo independente, a coerência de ciclo é um produto das coerências de aresta. O framework não precisa assumir essa hipótese, mas ela é útil como referência.

---

## 4. Topologia persistente como extensão global da ERC

## 4.1 De conectividade para filtragem

Se $W(t)$ é uma matriz de similaridade em $[0,1]$, pode-se definir a dissimilaridade

$$
D_{ij}(t)=1-W_{ij}(t).
$$

### 4.1.1 Caveat métrico

$D_{ij}=1-W_{ij}$ **não precisa** satisfazer a desigualdade triangular. Isso significa:

- como **dissimilaridade para clique filtration**, tudo bem;
- como **métrica no sentido geométrico estrito**, não necessariamente.

Por isso, a construção mais segura aqui é pensar em **flag/clique complex** de um grafo ponderado thresholdado, equivalente a uma filtragem tipo Vietoris–Rips em muitos usos computacionais, mas sem exigir métrica exata.

## 4.2 Filtragem por limiar de similaridade

Defina, para $\theta\in[0,1]$:

$$
A^\theta_{ij}(t)=\mathbf 1\{W_{ij}(t)\ge \theta\}.
$$

O complexo de cliques $K^\theta(t)$ é o complexo simplicial cujos $k$-símplices são subconjuntos totalmente conectados do grafo binário $A^\theta(t)$.

## 4.3 Números de Betti

$$
\beta_k(\theta,t)=\mathrm{rank}\,H_k(K^\theta(t);\mathbb F).
$$

Em particular:

- $\beta_0$ = número de componentes conexas;
- $\beta_1$ = número de ciclos independentes;
- $\beta_2$ e acima podem ser definidos, mas o backbone usa sobretudo $\beta_0,\beta_1$.

## 4.4 Persistência

Cada classe homológica nasce e morre ao longo da filtragem. O diagrama de persistência em dimensão $k$ é

$$
\mathrm{Dgm}_k(t)=\{(b_\ell,d_\ell)\}_\ell.
$$

Uma versão filtrada por persistência mínima é

$$
\beta_k^{(\pi_{\min})}(t)
=
\sum_{(b,d)\in\mathrm{Dgm}_k(t)}
\mathbf 1\{d-b\ge \pi_{\min}\}.
$$

## 4.5 Distância bottleneck

Entre duas janelas $t$ e $t'$:

$$
d_B(t,t')
=
W_\infty\big(\mathrm{Dgm}(t),\mathrm{Dgm}(t')\big).
$$

Interpretação:

- pequeno $d_B$ = topologia estável entre janelas;
- grande $d_B$ = novidade topológica / transição de regime.

## 4.6 Ponte ERC ↔ Betti

ERC triangular mede a holonomia de um **ciclo mínimo local**.

$\beta_1$ mede a dimensão do **espaço global de ciclos**.

Assim:

- ERC = geometria de fase-atraso sobre ciclos concretos;
- TDA = inventário topológico global das classes de ciclo.

Essa relação fica especialmente elegante via cochains:

- $\delta$ vive em arestas;
- $C^\top\delta$ mede integrados em ciclos;
- $\beta_1$ conta quantos ciclos independentes existem.

---

## 5. Estados latentes, TPM e integração operacional

## 5.1 Vetor de features multiescala

Uma forma consistente é

$$
f_t
=
[
\lambda_2(t),\,
GSE_{\mathrm{low}}(t),\,
R(t),\,
R_\triangle(t),\,
Q_\triangle(t),\,
\beta_0(t),\,
\beta_1(t),\,
d_B(t,t-\Delta),\,
\ldots
].
$$

## 5.2 HMM / estados discretos

Um modelo simples é

$$
z_t\in\{1,\dots,K\},
\qquad
f_t\mid z_t=k \sim \mathcal N(\mu_k,\Sigma_k).
$$

Matriz de transição:

$$
T_Z[i,j]=\mathbb P(z_{t+\Delta}=j\mid z_t=i).
$$

Distribuição estacionária:

$$
\pi^\top T_Z=\pi^\top,
\qquad
\sum_i \pi_i=1.
$$

## 5.3 Entropia de transição / taxa de entropia do processo latente

$$
h_Z
=
-\sum_i\pi_i\sum_j T_Z[i,j]\log T_Z[i,j].
$$

Essa é a forma matematicamente mais precisa do que chamar simplesmente de “entropia de transição”.

## 5.4 Sobreposição de janelas e Markovianidade

Se $f_t$ é calculado em janelas altamente sobrepostas, então a hipótese Markov simples em $z_t$ fica mais frágil. Em uso rigoroso, convém:

- reduzir a sobreposição para inferência de TPM,
- ou modelar dependência explícita,
- ou interpretar $T_Z$ como transição **operacional**, não necessariamente microscópica.

## 5.5 Proxy de irredutibilidade

Se o estado latente composto se decompõe como $Z_t=(Z_t^{(1)},\dots,Z_t^{(p)})$, define-se

$$
\Phi_{\mathrm{proxy}}
=
I(Z_t;Z_{t+\Delta})
-
\max_{\mathcal P\in\Pi}
\sum_{B\in\mathcal P}
I(Z_t^B;Z_{t+\Delta}^B).
$$

### 5.5.1 Caveat de estimação

Para amostras finitas, estimativas de informação mútua têm viés positivo. Em uso rigoroso, pode-se:

- usar regularização Bayesiana / Dirichlet,
- correção tipo Miller–Madow,
- ou calibrar por surrogates.

Além disso, $\widehat\Phi_{\mathrm{proxy}}$ pode sair negativa por erro de estimação; em score operacional, às vezes se usa

$$
\Phi_{\mathrm{proxy}}^{+}=\max(0,\widehat\Phi_{\mathrm{proxy}}).
$$

---

## 6. Ordem global, coerência estrutural e estabilidade

## 6.1 Parâmetro de ordem de Kuramoto

$$
R(t)e^{i\Psi(t)}
=
\frac{1}{n}\sum_{j=1}^{n} e^{i\phi_j(t)}.
$$

- $R(t)\in[0,1]$ mede alinhamento global;
- $\Psi(t)$ é a fase média.

## 6.2 Coerência estrutural normalizada por atraso

Uma forma mais robusta do que a média bruta é

$$
C_\Omega(t)
=
\frac{\sum_{i\neq j}\omega_{ij}(t)\cos(\Omega\tau_{ij})}
{\sum_{i\neq j}\omega_{ij}(t)},
\qquad
\omega_{ij}(t)\ge 0.
$$

Se $\omega_{ij}(t)=W_{ij}(t)$, então

$$
C_\Omega(t)\in[-1,1].
$$

Essa versão é preferível a uma média dividida por $n(n-1)$ quando os pesos não são homogêneos.

## 6.3 Estabilidade temporal

A forma

$$
S_{\mathrm{raw}}=1-\frac{\mathrm{Var}(R)}{\mathbb E[R]}
$$

é algebraicamente possível, mas não é naturalmente limitada a $[0,1]$.

Uma forma normalizada melhor é via coeficiente de variação:

$$
CV_R^2=\frac{\mathrm{Var}(R)}{\mathbb E[R]^2},
\qquad
S_{\mathrm{stab}}=\frac{1}{1+CV_R^2}\in(0,1].
$$

Essa definição preserva o espírito do índice de estabilidade, mas com melhor comportamento matemático.

## 6.4 Índice de integração como família de scores

Uma forma geral é

$$
q_t=
[
\widetilde R(t),\,
\widetilde C_\Omega(t),\,
\widetilde\lambda_2(t),\,
\widetilde GSE_{\mathrm{low}}(t),\,
\widetilde R_\triangle(t),\,
\widetilde Q_\triangle(t),\,
-\widetilde\beta_0(t),\,
\widetilde\beta_1(t),\,
\widetilde h_Z(t),\,
\widetilde\Phi_{\mathrm{proxy}}(t)
].
$$

E então

$$
I^\star_w(t)=\sigma(w^\top q_t+b).
$$

### 6.4.1 Interpretação rigorosa

$I^\star_w$ é um **score monotônico parametrizado por $w$**, não um invariante canônico da teoria. Isso é importante: sem escolha / ajuste / prior sobre $w$, não existe unicidade matemática do índice.

---

## 7. Hyperscanning e multiplex

## 7.1 Estrutura multiplex

Para dois participantes A e B:

$$
M(t)=
\begin{bmatrix}
A_A(t) & C_{AB}(t)\\
C_{BA}(t) & A_B(t)
\end{bmatrix}.
$$

Laplaciano supraestrutural:

$$
L_M(t)=D_M(t)-M(t).
$$

## 7.2 Estados latentes acoplados

$$
Z_{AB}(t)=(Z_A(t),Z_B(t)).
$$

Informação mútua:

$$
MI_{AB}=I(Z_A;Z_B).
$$

### 7.2.1 Forma mais rigorosa com controle de tarefa

Se ambos recebem drive comum $U$, a grandeza mais limpa é

$$
I(Z_A;Z_B\mid U).
$$

Isso separa sincronização “por tarefa compartilhada” de coordenação além do drive comum.

---

## 8. Ramo SHA-256 / DoubleSHA256 sob rigor matemático

## 8.1 Separação semântica obrigatória

Neste ramo:

- **DACM-core** não é a teoria principal;
- aparece um ramo separado em que DACM designa um operador de modulação / amplificação;
- **ERC-crypto** é um rótulo para modelos fenomenológicos de propagação.

O tratamento matemático correto é: **ramo separado, mas compatível por gramática formal de estado-propagação-observação**.

---

## 8.2 Álgebra básica da compressão SHA-256

## 8.2.1 Estado de round

Em cada round $r$ do estágio $s$:

$$
S_s(r)=(a_r,b_r,c_r,d_r,e_r,f_r,g_r,h_r)\in(\mathbb Z_{2^{32}})^8.
$$

## 8.2.2 Mensagem agendada

Para palavras $W_r$:

$$
W_r=
\begin{cases}
M_r, & 0\le r\le 15,\\[4pt]
\sigma_1(W_{r-2})\boxplus W_{r-7}\boxplus \sigma_0(W_{r-15})\boxplus W_{r-16},
& 16\le r\le 63.
\end{cases}
$$

com

$$
\sigma_0(x)=\mathrm{ROTR}^7(x)\oplus \mathrm{ROTR}^{18}(x)\oplus \mathrm{SHR}^3(x),
$$

$$
\sigma_1(x)=\mathrm{ROTR}^{17}(x)\oplus \mathrm{ROTR}^{19}(x)\oplus \mathrm{SHR}^{10}(x).
$$

## 8.2.3 Funções booleanas e somas do round

$$
\mathrm{Ch}(x,y,z)=(x\land y)\oplus(\neg x\land z),
$$

$$
\mathrm{Maj}(x,y,z)=(x\land y)\oplus(x\land z)\oplus(y\land z).
$$

Grandes sigmas:

$$
\Sigma_0(x)=\mathrm{ROTR}^2(x)\oplus \mathrm{ROTR}^{13}(x)\oplus \mathrm{ROTR}^{22}(x),
$$

$$
\Sigma_1(x)=\mathrm{ROTR}^6(x)\oplus \mathrm{ROTR}^{11}(x)\oplus \mathrm{ROTR}^{25}(x).
$$

Round update:

$$
T_1=h_r\boxplus \Sigma_1(e_r)\boxplus \mathrm{Ch}(e_r,f_r,g_r)\boxplus K_r\boxplus W_r,
$$

$$
T_2=\Sigma_0(a_r)\boxplus \mathrm{Maj}(a_r,b_r,c_r).
$$

Então

$$
a_{r+1}=T_1\boxplus T_2,\quad
b_{r+1}=a_r,\quad
c_{r+1}=b_r,\quad
d_{r+1}=c_r,
$$

$$
e_{r+1}=d_r\boxplus T_1,\quad
f_{r+1}=e_r,\quad
g_{r+1}=f_r,\quad
h_{r+1}=g_r.
$$

Esse bloco explicita o núcleo algébrico do qual partem todas as métricas de propagação.

---

## 8.3 DoubleSHA256 do header Bitcoin: 3 estágios / 192 rounds

Para um header de 80 bytes:

- a **primeira SHA-256** usa **2 blocos de 512 bits**;
- a **segunda SHA-256** usa **1 bloco de 512 bits**.

Logo:

$$
64+64+64=192\text{ rounds}.
$$

### 8.3.1 Estágios

Denote:

- $s=1a$: primeiro bloco da primeira passagem;
- $s=1b$: segundo bloco da primeira passagem;
- $s=2$: bloco único da segunda passagem.

Cada estágio possui rounds locais $r=0,\dots,63$.

Um índice global pode ser definido por

$$
R=
\begin{cases}
r, & s=1a,\\
64+r, & s=1b,\\
128+r, & s=2.
\end{cases}
$$

Essa é a forma correta de indexar o processo total.

---

## 8.4 Perturbação bit a bit e intensidades

## 8.4.1 Flip de bit

Se $x$ é a entrada e $e_i$ é o vetor unitário no bit $i$:

$$
x^{(i)}=x\oplus e_i.
$$

## 8.4.2 Intensidade por estágio e round

Se $S_s(r;x)$ e $S_s(r;x^{(i)})$ são os estados no mesmo estágio/round para entrada original e perturbada:

$$
I_s(i,r)
=
\frac{1}{256}
\operatorname{popcount}
\Big(
\mathrm{serState}(S_s(r;x))
\oplus
\mathrm{serState}(S_s(r;x^{(i)}))
\Big).
$$

Assim,

$$
I_s(i,r)\in[0,1].
$$

## 8.4.3 Intensidade por registrador

Para $R\in\{A,\dots,H\}$:

$$
I_{s,R}(i,r)
=
\frac{1}{32}
\operatorname{popcount}
\Big(
R_s(r;x)\oplus R_s(r;x^{(i)})
\Big).
$$

---

## 8.5 Modelo logístico como surrogate fenomenológico

## 8.5.1 Forma local por estágio

Uma parametrização coerente é

$$
I_s(i,r)\approx \sigma\!\big(k_s(r-r_{c,s}(i))\big).
$$

Aqui:

- $k_s>0$ = inclinação;
- $r_{c,s}(i)$ = round crítico do bit $i$ no estágio $s$.

### 8.5.1.1 Forma mais fiel ao mecanismo

A parametrização mais compatível com a estrutura do SHA é

$$
r_{c,s}(i)=t_{0,s}(i)+\Delta_s(i),
$$

onde

- $t_{0,s}(i)$ = primeiro round em que o bit pode influenciar o estágio;
- $\Delta_s(i)$ = atraso efetivo de difusão até meia saturação.

Isso é melhor que um modelo linear global ingênuo $r_c(i)=\alpha+\beta i$.

## 8.5.2 Derivadas em relação a round

Com $I=\sigma(k(r-r_c))$:

$$
\frac{\partial I}{\partial r}=kI(1-I),
$$

$$
\frac{\partial^2 I}{\partial r^2}=k^2I(1-I)(1-2I).
$$

O ponto de inflexão é

$$
I=\frac12
\quad\Longleftrightarrow\quad
r=r_c.
$$

## 8.5.3 Derivada em relação ao índice do bit: caveat

Como $i$ é discreto, a expressão

$$
\frac{\partial I}{\partial i}
=
-k\beta I(1-I)
$$

só faz sentido após **imersão contínua artificial** do índice do bit. Para rigor matemático, a quantidade primária é a diferença finita:

$$
\nabla_i I_s(i,r)=I_s(i+1,r)-I_s(i,r).
$$

A derivada contínua em $i$ deve ser lida apenas como surrogate suave.

## 8.5.4 Saturação

Se a saturação é definida por $I_s(i,r)\ge 1-\varepsilon$, então

$$
r_{sat,s}(i;\varepsilon)
=
r_{c,s}(i)+\frac{1}{k_s}\ln\frac{1-\varepsilon}{\varepsilon}.
$$

Se $r_{sat,s}>63$, o estágio não saturou na sua janela de 64 rounds.

---

## 8.6 SAC, BIC e difusão: formas corrigidas

## 8.6.1 Variáveis de flip por bit de saída

Para saída $F:\{0,1\}^n\to\{0,1\}^m$, defina

$$
\Delta_{i,j}(x)
=
F_j(x)\oplus F_j(x\oplus e_i)\in\{0,1\}.
$$

## 8.6.2 Strict Avalanche Criterion (SAC)

Forma por bit de saída:

$$
SAC_j(i)=\mathbb P_x[\Delta_{i,j}(x)=1].
$$

Critério ideal:

$$
SAC_j(i)=\frac12
\qquad
\forall i,j.
$$

### 8.6.2.1 Forma em distância de Hamming esperada

Equivalente:

$$
\mathbb E_x\big[w_H(F(x)\oplus F(x\oplus e_i))\big]
=
\frac{m}{2}.
$$

Para SHA-256, $m=256$, então o valor ideal é

$$
128.
$$

A igualdade “$=128$” só faz sentido nessa forma **não normalizada**. Se normalizar por $m$:

$$
\frac{1}{m}\mathbb E_x\big[w_H(F(x)\oplus F(x\oplus e_i))\big]=\frac12.
$$

## 8.6.3 Bit Independence Criterion (BIC)

Para $j\neq k$:

$$
BIC_{j,k}(i)
=
\mathrm{Corr}\big(\Delta_{i,j}(x),\Delta_{i,k}(x)\big).
$$

Critério ideal:

$$
BIC_{j,k}(i)\approx 0.
$$

## 8.6.4 Matriz probabilística de dependência

$$
D_p[o,i]
=
\mathbb P_x\big[\Delta_{i,o}(x)=1\big].
$$

Estimador Monte Carlo:

$$
\widehat D_p[o,i]
=
\frac{1}{M}
\sum_{m=1}^{M}
\mathbf 1\{\Delta_{i,o}(x^{(m)})=1\}.
$$

Essa é a forma mais rigorosa da “matriz de dependência” do framework.

---

## 8.7 Derivadas booleanas e carries

## 8.7.1 Derivada booleana

Sobre GF(2), para função booleana $f$:

$$
\frac{\partial f}{\partial x_i}(x)
=
f(x)\oplus f(x\oplus e_i).
$$

## 8.7.2 Derivadas de $\mathrm{Ch}$ e $\mathrm{Maj}$

Para $\mathrm{Ch}(x,y,z)$:

$$
\frac{\partial \mathrm{Ch}}{\partial x}=y\oplus z,
\qquad
\frac{\partial \mathrm{Ch}}{\partial y}=x,
\qquad
\frac{\partial \mathrm{Ch}}{\partial z}=\neg x.
$$

Para $\mathrm{Maj}(x,y,z)$:

$$
\frac{\partial \mathrm{Maj}}{\partial x}=y\oplus z,
\qquad
\frac{\partial \mathrm{Maj}}{\partial y}=x\oplus z,
\qquad
\frac{\partial \mathrm{Maj}}{\partial z}=x\oplus y.
$$

## 8.7.3 Adição módulo $2^{32}$ bit a bit

Para $x=\sum_i x_i2^i$ e $y=\sum_i y_i2^i$:

$$
c_0=0,
$$

$$
s_i=x_i\oplus y_i\oplus c_i,
$$

$$
c_{i+1}
=
(x_i\land y_i)\lor(x_i\land c_i)\lor(y_i\land c_i),
$$

$$
(x\boxplus y)_i=s_i.
$$

## 8.7.4 Probabilidade de overflow

Se $x,y$ são uniformes independentes em $\{0,\dots,2^{32}-1\}$:

$$
\mathbb P(x+y\ge 2^{32})
=
\frac{2^{32}-1}{2^{33}}
=
\frac12-\frac{1}{2^{33}}.
$$

## 8.7.5 Cadeias de carry

Defina os bits de geração e propagação:

$$
g_i=x_i y_i,
\qquad
p_i=x_i\oplus y_i.
$$

Então

$$
c_{i+1}=g_i\lor (p_i\land c_i).
$$

Condicionado em $c_i=1$ e com bits independentes uniformes:

- $c_{i+1}=0$ apenas se $(x_i,y_i)=(0,0)$, probabilidade $1/4$;
- portanto, a carry-chain continua com probabilidade $3/4$.

Se $L$ é o comprimento da cadeia a partir de uma injeção de carry, então, idealmente,

$$
\mathbb P(L=\ell)=\left(\frac34\right)^{\ell-1}\frac14,
\qquad \ell\ge 1,
$$

e

$$
\mathbb E[L]=4.
$$

Esse resultado é uma expansão nova e útil do ramo cripto: ele traduz a não linearidade por carries em uma estatística explícita.

## 8.7.6 Leitura geométrica

Fixado um vetor de carries $c=(c_0,\dots,c_{31})$, a soma é uma aplicação **afim em GF(2)**:

$$
s_i=x_i\oplus y_i\oplus c_i.
$$

Ou seja:

- em cada região de carry fixo, a adição é linear/afim;
- a não linearidade global vem da mudança do padrão de carry.

Isso formaliza a analogia com “catástrofes discretas”: as fronteiras entre regimes de carry são as superfícies de mudança estrutural do mapeamento.

---

## 8.8 Leading zeros: forma geral correta

## 8.8.1 Digest completo

Se o digest final (em ordem de bits significativa escolhida) é composto por palavras de 32 bits

$$
H=(h_0,h_1,\dots,h_7),
\qquad
h_q\in\{0,\dots,2^{32}-1\},
$$

e escrevemos

$$
K=32q+r,
\qquad
0\le r<32,
$$

então

$$
LZ(H)\ge K
$$

se e somente se

$$
h_0=h_1=\cdots=h_{q-1}=0
$$

e, quando $r>0$,

$$
h_q<2^{32-r}.
$$

Para $r=0$, basta que os primeiros $q$ words sejam zero.

### 8.8.1.1 Importância

A condição

$$
h_0<2^{32-K}
$$

só é suficiente para o caso **$K\le 32$**. Para $K>32$, ela é incompleta.

## 8.8.2 Probabilidade ideal

Se o hash é uniformemente distribuído em $\{0,1\}^{256}$:

$$
\mathbb P(LZ(H)\ge K)=2^{-K}.
$$

Isso serve como baseline matemático para qualquer teoria de “corredor” ou anomalia.

## 8.8.3 Subproblema do primeiro word

Se o foco é apenas o primeiro word de 32 bits e

$$
final\_h0=(H_{\mathrm{init}}[0]+a_{\mathrm{work}})\bmod 2^{32},
$$

então, para $K\le 32$,

$$
LZ(final\_h0)\ge K
\iff
final\_h0<2^{32-K}.
$$

Em termos de $a_{\mathrm{work}}$:

$$
a_{\mathrm{work}}
\in
\Big[-H_{\mathrm{init}}[0],\,-H_{\mathrm{init}}[0]+2^{32-K}-1\Big]
\pmod{2^{32}}.
$$

O conjunto favorável tem comprimento exatamente

$$
2^{32-K}.
$$

Essa formulação é mais precisa do que falar apenas em “faixa de overflow favorável”.

---

## 8.9 DACM-operator como operador híbrido

Se se quiser manter o ramo “Dynamic Amplification with Cyclic Modulation” de forma matematicamente limpa, uma forma abstrata é:

$$
\mathcal D_{\mathrm{DACM}}[x](t)
=
m(t)x(t)
+
\beta \frac{d}{dt}x(t)
+
\gamma \int_0^t K(t-\tau)x(\tau)\,d\tau,
$$

com modulador periódico

$$
m(t)=1+\alpha \varphi(2\pi f_{\mathrm{mod}}t+\psi).
$$

No contexto SHA, isso não é uma identidade algébrica do algoritmo; é um **meta-operador fenomenológico** para descrever amplificação / memória / modulação de influências.

Em particular, o ramo cripto continua mais seguro matematicamente quando expresso em:

- estados $S_s(r)$,
- dependências $D_p$,
- intensidades $I_s(i,r)$,
- carries,
- leading zeros,
- e não como redução literal da compressão a um sistema contínuo.

---

## 9. Síntese unificada: arestas, ciclos, topologia e propagação

## 9.1 Cadeia neuro

A formulação mais coerente do backbone é

$$
x_i(t)
\to
z_i(t)=A_i(t)e^{i\phi_i(t)}
\to
W(t,f)
\to
A(t),L(t)
\to
\lambda_2(t),\widehat x(t),GSE_{\mathrm{low}}(t)
\to
\Phi_\triangle(t),Z_\triangle,R_\triangle,\mu_\triangle,Q_\triangle
\to
\beta_0(t),\beta_1(t),d_B(t,t-\Delta)
\to
z_t,T_Z,h_Z,\Phi_{\mathrm{proxy}}(t)
\to
I^\star(t).
$$

## 9.2 Cadeia cripto

A cadeia mais rigorosa do ramo SHA é

$$
x
\to
x^{(i)}=x\oplus e_i
\to
S_s(r;x),S_s(r;x^{(i)})
\to
I_s(i,r),I_{s,R}(i,r)
\to
D_p[o,i],\mathrm{SAC},\mathrm{BIC}
\to
\text{carries, overflow, leading zeros}
\to
\text{scores e diagnósticos de propagação}.
$$

## 9.3 Gramática comum

As duas linhagens compartilham uma gramática abstrata:

1. **estado**;
2. **observação / serialização**;
3. **propagação**;
4. **agregação local**;
5. **estrutura global**;
6. **score / decisão**.

Mas os objetos concretos são diferentes:

- neuro: fase, atraso, grafos, ciclos;
- cripto: bits, rounds, carries, avalanche.

---

## 10. Correções e refinamentos incorporados nesta versão

### 10.1 Correções formais

1. **Coherency vs coherence**
   - Correto:
     $$
     \gamma_{ij}(f)=\frac{S_{ij}(f)}{\sqrt{S_{ii}(f)S_{jj}(f)}},
     \qquad
     \mathrm{Coh}_{ij}(f)=|\gamma_{ij}(f)|^2.
     $$

2. **dwPLI**
   - A forma correta debiased é
     $$
     \widehat{\mathrm{dwPLI}}^{\,2}
     =
     \frac{(\sum Y_n)^2-\sum Y_n^2}{(\sum |Y_n|)^2-\sum Y_n^2}.
     $$

3. **$R_\triangle$ vs proxy cossenoidal**
   - Não se deve usar o mesmo símbolo para
     - módulo circular $R_\triangle\in[0,1]$,
     - proxy real $Q_\triangle\in[-1,1]$.

4. **Janela temporal**
   - $200$–$400$ ms não é universal; a janela deve escalar com o número de ciclos:
     $$
     L\gtrsim n_c/f_0.
     $$

5. **GFT energy**
   - usar $|\widehat x_k|^2$, não $\widehat x_k^2$ em geral.

6. **Laplaciano**
   - iCoh com sinal pede convenção especial; não entrar cru em Laplaciano padrão.

7. **$\mathrm{VR}$ / clique filtration**
   - $D=1-W$ pode falhar como métrica, embora continue útil como dissimilaridade operacional.

8. **$S_{\mathrm{stab}}$**
   - a forma normalizada preferível é
     $$
     S_{\mathrm{stab}}=\frac{1}{1+CV_R^2}.
     $$

9. **SAC**
   - valor ideal “128” só faz sentido como distância de Hamming média em saída de 256 bits; normalizado, o ideal é $1/2$.

10. **Leading zeros**
    - a desigualdade numérica em um único word só resolve o caso $K\le 32$.

11. **DoubleSHA256**
    - header Bitcoin de 80 bytes implica 192 rounds totais:
      $$
      64+64+64.
      $$

12. **Derivada logística em índice de bit**
    - só é derivada genuína após embedding contínuo; primariamente é diferença finita.

### 10.2 Refinamentos novos

1. ERC como **holonomia discreta de uma 1-cochain**.
2. Fórmula para **largura de banda de ressonância**:
   $$
   |f-f_m|\le \mu_0/(2\pi\Sigma\tau).
   $$
3. Fórmula de **propagação de incerteza** para $f_m$.
4. Estatística explícita de **comprimento de carry-chain**:
   $$
   \mathbb E[L]=4
   $$
   sob bits uniformes independentes condicionados a carry de entrada.
5. Formulação geral correta de **leading zeros** para todo o digest.

---

## 11. Núcleo conceitual final

A forma mais econômica e matematicamente coerente da teoria fica:

$$
\boxed{
\text{DACM}=\text{dinâmica temporal das arestas}
}
$$

$$
\boxed{
\text{ERC}=\text{holonomia de fase-atraso em ciclos}
}
$$

$$
\boxed{
\beta_1=\text{dimensão topológica do espaço de ciclos}
}
$$

e, no ramo cripto,

$$
\boxed{
\text{propagação}=\text{difusão de influência por rounds, carries e avalanche}
}
$$

Com isso, a hierarquia completa pode ser lida como:

$$
\text{nós}
\to
\text{arestas}
\to
\text{ciclos}
\to
\text{topologia global}
\to
\text{macroestados}
\to
\text{score integrado}.
$$

No backbone neuro:

$$
\phi_i(t)
\to
W_{ij}(t)
\to
\Phi_\triangle(t)
\to
\beta_1(t)
\to
z_t
\to
I^\star(t).
$$

No backbone cripto:

$$
x
\to
x\oplus e_i
\to
S_s(r)
\to
I_s(i,r)
\to
D_p,\mathrm{SAC},\mathrm{BIC}
\to
\text{overflow / leading zeros}.
$$

---

## 12. Recomendação de notação canônica para desenvolvimento futuro

Para evitar ambiguidade no seu desenvolvimento teórico, a convenção mais limpa é:

- $\gamma_{ij}(f)$ = coherency complexa;
- $\mathrm{Coh}_{ij}(f)=|\gamma_{ij}(f)|^2$ = coherence;
- $\mathrm{iCoh}_{ij}(f)=\Im\gamma_{ij}(f)$;
- $\mathrm{aiCoh}_{ij}(f)=|\Im\gamma_{ij}(f)|$;
- $\mathrm{PLV}_{ij},\mathrm{PLI}_{ij},\mathrm{wPLI}_{ij},\mathrm{dwPLI}_{ij}^{\,2}$;
- $W(t,f)$ = matriz DACM;
- $L(t)$ = Laplaciano;
- $\lambda_2(t)$ = conectividade algébrica;
- $GSE_{\mathrm{low}}(t)$ = energia em modos baixos;
- $\Phi_\triangle(t)$ = holonomia triangular;
- $Z_\triangle=\langle e^{i\Phi_\triangle}\rangle$;
- $R_\triangle=|Z_\triangle|$;
- $\mu_\triangle=\arg Z_\triangle$;
- $Q_\triangle=\Re Z_\triangle$;
- $\beta_0,\beta_1$ = invariantes topológicos;
- $z_t$ = estado latente;
- $h_Z$ = taxa de entropia do HMM;
- $\Phi_{\mathrm{proxy}}$ = proxy de irredutibilidade;
- $I^\star_w$ = score de integração parametrizado.

No ramo SHA:

- $S_s(r)$ = estado no estágio $s$, round $r$;
- $I_s(i,r)$ = intensidade de influência;
- $D_p[o,i]$ = dependência probabilística;
- $\Delta_{i,j}(x)$ = flip do bit de saída;
- $\mathrm{SAC}_j(i)$, $\mathrm{BIC}_{j,k}(i)$;
- $LZ(H)$ = leading zeros do digest completo;
- $c_i,g_i,p_i$ = carry, generate, propagate.

---

## 13. Fecho

O ganho principal desta versão é simples, mas decisivo:

- a teoria fica mais clara quando **arestas, ciclos e topologia** são separados;
- o ERC ganha forma precisa como **holonomia discreta**;
- o DACM ganha forma precisa como **campo temporal de grafos**;
- o ramo SHA deixa de misturar analogia e álgebra, passando a ser tratado via **estados, perturbações, carries e critérios avalanche**;
- e os proxies globais passam a ser tratados como **scores parametrizados**, não invariantes mágicos.

Essa forma é suficientemente rigorosa para servir como base única de desenvolvimento teórico pessoal, sem perder a flexibilidade do seu framework original.
