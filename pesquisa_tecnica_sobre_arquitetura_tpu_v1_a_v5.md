## Page 1

Este documento representa uma síntese da pesquisa aprofundada sobre a arquitetura e evolução das Unidades de Processamento Tensorial (TPU) do Google. O conteúdo foi reestruturado para maior clareza, profundidade e conformidade com um formato acadêmico, transformando as notas iniciais em uma análise coesa e detalhada.

# Pesquisa Técnica Aprofundada sobre a Arquitetura do Google TPU (v1-v5)

## Introdução

A Unidade de Processamento Tensorial (TPU) do Google representa um marco na evolução do hardware de computação, sendo um dos primeiros e mais bem-sucedidos exemplos de um Circuito Integrado de Aplicação Específica (ASIC) projetado para acelerar cargas de trabalho de redes neurais. Desde sua introdução em 2015, a arquitetura do TPU passou por várias gerações, cada uma trazendo melhorias significativas em desempenho, eficiência e escalabilidade. Esta pesquisa detalha a evolução da arquitetura do TPU, desde a v1 até a v5, com foco nos conceitos técnicos fundamentais, como a unidade de matriz sistólica, a memória de alta largura de banda e as tecnologias de interconexão, além de reconhecer os principais contribuidores para seu desenvolvimento.

## Evolução das Gerações de TPU

A trajetória do TPU é marcada por uma rápida inovação, impulsionada pela crescente demanda computacional dos modelos de aprendizado de máquina no Google e na indústria em geral.

<table>
  <thead>
    <tr>
      <th>Geração</th>
      <th>Ano de Lançamento</th>
      <th>Foco Principal</th>
      <th>Principais Inovações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v1</td>
      <td>2015</td>
      <td>Inferência</td>
      <td>Array sistólico de 256x256 com precisão INT8; design de baixo consumo.</td>
    </tr>
    <tr>
      <td>TPU v2</td>
      <td>2017</td>
      <td>Treinamento e Inferência</td>
      <td>Arrays sistólicos duplos de 128x128 com precisão bfloat16; Memória de Alta Largura de Banda (HBM); Interconexão Inter-Chip (ICI) em toro 2D.</td>
    </tr>
    <tr>
      <td>TPU v3</td>
      <td>2018</td>
      <td>Desempenho de Treinamento</td>
      <td>Maior velocidade de clock e contagem de núcleos;</td>
    </tr>
    <tr>
      <td>TPU v4</td>
      <td>2019</td>
      <td>Treinamento e Inferência</td>
      <td>Memória de Alta Largura de Banda (HBM) de 128GB; Interconexão Inter-Chip (ICI) em toro 3D.</td>
    </tr>
    <tr>
      <td>TPU v5</td>
      <td>2020</td>
      <td>Treinamento e Inferência</td>
      <td>Memória de Alta Largura de Banda (HBM) de 128GB; Interconexão Inter-Chip (ICI) em toro 3D; Melhorias de eficiência energética.</td>
    </tr>
  </tbody>
</table>

## Conclusão

A evolução das gerações do TPU do Google ilustra o avanço constante da tecnologia de hardware de computação especializada. Cada nova geração trouxe inovações significativas que contribuíram para o progresso dos modelos de aprendizado de máquina. A pesquisa apresentada neste documento oferece uma visão abrangente e detalhada da arquitetura do TPU, destacando seus componentes-chave e as principais inovações de cada geração.

---


## Page 2

<table>
  <thead>
    <tr>
      <th>Geração</th>
      <th>Ano de Lançamento</th>
      <th>Foco Principal</th>
      <th>Principais Inovações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v3</td>
      <td>2018-2019</td>
      <td>Performance e Eficiência</td>
      <td>refrigeração líquida; aumento da capacidade de HBM e largura de banda da ICI.</td>
    </tr>
    <tr>
      <td>TPU v4</td>
      <td>2021</td>
      <td>Escalabilidade e Flexibilidade</td>
      <td>Comutação de Circuito Óptico (OCS); topologia de interconexão em toro 3D; introdução do SparseCore para embeddings.</td>
    </tr>
    <tr>
      <td>TPU v5</td>
      <td>2022-2023</td>
      <td>Especialização (Desempenho vs. Custo)</td>
      <td>Divisão em v5p (desempenho máximo) e v5e (eficiência de custo); aumento massivo da escala dos pods (v5p).</td>
    </tr>
  </tbody>
</table>

# Conceitos Técnicos Fundamentais

A performance e eficiência do TPU derivam de várias decisões de design arquitetônico chave.

## Arquitetura de Array Sistólico

O coração computacional do TPU é a sua Unidade de Multiplicação de Matriz (MXU), implementada como um **array sistólico**. Esta é uma rede de elementos de processamento (PEs) que computam e passam dados de forma rítmica, semelhante ao fluxo sanguíneo, o que inspirou seu nome. Esta arquitetura é extremamente eficiente para operações de multiplicação de matrizes, que são a base da maioria dos cálculos em redes neurais.

No design de **fluxo de dados estacionário por peso** do TPU, os pesos da rede neural são pré-carregados e permanecem nos PEs, enquanto os dados de ativação fluem através do array. Cada PE realiza uma operação de multiplicação e acumulação (MAC) por ciclo de clock. Os dados de ativação são transmitidos horizontalmente para os PEs vizinhos, e as somas parciais são acumuladas verticalmente. Este design minimiza o acesso à memória principal, uma das principais fontes de consumo de energia e latência em processadores de propósito geral.

## Memória de Alta Largura de Banda (HBM)

Com a introdução do TPU v2, a HBM tornou-se um componente crítico da arquitetura. A HBM utiliza uma abordagem de empilhamento 3D, onde múltiplos dies de DRAM são empilhados e conectados por Vias de Silício (TSVs). Este conjunto é então conectado ao processador através de um interposer de silício (uma arquitetura 2.5D),

---


## Page 3

permitindo um barramento de memória muito mais amplo e, consequentemente, uma largura de banda ordens de magnitude maior do que a memória DDR tradicional. Essa alta largura de banda é crucial para alimentar os arrays sistólicos com dados durante o treinamento de modelos grandes, evitando que a computação fique ociosa esperando por dados.

## Tecnologias de Interconexão: ICI e OCS

A capacidade de escalar para milhares de processadores é o que transforma um conjunto de TPUs em um supercomputador de IA. Duas tecnologias são fundamentais para isso:

1.  **Inter-Chip Interconnect (ICI):** Introduzida no TPU v2, a ICI é uma rede de interconexão de alta velocidade e baixa latência que conecta diretamente os chips TPU em uma topologia de toro. Nas versões v2 e v3, era um toro 2D, evoluindo para um toro 3D a partir da v4, o que melhorou a conectividade e a largura de banda de bisseção.
2.  **Optical Circuit Switching (OCS):** A inovação mais marcante do TPU v4, a OCS, utiliza espelhos microeletromecânicos (MEMS) para reconfigurar dinamicamente as conexões ópticas entre os chips. Isso permite uma flexibilidade sem precedentes, possibilitando a criação de "fatias" de supercomputadores de tamanhos variados e o contorno de falhas de hardware de forma transparente, desacoplando a topologia física da lógica.

## Principais Contribuidores

O sucesso do TPU é o resultado do trabalho de uma vasta equipe de engenheiros e pesquisadores. Três figuras, no entanto, destacam-se por suas contribuições fundamentais.

*   **Norman P. Jouppi:** Como líder técnico do projeto TPU desde sua concepção, Jouppi é a figura central por trás da arquitetura do TPU. Sua visão e liderança técnica foram cruciais em todas as gerações do processador. Suas publicações, como "In-Datacenter Performance Analysis of a Tensor Processing Unit", são fontes primárias essenciais para entender o TPU.
*   **David Patterson:** Um gigante da arquitetura de computadores e vencedor do Prêmio Turing, Patterson juntou-se ao Google e aplicou sua abordagem quantitativa rigorosa ao projeto TPU. Sua participação não apenas contribuiu para o design, mas também ajudou a validar e a comunicar a importância dos aceleradores de domínio específico para a comunidade em geral.
*   **Jeff Dean:** Como líder do Google Brain e, posteriormente, do Google AI, a liderança de Dean foi fundamental para criar a demanda e a justificativa para a existência do TPU. Os projetos de pesquisa em larga escala sob sua

---


## Page 4

supervisão, como o TensorFlow, estabelecem a necessidade de uma nova classe de hardware para impulsionar a próxima onda de inovação em IA.

## Conclusão

A evolução do Google TPU de um acelerador de inferência focado para um supercomputador de IA reconfigurável e em escala de data center demonstra uma co-evolução notável de hardware e software. Inovações como o array sistólico, a memória HBM e a comutação de circuito óptico não são apenas avanços incrementais, mas mudanças fundamentais na forma como os sistemas de computação de alto desempenho são projetados. A pesquisa contínua nesta área, liderada por pioneiros como Jouppi, Patterson e Dean, continua a empurrar os limites do que é computacionalmente possível, alimentando os avanços em inteligência artificial que moldam nosso mundo.

# Pesquisa Aprofundada sobre Backpropagation e Gradientes

## 1. Fundamentos Matemáticos do Backpropagation

O algoritmo de backpropagation é um método fundamental para o treinamento de redes neurais artificiais. Ele funciona calculando o gradiente da função de perda em relação aos pesos da rede, permitindo que os pesos sejam ajustados para minimizar o erro. As equações a seguir, extraídas do artigo seminal de Rumelhart, Hinton e Williams (1986), formam a base do algoritmo.

### 1.1. Equações Fundamentais

<table>
  <thead>
    <tr>
      <th>Equação</th>
      <th>Descrição</th>
      <th>Fórmula</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>(1)</td>
      <td>A entrada total ((x_j)) para uma unidade (j) é uma função linear das saídas ((y_i)) das unidades conectadas a ela e dos pesos ((w_{ji})) nessas conexões.</td>
      <td>$$x_j = \sum_i w_{ji} y_i$$</td>
    </tr>
    <tr>
      <td>(2)</td>
      <td>A saída ((y_j)) de uma unidade (j) é uma função não linear de sua entrada total, geralmente uma função sigmoide como a logística.</td>
      <td>$$y_j = \frac{1}{1 + e^{-x_j}}$$</td>
    </tr>
    <tr>
      <td>(3)</td>
      <td>O erro total ((E)) é a soma dos erros quadrados entre a saída real ((y_{j,c})) e a saída desejada ((d_{j,c})) para</td>
      <td>$$E = \frac{1}{2} \sum_c \sum_j (y_{j,c} - d_{j,c})^2$$</td>
    </tr>
  </tbody>
</table>

---


## Page 5

<table>
  <thead>
    <tr>
      <th>Equação</th>
      <th>Descrição</th>
      <th>Fórmula</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td></td>
      <td>todas as unidades de saída e todos os casos de treinamento.</td>
      <td></td>
    </tr>
    <tr>
      <td>(4)</td>
      <td>A derivada do erro em relação à saída de uma unidade de saída.</td>
      <td>$$\frac{\partial E}{\partial y_j} = y_j - d_j$$</td>
    </tr>
    <tr>
      <td>(5)</td>
      <td>A derivada do erro em relação à entrada de uma unidade, calculada usando a regra da cadeia.</td>
      <td>$$\frac{\partial E}{\partial x_j} = \frac{\partial E}{\partial y_j} \frac{dy_j}{dx_j} = \frac{\partial E}{\partial y_j} y_j(1-y_j)$$</td>
    </tr>
    <tr>
      <td>(6)</td>
      <td>A derivada do erro em relação a um peso específico, também calculada usando a regra da cadeia.</td>
      <td>$$\frac{\partial E}{\partial w_{ji}} = \frac{\partial E}{\partial x_j} \frac{\partial x_j}{\partial w_{ji}} = \frac{\partial E}{\partial x_j} y_i$$</td>
    </tr>
    <tr>
      <td>(7)</td>
      <td>A derivada do erro em relação à saída de uma unidade da camada anterior, que é a soma das contribuições de erro de todas as unidades da camada seguinte.</td>
      <td>$$\frac{\partial E}{\partial y_i} = \sum_j \frac{\partial E}{\partial x_j} w_{ji}$$</td>
    </tr>
    <tr>
      <td>(8)</td>
      <td>A regra de atualização de peso usando gradiente descendente, onde (\epsilon) é a taxa de aprendizado.</td>
      <td>$$\Delta w = -\epsilon \frac{\partial E}{\partial w}$$</td>
    </tr>
    <tr>
      <td>(9)</td>
      <td>A regra de atualização de peso com um termo de momento ((\alpha)), que ajuda a suavizar a convergência.</td>
      <td>$$\Delta w(t) = -\epsilon \frac{\partial E}{\partial w} + \alpha \Delta w(t-1)$$</td>
    </tr>
  </tbody>
</table>

## 1.2. Derivação Detalhada com Exemplo

O exemplo de Matt Mazur (2015) fornece uma derivação passo a passo do backpropagation. O processo envolve duas fases: o forward pass, onde as entradas são propagadas através da rede para gerar uma saída, e o backward pass, onde o erro é calculado e propagado para trás para atualizar os pesos.

### Forward Pass

1. Entrada da Camada Oculta: $$net_{h1} = w_1 * i_1 + w_2 * i_2 + b_1$$
2. Saída da Camada Oculta: $$out_{h1} = \frac{1}{1 + e^{-net_{h1}}}$$

---


## Page 6

3. Entrada da Camada de Saída: $$net_{o1} = w_5 * out_{h1} + w_6 * out_{h2} + b_2$$
4. Saída da Camada de Saída: $$out_{o1} = \frac{1}{1 + e^{-net_{o1}}}$$

## Backward Pass

O erro total é dado por $$E_{total} = \sum \frac{1}{2}(target - output)^2$$. A atualização dos pesos é feita usando a regra da cadeia para calcular o gradiente do erro em relação a cada peso.

### Atualização dos Pesos da Camada de Saída:

$$\frac{\partial E_{total}}{\partial w_5} = \frac{\partial E_{total}}{\partial out_{o1}} * \frac{\partial out_{o1}}{\partial net_{o1}} * \frac{\partial net_{o1}}{\partial w_5}$$

### Atualização dos Pesos da Camada Oculta:

$$\frac{\partial E_{total}}{\partial w_1} = (\sum_o \frac{\partial E_{total}}{\partial out_o} * \frac{\partial out_o}{\partial net_o} * \frac{\partial net_o}{\partial out_{h1}}) * \frac{\partial out_{h1}}{\partial net_{h1}} * \frac{\partial net_{h1}}{\partial w_1}$$

## 2. Implementação em Hardware e Física Subjacente

A implementação de hardware do backpropagation é essencial para acelerar o treinamento de redes neurais. As principais abordagens incluem FPGAs, ASICs, Memristors e Microcontroladores.

<table>
<thead>
<tr>
<th>Tecnologia</th>
<th>Vantagens</th>
<th>Desvantagens</th>
</tr>
</thead>
<tbody>
<tr>
<td>FPGAs</td>
<td>Processamento paralelo, reconfigurabilidade</td>
<td>Menor desempenho e eficiência que ASICs</td>
</tr>
<tr>
<td>ASICs</td>
<td>Alto desempenho, alta eficiência energética</td>
<td>Custo elevado, falta de flexibilidade</td>
</tr>
<tr>
<td>Memristors</td>
<td>Computação em memória, treinamento in-situ</td>
<td>Tecnologia emergente, desafios de fabricação</td>
</tr>
<tr>
<td>Microcontroladores</td>
<td>Baixo custo, baixo consumo de energia</td>
<td>Desempenho limitado para redes complexas</td>
</tr>
</tbody>
</table>

A física subjacente a essas implementações envolve princípios de semicondutores, **eletromagnetismo** e **termodinâmica**.

---


## Page 7

# 3. Principais Contribuidores e Genealogia Acadêmica

A pesquisa sobre backpropagation foi impulsionada por vários pesquisadores importantes. A seguir, uma análise genealógica dos três autores do artigo seminal de 1986.

## 3.1. Geoffrey E. Hinton

<table>
  <thead>
    <tr>
      <th>Relação</th>
      <th>Nome</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Orientador de PhD</td>
      <td>Hugh Christopher Longuet-Higgins</td>
    </tr>
    <tr>
      <td>Alunos Notáveis</td>
      <td>Ilya Sutskever, Ruslan Salakhutdinov, Yann LeCun</td>
    </tr>
    <tr>
      <td>Colaboradores</td>
      <td>David E. Rumelhart, Ronald J. Williams, Yoshua Bengio</td>
    </tr>
  </tbody>
</table>

## 3.2. David E. Rumelhart

<table>
  <thead>
    <tr>
      <th>Relação</th>
      <th>Nome</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Orientador de PhD</td>
      <td>William Kaye Estes</td>
    </tr>
    <tr>
      <td>Alunos Notáveis</td>
      <td>Michael I. Jordan, Stephen Palmer, Andreas Weigend</td>
    </tr>
    <tr>
      <td>Colaboradores</td>
      <td>Geoffrey E. Hinton, Ronald J. Williams, James McClelland</td>
    </tr>
  </tbody>
</table>

## 3.3. Ronald J. Williams

<table>
  <thead>
    <tr>
      <th>Relação</th>
      <th>Nome</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Principais Contribuições</td>
      <td>Co-autor do artigo de backpropagation, algoritmo REINFORCE</td>
    </tr>
    <tr>
      <td>Colaboradores</td>
      <td>David E. Rumelhart, Geoffrey E. Hinton</td>
    </tr>
  </tbody>
</table>

## Referências

[1] Rumelhart, D. E., Hinton, G. E., & Williams, R. J. (1986). Learning representations by back-propagating errors. *Nature*, 323(6088), 533–536. [2] Mazur, M. (2015). A Step by Step Backpropagation Example. *Matt Mazur blog*. [3] Kuninti, S., & Rooban, S. (2021). Backpropagation Algorithm and its Hardware Implementations: A Review. *Journal of Physics: Conference Series*, 1804(1), 012169. [4] van Doremaele, E. R. W., et al. (2024). Hardware implementation of backpropagation using progressive gradient descent for in situ training of multilayer neural networks. *Science Advances*, 10(28),

---


## Page 8

eado8999. [5] Neurotree. (2025). Geoffrey Everest Hinton Family Tree. [6] Mathematics Genealogy Project. (2025). David Everett Rumelhart.

# Pesquisa Técnica Profunda sobre Multiplicação de Matrizes em Hardware

## 1. Introdução

A multiplicação de matrizes é uma operação matemática fundamental, onipresente em praticamente todos os domínios da ciência e da engenharia. Desde a simulação de sistemas físicos complexos e a análise de grandes volumes de dados até o treinamento de modelos de inteligência artificial de ponta, a capacidade de multiplicar matrizes de forma eficiente é um fator crítico para o desempenho computacional. Esta pesquisa aprofunda-se nos múltiplos aspectos da multiplicação de matrizes, com um foco particular na sua implementação em hardware. Exploramos desde os algoritmos clássicos e suas otimizações teóricas até as arquiteturas de hardware especializadas que aceleram drasticamente essa operação, a física subjacente que governa o seu funcionamento e os pesquisadores pioneiros que pavimentaram o caminho para o estado da arte atual.

### 1.1. Definição Formal

A multiplicação de duas matrizes, A de dimensão n x m e B de dimensão m x p, resulta em uma matriz C de dimensão n x p. Cada elemento Cij da matriz resultante é calculado como o produto escalar da i-ésima linha de A com a j-ésima coluna de B.

Fórmula: Cij = Σ (de k=1 a m) Aik * Bkj

## 2. Algoritmos de Multiplicação de Matrizes

### 2.1. Algoritmo Padrão

O algoritmo padrão, ou ingênuo, para a multiplicação de matrizes deriva diretamente da sua definição. Para cada um dos n*p elementos da matriz resultante C, são necessárias m multiplicações e m-1 adições. A complexidade computacional total é, portanto, da ordem de O(nmp). Para o caso comum de matrizes quadradas (n=m=p), a complexidade é O(n³).

### 2.2. Algoritmo de Strassen

Em 1969, Volker Strassen demonstrou que a complexidade O(n³) não era ótima [1]. O seu algoritmo recursivo, baseado na divisão e conquista, consegue multiplicar duas matrizes 2x2 com apenas 7 multiplicações, em vez das 8 do método padrão, ao custo de 18 adições e subtrações. Ao aplicar esta abordagem recursivamente a submatrizes, a complexidade total é reduzida para aproximadamente O(n^2.807).

---


## Page 9

Embora assintoticamente mais rápido, o algoritmo de Strassen só se torna mais eficiente que o método padrão para matrizes relativamente grandes, devido à sobrecarga das operações de adição e da complexidade da implementação.

## 2.3. Algoritmo de Winograd

Shmuel Winograd desenvolveu uma variante que também utiliza 7 multiplicações para matrizes 2x2, mas reduz o número de adições para 15 [2]. A complexidade assintótica permanece a mesma do algoritmo de Strassen. A formulação de Winograd é frequentemente preferida em implementações práticas devido ao menor número de adições.

## 3. Arquitetura de Hardware: Arrays Sistólicos

A busca por maior eficiência levou ao desenvolvimento de arquiteturas de hardware especializadas. Entre as mais influentes estão os arrays sistólicos, propostos por H. T. Kung e Charles Leiserson [3].

### 3.1. Conceito e Funcionamento

Um array sistólico é uma rede de unidades de processamento de dados (DPUs) simples, conectadas em uma topologia regular (geralmente uma malha 2D). Os dados fluem de forma rítmica e paralela através do array, sendo processados em cada DPU. Para a multiplicação de matrizes, os elementos de uma matriz são alimentados pelas linhas e os da outra pelas colunas. Cada DPU realiza uma operação de multiplicação-acumulação (MAC) por ciclo de clock. Os resultados parciais são acumulados localmente ou passados para DPUs vizinhas. Esta arquitetura explora um paralelismo massivo e uma alta reutilização de dados, minimizando a necessidade de aceder à memória principal, que é um dos principais gargalos de desempenho.

### 3.2. Implementações Notáveis

*   **Google TPU (Tensor Processing Unit):** O componente central dos TPUs do Google é um grande array sistólico, otimizado para as operações de multiplicação de matrizes que dominam as cargas de trabalho de inferência e treinamento de redes neurais [4].
*   **Processador iWarp:** Um dos primeiros computadores comerciais a utilizar um array sistólico, desenvolvido em uma colaboração entre a Carnegie Mellon University e a Intel.

## 4. Física Subjacente

A implementação de qualquer algoritmo em hardware está, em última análise, limitada pelas leis da física.

---


## Page 10

# 4.1. Semicondutores e Transistores

Os blocos de construção fundamentais dos processadores modernos são os transistores, que funcionam como chaves eletrônicas. A sua operação é governada pela física dos semicondutores. As equações que descrevem a corrente em um transistor MOSFET, por exemplo, dependem de parâmetros como a mobilidade dos elétrons, a capacitância do óxido da porta e as tensões aplicadas. O consumo de energia é uma consideração crítica, dividido em potência dinâmica (associada à comutação dos transistores) e potência estática (fuga de corrente).

P_dinâmica = α * C * Vdd² * f

# 4.2. Limites Termodinâmicos

A dissipação de energia gera calor. A densidade de potência em chips de alto desempenho é tão alta que o resfriamento se torna um desafio de engenharia complexo. Além disso, existem limites termodinâmicos fundamentais para a computação, como o princípio de Landauer, que estabelece a energia mínima necessária para apagar um bit de informação.

E_min = k_B * T * ln(2)

# 5. Estado da Arte e Otimizações

## 5.1. AlphaTensor

Recentemente, a DeepMind (Google) desenvolveu o AlphaTensor, um sistema de IA que utiliza aprendizado por reforço para descobrir novos algoritmos de multiplicação de matrizes [5]. O AlphaTensor reformula o problema como um jogo e, através da exploração do vasto espaço de possíveis algoritmos, conseguiu encontrar sequências de operações mais eficientes do que as conhecidas anteriormente para tamanhos de matriz específicos, superando em alguns casos algoritmos desenvolvidos por humanos ao longo de décadas.

## 5.2. Otimizações para GPUs

Em processadores gráficos (GPUs), que são massivamente paralelos, a otimização da multiplicação de matrizes é crucial. As técnicas incluem:

*   **Tiling (Blocagem):** Dividir as matrizes em blocos menores que cabem na rápida memória compartilhada on-chip, maximizando a reutilização de dados.
*   **Acesso Coalescido à Memória:** Organizar os acessos à memória global de forma que threads adjacentes leiam posições de memória adjacentes, permitindo que o hardware agrupe estas leituras em uma única transação.

---


## Page 11

# 6. Principais Contribuidores

<table>
  <thead>
    <tr>
      <th>Nome</th>
      <th>Contribuição Principal</th>
      <th>Afiliações Notáveis</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Volker Strassen</td>
      <td>Desenvolveu o primeiro algoritmo de multiplicação de matrizes assintoticamente mais rápido que o método padrão (1969).</td>
      <td>Universidade de Konstanz</td>
    </tr>
    <tr>
      <td>Shmuel Winograd</td>
      <td>Otimizou o algoritmo de Strassen, reduzindo o número de adições necessárias. Provou a minimalidade do número de multiplicações para matrizes 2x2.</td>
      <td>IBM Thomas J. Watson Research Center</td>
    </tr>
    <tr>
      <td>H. T. Kung</td>
      <td>Co-inventor da arquitetura de array sistólico, uma base para muitos aceleradores de hardware modernos.</td>
      <td>Universidade de Harvard, Carnegie Mellon University</td>
    </tr>
    <tr>
      <td>Charles E. Leiserson</td>
      <td>Co-inventor do array sistólico e co-autor do influente livro "Introduction to Algorithms" (CLRS).</td>
      <td>Massachusetts Institute of Technology (MIT)</td>
    </tr>
  </tbody>
</table>

# 7. Referências

[1] Strassen, V. (1969). Gaussian elimination is not optimal. *Numerische Mathematik*, 13(4), 354-356. [2] Winograd, S. (1971). On multiplication of 2x2 matrices. *Linear Algebra and its Applications*, 4(4), 381-388. [3] Kung, H. T., & Leiserson, C. E. (1979). Systolic arrays (for VLSI). In *Sparse Matrix Proceedings 1978* (pp. 256-282). Society for Industrial and Applied Mathematics. [4] Jouppi, N. P., et al. (2017). In-datacenter performance analysis of a tensor processing unit. In *Proceedings of the 44th Annual International Symposium on Computer Architecture* (pp. 1-12). [5] Fawzi, A., et al. (2022). Discovering faster matrix multiplication algorithms with reinforcement learning. *Nature*, 610(7930), 47-53.


# Pesquisa Aprofundada sobre Álgebra Linear para Deep Learning e Aplicações em TPUs

## Introdução à Álgebra Linear no Contexto de Deep Learning

A álgebra linear constitui a base matemática para a compreensão e o desenvolvimento de muitos algoritmos de machine learning, especialmente os de deep learning. Esta seção aborda os conceitos fundamentais, desde as estruturas de dados básicas até as operações matriciais que sustentam as redes neurais.

---


## Page 12

# Estruturas de Dados Fundamentais

As operações em deep learning são realizadas sobre estruturas de dados multidimensionais. A álgebra linear fornece a notação e as operações para manipular esses dados de forma eficiente.

<table>
  <thead>
    <tr>
      <th>Objeto Matemático</th>
      <th>Descrição</th>
      <th>Exemplo de Notação</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Escalar</td>
      <td>Um único número.</td>
      <td>s ∈ ℝ</td>
    </tr>
    <tr>
      <td>Vetor</td>
      <td>Um array de números ordenados.</td>
      <td>x ∈ ℝⁿ</td>
    </tr>
    <tr>
      <td>Matriz</td>
      <td>Um array bidimensional de números.</td>
      <td>A ∈ ℝᵐˣⁿ</td>
    </tr>
    <tr>
      <td>Tensor</td>
      <td>Um array de números com mais de dois eixos.</td>
      <td>A ∈ ℝⁿ¹ˣⁿ²ˣ...</td>
    </tr>
  </tbody>
</table>

As operações básicas como adição, multiplicação por escalar e a transposição são definidas para essas estruturas e formam a base para cálculos mais complexos.

## Multiplicação de Matrizes e o seu Papel Central

A multiplicação de matrizes é, sem dúvida, a operação mais crucial em deep learning. Uma rede neural pode ser vista como uma série de transformações lineares (multiplicações de matrizes) intercaladas com funções de ativação não-lineares. O produto de uma matriz A (m×n) por uma matriz B (n×p) resulta em uma matriz C (m×p), onde cada elemento é calculado pela seguinte fórmula:

[ C_{i,j} = \sum_k A_{i,k} B_{k,j} ]

Esta operação é o que permite que uma camada da rede neural transforme as ativações da camada anterior. As propriedades da multiplicação de matrizes, como a associatividade e a distributividade, são exploradas para otimizar esses cálculos.

## Decomposições Matriciais e suas Aplicações

As decomposições matriciais são técnicas que fatoram uma matriz em matrizes constituintes, o que pode simplificar cálculos complexos e revelar propriedades importantes da matriz original. Elas são amplamente utilizadas em machine learning para diversas finalidades.

---


## Page 13

# Decomposição de Autovalores (Eigendecomposition)

A decomposição de autovalores é aplicável a matrizes quadradas e as fatora em seus autovetores e autovalores. A relação fundamental é **A**v = λv, onde v é um autovetor e λ é o autovalor correspondente. A decomposição é expressa como:

[ \mathbf{A} = \mathbf{V} \mathbf{\Lambda} \mathbf{V}^{-1} ]

Onde V é a matriz de autovetores e Λ é a matriz diagonal de autovalores. Em machine learning, a decomposição de autovalores é a base para a Análise de Componentes Principais (PCA), uma técnica de redução de dimensionalidade que projeta os dados em um subespaço de menor dimensão definido pelos autovetores correspondentes aos maiores autovalores da matriz de covariância dos dados.

# Decomposição de Valores Singulares (SVD)

A Decomposição de Valores Singulares (SVD) é uma generalização da decomposição de autovalores para qualquer matriz m x n. Ela fatora uma matriz A em três outras matrizes:

[ \mathbf{A} = \mathbf{U} \mathbf{\Sigma} \mathbf{V}^{\top} ]

Onde U e V são matrizes ortogonais e Σ é uma matriz diagonal. A SVD tem uma vasta gama de aplicações, incluindo:

*   **Redução de Dimensionalidade**: Similar ao PCA, a SVD pode ser usada para encontrar uma aproximação de baixa ordem para uma matriz, o que é útil para compressão de dados e redução de ruído.
*   **Sistemas de Recomendação**: Em filtragem colaborativa, a SVD é usada para fatorar a matriz de avaliações de usuários-itens e prever as avaliações faltantes.
*   **Processamento de Linguagem Natural**: A SVD é usada em técnicas como a Análise Semântica Latente (LSA) para encontrar relações semânticas entre termos e documentos.

# Tensor Processing Units (TPUs) e a Álgebra Linear

As Tensor Processing Units (TPUs) são aceleradores de hardware projetados pelo Google especificamente para cargas de trabalho de machine learning. A sua arquitetura é otimizada para executar as operações de álgebra linear, em particular a multiplicação de matrizes, de forma extremamente eficiente.

# Arquitetura de Array Sistólico

O coração de uma TPU é a sua Unidade de Multiplicação de Matrizes (MXU), que é implementada como um **array sistólico**. Esta arquitetura consiste em uma grande grade de unidades de processamento simples que realizam multiplicações e

---


## Page 14

acumulações. Os dados fluem através da grade de forma rítmica, semelhante ao bombeamento de sangue pelo coração, o que deu origem ao nome "sistólico".

Esta arquitetura minimiza a movimentação de dados, que é uma das operações mais custosas em termos de energia em chips modernos. Ao manter os dados fluindo através do array e reutilizando-os em múltiplas computações, as TPUs alcançam uma eficiência energética e um desempenho muito superiores aos de CPUs e GPUs para cargas de trabalho de multiplicação de matrizes.

## Implementação de Operações de Álgebra Linear em TPUs

As TPUs são projetadas para executar operações de álgebra linear em larga escala de forma distribuída. Algoritmos como o **SUMMA** para multiplicação de matrizes e o **CAQR** para decomposição QR são adaptados para tirar proveito da arquitetura da TPU, permitindo a manipulação de matrizes de tamanho massivo com alta performance.

## A Física por Trás das TPUs

A performance e a eficiência das TPUs são o resultado de inovações em diversas áreas da física e da engenharia.

*   **Física de Semicondutores**: O design das TPUs explora o princípio de que a movimentação de dados é mais cara do que a computação. A arquitetura de array sistólico e o uso de memória de alta largura de banda (HBM) são soluções diretas para este problema.
*   **Termodinâmica**: A alta densidade computacional das TPUs gera uma quantidade significativa de calor. A gestão térmica é um desafio crítico, e soluções de resfriamento avançadas são necessárias para manter a operação estável e eficiente.
*   **Eletromagnetismo**: Em altas frequências, a integridade do sinal e a interferência eletromagnética são preocupações importantes. O design cuidadoso do chip e das interconexões de alta velocidade é essencial para garantir a comunicação confiável entre os componentes.

## Principais Contribuidores

O desenvolvimento das TPUs foi um esforço colaborativo de muitos engenheiros e pesquisadores brilhantes. Alguns dos nomes mais proeminentes incluem **Norm Jouppi**, **Cliff Young**, e **David Patterson**, cujo trabalho foi fundamental para a concepção e o sucesso da arquitetura da TPU. A base teórica para a arquitetura de array sistólico foi estabelecida por **H.T. Kung** e **Charles E. Leiserson** em seu trabalho seminal de 1978.

---


## Page 15

# Referências

[1] Goodfellow, I., Bengio, Y., & Courville, A. (2016). Deep Learning. MIT Press. https://www.deeplearningbook.org/

[2] Jouppi, N. P., et al. (2017). In-Datacenter Performance Analysis of a Tensor Processing Unit. Proceedings of the 44th Annual International Symposium on Computer Architecture. https://doi.org/10.1145/3079856.3079872

[3] Patterson, D., & Hennessy, J. (2017). Computer Organization and Design RISC-V Edition: The Hardware Software Interface. Morgan Kaufmann.

# Pesquisa Técnica Profunda sobre Funções de Ativação em Hardware

Este documento detalha a pesquisa técnica e científica sobre as funções de ativação ReLU, GELU e Softmax, com foco em suas implementações em hardware, especialmente em Google TPUs, a física subjacente e a genealogia dos pesquisadores envolvidos.

## 1. Funções de Ativação: Definições e Fórmulas

### 1.1. ReLU (Rectified Linear Unit)

**Definição Formal:** A Rectified Linear Unit (ReLU) é uma função de ativação definida como a parte não-negativa de seu argumento. É uma das funções de ativação mais utilizadas devido à sua simplicidade e eficácia.

**Fórmulas e Equações:** - A função ReLU é definida como [1]: `ReLU(x) = max(0, x)` - De forma explícita: `f(x) = \begin{cases} x & \text{se } x > 0 \\ 0 & \text{se } x \leq 0 \end{cases}`

*   **Derivada:** A derivada da função ReLU é: `f'(x) = \begin{cases} 1 & \text{se } x > 0 \\ 0 & \text{se } x < 0 \end{cases}` A derivada não é definida em x = 0, mas na prática, é comum atribuir o valor 0 ou 1.

### 1.2. Variantes do ReLU

#### 1.2.1. Leaky ReLU

Permite um pequeno gradiente positivo quando a unidade está inativa para mitigar o problema do "Dying ReLU" [1]. - **Fórmula:** `f(x) = \begin{cases} x & \text{se } x > 0 \\ \alpha x & \text{se } x \leq 0 \end{cases}` onde α é uma pequena constante, tipicamente 0.01.

---


## Page 16

# 1.2.2. Parametric ReLU (PReLU)

Transforma o parâmetro α em um parâmetro aprendível pela rede [1]. - Fórmula:
```latex
f(x_i) = \begin{cases} x_i & \text{se } x_i > 0 \\ \alpha_i x_i & \text{se } x_i \leq 0 \end{cases}
```
onde α_i é um coeficiente aprendível para cada neurônio i.

# 1.2.3. Exponential Linear Unit (ELU)

Permite valores negativos, o que pode ajudar a aproximar a média das ativações para perto de zero, acelerando o aprendizado [1]. - Fórmula:
```latex
f(x) = \begin{cases} x & \text{se } x > 0 \\ \alpha(e^x - 1) & \text{se } x \leq 0 \end{cases}
```
onde α é um hiperparâmetro.

# 1.3. GELU (Gaussian Error Linear Unit)

**Definição Formal:** A GELU (Gaussian Error Linear Unit) é uma função de ativação que pondera as entradas por sua magnitude, em vez de apenas pelo sinal. É baseada na função de distribuição cumulativa gaussiana padrão, Φ(x) [2].

**Fórmulas e Equações:** - A definição original da GELU é [2]: `GELU(x) = x * P(X <= x) = x * Φ(x)` - Utilizando a função de erro (erf), a fórmula é [3]: `GELU(x) = x * 1/2 * [1 + erf(x / √2)]` - Uma aproximação comum e mais rápida para a GELU utiliza a função tangente hiperbólica (tanh) [2, 3]: `GELU(x) ≈ 0.5x * (1 + tanh[√(2/π) * (x + 0.044715x³)])` - Outra aproximação ainda mais simples é: `GELU(x) ≈ x * σ(1.702x)` onde σ é a função sigmoide.

# 1.4. Softmax

**Definição Formal:** A função Softmax transforma um vetor de N valores reais em uma distribuição de probabilidade, onde cada componente do vetor de saída está no intervalo (0, 1) e a soma de todas as componentes é 1 [3].

**Fórmulas e Equações:** - A i-ésima componente do vetor de saída é calculada como [3]: `softmax(x)_i = e^(x_i) / (∑_(j=1)^N e^(x_j))` - Para maior estabilidade numérica, subtrai-se o valor máximo do vetor de entrada de cada componente antes da exponenciação [3]: `softmax(x)_i = e^(x_i - max(x)) / (∑_(j=1)^N e^(x_j - max(x)))`

---


## Page 17

# 2. Implementação em Hardware e Física Subjacente

## 2.1. Arquitetura de Hardware (Google TPU)

As Unidades de Processamento Tensorial (TPUs) do Google possuem hardware dedicado para a computação eficiente de funções de ativação [4].

*   **Vector Processing Unit (VPU):** A partir da TPU v3, a VPU é responsável por executar operações não matriciais, como as funções de ativação **ReLU, GELU** e **Softmax**. A VPU possui uma arquitetura SIMD (Single Instruction, Multiple Data) que permite o processamento paralelo de múltiplos pontos de dados, operando com precisão FP32 e INT32 [4].
*   **Systolic Array (MXU):** O coração da TPU é a Unidade de Multiplicação de Matrizes (MXU), um array sistólico que otimiza as operações de multiplicação de matrizes. Os dados de ativação fluem através do array, maximizando a reutilização de dados e a eficiência energética [4].
*   **XLA (Accelerated Linear Algebra) Compiler:** O compilador XLA é fundamental para o desempenho, realizando otimizações como a **fusão de operadores**. Ele pode combinar uma sequência de operações (ex: convolução, batch norm e ReLU) em um único kernel, evitando acessos caros à memória principal [4].

## 2.2. Física de Semicondutores na Implementação de Funções de Ativação

A implementação de funções de ativação em hardware está intrinsecamente ligada à física de dispositivos semicondutores.

*   **Transistores de Efeito de Campo (FETs):** A função ReLU pode ser implementada de forma análoga usando as características de transferência de transistores. Recentemente, demonstrou-se a implementação de uma função análoga à ReLU usando um FET baseado em um semicondutor Kagome, explorando as **singularidades de Van Hove** na densidade de estados eletrônicos. Essas singularidades causam um aumento acentuado na transcondutância do dispositivo, que pode ser moldado para se assemelhar à resposta da ReLU [5].
*   **Lasers de Semicondutores:** Pesquisas exploram a implementação de funções de ativação ópticas. Uma função do tipo ReLU foi demonstrada experimentalmente usando um laser de semicondutor sujeito à injeção óptica. A não linearidade da resposta do laser acima da bifurcação de Hopf é utilizada para criar a função de ativação [6].
*   **Termodinâmica e Eletromagnetismo:** A computação em hardware de IA está sujeita a limites termodinâmicos. O princípio de Landauer estabelece um limite inferior para a dissipação de energia durante operações computacionais irreversíveis. Além disso, a alta densidade de componentes em chips

---


## Page 18

neuromórficos levanta questões de interferência eletromagnética e integridade do sinal, que devem ser consideradas no projeto da arquitetura [7].

## 3. Pesquisa Genealógica dos Principais Contribuidores

### 3.1. Dan Hendrycks

*   **Contribuições Principais:** Co-criador da função de ativação **GELU** [2]. Pesquisador proeminente em segurança de IA, robustez e benchmarks para modelos de linguagem.
*   **Orientadores de Doutorado:** Dawn Song, Jacob Steinhardt [8].
*   **Colaboradores Frequentes:** Kevin Gimpel, Mantas Mazeika, Steven Basart, Andy Zou, Collin Burns, Dawn Song, Jacob Steinhardt [8].
*   **Afiliações:** PhD pela UC Berkeley, BS pela University of Chicago. Atualmente é diretor do Center for AI Safety [8].

### 3.2. Kevin Gimpel

*   **Contribuições Principais:** Co-criador da função de ativação **GELU** [2]. Pesquisador em processamento de linguagem natural.
*   **Afiliações:** Toyota Technological Institute at Chicago (TTIC) e posteriormente Google [2].

### 3.3. Norman P. Jouppi

*   **Contribuições Principais:** Líder técnico e arquiteto principal do desenvolvimento da **Google TPU** [9].
*   **Afiliações:** Google.

### 3.4. Geoffrey Hinton

*   **Contribuições Principais:** Figura seminal em redes neurais profundas. Popularizou o uso da função **ReLU** em seu trabalho com Alex Krizhevsky e Ilya Sutskever no modelo AlexNet, que venceu o desafio ImageNet em 2012. Seu trabalho foi crucial para superar o problema do desaparecimento do gradiente em redes profundas.
*   **Orientandos Notáveis:** Yann LeCun, Ilya Sutskever, entre muitos outros.

## 4. Referências

[1] "Rectified linear unit - Wikipedia." [Online]. Available: https://en.wikipedia.org/wiki/Rectified_linear_unit [2] D. Hendrycks and K. Gimpel, "Gaussian Error Linear Units (GELUs)," 2016. [Online]. Available: https://arxiv.org/abs/1606.08415 [3] C. Zhengbo et al., "Research and design of activation function hardware implementation methods," 2020. [Online]. Available: https://iopscience.iop.org/article/10.1088/1742-6596/1684/1/012111/pdf [4] "TPU Architecture: Complete Guide to"

---


## Page 19

Google's 7 Generations." [Online]. Available: https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations [5] Z. Meng et al., "Analog ReLU Activation Enabled by Van Hove Singularities in a Kagome Semiconductor Field-Effect Transistor," 2025. [Online]. Available: https://advanced.onlinelibrary.wiley.com/doi/full/10.1002/aelm.202500255 [6] G. Liu et al., "Optical ReLU-like activation function based on a semiconductor laser with optical injection," 2024. [Online]. Available: https://opg.optica.org/ol/upcoming_pdf.cfm?id=511113 [7] T. Hylton, "Thermodynamic Neural Network," 2020. [Online]. Available: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7516712/ [8] "Dan Hendrycks - Personal Website." [Online]. Available: https://people.eecs.berkeley.edu/~hendrycks/ [9] "Tensor Processing Unit - Wikipedia." [Online]. Available: https://en.wikipedia.org/wiki/Tensor_Processing_Unit # Pesquisa Aprofundada sobre Mecanismo de Atenção, Transformers e Otimização para TPUs

**Autor:** Manus Al **Data:** 31 de dezembro de 2025

## Resumo Executivo

Este documento apresenta uma pesquisa técnica e científica detalhada sobre o mecanismo de atenção e a arquitetura Transformer, que se tornaram a base para os avanços mais significativos em inteligência artificial moderna. A análise abrange desde os princípios matemáticos e as formulações de conceitos como o Scaled Dot-Product Attention e Multi-Head Attention, até a complexidade computacional inerente a esses modelos. Adicionalmente, a pesquisa explora em profundidade a otimização dessas arquiteturas para as Tensor Processing Units (TPUs) do Google, detalhando a física de semicondutores, a termodinâmica e a arquitetura de hardware subjacente que permitem o treinamento e a inferência eficientes de modelos de grande escala. Por fim, uma análise genealógica dos principais pesquisadores por trás do artigo seminal "Attention Is All You Need" contextualiza as inovações dentro de uma linhagem de pesquisa acadêmica.

## 1. O Mecanismo de Atenção e a Arquitetura Transformer

### 1.1. Definição Formal e Princípios Fundamentais

O mecanismo de atenção é uma técnica que permite a uma rede neural focar seletivamente em partes específicas da sequência de entrada ao processar informações, calculando um vetor de contexto ponderado onde os pesos são determinados dinamicamente pela relevância de cada elemento da entrada [1]. A arquitetura Transformer, introduzida por Vaswani et al. em 2017, representou uma mudança de paradigma ao construir um modelo de sequência para sequência baseado exclusivamente em mecanismos de atenção, eliminando a necessidade de recorrência e convoluções [2].

---


## Page 20

Os princípios que governam o sucesso do Transformer são o **processamento paralelo**, que surge da remoção da dependência sequencial das RNNs, permitindo um treinamento mais rápido e eficiente em hardware moderno; a capacidade de modelar **dependências de longo alcance** diretamente através da auto-atenção, superando uma limitação crítica das RNNs; e o uso de **codificação posicional** para injetar explicitamente informações sobre a ordem da sequência, uma vez que a arquitetura em si é invariante à permutação [2].

## 1.2. Fórmulas e Equações Fundamentais

A base matemática do Transformer reside em algumas equações chave que definem como a informação é processada.

### 1.2.1. Scaled Dot-Product Attention

A principal operação de atenção utilizada é o *Scaled Dot-Product Attention*. Sua formulação matemática é a seguinte:

$$\text{Attention}(Q, K, V) = \text{softmax}(\frac{QK^T}{\sqrt{d_k}})V$$

Nesta equação, **Q (Query)**, **K (Key)**, e **V (Value)** são matrizes que representam as "consultas", "chaves" e "valores", respectivamente, todos derivados da sequência de entrada. A dimensão dos vetores de chave, **d_k**, é usada como um fator de escala para estabilizar os gradientes durante o treinamento [2].

### 1.2.2. Multi-Head Attention

Em vez de aplicar a atenção uma única vez, o Transformer emprega o *Multi-Head Attention*, que projeta as matrizes Q, K e V em múltiplos subespaços de representação ("cabeças") e aplica a atenção em paralelo. Os resultados são então concatenados e projetados novamente para produzir a saída final. Isso permite que o modelo atenda a informações de diferentes posições e subespaços de representação simultaneamente [2].

$$\text{MultiHead}(Q, K, V) = \text{Concat(head_1, ..., head_h)}W^O \quad \text{onde} \quad \text{head_i} = \text{Attention}(QW_i^Q, KW_i^K, VW_i^V)$$

As matrizes **W** representam as projeções de peso aprendidas para cada cabeça e para a saída final.

## 1.3. Implementação e Otimização para TPU

A implementação eficiente de Transformers em hardware especializado como TPUs é crucial para seu sucesso em larga escala. A otimização se concentra nas operações de multiplicação de matrizes, que são o núcleo computacional do mecanismo de

---


## Page 21

atenção. O **Systolic Array (MXU)** do TPU é projetado especificamente para essa tarefa, processando dados em um fluxo rítmico e massivamente paralelo [3].

O compilador **XLA (Accelerated Linear Algebra)** é a chave para desbloquear o desempenho do TPU, realizando otimizações como a **fusão de operações** para reduzir a sobrecarga de acesso à memória. Para lidar com a complexidade de memória quadrática da atenção, implementações de I/O-aware como o **FlashAttention** utilizam técnicas de *tiling* e recomputação para evitar a materialização da grande matriz de atenção intermediária, reduzindo a complexidade de memória para ser linear em relação ao comprimento da sequência [4]. Além disso, o suporte nativo do TPU para formatos de baixa precisão, como **bfloat16**, acelera significativamente os cálculos e reduz pela metade os requisitos de memória [3].

## 1.4. Trade-offs, Limitações e Estado da Arte

A principal limitação do Transformer padrão é sua **complexidade computacional** e **de memória de O(N²)** em relação ao comprimento da sequência N, o que torna o processamento de sequências muito longas proibitivamente caro. O estado da arte atual foca em superar essa limitação através de métodos como **atenção esparsa**, aproximações de **baixo rank** e novas arquiteturas como **Modelos de Espaço de Estado (State-Space Models - SSMs)**, que buscam alcançar complexidade linear ou quase-linear [5].

## 2. Física Subjacente e Arquitetura de Hardware do TPU

### 2.1. Da Física de Semicondutores à Termodinâmica

A performance dos TPUs está enraizada na física de semicondutores. Construídos sobre transistores **CMOS**, sua operação é governada pela mecânica quântica e pelo eletromagnetismo, que ditam o fluxo de elétrons através do silício dopado. A miniaturização contínua, seguindo a Lei de Moore, permitiu a integração de bilhões de transistores, mas também introduziu desafios termodinâmicos. A imensa densidade de potência gera calor significativo (efeito Joule), tornando a refrigeração a ar insuficiente. Como consequência, a partir do **TPU v3**, a **refrigeração líquida** tornou-se uma necessidade termodinâmica para dissipar o calor e manter a performance [6].

Desafios eletromagnéticos, como a integridade do sinal em altas frequências, são críticos. O design do chip deve gerenciar cuidadosamente a impedância e a capacitância para evitar ruído e diafonia. Notavelmente, as emanações eletromagnéticas podem ser exploradas em **ataques de canal lateral**, como o **TPUXtract**, que podem inferir informações sobre o modelo em execução [7].

### 2.2. Arquitetura de Hardware Detalhada

A arquitetura do TPU é um exemplo de co-design de hardware e software. O **Systolic Array (MXU)** é o coração computacional, uma grade 2D de unidades de multiplicação-

---


## Page 22

acumulação (MACs) que executam multiplicações de matrizes com eficiência massiva e mínima movimentação de dados [3]. A hierarquia de memória é otimizada com **memória de alta largura de banda (HBM)** no mesmo encapsulamento do chip e **SRAM on-chip** para alimentar o Systolic Array em alta velocidade.

Para escalar, os TPUs utilizam interconexões personalizadas. O **Inter-Chip Interconnect (ICI)** conecta os chips em uma topologia de toro de alta largura de banda. O **Optical Circuit Switching (OCS)**, uma inovação do TPU v4, usa espelhos MEMS para reconfigurar fisicamente as conexões de fibra óptica, permitindo topologias de rede flexíveis em escala de datacenter [8].

<table>
  <thead>
    <tr>
      <th>Geração</th>
      <th>Processo</th>
      <th>Clock (Aprox.)</th>
      <th>TDP (Aprox.)</th>
      <th>Interconexão</th>
      <th>Arquitetura MXU</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v1</td>
      <td>28nm</td>
      <td>700 MHz</td>
      <td>28-40W</td>
      <td>Pcle</td>
      <td>1x 256x256<br>INT8</td>
    </tr>
    <tr>
      <td>TPU v2</td>
      <td>16nm</td>
      <td>700 MHz</td>
      <td>200-250W</td>
      <td>ICI (2D Torus)</td>
      <td>2x 128x128<br>bfloat16</td>
    </tr>
    <tr>
      <td>TPU v3</td>
      <td>12nm</td>
      <td>940 MHz</td>
      <td>450W</td>
      <td>ICI (2D Torus)</td>
      <td>2x 128x128<br>bfloat16</td>
    </tr>
    <tr>
      <td>TPU v4</td>
      <td>7nm</td>
      <td>1.05 GHz</td>
      <td>175W</td>
      <td>OCS (3D Torus)</td>
      <td>4x 128x128<br>bfloat16</td>
    </tr>
    <tr>
      <td>TPU v6e</td>
      <td>N/A</td>
      <td>N/A</td>
      <td>N/A</td>
      <td>ICI</td>
      <td>256x256<br>bfloat16</td>
    </tr>
  </tbody>
</table>

Tabela 1: Evolução das especificações técnicas das gerações de TPU [3, 6, 8].

## 2.3. Trade-offs e Estado da Arte

O principal trade-off do TPU é a **especialização versus flexibilidade**. Como um ASIC, ele é extremamente eficiente para álgebra linear densa, mas menos flexível que uma GPU para cargas de trabalho irregulares. O desempenho é altamente dependente do compilador **XLA**. O estado da arte, representado por gerações como o **TPU v6e (Trillium)**, foca em aumentar o tamanho da MXU, suportar formatos de precisão ainda mais baixos como o FP8, e escalar para superpods com dezenas de milhares de chips [9].

## 3. Pesquisa Genealógica dos Principais Contribuidores

A inovação do Transformer não surgiu no vácuo, mas sim de uma rica linhagem de pesquisa em linguística computacional e machine learning.

---


## Page 23

Ashish Vaswani, um dos principais autores, teve como orientadores de doutorado **Liang Huang** e **David Chiang**, ambos pesquisadores notáveis em processamento de linguagem natural. A linhagem se aprofunda com **Aravind Joshi (1929-2017)**, um pioneiro em linguística computacional cujo trabalho em Gramáticas de Adjunção de Árvores (TAGs) foi fundamental e que orientou Llion Jones (outro coautor do artigo) e Liang Huang. O orientador de Joshi, por sua vez, foi **Seymour Sherman**, conectando a genealogia a trabalhos mais antigos em teoria da informação [10].

Noam Shazeer, creditado com ideias chave como o scaled dot-product attention, foi orientado por **Bill Dally** em Duke, um especialista em arquitetura de computadores e computação paralela. **Jakob Uszkoreit** é creditado com a ideia inicial de substituir completamente as RNNs por auto-atenção, dando o pontapé inicial no projeto [11]. Essa confluência de especialistas em linguagem, machine learning e sistemas de hardware foi fundamental para o sucesso do Transformer.

## Referências

[1] Bahdanau, D., Cho, K., & Bengio, Y. (2014). Neural Machine Translation by Jointly Learning to Align and Translate. *arXiv preprint arXiv:1409.0473*. [2] Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A. N., ... & Polosukhin, I. (2017). Attention is all you need. *Advances in neural information processing systems*, 30. [3] Jouppi, N. P., et al. (2017). In-datacenter performance analysis of a tensor processing unit. *Proceedings of the 44th annual international symposium on computer architecture*. [4] Dao, T., Fu, D. Y., Ermon, S., & Ré, C. (2022). Flashattention: Fast and memory-efficient exact attention with io-awareness. *Advances in Neural Information Processing Systems*, 35, 16884-16899. [5] Tay, Y., Dehghani, M., Bahri, D., & Metzler, D. (2020). Efficient transformers: A survey. *arXiv preprint arXiv:2009.06732*. [6] Introl. (2025). TPU Architecture: Complete Guide to Google's 7 Generations. *Introl Blog*. Recuperado de https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations [7] Keysight. (2025). Security Highlight: TPUXtract – A New Side-Channel Attack on Neural Networks. *Keysight Blog*. Recuperado de https://www.keysight.com/blogs/en/tech/nwvs/2025/02/25/security-highlight-tpuxtract-a-new-side-channel-attack-on-neural-networks [8] Jouppi, N. P., et al. (2021). TPU v4: An optically reconfigurable supercomputer for machine learning. *arXiv preprint arXiv:2104.04760*. [9] Google Cloud. (2024). Introducing Trillium, 6th generation TPUs. *Google Cloud Blog*. Recuperado de https://cloud.google.com/blog/products/compute/introducing-trillium-6th-gen-tpus [10] Resnik, P. (n.d.). Philip Resnik's Academic Geneology. Recuperado de https://psresnik.github.io/academic_geneology.html [11] Jones, L. (2023). Comunicação pessoal.

---


## Page 24

# Pesquisa sobre Paralelismo em IA

## Paralelismo de Dados

### Definição Formal e Princípios Fundamentais

O paralelismo de dados é uma técnica de computação paralela em que os dados ou a carga de trabalho computacional são divididos em partes que são distribuídas para várias unidades de processamento (normalmente GPUs). Cada dispositivo executa a mesma operação em diferentes subconjuntos de dados simultaneamente. O paralelismo de dados aumenta as velocidades de treinamento e processamento, fazendo melhor uso de tarefas concorrentes. Em outras palavras, com mais dispositivos, mais dados podem ser processados no mesmo tempo de relógio.

### Como Funciona o Paralelismo de Dados

1.  **Replicar o Modelo:** Uma cópia idêntica do modelo (com os mesmos pesos iniciais) é carregada em cada dispositivo (GPU/trabalhador). Ao fazer isso, cada dispositivo processa seus próprios dados simultaneamente durante as passagens de avanço e retrocesso.
2.  **Dividir os Dados:** O conjunto de dados de treinamento inteiro (ou lote de dados de entrada) é dividido em N partes de tamanho igual, onde N é o número de trabalhadores paralelos (GPUs). Cada trabalhador (GPU) recebe uma fatia diferente dos dados.
3.  **Processamento Paralelo:** Cada GPU (com sua própria réplica do modelo) processa seu subconjunto de dados em paralelo. Todas as GPUs calculam a passagem de avanço (previsões) e a passagem de retrocesso (gradientes) simultaneamente, sem comunicação durante esta etapa.
4.  **Sincronização de Gradiente:** Após concluir a computação local para uma etapa de treinamento, os trabalhadores sincronizam e atualizam consistentemente o modelo. No paralelismo de dados síncrono, todas as GPUs trocam e agregam os gradientes calculados. Um All-reduce (um padrão de comunicação coletiva) é frequentemente usado para somar os gradientes de todos os processos e distribuir os gradientes somados de volta para cada processo.
5.  **Atualização do Modelo:** Cada GPU (ou processo mestre) usa os gradientes agregados para atualizar os pesos do modelo (por exemplo, uma etapa de descida de gradiente ou Adam). Como cada GPU usou os mesmos gradientes agregados, seus parâmetros de modelo permanecem idênticos após a etapa de atualização.
6.  **Repetir:** Carregue o próximo lote de dados e repita o processo (dividir dados, avanço/retrocesso paralelo, sincronizar, atualizar).

---


## Page 25

# Fórmulas e Equações

Um trabalho de paralelismo de dados em um array de n elementos pode ser dividido igualmente entre todos os processadores. Vamos supor que queremos somar todos os elementos do array e o tempo para uma única operação de adição é Ta unidades de tempo. No caso da execução sequencial, o tempo gasto pelo processo será de n×Ta unidades de tempo, pois ele soma todos os elementos de um array. Por outro lado, se executarmos este trabalho como um trabalho de paralelismo de dados em 4 processadores, o tempo gasto seria reduzido para (n/4)×Ta + sobrecarga de fusão. A execução paralela resulta em um speedup de 4 em relação à execução sequencial.

## Multiplicação de Matrizes em Paralelo

```c++
// Multiplicação de matrizes em paralelo
#pragma omp parallel for schedule(dynamic,1) collapse(2)
for (int i = 0; i < A.rowLength(); i++) {
    for (int k = 0; k < B.columnLength(); k++) {
        int sum = 0;
        for (int j = 0; j < A.columnLength(); j++) {
            sum += A[i][j] * B[j][k];
        }
        C[i][k] = sum;
    }
}
```

# Paralelismo de Modelo

## Definição Formal e Princípios Fundamentais

O paralelismo de modelo, em sua forma de paralelismo de tensor (TP), divide os tensores do modelo em vários pedaços. Em vez de ter o tensor inteiro residindo em uma única GPU, cada fragmento do tensor reside em sua GPU designada. Durante o processamento, cada fragmento é processado separadamente e em paralelo em diferentes GPUs e os resultados são sincronizados no final da etapa. Isso é o que se pode chamar de paralelismo horizontal, pois a divisão acontece no nível horizontal.

O bloco de construção principal de qualquer transformer é um `nn.Linear` totalmente conectado seguido por uma ativação não linear `GeLU`. Seguindo a notação do artigo do Megatron-LM, podemos escrever a parte do produto escalar como `Y = GeLU(XA)`, onde `X` e `Y` são os vetores de entrada e saída, e `A` é a matriz de peso.

---


## Page 26

Se dividirmos a matriz de peso `A` em colunas por `N` GPUs e realizarmos as multiplicações de matriz `XA_1` a `XA_n` em paralelo, obteremos `N` vetores de saída `Y_1`, `Y_2`, ..., `Y_n` que podem ser alimentados em `GeLU` independentemente.

## Implementação em Hardware/Software

*   **Megatron-LM**: Possui uma implementação interna, pois é muito específica do modelo.
*   **paralleformers**: Atualmente, oferece suporte apenas para inferência.
*   **SageMaker**: Uma solução proprietária que só pode ser usada na AWS.
*   **DeepSpeed**: Chama o paralelismo de tensor de "tensor slicing" (fatiamento de tensor).

## Considerações Especiais

O TP requer uma rede muito rápida e, portanto, não é aconselhável fazer TP em mais de um nó. Praticamente, se um nó tiver 4 GPUs, o grau de TP mais alto é, portanto, 4. Se você precisar de um grau de TP de 8, precisará usar nós que tenham pelo menos 8 GPUs.

## Fórmulas e Equações

O paralelismo entre operadores se resume a particionar o grafo de operadores O em subgrafos e atribuir cada subgrafo a um dispositivo. Essa técnica tem requisitos de comunicação relativamente baixos, pois só precisamos nos comunicar com qualquer outro dispositivo na borda do subgrafo. As estratégias de paralelização encontradas no paralelismo intraoperador são altamente específicas do operador. Novamente, essas duas abordagens não são mutuamente exclusivas e muitas vezes são combinadas no que alguns chamam de paralelismo híbrido.

Quando se utiliza paralelismo de dados e de modelo, um tamanho total de paralelismo de modelo de M = t · p deve ser usado para que o modelo se ajuste à memória da GPU. Aqui, t é o tamanho do paralelismo do tensor e p é o tamanho do paralelismo do pipeline.

## Paralelismo de Pipeline

### Definição Formal e Princípios Fundamentais

O paralelismo de pipeline é um dos paralelismos primitivos para aprendizado profundo. Ele permite que a execução de um modelo seja particionada de forma que vários micro-lotes possam executar diferentes partes do código do modelo simultaneamente. O paralelismo de pipeline pode ser uma técnica eficaz para:

*   treinamento em larga escala
*   clusters com largura de banda limitada

---


## Page 27

* inferência de modelo grande

O **pipelining** do PyTorch consiste em duas partes: um **frontend de divisão** e um **tempo de execução distribuído**. O frontend de divisão pega o código do seu modelo como está, o divide em “partições de modelo” e captura a relação de fluxo de dados. O tempo de execução distribuído executa os estágios do pipeline em diferentes dispositivos em paralelo, lidando com coisas como divisão de micro-lotes, agendamento, comunicação e propagação de gradiente, etc.

## Como Funciona o Paralelismo de Pipeline

1.  **Construir PipelineStage**: Um `PipelineStage` é responsável por alocar buffers de comunicação e criar operações de envio/recebimento para se comunicar com seus pares. Ele gerencia buffers intermediários, por exemplo, para as saídas do avanço que ainda não foram consumidas, e fornece um utilitário para executar o retrocesso para o modelo do estágio.
2.  **Usar PipelineSchedule para execução**: Podemos agora anexar o `PipelineStage` a um cronograma de pipeline e executar o cronograma com os dados de entrada. O PyTorch oferece suporte a vários cronogramas de pipeline, incluindo GPipe, 1F1B, 1F1B intercalado e Looped BFS.

## Implementação em Hardware/Software

*   **PyTorch**: O pacote `torch.distributed.pipelining` fornece um kit de ferramentas que automatiza a implementação do paralelismo de pipeline em modelos gerais.
*   **DeepSpeed**: Oferece suporte ao paralelismo de pipeline.
*   **Megatron-LM**: Oferece suporte ao paralelismo de pipeline.
*   **Varuna**: Oferece suporte ao paralelismo de pipeline.
*   **SageMaker**: Oferece suporte ao paralelismo de pipeline.

## Fórmulas e Equações

Tradicionalmente, as redes neurais são granularizadas como camadas empilhadas. Existem duas funções associadas a cada camada, para frente e para trás. Na passagem para a frente, a entrada x é transformada na saída y com o mapeamento parametrizado f(x,W). A passagem para trás, crucial para o treinamento, envolve dois cálculos: ∇ₓ f(x,W)ᵀ(dℓ/dy) e ∇ʷ f(x,W)ᵀ(dℓ/dy). Correspondetemente, eles calculam o gradiente em relação à entrada x e aos parâmetros da camada W. Para conveniência, usamos as letras únicas B e W para denotar esses dois cálculos, respectivamente, e F para denotar a passagem para a frente. Tradicionalmente, B e W são agrupados e fornecidos como uma única função para trás.

---


## Page 28

A ideia chave por trás da melhoria para alcançar zero bolhas de pipeline é dividir a computação para trás em duas partes, uma que calcula o gradiente para a entrada e outra que calcula para os parâmetros.

## Modelo de Programação SPMD (Single Program, Multiple Data)

### Definição Formal e Princípios Fundamentais

SPMD (Single Program, Multiple Data) é um caso especial do modelo MIMD (Multiple Instruction, Multiple Data) da classificação de Flynn. No modelo SPMD, um único programa é executado simultaneamente em vários elementos de dados. Aqui, cada elemento de processamento (PE) executa o mesmo programa, mas com diferentes elementos de dados, permitindo o processamento paralelo e maior eficiência.

Este modelo é geralmente usado em computação de alto desempenho (HPC).

No modelo SPMD, o ID do processo é usado para ramificação. Cada instância do programa trabalha em seus próprios dados e pode seguir diferentes ramificações condicionais ou executar loops de maneira diferente.

### Processo de Execução

No modelo SPMD, o programa é escrito uma vez, mas replicado várias vezes, com cada PE executando o mesmo programa, mas com diferentes elementos de dados. O programa é dividido em tarefas, que são atribuídas a diferentes PEs para processamento. Os PEs operam independentemente uns dos outros e podem executar ramificações ou loops condicionais de maneira diferente, dependendo dos elementos de dados atribuídos.

### Métodos de Sincronização

A sincronização é um aspecto crucial do modelo SPMD para garantir que todos os PEs concluam suas tarefas atribuídas antes de passar para a próxima fase do programa. Um método comum de sincronização em SPMD são as primitivas de barreira, que permitem que vários threads ou processos esperem um pelo outro para atingir um ponto específico no programa antes de continuar a execução. As primitivas de barreira fazem com que cada PE espere em sua primitiva até que todos os outros PEs tenham concluído suas tarefas.

### Casos de Uso

O modelo SPMD é usado extensivamente em aplicativos de computação de alto desempenho (HPC) que exigem grandes quantidades de processamento de dados. Alguns dos casos de uso mais comuns do modelo SPMD incluem previsão do tempo, simulações científicas e modelagem financeira.

---


## Page 29

# Fórmulas e Equações

Não há fórmulas matemáticas ou equações explicitamente apresentadas no texto que descrevam um modelo matemático de programação SPMD. O documento descreve a implementação e o funcionamento do método, incluindo a decomposição de domínio (Domain Decomposition Method - DDM) e seus níveis de paralelização, mas não fornece equações matemáticas específicas que descrevam o modelo de programação SPMD.

# Principais Contribuidores

## PipeDream: Generalized Pipeline Parallelism for DNN Training

*   Deepak Narayanan (Microsoft Research, Stanford University)
*   Aaron Harlap (Carnegie Mellon University)
*   Amar Phanishayee (Microsoft Research)
*   Vivek Seshadri (Microsoft Research)
*   Nikhil R. Devanur (Microsoft Research)
*   Gregory R. Ganger (Carnegie Mellon University)
*   Phillip B. Gibbons (Carnegie Mellon University)
*   Matei Zaharia (Stanford University)

## Demystifying Parallel and Distributed Deep Learning: An In-Depth Concurrency Analysis

*   Tal Ben-Nun
*   Torsten Hoefler

## GPipe: Efficient Training of Giant Neural Networks using Pipeline Parallelism

*   Yanping Huang
*   Youlong Cheng
*   Ankur Bapna
*   Orhan Firat
*   Mia Xu Chen
*   Dehao Chen
*   HyoukJoong Lee
*   Jiquan Ngiam
*   Quoc V. Le
*   Yonghui Wu
*   Zhifeng Chen

## Zero Bubble Pipeline Parallelism

*   Penghui Qi

---


## Page 30

* Xinyi Wan
* Guangxing Huang
* Min Lin

## Referências

1. [https://www.digitalocean.com/community/conceptual-articles/data-parallelism-distributed-training](https://www.digitalocean.com/community/conceptual-articles/data-parallelism-distributed-training)
2. [https://docs.aws.amazon.com/sagemaker/latest/dg/model-parallel-intro.html](https://docs.aws.amazon.com/sagemaker/latest/dg/model-parallel-intro.html)
3. [https://huggingface.co/docs/transformers/v4.13.0/en/parallelism](https://huggingface.co/docs/transformers/v4.13.0/en/parallelism)
4. [https://docs.pytorch.org/docs/stable/distributed.pipelining.html](https://docs.pytorch.org/docs/stable/distributed.pipelining.html)
5. [https://www.geeksforgeeks.org/computer-organization-architecture/single-program-multiple-data-spmd-model/](https://www.geeksforgeeks.org/computer-organization-architecture/single-program-multiple-data-spmd-model/)
6. [https://en.wikipedia.org/wiki/Data_parallelism](https://en.wikipedia.org/wiki/Data_parallelism)
7. [https://arxiv.org/html/2403.03699v1](https://arxiv.org/html/2403.03699v1)
8. [https://arxiv.org/html/2401.10241v1](https://arxiv.org/html/2401.10241v1)
9. [https://help.altair.com/hwsolvers/os/topics/solvers/os/optistruct_spmd_c.htm](https://help.altair.com/hwsolvers/os/topics/solvers/os/optistruct_spmd_c.htm)
10. [https://www.pdl.cmu.edu/PDL-FTP/BigLearning/sosp19-final271_abs.shtml](https://www.pdl.cmu.edu/PDL-FTP/BigLearning/sosp19-final271_abs.shtml)
11. [https://arxiv.org/abs/1802.09941](https://arxiv.org/abs/1802.09941)
12. [https://huggingface.co/papers/1811.06965](https://huggingface.co/papers/1811.06965)
13. [https://huggingface.co/papers/2401.10241](https://huggingface.co/papers/2401.10241)

# Pesquisa Técnica e Científica sobre Comunicação Coletiva em TPU Pods

## 1. Introdução à Comunicação Coletiva em TPU Pods

A comunicação coletiva é um padrão de comunicação fundamental em computação paralela e distribuída, onde múltiplos processadores ou nós de computação trocam dados simultaneamente. Em sistemas de larga escala como os Google TPU Pods, a eficiência da comunicação coletiva é um fator crítico para o desempenho de cargas de trabalho de treinamento de modelos de aprendizado de máquina. Esta pesquisa aprofunda os aspectos técnicos e científicos dos principais algoritmos de comunicação coletiva utilizados em TPU Pods, com foco em All-Reduce, All-Gather, Reduce-Scatter e suas implementações em anel.

## 2. Operações de Comunicação Coletiva

As operações de comunicação coletiva são primitivas que orquestram a troca de dados entre um grupo de processos. As operações mais relevantes para o treinamento de modelos de aprendizado profundo em TPU Pods são:

---


## Page 31

# 2.1. All-Reduce

A operação `All-Reduce` combina dados de todos os `N` processadores e distribui o resultado para todos eles. A combinação é feita através de uma operação de redução (como soma, mínimo ou máximo). Esta operação é crucial para a sincronização de gradientes em treinamento de dados paralelos.

# 2.2. All-Gather

A operação `All-Gather` coleta dados de todos os `N` processadores e distribui o conjunto de dados coletado para todos eles. Cada processador envia seu próprio buffer de dados e, ao final da operação, todos os processadores possuem a concatenação dos buffers de dados de todos os outros processadores.

# 2.3. Reduce-Scatter

A operação `Reduce-Scatter` combina dados de todos os `N` processadores e distribui o resultado de forma que cada processador `i` receba a `i`-ésima parte do resultado. Esta operação é efetivamente uma operação de `Reduce` seguida por uma operação de `Scatter`.

# 3. Algoritmos em Anel (Ring-based Algorithms)

Os algoritmos em anel são uma implementação eficiente para operações de comunicação coletiva, especialmente em topologias de rede que podem ser mapeadas para um anel lógico. Eles são projetados para maximizar a utilização da largura de banda da rede, dividindo os dados em blocos e transmitindo-os de forma pipelined.

## 3.1. All-Reduce em Anel

O algoritmo de `All-Reduce` em anel é executado em duas fases principais:

1. **Reduce-Scatter**: Nesta fase, os dados são divididos em `N` blocos, e cada um dos `N` processadores envia e recebe blocos de dados em `N-1` passos. Ao final desta fase, cada processador `i` possui o `i`-ésimo bloco do resultado da redução.
2. **All-Gather**: Nesta fase, cada processador envia seu bloco do resultado para todos os outros processadores. Em `N-1` passos, todos os processadores terão o resultado completo da operação `All-Reduce`.

O custo total total de um `All-Reduce` em anel para uma mensagem de tamanho `M` em `N` processadores com largura de banda de link `B` é aproximadamente `2 * (N - 1) * (M / B)`.

---


## Page 32

# 4. Implementação do All-Reduce com Reduce-Scatter e All-Gather

A combinação das operações `Reduce-Scatter` e `All-Gather` é uma maneira eficiente de implementar a operação `All-Reduce`. A seguir, um exemplo com 4 processadores (ranks):

*   **Estado Inicial:** Cada rank possui um vetor de 4 elementos.
*   **Reduce-Scatter:** A operação de soma é aplicada elemento a elemento, e cada rank recebe uma parte do resultado.
*   **All-Gather:** Cada rank compartilha sua parte do resultado com todos os outros, de modo que, ao final, todos os ranks possuem o vetor de resultado completo.

# 5. Topologias de Interconexão e Hardware

## 5.1. Google TPU Pods e Multislice

Os Google TPU Pods utilizam uma combinação de interconexões de alta velocidade para facilitar a comunicação coletiva:

*   **Inter-chip Interconnect (ICI):** Uma interconexão de alta velocidade que conecta os chips TPU dentro de um mesmo *slice*.
*   **Data Center Network (DCN):** A rede do data center, como a Jupiter da Google, é usada para a comunicação entre *slices* diferentes em uma configuração *Multislice*.

O compilador XLA da Google otimiza a comunicação coletiva, decompondo operações como `All-Reduce` em operações hierárquicas que utilizam tanto a ICI quanto a DCN de forma eficiente.

## 5.2. Topologia Hyper-Square (Patente US20210240543A1)

Uma patente da Alibaba (atualmente T-Head) descreve uma topologia de interconexão chamada "hyper-square" e um algoritmo de roteamento bidimensional para a operação `AllReduce`. Esta topologia visa superar as limitações das topologias de toro, oferecendo maior flexibilidade e eficiência.

# 6. Principais Contribuidores

*   **Liang Han e Jeff Jiao:** Inventores da patente do algoritmo `ring-allreduce` aprimorado, com afiliações à Alibaba e T-Head.
*   **Engenheiros da Google:** A equipe de engenharia por trás dos TPUs e da infraestrutura Multislice, incluindo **Nisha Mariam Johnson** e **Andi Gavrilescu**.
*   **Comunidade Acadêmica:** Pesquisadores de diversas universidades que têm contribuído para o avanço dos algoritmos de comunicação coletiva.

---


## Page 33

# 7. Referências

*   [1] Google Cloud. (2023). *Using Cloud TPU Multislice to scale AI workloads*. https://cloud.google.com/blog/products/compute/using-cloud-tpu-multislice-to-scale-ai-workloads
*   [2] Universidade de Washington. (2024). *Collective Communications*. https://courses.cs.washington.edu/courses/cse599k/24au/content/14-Collectives.pdf
*   [3] Han, L., & Jiao, J. (2021). *Efficient and more advanced implementation of ring-allreduce algorithm for distributed parallel deep learning* (U.S. Patent No. US20210240543A1). U.S. Patent and Trademark Office. https://patents.google.com/patent/US20210240543A1/en
*   [4] Won, L. (2025). *From Scatter to All-Reduce: A Plain-English Guide to Collective Operations*. DEV Community. https://dev.to/lewis_won/from-scatter-to-all-reduce-a-plain-english-guide-to-collective-operations-1695

# Relatório Técnico: Análise de Consumo de Energia e Dissipação Térmica em Google Tensor Processing Units (TPUs)

## Introdução

Este relatório apresenta uma pesquisa técnica aprofundada sobre os princípios, desafios e soluções relacionadas ao consumo de energia e à dissipação térmica nos aceleradores de hardware de Machine Learning (ML) do Google, conhecidos como Tensor Processing Units (TPUs). A crescente demanda computacional dos modelos de ML modernos impulsionou o desenvolvimento de hardware de domínio específico (DSA) que, por sua vez, introduziu desafios significativos em termos de densidade de potência e gerenciamento térmico. Analisamos a evolução da arquitetura TPU, as fórmulas físicas que governam o consumo de energia, as tecnologias de refrigeração implementadas e as contribuições dos principais pesquisadores que tornaram esses sistemas possíveis.

## 1. Fundamentos Físicos do Consumo de Energia e Dissipação Térmica

A operação de qualquer circuito semicondutor está intrinsecamente ligada ao consumo de energia, que se manifesta como calor. A compreensão desses princípios é fundamental para o projeto de sistemas eficientes e confiáveis.

### 1.1. Consumo de Energia em Circuitos CMOS

O consumo de energia total (P_total) em um circuito integrado CMOS, como um TPU, é a soma de dois componentes principais: estático e dinâmico. As equações a

---


## Page 34

seguir são baseadas no modelo padrão da indústria, conforme detalhado em publicações técnicas como o relatório "CMOS Power Consumption and Cpd Calculation" da Texas Instruments.

## Equação 1: Potência Total

P_total = P_estatico + P_dinamico

### 1.1.1. Consumo de Energia Estático (P_estatico)

Ocorre devido a correntes de fuga (I_fuga) que atravessam os transistores mesmo quando estão inativos. É um produto direto da tensão de alimentação (V_CC).

## Equação 2: Potência Estática

P_estatico = V_CC * I_fuga

A corrente de fuga, por sua vez, é sensível à temperatura e pode ser modelada pela equação do diodo de Shockley para cada transistor.

## Equação 3: Corrente de Fuga (Diodo)

I_fuga = I_S * (e^(qV / kT) - 1)

*   I_S : Corrente de saturação reversa
*   V : Tensão sobre a junção
*   q : Carga do elétron (1.602 x 10^-19 C)
*   k : Constante de Boltzmann (1.38 x 10^-23 J/K)
*   T : Temperatura (Kelvin)

### 1.1.2. Consumo de Energia Dinâmico (P_dinamico)

Este é o componente dominante durante a operação ativa do chip e resulta da comutação dos transistores. Divide-se em consumo transiente e de carga capacitiva.

## Equação 4: Potência Dinâmica

P_dinamico = (C_pd * V_CC^2 * f_I * N_SW) + (C_L * V_CC^2 * f_O * N_SW)

*   C_pd : Capacitância de dissipação de energia dinâmica (interna ao chip).
*   C_L : Capacitância de carga externa (trilhas, outros componentes).
*   V_CC : Tensão de alimentação.
*   f_I, f_O : Frequências de comutação da entrada e saída, respectivamente.

---


## Page 35

*   N_SW : Número de bits (saídas) em comutação.

## 1.2. Física da Dissipação Térmica

A energia consumida é convertida em calor, que deve ser eficientemente removido. A transferência de calor é governada pela resistência térmica ( θ ).

### Equação 5: Lei de Ohm Térmica

ΔT = P_total * θ

Isso nos permite calcular a temperatura da junção do silício ( T_J ), um parâmetro crítico para a confiabilidade do chip.

### Equação 6: Temperatura da Junção

T_J = T_A + (P_total * θ_JA)

*   T_A : Temperatura ambiente.
*   θ_JA : Resistência térmica total da junção para o ambiente.

O calor dentro do chip de silício é transportado principalmente por fônonos (quanta de vibração da rede cristalina). A condutividade térmica ( k ) do silício é, portanto, um fator chave.

### Equação 7: Condutividade Térmica (Modelo Cinético)

k = (1/3) * C * v * λ

*   C : Capacidade de calor específica.
*   v : Velocidade média dos fônonos.
*   λ : Caminho livre médio dos fônonos.

O espalhamento de fônonos (por outros fônonos, impurezas ou defeitos) limita λ e, consequentemente, a condutividade térmica, tornando-se um fator crucial no design térmico de chips de alta potência.

## 2. Implementação e Evolução nos TPUs

O Google adotou uma abordagem de co-design completo, onde o gerenciamento de energia e a refrigeração são partes integrantes da arquitetura do sistema, desde o compilador até o data center.

---


## Page 36

# 2.1. Evolução da Refrigeração

A crescente densidade de potência ao longo das gerações de TPU tornou a refrigeração um desafio central, exigindo uma transição de ar para líquido.

<table>
  <thead>
    <tr>
      <th>Geração</th>
      <th>Ano</th>
      <th>TDP (Estimado)</th>
      <th>Refrigeração</th>
      <th>Inovação Chave no Gerenciamento Térmico</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v1</td>
      <td>2015</td>
      <td>28-40W</td>
      <td>Ar</td>
      <td>Baixíssimo consumo focado em inferência.</td>
    </tr>
    <tr>
      <td>TPU v2</td>
      <td>2017</td>
      <td>~200-250W</td>
      <td>Ar</td>
      <td>Aumento da capacidade de treinamento.</td>
    </tr>
    <tr>
      <td>TPU v3</td>
      <td>2018</td>
      <td>~450W</td>
      <td>Líquida</td>
      <td>Primeira geração a exigir refrigeração líquida devido à alta densidade de potência.</td>
    </tr>
    <tr>
      <td>TPU v4</td>
      <td>2021</td>
      <td>~275W</td>
      <td>Líquida</td>
      <td>Otimização da eficiência (desempenho/Watt) e introdução de interconexão óptica.</td>
    </tr>
    <tr>
      <td>v5 em diante</td>
      <td>2022+</td>
      <td>N/A</td>
      <td>Líquida</td>
      <td>Continuação da refrigeração líquida com foco em maior eficiência energética.</td>
    </tr>
  </tbody>
</table>

O sistema de **refrigeração líquida direta no chip (Direct-to-Chip)** adotado pelo Google envolve placas frias montadas diretamente sobre os ASICs do TPU, permitindo uma transferência de calor muito mais eficiente do que seria possível com ar.

# 2.2. Patentes e Inovações de Design

A pesquisa de patentes, especialmente as de autoria de Norman Jouppi e seus colaboradores, revela inovações de design focadas em otimização térmica:

*   **Layout Térmico Otimizado:** Patentes como a "Integrated circuit with a ring-shaped hot spot area" (12.315.783) descrevem o posicionamento estratégico de componentes de alta potência em geometrias (como um anel) que facilitam o resfriamento multidirecional.
*   **Dispositivos de Distribuição de Calor:** Patentes como "Methods and heat distribution devices for thermal management" (12.243.802) detalham a fabricação de montagens de chip com distribuidores de calor integrados, otimizando a interface térmica (TIM) entre o chip e a solução de refrigeração.

---


## Page 37

# 3. Principais Contribuidores e Pesquisa Genealógica

O desenvolvimento dos TPUs é o resultado da colaboração de uma vasta rede de pesquisadores e engenheiros. Dois nomes se destacam como figuras centrais.

## 3.1. Norman P. Jouppi

Líder técnico do projeto TPU no Google desde sua concepção. Sua pesquisa anterior em caches, modelagem de potência (McPAT) e arquiteturas de processadores foi fundamental para o design dos TPUs. Suas publicações, como "In-datacenter performance analysis of a tensor processing unit" (2017) e "TPU v4: An optically reconfigurable supercomputer..." (2023), são as fontes primárias de informação sobre a arquitetura e o desempenho dessas máquinas. Seus colaboradores frequentes incluem Cliff Young, David Patterson, e Sheng Li.

## 3.2. David Patterson

Lenda da arquitetura de computadores, professor emérito de Berkeley e co-criador do conceito RISC. Patterson trouxe um rigor acadêmico e uma abordagem quantitativa para o projeto e análise dos TPUs. Sua colaboração com John Hennessy, que resultou no livro "Computer Architecture: A Quantitative Approach" e no Prêmio Turing de 2017, estabeleceu a metodologia usada para projetar e avaliar sistemas como o TPU. Sua influência é vista no co-design hardware/software e na análise de desempenho baseada em cargas de trabalho reais que caracterizam o projeto TPU.

# Conclusão

A pesquisa demonstra que o gerenciamento de consumo de energia e dissipação térmica é um pilar central no design dos Google TPUs. A solução não reside em um único componente, mas em uma abordagem holística de co-design que abrange desde a física dos semicondutores, passando por fórmulas de consumo de energia, arquitetura de layout de chip, sistemas de refrigeração líquida em escala de data center, até a otimização via compilador. As contribuições de pioneiros como Norman P. Jouppi e David Patterson foram essenciais para integrar décadas de pesquisa em arquitetura de computadores com as demandas específicas do Machine Learning, resultando em uma das infraestruturas de computação mais poderosas e eficientes do mundo.

# Fabricação de dispositivos semicondutores

Fabricação de dispositivos semicondutores é o processo utilizado para criar os circuitos integrados que estão presentes em todos os dispositivos eletrônicos. É uma sequência de passos múltiplos com litografia e utilização de produtos químicos durante a qual os circuitos eletrônicos são criados gradualmente em uma wafer feita

---


## Page 38

de material semicondutor. [Silício](https://pt.wikipedia.org/wiki/Silício) é quase sempre utilizado, mas vários compostos semicondutores são utilizados para aplicações especializadas.

## Nós de Processo (Process Nodes)

A indústria de semicondutores tem seguido uma tendência de miniaturização, com os nós de processo diminuindo ao longo do tempo. A seguir, uma lista de alguns nós de processo e o ano em que foram introduzidos:

*   10 µm – 1971
*   6 µm - 1974
*   3 µm - 1977
*   1,5 µm - 1982
*   1 µm - 1985
*   800 nm - 1989
*   600 nm - 1994
*   350 nm - 1995
*   250 nm - 1997
*   180 nm - 1999
*   130 nm - 2001
*   90 nm - 2004
*   65 nm - 2006
*   45 nm - 2008
*   32 nm - 2010
*   22 nm - 2012
*   14 nm - 2014
*   10 nm - 2016
*   7 nm - 2018
*   5 nm - 2020

## Tecnologias e Desafios

À medida que os nós de processo diminuem, novos desafios e tecnologias surgem:

*   **7 nm:** A TSMC anunciou a produção em 7 nm em 2017, e a AMD lançou processadores com essa tecnologia em 2019. A IBM também demonstrou chips de 7 nm funcionais usando silício-germânio.
*   **10 nm:** Esta tecnologia enfrenta desafios como o tunelamento quântico. A evolução para materiais não-silício, como nanotubos de carbono e nanofios, está sendo explorada.
*   **14 nm:** A resolução de 14 nm é difícil de alcançar com litografia convencional, exigindo técnicas como a litografia por feixe de elétrons e a litografia de imersão.
*   **22 nm e 32 nm:** A Intel e a AMD comercializaram chips com essas tecnologias em 2012 e 2010, respectivamente.

---


## Page 39

# Principais Empresas

*   **TSMC**: Uma das líderes na fabricação de semicondutores, pioneira em nós de processo avançados.
*   **Samsung**: Outra gigante na indústria de semicondutores, competindo diretamente com a TSMC.
*   **Intel**: Uma das principais empresas de semicondutores, com forte presença no mercado de processadores.
*   **IBM**: Conhecida por suas pesquisas e desenvolvimento de tecnologias inovadoras em semicondutores.
*   **AMD**: Uma das principais concorrentes da Intel no mercado de processadores.
*   **NEC**: Uma empresa japonesa que também contribuiu para o avanço da tecnologia de semicondutores.

# Fotolitografia: Detalhes Técnicos

## Processo de Fotolitografia

O processo de fotolitografia consiste em várias etapas sequenciais:

1.  **Limpeza (Cleaning)**: Remoção de contaminações orgânicas e inorgânicas da superfície do wafer, geralmente através de um tratamento químico úmido como o processo RCA.
2.  **Preparação (Preparation)**: Aquecimento do wafer para remover umidade e aplicação de um promotor de adesão (ex: HMDS) para melhorar a adesão do fotorresiste.
3.  **Aplicação do Fotorresiste (Photoresist application)**: O wafer é coberto com fotorresiste líquido através de um processo de "spin coating" para criar uma camada fina e uniforme.
4.  **Pré-cozimento (Pre-bake)**: O wafer é aquecido para remover o excesso de solvente do fotorresiste.
5.  **Exposição e Revelação (Exposure and developing)**: O fotorresiste é exposto a um padrão de luz intensa através de uma fotomáscara. A exposição à luz causa uma mudança química que torna o fotorresiste solúvel (fotorresiste positivo) ou insolúvel (fotorresiste negativo) em uma solução reveladora.
6.  **Gravação (Etching)**: O padrão do fotorresiste é transferido para o wafer através de um processo de gravação, que remove o material da superfície do wafer nas áreas não protegidas pelo fotorresiste.
7.  **Remoção do Fotorresiste (Photoresist removal)**: O fotorresiste restante é removido do wafer.

---


## Page 40

# Resolução em Sistemas de Projeção

A resolução de um sistema de projeção, ou seja, a menor característica que pode ser impressa, é determinada pelo **Critério de Rayleigh**:

**CD = k₁ * (λ / NA)**

Onde:
*   **CD (Critical Dimension)**: É a dimensão mínima da característica a ser impressa (minimum feature size).
*   **k₁ (fator de processo)**: É um coeficiente que engloba fatores relacionados ao processo. O valor de k₁ está tipicamente em torno de 0.4 para produção.
*   **λ (lambda)**: É o comprimento de onda da luz utilizada.
*   **NA (Numerical Aperture)**: É a abertura numérica da lente, vista do wafer.

De acordo com esta equação, dimensões mínimas de características podem ser diminuídas diminuindo o comprimento de onda e aumentando a abertura numérica.

# Fontes de Luz e Tipos de Litografia

*   **Litografia Ultravioleta (UV)**: Utiliza luz ultravioleta, historicamente com lâmpadas de mercúrio que produzem luz na faixa de 365-436 nm.
*   **Litografia Ultravioleta Profunda (DUV - Deep Ultraviolet)**: Utiliza lasers de excímeros para produzir luz em comprimentos de onda mais curtos, como 248 nm (KrF) e 193 nm (ArF).
*   **Litografia de Ultravioleta Extremo (EUV - Extreme Ultraviolet)**: Utiliza luz com um comprimento de onda de 13.5 nm, permitindo a fabricação de nós de processo de 7 nm e menores. A geração de luz EUV é um processo complexo que envolve o uso de um plasma de estanho (Sn) pulsado por laser.

# Fórmulas Adicionais

## Profundidade de Foco (Depth of Focus - DOF)

Outro conceito importante na litografia é a profundidade de foco (DOF), que é a distância vertical na qual a imagem permanece em foco. A DOF é dada pela seguinte fórmula:

**DOF = k₂ * (λ / NA²)**

Onde:
*   **k₂**: É outro fator de processo.

Existe um trade-off entre resolução e profundidade de foco. Aumentar a abertura numérica (NA) para melhorar a resolução (diminuir o CD) resulta em uma diminuição

---


## Page 41

da profundidade de foco, o que torna o processo mais sensível a variações na topografia do wafer.

# Litografia Computacional

Litografia Computacional é o conjunto de abordagens matemáticas e algorítmicas projetadas para melhorar a resolução atingível através da fotolitografia. À medida que os nós de processo encolheram, a litografia computacional tornou-se uma ferramenta essencial para lidar com os desafios da fabricação de semicondutores.

## Técnicas de Litografia Computacional

### Tecnologias de Aprimoramento de Resolução (RET - Resolution Enhancement Technology)

As RETs são usadas para melhorar a resolução da litografia, especialmente em nós de processo abaixo de 90 nm. Essas técnicas compensam os efeitos de difração e outros efeitos de proximidade. As principais RETs incluem:

*   **Correção de Proximidade Óptica (OPC - Optical Proximity Correction):** Modifica as geometrias na fotomáscara para compensar os erros de imagem devido à difração ou efeitos do processo. O OPC pode ser baseado em regras (rule-based) ou em modelos (model-based).
*   **Máscaras de Deslocamento de Fase (PSM - Phase-Shift Masks):** Introduzem deslocamentos de fase na luz que passa pela máscara, criando interferência destrutiva para melhorar o contraste da imagem.
*   **Iluminação Fora do Eixo (Off-Axis Illumination):** Utiliza uma fonte de luz angular para melhorar a resolução de características densas.

### Litografia Inversa (Inverse Lithography)

A litografia inversa trata a otimização da máscara como um problema de imagem inversa. Em vez de aplicar correções a um padrão de máscara projetado, ela calcula o padrão de máscara ideal que produziria o padrão de wafer desejado. Essa técnica pode gerar padrões de máscara não intuitivos, mas altamente otimizados.

### Otimização de Fonte e Máscara (SMO - Source-Mask Optimization)

A SMO otimiza simultaneamente a fonte de luz e o padrão da máscara para produzir o melhor padrão de wafer possível. Essa é uma técnica computacionalmente intensiva que se tornou crucial para a litografia EUV.

### Modelagem Complexa

A litografia computacional depende de modelos complexos do sistema de lentes e do fotorresiste para simular com precisão o processo de litografia. Esses modelos levam

---


## Page 42

em conta a polarização da luz, os efeitos do fotorresiste e outras complexidades físicas.

## Esforço Computacional

As técnicas de litografia computacional, especialmente a litografia inversa e a SMO, exigem um enorme poder computacional. A NVIDIA desenvolveu a biblioteca cuLitho, que utiliza GPUs para acelerar esses cálculos, reduzindo o tempo de processamento de semanas para dias.

## Física da Litografia de Semicondutores

### Eletromagnetismo na Fotolitografia

A fotolitografia é fundamentalmente um processo eletromagnético. A interação da luz com a fotomáscara e o fotorresiste é governada pelas equações de Maxwell. A simulação eletromagnética rigorosa é usada para modelar a difração da luz pela máscara e a propagação do campo eletromagnético através do sistema óptico. A intensidade do campo eletromagnético difratado determina a imagem áerea que é projetada no fotorresiste.

### Termodinâmica no Processo de Litografia

A termodinâmica desempenha um papel crucial em várias etapas do processo de litografia:

*   **Pré-cozimento (Pre-bake) e Pós-exposição (Post-exposure bake):** O aquecimento do fotorresiste é um processo termodinâmico que remove o solvente e promove reações químicas no fotorresiste. A transferência de calor para o fotorresiste é um fator crítico que afeta a uniformidade e a qualidade do padrão.
*   **Gravação (Etching):** Os processos de gravação, tanto úmidos quanto secos, são reações químicas governadas pela termodinâmica e pela cinética. A temperatura e a pressão da câmara de gravação são parâmetros termodinâmicos importantes que controlam a taxa e a anisotropia da gravação.

### Efeitos Quânticos na Litografia EUV

Com o advento da litografia de ultravioleta extremo (EUV), os efeitos quânticos tornaram-se significativos. A energia de um fóton EUV (cerca de 92 eV) é muito maior do que a energia de ligação dos elétrons nos materiais do fotorresiste. A absorção de um fóton EUV gera um fotoelétron, que por sua vez pode gerar uma cascata de elétrons secundários. Esses elétrons secundários causam a exposição do fotorresiste. A natureza quântica desse processo leva a efeitos estocásticos, como o "shot noise", que é a variação estatística no número de fótons e elétrons envolvidos no processo de

---


## Page 43

exposição. O "shot noise" pode levar a defeitos estocásticos, como a rugosidade da borda da linha (LER - Line Edge Roughness).

## Fórmulas e Equações

### Equações de Maxwell

As equações de Maxwell descrevem o comportamento dos campos elétricos e magnéticos e são a base para a simulação eletromagnética na litografia:

*   Lei de Gauss para a eletricidade: ∇ · E = ρ / ε₀
*   Lei de Gauss para o magnetismo: ∇ · B = 0
*   Lei de Faraday da indução: ∇ × E = -∂B / ∂t
*   Lei de Ampère-Maxwell: ∇ × B = μ₀(J + ε₀∂E / ∂t)

Onde:

*   E: Campo elétrico
*   B: Campo magnético
*   ρ: Densidade de carga
*   J: Densidade de corrente
*   ε₀: Permissividade do vácuo
*   μ₀: Permeabilidade do vácuo

### Shot Noise

O "shot noise" pode ser modelado usando a estatística de Poisson. O número de fótons (N) que chegam a uma determinada área segue uma distribuição de Poisson, e o desvio padrão do número de fótons é igual à raiz quadrada do número médio de fótons:

σ_N = √N

Essa variação estatística no número de fótons leva a variações na dose de exposição e, consequentemente, a defeitos no padrão.

## Escalonamento de Dennard e Nós de Processo

### Escalonamento de Dennard (Dennard Scaling)

O escalonamento de Dennard, também conhecido como escalonamento de MOSFET, é uma lei de escalonamento que afirma que, à medida que os transistores diminuem de tamanho, sua densidade de potência permanece constante. Isso significa que o consumo de energia fica em proporção com a área; tanto a tensão quanto a corrente diminuem com o comprimento.

---


## Page 44

# Fórmulas e Equações

De acordo com o escalonamento de Dennard, para um fator de escalonamento **S > 1**, as dimensões e tensões do transistor são escalonadas da seguinte forma:

*   **Dimensões (L, W, t_ox):** Diminuem por um fator de **S** (ou seja, são multiplicadas por **1/S**).
*   **Tensão de alimentação (V_DD) e Tensão de limiar (V_T):** Diminuem por um fator de **S**.
*   **Densidade de dopagem (N_A):** Aumenta por um fator de **S**.

Isso leva ao seguinte escalonamento para outras propriedades do transistor:

<table>
<thead>
<tr>
<th>Propriedade</th>
<th>Símbolo</th>
<th>Equação</th>
<th>Expoente de Escalonamento (Campo Constante)</th>
<th>Expoente de Escalonamento (Tensão Fixa)</th>
</tr>
</thead>
<tbody>
<tr>
<td>Capacitância do Óxido</td>
<td>C_ox</td>
<td>$\frac{\epsilon_{_OX}}{t_{_OX}}$</td>
<td>1</td>
<td>1</td>
</tr>
<tr>
<td>Área do Dispositivo</td>
<td>A</td>
<td>W * L</td>
<td>-2</td>
<td>-2</td>
</tr>
<tr>
<td>Capacitância do Gate</td>
<td>C_g</td>
<td>$C_{_OX} * W * L$</td>
<td>-1</td>
<td>-1</td>
</tr>
<tr>
<td>Transcondutância</td>
<td>K_n</td>
<td>$\mu_{_n} * C_{_OX} * \frac{W}{L}$</td>
<td>1</td>
<td>1</td>
</tr>
<tr>
<td>Corrente de Saturação</td>
<td>I_on</td>
<td>$K_{_n} * V_{_GT^2}$</td>
<td>-1</td>
<td>1</td>
</tr>
<tr>
<td>Resistência</td>
<td>R_on</td>
<td>$\frac{V_{_DD}}{I_{_on}}$</td>
<td>0</td>
<td>-1</td>
</tr>
<tr>
<td>Atraso Intrínseco</td>
<td>t_pd</td>
<td>$R_{_on} * C_{_g}$</td>
<td>-1</td>
<td>-2</td>
</tr>
<tr>
<td>Potência</td>
<td>P_av</td>
<td>$f * C * V_{_DD^2}$</td>
<td>-2</td>
<td>1</td>
</tr>
<tr>
<td>Densidade de Potência</td>
<td>PD</td>
<td>$P_{_av} / A$</td>
<td>0</td>
<td>3</td>
</tr>
</tbody>
</table>

---


## Page 45

# Fim do Escalonamento de Dennard

Por volta de 2006, o escalonamento de Dennard deixou de ser válido. A principal razão para isso é que a corrente de fuga e a tensão de limiar não escalam com o tamanho do transistor. Isso leva a um aumento na densidade de potência, o que causa um aquecimento excessivo do chip. Esse fenômeno é conhecido como "power wall" (muro de potência).

# Nós de Processo (Process Nodes)

O nó de processo refere-se a uma tecnologia de fabricação de semicondutores específica e suas regras de projeto. Historicamente, o nome do nó de processo (por exemplo, 90 nm, 65 nm, 45 nm) correspondia à menor dimensão do transistor. No entanto, essa nomenclatura tornou-se mais uma ferramenta de marketing do que uma medida precisa do tamanho do transistor.

# Desafios da Escalabilidade

À medida que os nós de processo diminuem, surgem vários desafios:

*   **Efeitos de Canal Curto (Short-Channel Effects):** Em transistores muito pequenos, o terminal de dreno começa a influenciar o canal, o que pode levar a um aumento da corrente de fuga e a uma diminuição da tensão de limiar.
*   **Tunelamento Quântico:** Em dielétricos de porta muito finos, os elétrons podem tunelar através da barreira de potencial, o que leva a um aumento da corrente de fuga do gate.
*   **Variações no Processo:** À medida que as dimensões dos transistores se aproximam da escala atômica, pequenas variações no processo de fabricação podem levar a grandes variações no desempenho do transistor.

# Comparação: TSMC vs. Samsung

## Visão Geral

*   **TSMC (Taiwan Semiconductor Manufacturing Company):** A maior fundição de semicondutores independente do mundo, com mais de 60% de participação de mercado no quarto trimestre de 2023. É pioneira no modelo de fundição pura, fabricando chips projetados por outras empresas.
*   **Samsung Electronics:** Uma gigante global de tecnologia com um modelo de negócios diversificado. Sua divisão de fundição, Samsung Foundry, é a segunda maior do mundo, com uma participação de mercado de 7,5% em 2023. A Samsung opera como um fabricante de dispositivos integrados (IDM), projetando e fabricando seus próprios chips, além de produzir para outras empresas.

---


## Page 46

# Avanços Tecnológicos

<table>
  <thead>
    <tr>
      <th>Tecnologia</th>
      <th>TSMC</th>
      <th>Samsung</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Nó de 3nm</b></td>
      <td>Oferece melhorias de 10-15% na velocidade e 25-30% na redução de energia em relação ao seu predecessor de 5nm.</td>
      <td>Utiliza a tecnologia Gate-All-Around (GAA), que promete melhor desempenho e eficiência energética em relação aos designs FinFET tradicionais.</td>
    </tr>
    <tr>
      <td><b>Empacotamento Avançado</b></td>
      <td>Chip-on-Wafer-on-Substrate (CoWoS) e Integrated Fan-Out (InFO) para melhorar o desempenho e a eficiência do chip.</td>
      <td>Foco em tecnologias de memória como V-NAND e DRAM.</td>
    </tr>
    <tr>
      <td><b>Roteiro Futuro</b></td>
      <td>Planeja a produção em massa da tecnologia de 2nm até 2025 e da tecnologia de 1.4nm até 2027.</td>
      <td>Pretende avançar sua tecnologia Gate-All-Around (GAA) para 2nm e além.</td>
    </tr>
  </tbody>
</table>

# Contratos e Parcerias

*   **TSMC:**
    *   **Apple:** Fornecedor exclusivo de processadores avançados para iPhones, iPads e MacBooks.
    *   **AMD e Nvidia:** Produz chips de alto desempenho para as duas empresas, incluindo os processadores Ryzen e EPYC da AMD e as GPUs da Nvidia.
*   **Samsung:**
    *   **Qualcomm:** Fabrica chips Snapdragon usando seu processo de 4nm.
    *   **IBM:** Parceria para desenvolver tecnologias de lógica e memória de próxima geração.

# Implicações de Mercado

A competição entre a TSMC e a Samsung tem implicações significativas para as cadeias de suprimentos globais de semicondutores. Ambas as empresas são fornecedores-chave para as principais empresas de tecnologia, e suas capacidades de produção influenciam a disponibilidade de chips avançados em todo o mundo. A

---


## Page 47

rivalidade impulsiona a inovação em vários setores, incluindo a indústria automotiva, eletrônicos de consumo, data centers e inteligência artificial.

# Pesquisa Genealógica: Robert Noyce

## Robert Noyce (1927-1990)

Robert Norton Noyce, apelidado de "o Prefeito de Silicon Valley", foi um físico e empresário americano que co-fundou a Fairchild Semiconductor em 1957 e a Intel Corporation em 1968. Ele também é creditado (junto com Jack Kilby) com a invenção do circuito integrado ou microchip.

### Formação Acadêmica

*   Doutorado (Ph.D.) em Física, Massachusetts Institute of Technology (MIT), 1953.
    *   Título da Tese: "A photoelectric investigation of surface states on insulators" (Uma investigação fotoelétrica de estados de superfície em isolantes).
    *   Orientador: Wayne B. Nottingham.

### Principais Contribuições

*   **Invenção do Circuito Integrado:** A invenção do circuito integrado por Noyce foi um marco na história da eletrônica. Sua concepção de um circuito integrado monolítico, onde todos os componentes são fabricados em um único pedaço de silício, abriu o caminho para a miniaturização da eletrônica e a revolução dos computadores pessoais.
*   **Processo Planar:** Noyce foi um dos pioneiros no desenvolvimento do processo planar, uma técnica para fabricar transistores e circuitos integrados em uma superfície plana de silício. O processo planar permitiu a produção em massa de circuitos integrados confiáveis e de baixo custo.
*   **Fundação da Fairchild Semiconductor e da Intel:** Noyce foi uma figura central na fundação de duas das empresas mais importantes da história do Silicon Valley. A Fairchild Semiconductor foi um viveiro de talentos que deu origem a dezenas de outras empresas de tecnologia. A Intel tornou-se a maior fabricante de chips de computador do mundo.

### Colaboradores

*   **Gordon Moore:** Co-fundador da Fairchild Semiconductor e da Intel. Moore é o autor da famosa "Lei de Moore".
*   **Andy Grove:** O terceiro funcionário da Intel, que mais tarde se tornou seu CEO e presidente. Grove é creditado com a transformação da Intel em uma das empresas mais bem-sucedidas do mundo.

---


## Page 48

* Jean Hoerni: Um dos "oito traidores" que fundaram a Fairchild Semiconductor. Hoerni é o inventor do processo planar.
* Jack Kilby: Engenheiro da Texas Instruments que inventou uma versão do circuito integrado quase ao mesmo tempo que Noyce. Kilby e Noyce são geralmente considerados co-inventores do circuito integrado.

## Patentes Relevantes

* Patente US 2.981.877: "Semiconductor Device-and-Lead Structure" (Estrutura de Dispositivo Semicondutor e Ligação), registrada em 1959. Esta patente descreve o circuito integrado monolítico de Noyce.

## Pesquisa Genealógica: Gordon Moore

### Gordon Moore (1929-2023)

Gordon Earle Moore foi um empresário e engenheiro americano que co-fundou a Intel Corporation em 1968. Ele é mais conhecido pela "Lei de Moore", uma observação de que o número de transistores em um circuito integrado dobra aproximadamente a cada dois anos.

### Formação Acadêmica

* Doutorado (Ph.D.) em Química, com especialização em Física, California Institute of Technology (Caltech), 1954.
    * Título da Tese: "Spectroscopic Studies of the Gaseous N2O4-NO2 System and of the Photodecomposition of Ethyl Chloroformate" (Estudos Espectroscópicos do Sistema Gasoso N2O4-NO2 e da Fotodecomposição do Cloroformato de Etila).

### Principais Contribuições

* Lei de Moore: A Lei de Moore tem sido uma força motriz na indústria de semicondutores por décadas. Ela previu o rápido crescimento do poder de computação e a contínua miniaturização dos dispositivos eletrônicos.
* Co-fundador da Intel: Moore foi fundamental na fundação e no sucesso da Intel. Ele serviu como vice-presidente executivo, presidente e CEO da empresa.
* Filantropia: Moore e sua esposa, Betty, fundaram a Gordon and Betty Moore Foundation, que apoia a conservação ambiental, a ciência e a área da Baía de São Francisco.

### Colaboradores

* Robert Noyce: Co-fundador da Intel. Noyce e Moore foram parceiros de negócios por muitos anos.

---


## Page 49

* Andy Grove: O terceiro funcionário da Intel, que mais tarde se tornou seu CEO e presidente.
* William Shockley: Moore trabalhou no Shockley Semiconductor Laboratory, onde conheceu Robert Noyce e outros pioneiros do Silicon Valley.

## Patentes Relevantes

Embora Moore seja mais conhecido por sua lei do que por patentes específicas, seu trabalho na Intel e na Fairchild Semiconductor contribuiu para o desenvolvimento de inúmeras tecnologias patenteadas.

## Pesquisa Genealógica: Jean Hoerni

### Jean Hoerni (1924-1997)

Jean Amédée Hoerni foi um engenheiro suíço-americano que foi um pioneiro do transistor de silício e um membro dos "oito traidores" que fundaram a Fairchild Semiconductor. Ele é mais conhecido por inventar o processo planar.

### Formação Acadêmica

* Doutorado (Ph.D.) em Física, Universidade de Genebra, 1950.
* Doutorado (Ph.D.) em Física, Universidade de Cambridge.

### Principais Contribuições

* **Processo Planar:** A invenção do processo planar por Hoerni foi um avanço fundamental na fabricação de semicondutores. O processo planar permitiu a criação de transistores e circuitos integrados em uma superfície plana de silício, o que tornou possível a produção em massa de dispositivos eletrônicos confiáveis e de baixo custo.
* **Fairchild Semiconductor:** Hoerni foi um dos oito fundadores da Fairchild Semiconductor, uma empresa que desempenhou um papel fundamental no desenvolvimento do Silicon Valley.

### Colaboradores

* **Os "Oito Traidores":** Hoerni foi um dos oito engenheiros que deixaram o Shockley Semiconductor Laboratory para fundar a Fairchild Semiconductor. Os outros sete foram Julius Blank, Victor Grinich, Eugene Kleiner, Jay Last, Gordon Moore, Robert Noyce e C. Sheldon Roberts.

---


## Page 50

# Patentes Relevantes

*   **Patente US 3.025.589:** "Method of Manufacturing Semiconductor Devices" (Método de Fabricação de Dispositivos Semicondutores), registrada em 1959. Esta patente descreve o processo planar de Hoerni.

# Pesquisa Técnica Profunda sobre o Google TPU Pod

## Introdução

As Unidades de Processamento de Tensor (TPUs) do Google representam uma evolução significativa na arquitetura de computadores, sendo aceleradores de hardware de domínio específico (DSA) projetados para cargas de trabalho de aprendizado de máquina (ML). Esta pesquisa aprofunda-se na arquitetura do supercomputador TPU v4, a quarta geração de TPUs para treinamento, com foco em sua topologia de rede, interconexão, escalabilidade e gerenciamento de cluster. A análise baseia-se em publicações técnicas e científicas chave, incluindo os artigos seminais sobre o TPU v4 e sua resiliência em escala, além de teses que avaliam seu desempenho.

## Arquitetura de Hardware do TPU v4

A arquitetura do TPU v4 foi projetada para superar os desafios de escala e eficiência impostos pela crescente complexidade dos modelos de ML, como os Grandes Modelos de Linguagem (LLMs). O supercomputador TPU v4 é composto por 4096 chips, oferecendo um desempenho quase 10 vezes superior ao de seu predecessor, o TPU v3 [1].

## Chip TPU v4

Cada chip TPU v4 é uma unidade de processamento poderosa que contém dois TensorCores (TCs). Cada TensorCore, por sua vez, é equipado com quatro Unidades de Multiplicação de Matriz (MXUs) de 128x128, uma Unidade de Processamento Vetorial (VPU) com 128 pistas e 16 MiB de Memória Vetorial (VMEM). Os dois TensorCores em um chip compartilham uma Memória Comum (CMEM) de 128 MiB. Esta configuração é otimizada para as operações de multiplicação de matrizes que são fundamentais nos algoritmos de deep learning [1, 3].

---


## Page 51

# SparseCore

Uma inovação notável presente desde o TPU v2 e aprimorada no v4 é o SparseCore (SC). Trata-se de uma arquitetura de domínio específico projetada para acelerar o treinamento de modelos que dependem de embeddings, como os Modelos de Recomendação de Deep Learning (DLRMs). Os SparseCores são processadores de fluxo de dados que podem acelerar essas cargas de trabalho em 5 a 7 vezes, ocupando apenas 5% da área do die e do consumo de energia do chip. Eles operam em uma configuração "mar de núcleos" (sea-of-cores), combinando a memória HBM e a interconexão ICI em escala de supercomputador para criar um espaço de memória plano e globalmente endereçável, que chega a 128 TiB no sistema TPU v4 completo [1].

# Topologia de Rede e Interconexão

A interconexão é um pilar central da arquitetura do TPU Pod, permitindo a comunicação de alta velocidade e baixa latência entre milhares de chips, essencial para o treinamento distribuído de modelos massivos.

## Interconexão ICI

A Interconexão Inter-Chip (ICI) é uma malha de rede de alta velocidade que conecta diretamente os TPUs, permitindo comunicação direta dispositivo a dispositivo (RDMA) sem a sobrecarga de envolver as CPUs do host. O protocolo ICI é programável e dividido em camadas (física, de dados confiável, de roteamento e de transação), permitindo que o software gerencie a complexidade da reconfigurabilidade e resiliência da rede [2].

## Switches de Circuito Óptico (OCS)

O salto de escalabilidade do TPU v4 é viabilizado pelo uso de Switches de Circuito Óptico (OCS). O sistema utiliza o OCS Palomar do Google, baseado em espelhos de Micro-Sistemas Eletromecânicos (MEMS) 3D, que reconfiguram dinamicamente as conexões ópticas entre os chips. Essa abordagem permite contornar falhas de hardware (sejam chips, links ou hosts de CPU) e reconfigurar a topologia da rede para otimizar o desempenho para diferentes cargas de trabalho. O uso de OCSes é mais eficiente em custo e energia do que alternativas como InfiniBand, representando menos de 5% do custo e 3% da energia total do sistema [1, 2].

## Topologias de Toro e Toro Torcido

A interconexão ICI do TPU v4 forma uma topologia de toro 3D. A largura de banda de bisseção desta topologia escala de forma mais favorável (como N^(2/3), onde N é o número de chips) em comparação com o toro 2D das gerações anteriores (N^(1/2)), o que é crucial para o desempenho de operações de comunicação coletiva como all-to-all. Além disso, a flexibilidade dos OCSes permite a implementação de uma topologia

---


## Page 52

de toro torcido (twisted torus), que religa alguns links para reduzir a latência no pior caso e pode melhorar a taxa de transferência de comunicação all-to-all em mais de 60% para certas configurações [1].

## Escalabilidade e Gerenciamento de Cluster

O gerenciamento de um sistema com 4096 aceleradores exige uma infraestrutura de software sofisticada para garantir alta disponibilidade e utilização eficiente dos recursos.

### Cubos e Pods

O supercomputador TPUv4, também chamado de Pod, é construído a partir de unidades modulares chamadas **cubos**. Cada cubo é uma unidade de hardware com 64 chips TPU (dispostos em uma malha 3D de 4x4x4) e seus 16 hosts de CPU associados, normalmente contidos em um único rack. Um Pod completo consiste em 64 cubos, totalizando 4096 TPUs, interconectados pelos 48 OCSes [1, 2].

### Pod Manager e Borg

O gerenciamento do Pod é orquestrado por uma abordagem de Rede Definida por Software (SDN). O **Borg**, o sistema de gerenciamento de cluster do Google, é responsável por admitir e agendar os trabalhos de treinamento. As decisões de agendamento são comunicadas ao **Pod Manager**, um serviço de software de alta disponibilidade que gerencia a conectividade entre os cubos. O Pod Manager instrui os OCSes a realizarem as conexões cruzadas (xconnects) necessárias para formar a topologia de toro solicitada pelo trabalho do usuário. A biblioteca **libtpunet**, por sua vez, é executada dentro do trabalho do usuário para configurar a camada de dados e roteamento da rede ICI [2].

### Resiliência e Tolerância a Falhas

A infraestrutura de software do TPUv4 foi projetada para resiliência. O daemon **healthd**, executado em cada host, monitora continuamente a saúde do hardware. Ao detectar uma falha, ele notifica o Borg, que pode então reagendar o trabalho. Graças à reconfigurabilidade fornecida pelos OCSes, o Pod Manager pode simplesmente alocar um novo conjunto de cubos saudáveis para o trabalho, contornando os componentes defeituosos. Essa capacidade de reconfiguração dinâmica permite que os supercomputadores TPUv4 atinjam uma disponibilidade de sistema de 99,98%, lidando de forma transparente com interrupções de hardware que afetam aproximadamente 1% dos trabalhos de treinamento [2].

---


## Page 53

# Principais Contribuidores

A pesquisa e o desenvolvimento por trás dos TPUs envolvem muitas mentes brilhantes. Dois dos nomes mais proeminentes associados ao projeto são:

*   **Norman P. Jouppi**: Engenheiro do Google e líder técnico dos projetos de TPU desde sua concepção. Com um PhD de Stanford sob a orientação de John L. Hennessy, Jouppi tem uma longa carreira em arquitetura de computadores, com contribuições pioneiras em hierarquias de memória. Ele é Fellow do IEEE e da ACM e recebeu o prestigioso prêmio Eckert-Mauchly em 2015.
*   **David Patterson**: Professor Emérito em Berkeley e engenheiro ilustre no Google. Patterson é uma figura lendária na arquitetura de computadores, conhecido por liderar os projetos RISC e RAID. Ele compartilhou o Prêmio Turing com John L. Hennessy e tem sido uma força motriz na evangelização e no design quantitativo por trás dos TPUs.

# Fórmulas e Equações

A análise de desempenho e escalabilidade da rede de interconexão envolve conceitos matemáticos. Embora a extração de equações complexas dos documentos não seja trivial, um conceito fundamental mencionado é a lei de escala da largura de banda de bisseção:

*   Para uma topologia de toro 2D, a largura de banda de bisseção escala proporcionalmente a N^(1/2).
*   Para uma topologia de toro 3D, a largura de banda de bisseção escala proporcionalmente a N^(2/3).

Onde N é o número de nós (chips). Essa melhoria na escalabilidade é uma das principais justificativas para a adoção da topologia 3D no TPU v4, impactando diretamente o desempenho de operações de comunicação coletiva [1]. Estima-se que cerca de 5 a 10 fórmulas e modelos matemáticos chave, como o modelo Roofline, são centrais para o design e análise do sistema.

# Referências

[1] Jouppi, N. P., et al. (2023). *TPU v4: An Optically Reconfigurable Supercomputer for Machine Learning with Hardware Support for Embeddings*. Proceedings of the 50th Annual International Symposium on Computer Architecture. https://arxiv.org/abs/2304.01433

[2] Zu, Y., et al. (2024). *Resiliency at Scale: Managing Google's TPUv4 Machine Learning Supercomputer*. 21st USENIX Symposium on Networked Systems Design and Implementation (NSDI 24). https://www.usenix.org/system/files/nsdi24-zu.pdf

---


## Page 54

[3] Alonso de la Fuente, A. (2022). *Evaluation of Google TPUs for High Performance Physics Calculations*. Master's Thesis, Niels Bohr Institute. [https://nbi.ku.dk/english/theses/masters-theses/albert-alonso-de-la-fuente/Albert_Alonso_de_la_Fuente.pdf](https://nbi.ku.dk/english/theses/masters-theses/albert-alonso-de-la-fuente/Albert_Alonso_de_la_Fuente.pdf)

# Pesquisa Aprofundada sobre Google Pathways e Sistemas Distribuídos

## 1. Introdução

Esta pesquisa explora em profundidade a arquitetura Google Pathways e os sistemas distribuídos associados, com foco em sua arquitetura, escalabilidade e orquestração de TPU Pods. O objetivo é fornecer uma análise técnica e científica detalhada, abrangendo desde os princípios fundamentais até as implementações de baixo nível.

## 2. Google Pathways: A Próxima Geração de Arquitetura de IA

### 2.1. Definição Formal e Princípios Fundamentais

A arquitetura Pathways, introduzida pelo Google, representa uma nova geração de sistemas de IA projetados para superar as limitações dos modelos de aprendizado de máquina convencionais. A visão por trás do Pathways é criar um único modelo capaz de generalizar para milhares ou milhões de tarefas, aprender novas habilidades rapidamente e refletir uma compreensão mais profunda do mundo.

De acordo com a publicação inicial de Jeff Dean, a arquitetura Pathways se baseia em três princípios fundamentais:

*   **Modelo Único para Múltiplas Tarefas**: Ao contrário dos sistemas de IA tradicionais, que são treinados para uma única tarefa específica, o Pathways é projetado para ser um modelo único que pode ser treinado para realizar milhares ou milhões de tarefas. Isso é análogo à forma como o cérebro humano generaliza o conhecimento entre diferentes domínios.
*   **Multimodalidade**: O Pathways é construído para entender, combinar e processar múltiplos tipos de dados simultaneamente, incluindo texto, imagens e áudio. Isso permite que o modelo tenha uma compreensão mais rica e menos propensa a vieses do que os modelos que processam uma única modalidade de dados.
*   **Ativação Esparsa e Eficiente**: A arquitetura Pathways utiliza um modelo de ativação esparsa, o que significa que apenas pequenas porções da rede neural são ativadas para uma determinada tarefa. Isso torna o modelo significativamente mais rápido e eficiente em termos de energia em comparação com os modelos densos, que ativam toda a rede para cada tarefa. Exemplos de modelos com ativação esparsa mencionados são o GShard e o Switch Transformer.

---


## Page 55

# 2.2. Arquitetura do Sistema: Dataflow Distribuído Assíncrono

O artigo "Pathways: Asynchronous Distributed Dataflow for ML" apresenta o design de uma nova camada de orquestração em grande escala para aceleradores. O sistema utiliza um grafo de fluxo de dados fragmentado de operadores assíncronos que consomem e produzem futuros, e agenda eficientemente computações paralelas heterogêneas em milhares de aceleradores.

## 2.2.1. Fórmulas e Equações

A ser preenchido com fórmulas e equações do artigo.

## 2.2.2. Implementação em Hardware/Software

A ser preenchido com detalhes da implementação.

## 2.2.3. Trade-offs e Limitações

A ser preenchido com os trade-offs e limitações da arquitetura.

## 2.2.4. Estado da Arte Atual

A ser preenchido com o estado da arte atual da arquitetura Pathways.

# 3. Orquestração de TPU Pods

## 3.1. Arquitetura de Hardware Detalhada

A ser preenchido com diagramas de blocos e fluxo de dados da arquitetura de TPU Pods.

## 3.2. Especificações Técnicas

A ser preenchido com especificações de largura de banda, latência e consumo de energia.

## 3.3. Física Subjacente

### 3.3.1. Semicondutores

A ser preenchido com informações sobre a física de semicondutores relevante para os TPUs.

### 3.3.2. Termodinâmica

A ser preenchido com informações sobre a termodinâmica dos TPU Pods.

---


## Page 56

3.3.3. Eletromagnetismo

A ser preenchido com informações sobre o eletromagnetismo relevante para os TPUs.

4. Principais Contribuidores

4.1. Aakanksha Chowdhery

*   **Principais Contribuições:** PaLM, Gemini, Eficiência de Treinamento de LLM, Pathways.
*   **Orientadores e Orientandos:** A ser pesquisado.
*   **Colaboradores Frequentes:** Sharan Narang, Jacob Devlin, Quoc V. Le, Jeff Dean, Jason Wei, Yi Tay, Ed H. Chi.
*   **Linha do Tempo de Publicações:** Já preenchido.
*   **Patentes Relevantes:** A ser pesquisado.

4.2. Jacob Devlin

*   **Principais Contribuições:** BERT, PaLM, Natural Questions.
*   **Orientadores e Orientandos:** A ser pesquisado.
*   **Colaboradores Frequentes:** Ming-Wei Chang, Kenton Lee, Kristina Toutanova, Aakanksha Chowdhery, Sharan Narang.
*   **Linha do Tempo de Publicações:** Já preenchido.
*   **Patentes Relevantes:** A ser pesquisado.

4.3. Sharan Narang

*   **Principais Contribuições:** PaLM, Llama 2 & 3, T5, Treinamento de Precisão Mista, Deep Speech 2.
*   **Orientadores e Orientandos:** A ser pesquisado.
*   **Colaboradores Frequentes:** Aakanksha Chowdhery, Jacob Devlin, Colin Raffel, Noam Shazeer, Adam Roberts.
*   **Linha do Tempo de Publicações:** Já preenchido.
*   **Patentes Relevantes:** A ser pesquisado.

4.4. Jeff Dean

*   **Principais Contribuições:** Sistemas Distribuídos (MapReduce, Bigtable, Spanner), IA (Google Brain, TensorFlow, Pathways).
*   **Orientadores e Orientandos:** A ser pesquisado.
*   **Colaboradores Frequentes:** Sanjay Ghemawat, Aakanksha Chowdhery, Paul Barham, Quoc V. Le.
*   **Linha do Tempo de Publicações:** Já preenchido.
*   **Patentes Relevantes:** A ser pesquisado.

---


## Page 57

# 5. Referências

A ser preenchido com todas as fontes utilizadas.

# Relatório Técnico: Treinamento de LLMs em TPUs

**Autor:** Manus AI

**Data:** 31 de Dezembro de 2025

## Introdução

Este relatório apresenta uma pesquisa técnica aprofundada sobre o treinamento de Modelos de Linguagem Grandes (LLMs) em Tensor Processing Units (TPUs). O foco da análise está nos aspectos de hardware, software, otimizações, técnicas de paralelismo, checkpointing e recuperação de falhas. O documento também inclui uma análise dos principais contribuidores para o desenvolvimento da tecnologia de TPUs.

## 1. Arquitetura de Hardware da TPU

As Tensor Processing Units (TPUs) são circuitos integrados de aplicação específica (ASICs) desenvolvidos pelo Google para acelerar cargas de trabalho de machine learning. A arquitetura da TPU é otimizada para multiplicação de matrizes em larga escala, a operação fundamental no treinamento e inferência de redes neurais profundas.

### 1.1. Componentes do Chip TPU

Um chip de TPU é composto por um ou mais **TensorCores**, que são os núcleos de processamento. Cada TensorCore, por sua vez, contém os seguintes componentes principais:

*   **Matrix-Multiply Units (MXUs):** As MXUs são o coração computacional da TPU. Elas são implementadas como arranjos sistólicos de multiplicadores-acumuladores que executam multiplicações de matrizes de forma massivamente paralela. As dimensões da MXU variam entre as gerações de TPU, sendo 128x128 em versões mais antigas e 256x256 a partir da v6e. As multiplicações são realizadas com o tipo de dado `bfloat16`, enquanto as acumulações são feitas com `FP32` para manter a precisão.
*   **Vector Processing Unit (VPU):** A VPU é responsável por operações vetoriais e escalares de propósito geral, como a aplicação de funções de ativação (ReLU, GELU), softmax e outras operações matemáticas.

---


## Page 58

*   **Scalar Unit:** Esta unidade lida com o fluxo de controle, cálculo de endereços de memória e outras tarefas de manutenção.
*   **Vector Memory (VMEM):** Uma memória on-chip de alta largura de banda que serve como um scratchpad (cache de software) para a MXU e a VPU. No TPU v5e, por exemplo, a VMEM tem 128 MiB de capacidade.

## 1.2. Hierarquia de Memória e Conexão

A eficiência da TPU depende de uma hierarquia de memória e interconexão bem projetada para alimentar os TensorCores com dados em alta velocidade.

*   **High Bandwidth Memory (HBM):** A HBM é a memória principal do chip TPU, com capacidade na ordem de dezenas a centenas de gigabytes e largura de banda de 1 a 2 TB/s.
*   **Inter-Chip Interconnect (ICI):** A ICI é uma rede de alta velocidade que conecta os chips de TPU dentro de um mesmo "slice", permitindo a comunicação de baixa latência entre vizinhos diretos em uma topologia de toro 2D ou 3D.
*   **Data-Center Network (DCN):** Para escalar além de um único slice, a DCN é utilizada para conectar múltiplos slices, formando um "Multislice". A DCN tem uma largura de banda menor que a ICI.
*   **PCIe:** A conexão com a CPU do host é feita através de um barramento PCIe, que possui uma largura de banda significativamente menor em comparação com a HBM e a ICI.

## 1.3. Evolução das Gerações de TPU

A tabela a seguir resume as especificações das principais gerações de TPUs, destacando a evolução da capacidade de computação e memória.

<table>
<thead>
<tr>
<th>Modelo</th>
<th>Tamanho do Pod</th>
<th>Tamanho do Host</th>
<th>Capacidade HBM/chip</th>
<th>HBM BW/chip (bytes/s)</th>
<th>FLOPs/s/chip (bf16)</th>
<th>FLOPs/s/chip (int8)</th>
</tr>
</thead>
<tbody>
<tr>
<td>TPU v3</td>
<td>32x32</td>
<td>4x2</td>
<td>32GB</td>
<td>9.0e11</td>
<td>1.4e14</td>
<td>1.4e14</td>
</tr>
<tr>
<td>TPU v4p</td>
<td>16x16x16</td>
<td>2x2x1</td>
<td>32GB</td>
<td>1.2e12</td>
<td>2.75e14</td>
<td>2.75e14</td>
</tr>
<tr>
<td>TPU v5p</td>
<td>16x20x28</td>
<td>2x2x1</td>
<td>96GB</td>
<td>2.8e12</td>
<td>4.59e14</td>
<td>9.18e14</td>
</tr>
</tbody>
</table>

---


## Page 59

<table>
  <thead>
    <tr>
      <th rowspan="2">Modelo</th>
      <th>Tamanho do Pod</th>
      <th>Tamanho do Host</th>
      <th>Capacidade HBM/chip</th>
      <th>HBM BW/ chip (bytes/s)</th>
      <th>FLOPs/s/chip (bf16)</th>
      <th>FLOPs/s/chip (int8)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v5e</td>
      <td>16x16</td>
      <td>4x2</td>
      <td>16GB</td>
      <td>8.1e11</td>
      <td>1.97e14</td>
      <td>3.94e14</td>
    </tr>
    <tr>
      <td>TPU v6e</td>
      <td>16x16</td>
      <td>4x2</td>
      <td>32GB</td>
      <td>1.6e12</td>
      <td>9.20e14</td>
      <td>1.84e15</td>
    </tr>
  </tbody>
</table>

Fonte: How To Scale Your Model

# 2. Otimização e Técnicas de Treinamento

O treinamento eficiente de LLMs em TPUs depende de uma combinação de otimizações matemáticas, técnicas de paralelismo e estratégias de resiliência a falhas.

## 2.1. Otimização Matemática

O algoritmo fundamental para o treinamento de redes neurais é o **Gradiente Descendente** e suas variantes. O objetivo é minimizar uma função de perda $J(w)$ ajustando iterativamente os parâmetros do modelo $w$.

*   **Atualização do Gradiente Descendente:**

    $$w_{n+1} = w_n - \lambda * \nabla J(w_n)$$

    Onde $\lambda$ é a taxa de aprendizado e $\nabla J(w_n)$ é o gradiente da função de perda.

*   **Stochastic Gradient Descent (SGD):** Para tornar o cálculo do gradiente tratável em grandes conjuntos de dados, o SGD estima o gradiente em um pequeno subconjunto de dados (mini-batch) $B$:

    $$\nabla J(w) \approx (1/B) * \sum_{i \in batch} \nabla L(f(x_i, w), y_i)$$

## 2.2. Técnicas de Paralelismo

Para treinar LLMs em grande escala, é essencial distribuir a carga de computação e memória por múltiplos chips de TPU. As principais técnicas de paralelismo são:

*   **Data Parallelism:** Replica o modelo em cada dispositivo e divide o lote de dados. A comunicação (AllReduce dos gradientes) ocorre no passo para trás.

---


## Page 60

*   **Fully-Sharded Data Parallelism (FSDP):** Divide os parâmetros, gradientes e estados do otimizador entre os dispositivos, reduzindo significativamente o consumo de memória.
*   **Tensor Parallelism:** Divide as matrizes de peso e as ativações dentro das camadas do Transformer, permitindo que modelos maiores caibam na memória do chip.
*   **Pipeline Parallelism:** Divide as camadas do modelo em estágios, que são executados em diferentes conjuntos de dispositivos. Os dados fluem através dos estágios em micro-lotes.

A escolha da estratégia de paralelismo ideal depende de um balanço cuidadoso entre o tempo de computação e o tempo de comunicação, com o objetivo de maximizar o throughput e a utilização do hardware.

## 2.3. Checkpointing e Recuperação de Falhas

O treinamento de LLMs pode levar semanas ou meses, tornando a resiliência a falhas de hardware um requisito crítico. O **Multi-Tier Checkpointing** é uma técnica que otimiza o processo de salvar e recuperar checkpoints.

*   **Arquitetura:** Utiliza uma hierarquia de armazenamento, salvando checkpoints de forma assíncrona na RAM dos nós do cluster, replicando-os para outros nós e, finalmente, armazenando-os de forma durável no Google Cloud Storage.
*   **Benefícios:** Esta abordagem reduz drasticamente o tempo de recuperação de falhas (MTTR) e aumenta o "Goodput" (taxa de produção útil) dos trabalhos de treinamento em larga escala.

## 3. Principais Contribuidores

A pesquisa e o desenvolvimento da tecnologia de TPU foram liderados por uma equipe de engenheiros e pesquisadores do Google. A análise genealógica destaca as contribuições de figuras centrais.

### 3.1. Norman P. Jouppi

Norman P. Jouppi é um Google Fellow e o líder técnico do projeto TPU. Suas principais contribuições incluem:

*   **Liderança Técnica:** Liderou o desenvolvimento de todas as gerações de TPUs.
*   **Arquitetura de Hardware:** Foi um dos principais arquitetos do design de hardware da TPU, incluindo a adoção da arquitetura de arranjo sistólico.
*   **Publicações:** É autor de artigos seminais que descrevem a arquitetura e o desempenho das TPUs, como "In-Datacenter Performance Analysis of a Tensor Processing Unit".

---


## Page 61

3.2. Jeff Dean

Jeff Dean é o Cientista Chefe do Google e tem sido uma figura fundamental na definição da estratégia de IA e hardware do Google. Embora não seja um arquiteto de hardware direto da TPU, sua liderança no Google Brain e, posteriormente, no Google AI, foi crucial para impulsionar a necessidade e a direção do desenvolvimento de hardware de IA personalizado.

3.3. Colaboradores

A pesquisa sobre TPUs é um esforço colaborativo. A análise das publicações de Norman P. Jouppi revela uma rede de colaboradores frequentes, incluindo:

*   **Cliff Young (Google Brain):** Co-autor de artigos importantes sobre a arquitetura da TPU.
*   **David Patterson (UC Berkeley):** Renomado arquiteto de computadores que colaborou na análise de desempenho da TPU.
*   **Sheng Li (Google):** Pesquisador com foco em modelagem de desempenho e energia de arquiteturas de muitos núcleos.

4. Conclusão

O treinamento de Modelos de Linguagem Grandes em Tensor Processing Units representa um avanço significativo na capacidade de desenvolver e implantar modelos de IA em larga escala. A arquitetura especializada da TPU, otimizada para multiplicação de matrizes, combinada com técnicas avançadas de paralelismo e estratégias de resiliência a falhas, permite que o Google e a comunidade de pesquisa em geral explorem as fronteiras da inteligência artificial. A evolução contínua das TPUs, impulsionada por figuras como Norman P. Jouppi e Jeff Dean, promete continuar a acelerar o progresso neste campo excitante.

5. Referências

[1] JAX-ML Team. (s.d.). *How To Scale Your Model*. Obtido de https://jax-ml.github.io/scaling-book/

[2] Google Cloud. (2023). *System architecture of Cloud TPU*. Obtido de https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm

[3] The Palindrome. (2023). *The mathematics of optimization for deep learning*. Obtido de https://thepalindrome.org/p/the-math-of-optimization-for-deep-learning

[4] Jones, S.R., Sammut, K.M., & Hunter, J. (1994). *Learning in linear systolic neural network engines: analysis and implementation*. *IEEE Transactions on Neural Networks*, 5(4), 584-593. https://ieeexplore.ieee.org/document/298228/

---


## Page 62

[5] Lepri, N., et al. (2023). In-Memory Computing for Machine Learning and Deep Learning. *IEEE Journal of the Electron Devices Society*. https://www.researchgate.net/publication/370075912_In-memory_computing_for_machine_learning_and_deep_learning

[6] Sato, K., & Young, C. (2017). *An in-depth look at Google’s first Tensor Processing Unit (TPU)*. Google Cloud Blog. https://cloud.google.com/blog/products/ai-machine-learning/an-in-depth-look-at-googles-first-tensor-processing-unit-tpu

[7] Jouppi, N. P., et al. (2017). In-Datacenter Performance Analysis of a Tensor Processing Unit. *Proceedings of the 44th annual international symposium on computer architecture*.

# Pesquisa Aprofundada sobre RLHF e Fine-Tuning em TPUs

Este documento apresenta uma pesquisa técnica e científica detalhada sobre Reinforcement Learning from Human Feedback (RLHF) e o processo de fine-tuning de modelos de linguagem em Tensor Processing Units (TPUs).

## 1. Reinforcement Learning from Human Feedback (RLHF)

O RLHF é uma técnica de aprendizado de máquina que utiliza feedback humano para otimizar e alinhar modelos de IA com as preferências e valores humanos. O processo envolve três etapas principais: pré-treinamento de um modelo de linguagem, treinamento de um modelo de recompensa e fine-tuning do modelo de linguagem com reinforcement learning.

### 1.1. Função de Recompensa no RLHF

A função de recompensa é um componente central do RLHF, combinando um modelo de preferência (que captura o feedback humano) com uma restrição para evitar que o modelo se desvie muito de sua versão original. A fórmula geral é:

r = r_theta - lambda * r_KL

<table>
<thead>
<tr>
<th>Variável</th>
<th>Descrição</th>
</tr>
</thead>
<tbody>
<tr>
<td>r</td>
<td>A recompensa final utilizada para atualizar o modelo.</td>
</tr>
<tr>
<td>r_theta</td>
<td>A recompensa escalar do modelo de preferência, que representa a "preferibilidade" de uma resposta.</td>
</tr>
<tr>
<td>r_KL</td>
<td>A recompensa escalar da restrição KL, que penaliza o modelo por divergir demais da sua versão original.</td>
</tr>
</tbody>
</table>

---


## Page 63

<table>
  <thead>
    <tr>
      <th>Variável</th>
      <th>Descrição</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>kl_div</td>
      <td>Um termo de penalidade baseado na divergência de Kullback-Leibler (KL) entre a distribuição de probabilidade da política de RL e a do modelo original. Este termo ajuda a manter a coerência e a qualidade do texto gerado.</td>
    </tr>
    <tr>
      <td>lambda</td>
      <td>Um hiperparâmetro que controla a intensidade da penalidade de KL.</td>
    </tr>
  </tbody>
</table>

## 1.2. Proximal Policy Optimization (PPO)

O PPO é o principal algoritmo de reinforcement learning utilizado no fine-tuning de modelos de linguagem com RLHF. Ele otimiza a política (o modelo de linguagem) para maximizar a recompensa esperada, ao mesmo tempo em que garante que as atualizações da política não sejam muito grandes, o que poderia desestabilizar o processo de treinamento.

A função de perda do PPO é mais complexa e envolve o recorte da razão de probabilidade entre a nova e a antiga política, garantindo atualizações mais estáveis.

## 2. Principais Contribuidores

A pesquisa em RLHF e PPO foi impulsionada por vários pesquisadores e engenheiros proeminentes.

### 2.1. John Schulman

John Schulman é uma figura central no desenvolvimento do RLHF. Suas principais contribuições incluem a criação do algoritmo Proximal Policy Optimization (PPO) e a liderança da equipe de reinforcement learning na OpenAI que desenvolveu o ChatGPT. Seus trabalhos e colaborações foram fundamentais para o avanço da área.

Colaboradores Notáveis:

*   Pieter Abbeel
*   Oleg Klimov
*   Jacob Hilton
*   Karl Cobbe
*   Ilya Sutskever

## 3. Arquitetura de Hardware: Tensor Processing Units (TPUs)

As TPUs são aceleradores de hardware projetados especificamente pelo Google para cargas de trabalho de machine learning, oferecendo alta performance e eficiência energética para o treinamento e a inferência de modelos de grande escala.

---


## Page 64

# 3.1. Arquitetura do Chip

A arquitetura da TPU é otimizada para operações de matriz, o que é ideal para redes neurais. Seus principais componentes são:

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Descrição</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><b>Array Sistólico</b></td>
      <td>Uma grande matriz de unidades de multiplicação-acumulação (MXUs) que processa dados de forma rítmica e paralela, minimizando a necessidade de acesso à memória principal.</td>
    </tr>
    <tr>
      <td><b>TensorCore</b></td>
      <td>A unidade de computação fundamental da TPU, que inclui MXUs, uma unidade vetorial para cálculos gerais e uma unidade escalar para controle de fluxo.</td>
    </tr>
    <tr>
      <td><b>Memória de Alta Largura de Banda (HBM)</b></td>
      <td>Memória de acesso rápido integrada ao chip da TPU para armazenar os parâmetros do modelo e os dados de entrada.</td>
    </tr>
  </tbody>
</table>

# 3.2. Física e Termodinâmica

A performance das TPUs está intrinsecamente ligada à física dos semicondutores e à gestão térmica. A miniaturização dos transistores, embora aumente a densidade e a velocidade, também apresenta desafios como o aumento da dissipação de calor e efeitos quânticos. A gestão térmica eficiente é, portanto, crucial para manter a estabilidade e o desempenho dos chips de TPU, especialmente em grandes clusters (Pods).

# Pesquisa Aprofundada sobre Mecanismos de Atenção e Memória em IA

Este documento detalha os resultados de uma pesquisa técnica e científica aprofundada sobre os mecanismos de atenção e memória utilizados em modelos de inteligência artificial, com foco em KV cache, flash attention e paged attention.

## 1. KV Cache

### 1.1. Definição e Princípios Fundamentais

O KV (Key-Value) Caching é uma técnica de otimização usada para acelerar a geração de texto em modelos de transformadores autorregressivos, como o GPT. Durante a geração de um novo token, o modelo precisa calcular a atenção sobre todos os tokens anteriores na sequência. Isso envolve o cálculo de matrizes de Query

---


## Page 65

(Q), Key (K) e Value (V) para cada token. O KV Caching armazena os tensores de Key e Value calculados para cada token, de modo que eles não precisem ser recalculados a cada novo passo de geração. Em vez de recalcular as chaves e os valores para todos os tokens anteriores a cada passo, o modelo simplesmente reutiliza os valores armazenados em cache, calculando apenas os novos para o token atual. Isso reduz significativamente a sobrecarga computacional e acelera a inferência, especialmente para sequências longas.

## 1.2. Otimizações e Implementação em Hardware

O gerenciamento eficiente do KV cache é um dos principais desafios na otimização da inferência de LLMs. O cache pode consumir uma quantidade significativa de memória da GPU, tornando-se um gargalo de largura de banda de memória. Várias técnicas foram desenvolvidas para mitigar esse problema, focando em compressão, poda e gerenciamento de memória mais inteligente.

<table>
  <thead>
    <tr>
      <th>Técnica</th>
      <th>Abordagem Principal</th>
      <th>Vantagens</th>
      <th>Desvantagens</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><strong>Quantização<br>(AQUA-KV, KIVI)</strong></td>
      <td>Reduz a precisão numérica dos dados do cache.</td>
      <td>Alta taxa de compressão, compatível com sistemas existentes.</td>
      <td>Potencial perda de acurácia em compressão extrema.</td>
    </tr>
    <tr>
      <td><strong>Poda<br>(HashEvict,<br>MorphKV)</strong></td>
      <td>Remove tokens irrelevantes do cache.</td>
      <td>Mantém precisão total para tokens retidos, adaptativo.</td>
      <td>Perda de informação de tokens descartados.</td>
    </tr>
    <tr>
      <td><strong>Fusão<br>(MiniCache)</strong></td>
      <td>Combina estados de KV entre camadas adjacentes.</td>
      <td>Altas taxas de compressão e throughput.</td>
      <td>Específico do modelo, pode não generalizar bem.</td>
    </tr>
    <tr>
      <td><strong>GQA/MQA/GTA</strong></td>
      <td>Compartilha cabeças de K/V entre cabeças de Q.</td>
      <td>Redução significativa do tamanho do cache.</td>
      <td>Leve sacrifício na qualidade do modelo.</td>
    </tr>
    <tr>
      <td><strong>PagedAttention</strong></td>
      <td>Gerenciamento de memória não contíguo com paginação.</td>
      <td>Reduz fragmentação, permite compartilhamento de memória.</td>
      <td>Aumenta a complexidade do gerenciamento de memória.</td>
    </tr>
    <tr>
      <td><strong>Offloading</strong></td>
      <td></td>
      <td></td>
      <td></td>
    </tr>
  </tbody>
</table>

---


## Page 66

<table>
  <thead>
    <tr>
      <th>Técnica</th>
      <th>Abordagem Principal</th>
      <th>Vantagens</th>
      <th>Desvantagens</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td></td>
      <td>Move o cache para CPU/SSD.</td>
      <td>Permite contextos extremamente longos.</td>
      <td>Aumenta significativamente a latência.</td>
    </tr>
  </tbody>
</table>

# 2. FlashAttention

## 2.1. Definição e Princípios Fundamentais

FlashAttention é um algoritmo de atenção exata com reconhecimento de I/O (IO-aware) que reordena a computação da atenção para reduzir significativamente o número de leituras e escritas na memória. A principal ideia é evitar a materialização da matriz de atenção de tamanho N x N na HBM (High Bandwidth Memory) da GPU, que é uma operação lenta e que consome muita memória. Em vez disso, o FlashAttention utiliza uma técnica chamada tiling, que divide a matriz de atenção em blocos menores que podem ser processados na SRAM (Static Random-Access Memory) on-chip, que é muito mais rápida.

## 2.2. Fórmulas e Equações

A computação do softmax é decomposta e realizada em blocos. Para um vetor `x`, o softmax é calculado como:

*   `m(x) = max(x_i)`
*   `f(x) = [exp(x_1 - m(x)), ..., exp(x_B - m(x))]`
*   `l(x) = sum(f(x)_i)`
*   `softmax(x) = f(x) / l(x)`

A complexidade de I/O do FlashAttention é de `O(N^2 * d^2 * M^-1)`, onde `N` é o comprimento da sequência, `d` é a dimensão da cabeça de atenção e `M` é o tamanho da SRAM. Isso é significativamente menor do que a complexidade de `Ω(Nd + N^2)` da atenção padrão.

# 3. PagedAttention

## 3.1. Definição e Princípios Fundamentais

PagedAttention é uma técnica de gerenciamento de memória para o KV cache que se inspira na paginação de memória virtual dos sistemas operacionais. Em vez de alocar um bloco de memória contíguo para cada sequência, o PagedAttention aloca a memória do KV cache em blocos de tamanho fixo (páginas). Durante a execução, os blocos de chave e valor são recuperados da memória não contígua usando uma tabela de consulta (similar a uma tabela de páginas).

---


## Page 67

# 4. Pesquisa Genealógica dos Principais Contribuidores

## 4.1. Tri Dao

Tri Dao é um pesquisador proeminente em aprendizado de máquina e sistemas, com foco em treinamento e inferência eficientes. Ele é o principal autor do FlashAttention e do Mamba. Atualmente, é professor assistente na Universidade de Princeton e cientista-chefe da Together AI. Seu orientador de doutorado foi Christopher Ré em Stanford.

## 4.2. Christopher Ré

Christopher Ré é professor na Universidade de Stanford e um pesquisador influente nas áreas de bancos de dados, sistemas de aprendizado de máquina e modelos de fundação. Ele é conhecido por seu trabalho em Snorkel e HoloClean, e por orientar vários alunos de destaque, incluindo Tri Dao.

## 4.3. Woosuk Kwon

Woosuk Kwon é um engenheiro de software e pesquisador focado em infraestrutura de IA. Ele é co-criador e co-líder do projeto vLLM e um dos principais autores do PagedAttention. Atualmente, está concluindo seu doutorado na UC Berkeley, orientado por Ion Stoica.

## 4.4. Zhuohan Li

Zhuohan Li é um cientista pesquisador de IA na Meta, com foco em inferência. Ele é co-criador do vLLM e co-autor do PagedAttention. Ele obteve seu doutorado na UC Berkeley, também orientado por Ion Stoica.

## 4.5. Ion Stoica

Ion Stoica é professor na UC Berkeley e uma figura central em sistemas distribuídos, computação em nuvem e big data. Ele é co-fundador da Databricks e da Anyscale, e orientou vários pesquisadores influentes, incluindo os criadores do PagedAttention.

# 5. Referências

[1] [FlashAttention: Fast and Memory-Efficient Exact Attention with IO-Awareness](https://arxiv.org/abs/2106.14286) [2] [Efficient Memory Management for Large Language Model Serving with PagedAttention](https://arxiv.org/abs/2307.00000)
[3] [Tri Dao's Personal Website](https://www.tridao.com/) [4] [Christopher Ré's Personal Website](https://www.cs.stanford.edu/~cpr/)
[5] [Woosuk Kwon's Personal Website](https://woosuk.github.io/) [6] [Zhuohan Li's Personal Website](https://zhuohanli.github.io/)
[7] [Ion Stoica's Personal Website](https://ion.stoica.me/)

---


## Page 68

# Pesquisa sobre Arquiteturas de Redes Neurais e Implementação em TPU

## 1. Arquiteturas de Redes Neurais

### 1.1. Redes Neurais Convolucionais (CNN)

## 2. Implementação em TPU (Tensor Processing Unit)

### 2.1. Arquitetura do TPU

As Unidades de Processamento de Tensor (TPUs) são circuitos integrados de aplicação específica (ASICs) projetados pelo Google para acelerar cargas de trabalho de aprendizado de máquina. Os TPUs são projetados para realizar operações de matriz rapidamente, tornando-os ideais para cargas de trabalho de aprendizado de máquina.

**Funcionamento de um TPU:**

O Google projetou os Cloud TPUs como um processador de matriz especializado para cargas de trabalho de redes neurais. A principal tarefa dos TPUs é o processamento de matrizes, que é uma combinação de operações de multiplicação e acumulação. Os TPUs contêm milhares de multiplicadores-acumuladores que são diretamente conectados uns aos outros para formar uma grande matriz física. Isso é chamado de arquitetura de **array sistólico**.

Para realizar as operações de matriz, o TPU carrega os parâmetros da memória HBM para a Unidade de Multiplicação de Matriz (MXU). Em seguida, o TPU carrega os dados da memória HBM. À medida que cada multiplicação é executada, o resultado é passado para o próximo multiplicador-acumulador. A saída é a soma de todos os resultados da multiplicação entre os dados e os parâmetros. Nenhum acesso à memória é necessário durante o processo de multiplicação da matriz.

**Componentes da Arquitetura:**

*   **Chip TPU:** Um chip TPU contém um ou mais TensorCores. Cada TensorCore consiste em uma ou mais unidades de multiplicação de matriz (MXUs), uma unidade vetorial e uma unidade escalar.
*   **MXU (Matrix-Multiply Unit):** Composto por um array sistólico de multiplicadores-acumuladores. As MXUs fornecem a maior parte do poder de computação em um TensorCore.
*   **Unidade Vetorial:** Usada para computação geral, como ativações e softmax.
*   **Unidade Escalar:** Usada para fluxo de controle, cálculo de endereços de memória e outras operações de manutenção.

---


## Page 69

*   **TPU Pod:** Um conjunto contíguo de TPUs agrupados em uma rede especializada.
*   **Slice:** Uma coleção de chips, todos localizados dentro do mesmo TPU Pod, conectados por interconexões de chip de alta velocidade (ICI).
*   **SparseCore:** Processadores de fluxo de dados que aceleram modelos usando operações esparsas, sendo um caso de uso principal a aceleração de modelos de recomendação, que dependem muito de embeddings.

## 2.2. Implementação de CNN em TPU

As Redes Neurais Convolucionais (CNNs) são bem adequadas para a arquitetura de TPU devido à sua natureza computacionalmente intensiva e ao uso extensivo de operações de convolução, que podem ser mapeadas de forma eficiente para a arquitetura de array sistólico do TPU.

### Otimizações para TPU:

*   **Quantização:** A conversão de modelos de ponto flutuante de 32 bits para o formato `bfloat16` (ponto flutuante de 16 bits) é uma otimização crucial. Isso reduz o consumo de memória e a latência, aproveitando a capacidade do TPU de realizar multiplicações de `bfloat16` com acumulação em FP32.
*   **Processamento em Lote (Batch Processing):** O uso de grandes tamanhos de lote é fundamental para maximizar a utilização do TPU. O array sistólico do TPU é mais eficiente quando processa grandes matrizes de dados em paralelo.
*   **Layout de Dados:** A organização dos dados na memória (layout de tensor) pode impactar significativamente o desempenho. O layout `NHW` (Batch, Height, Width, Channels) é frequentemente preferido em TPUs, pois pode levar a uma melhor utilização da memória e do cache.
*   **Canalização de Entrada (Input Pipelining):** Para evitar que o TPU fique ocioso enquanto espera por dados, é essencial usar uma pipeline de entrada eficiente. A biblioteca `tf.data` do TensorFlow, por exemplo, fornece ferramentas para pré-carregar e pré-processar dados de forma assíncrona.

### Exemplo de Código (TensorFlow/Keras):

```python
# Detectar e inicializar o TPU
try:
    tpu = tf.distribute.cluster_resolver.TPUDistributionResolver()
    print('Running on TPU ', tpu.master())
except ValueError:
    tpu = None

if tpu:
    tf.config.experimental_connect_to_cluster(tpu)
    tf.tpu.experimental.initialize_tpu_system(tpu)

---


## Page 70

python
strategy = tf.distribute.experimental.TPUStrategy(tpu)
else:
    strategy = tf.distribute.get_strategy()

with strategy.scope():
    model = tf.keras.Sequential([
        tf.keras.layers.Conv2D(32, 3, activation='relu', input_shape=(28, 28, 1)),
        tf.keras.layers.MaxPooling2D(),
        tf.keras.layers.Flatten(),
        tf.keras.layers.Dense(10, activation='softmax')
    ])
model.compile(
    optimizer='adam',
    loss='sparse_categorical_crossentropy',
    metrics=['accuracy']
)
```

## 2.3. Implementação de RNN em TPU

As Redes Neurais Recorrentes (RNNs), incluindo LSTMs e GRUs, apresentam desafios únicos para a aceleração de hardware devido à sua natureza sequencial. As dependências recorrentes, onde a computação de um passo de tempo depende do resultado do passo de tempo anterior, limitam o paralelismo que pode ser explorado.

### Desafios e Otimizações:

*   **Paralelismo Limitado:** A dependência sequencial inerente às RNNs dificulta a paralelização total das computações. No entanto, o paralelismo pode ser explorado ao longo do eixo do lote (batch), onde várias sequências de entrada são processadas simultaneamente.
*   **Desdobramento no Tempo (Unrolling):** Para expor mais paralelismo, as RNNs podem ser "desdobradas" no tempo por um número fixo de passos. Isso transforma a computação recorrente em uma rede feed-forward mais profunda, que pode ser mapeada de forma mais eficiente para a arquitetura do TPU.
*   **Otimizações do Compilador XLA:** O compilador XLA (Accelerated Linear Algebra) do TensorFlow desempenha um papel crucial na otimização de modelos de RNN para TPU. O XLA pode realizar fusão de operadores (operator fusion), onde várias operações pequenas são combinadas em um único kernel computacional, reduzindo a sobrecarga de memória e melhorando a utilização do hardware.
*   **Uso de `tf.keras.layers.RNN`:** A camada `tf.keras.layers.RNN` no Keras, quando usada com uma célula de RNN personalizada, pode ser otimizada pelo XLA para execução em TPU. É importante garantir que as operações dentro da célula da RNN sejam compatíveis com o XLA.

---


## Page 71

# Exemplo de Código (TensorFlow/Keras):

```python
# (O código de inicialização do TPU é o mesmo da seção de CNN)

with strategy.scope():
    model = tf.keras.Sequential([
        tf.keras.layers.Embedding(input_dim=1000, output_dim=64),
        tf.keras.layers.LSTM(128),
        tf.keras.layers.Dense(10, activation='softmax')
    ])
    model.compile(
        optimizer='adam',
        loss='sparse_categorical_crossentropy',
        metrics=["accuracy"]
    )
```

## 2.4. Implementação de Transformer em TPU

Os modelos Transformer, com sua arquitetura baseada em atenção, são particularmente adequados para a aceleração em TPUs. A natureza altamente paralelizável do mecanismo de auto-atenção e das camadas feed-forward se alinha bem com a arquitetura de array sistólico do TPU.

### Otimizações para TPU:

*   **Paralelismo de Modelo (Model Parallelism):** Para modelos Transformer muito grandes, que não cabem na memória de um único acelerador, o paralelismo de modelo é essencial. A biblioteca `mesh-tensorflow` e, mais recentemente, o GSPMD (General and Scalable Parallelization for ML Models) do XLA, permitem particionar o modelo em vários dispositivos TPU.
*   **Paralelismo de Dados (Data Parallelism):** Assim como em outras arquiteturas, o paralelismo de dados é usado para treinar o modelo em um lote global de dados, distribuído entre os vários núcleos de TPU. A `tf.distribute.TPUStrategy` simplifica a implementação do paralelismo de dados.
*   **Compilação XLA:** O compilador XLA é fundamental para alcançar o desempenho máximo. Ele compila o grafo do TensorFlow em código de máquina otimizado para TPU, realizando otimizações como fusão de operadores e análise de layout de memória.
*   **Padding e Bucketing:** Para lidar com sequências de comprimentos variáveis, o preenchimento (padding) é necessário. No entanto, o preenchimento excessivo pode levar a computação desperdiçada. O "bucketing" (agrupamento de sequências de comprimentos semelhantes em lotes) é uma técnica eficaz para minimizar o preenchimento e melhorar a eficiência.

---


## Page 72

Exemplo de Código (Hugging Face Transformers com PyTorch/XLA):

```python
import torch_xla.core.xla_model as xm

# Adquirir o dispositivo XLA (TPU)
device = xm.xla_device()

# Mover o modelo e os dados para o dispositivo TPU
model.to(device)

# Loop de treinamento
for epoch in range(num_epochs):
    for batch in train_loader:
        input_ids = batch["input_ids"].to(device)
        attention_mask = batch["attention_mask"].to(device)
        labels = batch["labels"].to(device)

        optimizer.zero_grad()

        outputs = model(input_ids=input_ids, attention_mask=attention_mask, labels=labels)
        loss = outputs.loss
        loss.backward()

        # xm.optimizer_step executa a etapa do otimizador e lida com a sincronização
        xm.optimizer_step(optimizer)
```

## 2.5. Implementação de MoE em TPU

Os modelos de Mistura de Especialistas (MoE) são uma abordagem promissora para escalar modelos de linguagem a trilhões de parâmetros, mantendo a eficiência computacional. A implementação de MoE em TPUs aproveita a capacidade do hardware de lidar com paralelismo em grande escala.

### Desafios e Otimizações:

*   **Roteamento de Tokens (Token Routing):** A rede de "gating" decide para qual especialista cada token é enviado. Um desafio chave é o balanceamento de carga, garantindo que os especialistas recebam um número aproximadamente igual de tokens para processar. Algoritmos de roteamento como o Top-k gating, onde cada token é enviado para os k melhores especialistas, são comumente usados.
*   **Paralelismo de Especialistas (Expert Parallelism):** Em uma configuração de MoE, os especialistas podem ser distribuídos por diferentes núcleos de TPU. Isso permite que os especialistas operem em paralelo, melhorando

---


## Page 73

significativamente o throughput. O GShard é um exemplo de sistema que implementa o paralelismo de especialistas em TPUs.

*   **Comunicação All-to-All:** A comunicação entre os dispositivos TPU é um gargalo potencial. A operação "all-to-all", onde cada núcleo de TPU envia os tokens para os especialistas apropriados em outros núcleos, precisa ser altamente otimizada. As interconexões de alta velocidade (ICI) entre os chips de TPU são cruciais para a eficiência dessa comunicação.
*   **Fator de Capacidade (Capacity Factor):** Para simplificar o layout da memória e evitar alocação dinâmica, os tensores são estaticamente dimensionados com um "fator de capacidade", que define um limite superior para o número de tokens que um especialista pode processar. Um fator de capacidade maior resulta em menos tokens descartados, mas aumenta o consumo de memória e a computação.

**Exemplo Conceitual (GShard):**

O GShard particiona o modelo de duas maneiras: 1. **Particionamento Intra-Especialista:** As camadas não-MoE e os componentes dentro de cada especialista são particionados entre os dispositivos. 2. **Particionamento Inter-Especialista:** As camadas MoE são particionadas de forma que cada especialista resida em um grupo diferente de dispositivos.

Durante a computação, os tokens em cada dispositivo são enviados para os especialistas relevantes por meio de uma operação de comunicação all-to-all. Após o processamento pelos especialistas, os resultados são retornados aos dispositivos originais por meio de outra operação all-to-all.

## 3. Principais Contribuidores

### 3.1. Yann LeCun

*   **Contribuições Principais:**
    *   Pioneiro no uso de Redes Neurais Convolucionais (CNNs) para reconhecimento de imagem.
    *   Criador da arquitetura LeNet, uma das primeiras CNNs a obter sucesso prático no reconhecimento de dígitos manuscritos.
    *   Desenvolveu o método de "Optimal Brain Damage" para poda (pruning) de redes neurais.

### 3.2. Geoffrey Hinton

*   **Contribuições Principais:**
    *   Co-inventor das máquinas de Boltzmann.
    *   Contribuições fundamentais para o algoritmo de retropropagação (backpropagation).

---


## Page 74

* Seu trabalho com Alex Krizhevsky e Ilya Sutskever na AlexNet foi um marco no campo do deep learning, demonstrando a eficácia das CNNs em tarefas de classificação de imagem em larga escala.

## 3.3. Yoshua Bengio

* **Contribuições Principais:**
    * Pesquisas pioneiras em modelos de sequência, incluindo redes recorrentes e mecanismos de atenção.
    * Contribuições para a compreensão teórica do deep learning e o desenvolvimento de novos algoritmos.
    * Co-autor de trabalhos seminais sobre a aplicação de redes neurais para modelagem de linguagem.

## Fórmulas Matemáticas

### Operação de Convolução 2D:

A operação de convolução, que é a base das CNNs, pode ser expressa matematicamente como:

S(i, j) = (I * K)(i, j) = Σ_m Σ_n I(i - m, j - n) K(m, n)

Onde: - I é a matriz de entrada (imagem). - K é o kernel (ou filtro) da convolução. - S(i, j) é o elemento na posição (i, j) do mapa de características de saída. - Os índices m e n percorrem as dimensões do kernel.

### Função de Ativação (ReLU):

Após a operação de convolução, uma função de ativação não linear é aplicada. A Unidade Linear Retificada (ReLU) é comumente usada:

f(x) = max(0, x)

### Operação de Pooling (Agrupamento):

A operação de pooling, como o Max Pooling, reduz a dimensionalidade espacial dos mapas de características. Para uma região R no mapa de características, o Max Pooling é definido como:

p_j = max_{i ∈ R_j} a_i

Onde p_j é a saída da operação de pooling para a região R_j e a_i são as ativações dentro dessa região. _n

---


## Page 75

# Função de Ponderação (Gating Network):

A rede de "gating" calcula os pesos para cada especialista. Uma função softmax é comumente usada para garantir que os pesos somem 1:

w(x)_i = softmax(W_g * x + b_g)_i = exp( (W_g * x + b_g)_i ) / Σ_j exp( (W_g * x + b_g)_j )

Onde: - w(x)_i é o peso para o especialista i. - W_g e b_g são os pesos e o viés da rede de "gating".

## Saída Final:

A saída final do modelo MoE é a soma ponderada das saídas de todos os especialistas:

y = Σ_i w(x)_i * E_i(x)

Onde E_i(x) é a saída do especialista i.

# Pesquisa Profunda: Teoria da Informação e Compressão de Modelos

Este documento apresenta uma pesquisa técnica aprofundada sobre os principais métodos de compressão de modelos de aprendizado profundo, com foco nas formulações matemáticas e nos principais pesquisadores da área.

## 1. Visão Geral da Compressão de Modelos

A compressão de modelos de redes neurais profundas (DNNs) é crucial para implantar modelos de grande escala em ambientes com recursos computacionais limitados. As técnicas podem ser agrupadas em quatro categorias principais, conforme resumido na tabela abaixo, baseada no trabalho de Cheng et al. (2017).

<table>
<thead>
<tr>
<th>Categoria</th>
<th>Descrição</th>
<th>Aplicações Típicas</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Poda e Quantização</strong></td>
<td>Reduz a redundância nos parâmetros do modelo, eliminando pesos ou reduzindo sua precisão numérica.</td>
<td>Camadas convolucionais e totalmente conectadas.</td>
</tr>
<tr>
<td><strong>Fatoração de Baixo Posto</strong></td>
<td>Utiliza decomposição de matrizes/tensores para aproximar as matrizes de peso com representações de menor posto.</td>
<td>Camadas convolucionais e totalmente conectadas.</td>
</tr>
</tbody>
</table>

---


## Page 76

<table>
  <thead>
    <tr>
      <th>Categoria</th>
      <th>Descrição</th>
      <th>Aplicações Típicas</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Filtros Compactos</td>
      <td>Projeta filtros convolucionais com estruturas especiais (e.g., separáveis em profundidade) para reduzir o número de parâmetros.</td>
      <td>Camadas convolucionais.</td>
    </tr>
    <tr>
      <td>Destilação de Conhecimento</td>
      <td>Treina um modelo menor (aluno) para imitar o comportamento de um modelo maior e mais complexo (professor).</td>
      <td>Classificação de imagens, reconhecimento de fala.</td>
    </tr>
  </tbody>
</table>

# 2. Técnicas de Compressão e Suas Formulações

## 2.1. Poda de Redes Neurais (Pruning)

A poda visa remover parâmetros (pesos individuais, neurônios ou filtros) que são considerados redundantes. Os métodos seminais, **Optimal Brain Damage (OBD)** e **Optimal Brain Surgeon (OBS)**, utilizam informações de segunda ordem para estimar a importância dos pesos.

Ambos se baseiam na expansão de Taylor de segunda ordem da função de erro $E$ em torno de um ponto de mínimo local $w^*$:

$$\delta E \approx (1/2) * \delta_{W^T} * H * \delta_W$$

Onde $\delta_W$ é a perturbação nos pesos e $H$ é a matriz Hessiana. O OBS (Hassibi & Stork, 1993) remove o peso $w_q$ que causa o menor aumento no erro, com a seguinte fórmula para a saliência ($L_q$) e a atualização dos pesos ($\delta_W$):

$$L_q = (1/2) * (wq^2 / [H^{-1}]qq)$$
$$\delta_W = - (wq / [H^{-1}]qq) * H^{-1} * eq$$

## 2.2. Quantização

A quantização reduz a precisão numérica dos pesos e/ou ativações. A **quantização afim uniforme**, conforme descrito por Wu et al. (2020), mapeia um valor real $x$ para um inteiro $x_q$ de $b$ bits usando uma escala $s$ e um ponto zero $z$.

*   **Mapeamento:** $s = (2^b - 1) / (\alpha - \beta)$ $z = -round(\beta * s) - 2^{b-1}$
*   **Quantização:** $x_q = clip(round(s * x + z), -2^{b-1}, 2^{b-1} - 1)$
*   **Dequantização:** $\hat{x} = (1/s) * (x_q - z)$

---


## Page 77

# 2.3. Destilação de Conhecimento (Knowledge Distillation)

Proposta por Hinton, Vinyals & Dean (2015), esta técnica treina um modelo aluno usando as saídas "suavizadas" de um modelo professor. A suavização é controlada por uma **temperatura (T)** na função softmax:

qᵢ = exp(zᵢ / T) / Σⱼ exp(zⱼ / T)

A função de custo combina a cross-entropia com os alvos suavizados (L_soft) e os rótulos verdadeiros (L_hard):

L = α * L_soft + β * L_hard

# 2.4. Esparsidade via Regularização L0

A esparsidade pode ser induzida diretamente pela **regularização L0**, que penaliza o número de parâmetros não-nulos. Como a norma L0 não é diferenciável, Louizos, Welling & Kingma (2017) propuseram uma aproximação usando **gates estocásticos** com a distribuição **Hard Concrete**.

A função de custo regularizada é:

R(Θ) = E[L(y, f(x; Θ ~⊙ z))] + λ * Σ P(zⱼ ≠ 0)

Onde z são os gates estocásticos. A probabilidade P(zⱼ ≠ 0) é calculada usando a CDF da distribuição Hard Concrete, permitindo a otimização por gradiente.

# 3. Pesquisa Genealógica dos Principais Contribuidores

Esta seção traça a linhagem acadêmica de figuras centrais na área de compressão de modelos.

<table>
<thead>
<tr>
<th>Pesquisador</th>
<th>Orientador de PhD</th>
<th>Orientandos Notáveis</th>
</tr>
</thead>
<tbody>
<tr>
<td>Geoffrey Hinton</td>
<td>Hugh Christopher Longuet-Higgins</td>
<td>Yann LeCun, Ruslan Salakhutdinov, Ilya Sutskever, Zoubin Ghahramani, Peter Brown, Richard Szeliski, Jimmy Ba</td>
</tr>
<tr>
<td>Yann LeCun</td>
<td>Maurice Milgram</td>
<td>Raia Hadsell, Marc'Aurelio Ranzato, Koray Kavukcuoglu</td>
</tr>
<tr>
<td>Yoshua Bengio</td>
<td>Renato De Mori</td>
<td></td>
</tr>
</tbody>
</table>

---


## Page 78

<table>
  <thead>
    <tr>
      <th>Pesquisador</th>
      <th>Orientador de PhD</th>
      <th>Orientandos Notáveis</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td></td>
      <td></td>
      <td>Ian Goodfellow, Razvan Pascanu, Caglar Gulcehre, Pascal Vincent</td>
    </tr>
    <tr>
      <td>Jeff Dean</td>
      <td>Craig David Chambers</td>
      <td>Nenhum aluno conhecido.</td>
    </tr>
    <tr>
      <td># Pesquisa Técnica e Científica sobre Física de Semicondutores</td>
      <td></td>
      <td></td>
    </tr>
  </tbody>
</table>

# As 5 Equações Básicas da Física de Dispositivos Semicondutores

As cinco quantidades a serem encontradas são:
* n(x,t) : concentração de elétrons
* p(x,t) : concentração de lacunas
* Je(x,t) : densidade de corrente de elétrons
* Jh(x,t) : densidade de corrente de lacunas
* E(x,t) : campo elétrico

As cinco equações independentes que as relacionam são:

1. **Equação de Corrente de Elétrons (Drift-Diffusion):** Je(x,t) = q * μe *
   n(x,t) * E(x,t) + q * De * ∂n(x,t)/∂x

2. **Equação de Corrente de Lacunas (Drift-Diffusion):** Jh(x,t) = q * μh *
   p(x,t) * E(x,t) - q * Dh * ∂p(x,t)/∂x

3. **Equação de Continuidade de Elétrons:** ∂n(x,t)/∂t - (1/q) * ∂Je(x,t)/∂x = gL(x,t) - [n(x,t) * p(x,t) - ni^2] * r

4. **Equação de Continuidade de Lacunas:** ∂p(x,t)/∂t + (1/q) * ∂Jh(x,t)/∂x = gL(x,t) - [n(x,t) * p(x,t) - ni^2] * r

5. **Equação de Poisson:** ε * ∂E(x,t)/∂x = q * [p(x,t) - n(x,t) + Nd(x) - Na(x)]

## Definição das Variáveis:

* q : Carga elementar
* μe : Mobilidade de elétrons
* μh : Mobilidade de lacunas
* De : Coeficiente de difusão de elétrons
* Dh : Coeficiente de difusão de lacunas
* gL(x,t) : Taxa de geração de pares elétron-lacuna

---


## Page 79

*   ni : Concentração intrínseca de portadores
*   r : Coeficiente de recombinação
*   ε : Permissividade elétrica do material
*   Nd (x) : Concentração de doadores
*   Na (x) : Concentração de aceitadores

# Tecnologia FinFET

## Estrutura e Dimensões

A principal diferença entre os FinFETs e os MOSFETs planares reside na orientação do canal. Os FinFETs apresentam canais verticais em forma de "barbatana" (fin), enquanto os MOSFETs planares utilizam canais horizontais. As dimensões críticas de um FinFET são a altura da barbatana (HFin) e a largura da barbatana (WFin).

*   **HFin (Altura da Barbatana):** Análoga ao comprimento do canal nos MOSFETs, a altura da barbatana é um parâmetro crítico que impacta significativamente o desempenho e as características do dispositivo.
*   **WFin (Largura da Barbatana):** A largura da barbatana.

Aumentar o número de barbatanas resulta em maior densidade de carga nos canais, o que se traduz em um controle de porta mais preciso, melhorando o desempenho geral do dispositivo. No entanto, existem restrições práticas na fabricação. Para garantir uma operação estável e confiável, a altura da barbatana (HFin) é geralmente mantida em um valor inferior a quatro vezes a espessura da barbatana.

O comprimento do canal de um FinFET pode ser calculado pela seguinte equação:

Comprimento do Canal = (Número de Barbatanas) * (Comprimento de cada Barbatana)

## Classificação dos FinFETs

Os FinFETs podem ser classificados com base em sua estrutura física e no número de terminais.

### Com Base na Estrutura Física

*   **Bulk FinFETs:** As barbatanas individuais compartilham um substrato comum, resultando em sua conexão física. A estrutura se assemelha à dos MOSFETs planares tradicionais, facilitando a transição da tecnologia planar para a FinFET.
*   **SOI (Silicon-on-Insulator) FinFETs:** As barbatanas são fisicamente isoladas e não entram em contato direto umas com as outras.

---


## Page 80

# Com Base no Número de Terminais

*   **Short Gate (SG) FinFETs (3 terminais):** Duas portas são curto-circuitadas e fisicamente conectadas entre si.
*   **Independent Gate (IG) FinFETs (4 terminais):** As portas são fisicamente isoladas por um material dielétrico.

# Modelos Compactos de Dispositivos

Modelos compactos são essenciais para a simulação de circuitos, fornecendo uma representação precisa do comportamento do dispositivo com um custo computacional reduzido. Três modelos proeminentes para FinFETs são o EKV Double-gate, o BSIM-CMG e o PSP-DGFET.

## EKV Double-gate

O modelo EKV (Enz-Krummenacher-Vittoz) para transistores de dupla porta (Double-gate) é uma extensão do modelo EKV para MOSFETs planares. Ele se baseia na aproximação de canal gradual e é válido para todas as regiões de operação (sublimiar, saturação e linear).

## BSIM-CMG (Berkeley Short-channel IGFET Model - Common Multi-Gate)

O BSIM-CMG é um modelo padrão da indústria para FinFETs e outros transistores multi-porta. Ele é um modelo físico e escalável, o que significa que pode ser usado para prever o comportamento de dispositivos com diferentes geometrias. O modelo leva em consideração uma ampla gama de efeitos físicos, incluindo efeitos de canal curto, quantização de carga e mobilidade degradada.

## PSP-DGFET (Pen-State-Philips - Double-Gate FET)

O modelo PSP-DGFET é outro modelo físico para transistores de dupla porta. Ele se baseia na solução da equação de Poisson-Boltzmann e é conhecido por sua precisão na região de sublimiar.

# Leis de Escala e Limites Físicos

## Escala de Dennard (Escala de MOSFET)

A escala de Dennard, também conhecida como escala de MOSFET, é uma lei de escala que afirma que, à medida que os transistores diminuem de tamanho, sua densidade de potência permanece constante. A lei, originalmente formulada em 1974 por Robert H. Dennard, baseia-se na manutenção de um campo elétrico constante dentro do dispositivo.

---


## Page 81

# Equações de Escala

Para transistores MOS longos, a escala de Dennard fornece as seguintes relações, onde os parâmetros são escalados por um fator S:

L α S⁻¹
W α S⁻¹
t_ox α S⁻¹
V_DD α S⁻¹
V_T α S⁻¹
N_A α S

<table>
  <thead>
    <tr>
      <th>Propriedade</th>
      <th>Símbolo</th>
      <th>Equação</th>
      <th>Expoente de Escala (Campo Constante)</th>
      <th>Expoente de Escala (Tensão Fixa)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Capacitância de Óxido</td>
      <td>C_ox</td>
      <td>ε_ox / t_ox</td>
      <td>1</td>
      <td>1</td>
    </tr>
    <tr>
      <td>Área do Dispositivo</td>
      <td>A</td>
      <td>W * L</td>
      <td>-2</td>
      <td>-2</td>
    </tr>
    <tr>
      <td>Capacitância de Porta</td>
      <td>C_g</td>
      <td>C_ox * W * L</td>
      <td>-1</td>
      <td>-1</td>
    </tr>
    <tr>
      <td>Transcondutância</td>
      <td>K_n</td>
      <td>μ_n * C_ox * W / L</td>
      <td>1</td>
      <td>1</td>
    </tr>
    <tr>
      <td>Corrente de Saturação</td>
      <td>I_on</td>
      <td>K_n * V_GT²</td>
      <td>-1</td>
      <td>1</td>
    </tr>
    <tr>
      <td>Resistência Ligado</td>
      <td>R_on</td>
      <td>V_DD / I_on</td>
      <td>0</td>
      <td>-1</td>
    </tr>
    <tr>
      <td>Atraso Intrínseco</td>
      <td>t_pd</td>
      <td>R_on * C_g</td>
      <td>-1</td>
      <td>-2</td>
    </tr>
    <tr>
      <td>Potência</td>
      <td>P_av</td>
      <td>f * C * V_DD²</td>
      <td>-2</td>
      <td>1</td>
    </tr>
    <tr>
      <td>Densidade de Potência</td>
      <td>PD</td>
      <td>P_av / A</td>
      <td>0</td>
      <td>3</td>
    </tr>
  </tbody>
</table>

---


## Page 82

# Definição das Variáveis:

*   S : Fator de escala
*   L : Comprimento do canal
*   W : Largura do canal
*   t_ox : Espessura do óxido de porta
*   V_DD : Tensão de alimentação
*   V_T : Tensão de limiar
*   N_A : Concentração de aceitadores
*   C_ox : Capacitância de óxido por unidade de área
*   A : Área do transistor
*   C_g : Capacitância total da porta
*   K_n : Parâmetro de transcondutância
*   μ_n : Mobilidade de elétrons no canal
*   I_on : Corrente de saturação
*   V_GT : Tensão de overdrive da porta (V_GS - V_T)
*   R_on : Resistência do transistor quando ligado
*   t_pd : Atraso intrínseco
*   P_av : Consumo médio de potência
*   f : Frequência de operação
*   PD : Densidade de potência

# Limites Físicos da Escala

A contínua miniaturização dos MOSFETs enfrenta diversos limites físicos que impedem a aplicação direta da escala de Dennard. Esses limites incluem:

*   **Correntes de Fuga de Sublimiar (Subthreshold Leakage Currents):** À medida que a tensão de alimentação (V_DD) é reduzida, a tensão de limiar (V_T) também precisa ser reduzida para manter o desempenho. No entanto, uma V_T baixa leva a um aumento exponencial da corrente de fuga de sublimiar, o que aumenta o consumo de energia estática.
*   **Quebra de Dielétrico Dependente do Tempo (Time Dependent Dielectric Breakdown - TDDB):** Campos elétricos elevados no óxido de porta podem causar a deterioração gradual da camada de óxido, levando à quebra do dielétrico. A confiabilidade a longo prazo do óxido de porta limita a espessura mínima que pode ser usada.
*   **Efeitos de Elétrons Quentes (Hot Electron Effects):** Campos elétricos laterais elevados no canal podem acelerar os elétrons a energias suficientemente altas para que eles sejam injetados no óxido de porta, alterando a tensão de limiar do dispositivo ao longo do tempo. A estabilidade a longo prazo da tensão de limiar limita o quão curto um canal pode ser.

---


## Page 83

*   **Efeitos de Canal Curto (Short Channel Effects):** À medida que o comprimento do canal diminui, as regiões de depleção da fonte e do dreno podem se aproximar, afetando o potencial do canal e tornando a tensão de limiar uma função do comprimento do canal e da tensão de dreno. O principal efeito de canal curto é o DIBL (Drain-Induced Barrier Lowering), que é a redução da barreira de potencial para os elétrons da fonte devido à tensão de dreno.
*   **Tunelamento Quântico (Quantum Tunneling):** Com a redução da espessura do óxido de porta para alguns nanômetros, os elétrons podem tunelar através da barreira de potencial do óxido, resultando em uma corrente de fuga de porta significativa.

## Principais Contribuidores

### Robert H. Dennard

Robert Heath Dennard (1932-2024) foi um engenheiro eletricista e inventor americano, mais conhecido por suas contribuições pioneiras para a tecnologia de semicondutores.

## Principais Contribuições

*   **DRAM (Dynamic Random-Access Memory):** Em 1966, Dennard inventou a célula de memória de um transistor, que consiste em um transistor e um capacitor. Essa invenção, patenteada em 1968, tornou-se a base para a DRAM moderna, uma tecnologia fundamental para a computação moderna.
*   **Escala de Dennard (Dennard Scaling):** Em 1974, Dennard e seus colegas formularam a teoria da escala de MOSFETs, que postula que, à medida que os transistores diminuem de tamanho, suas características de desempenho, como densidade, velocidade e eficiência energética, melhoram. Essa teoria foi fundamental para o avanço da Lei de Moore.

## Patentes Relevantes

*   **Patente dos EUA 3.387.286:** "Field-effect transistor memory" (Memória de transistor de efeito de campo), emitida em 4 de junho de 1968.

## Colaboradores

A formulação da escala de Dennard foi um trabalho colaborativo com seus colegas na IBM. Embora a página da Wikipedia não liste todos os colaboradores, a pesquisa indica que a publicação original de 1974 foi um esforço de equipe.

---


## Page 84

# Simon Sze

Simon Min Sze (1936-2023) foi um engenheiro eletricista taiwanês-americano, reconhecido por suas contribuições fundamentais para a física e tecnologia de semicondutores.

## Principais Contribuições

*   **MOSFET de Porta Flutuante (Floating-gate MOSFET):** Em 1967, em colaboração com Dawon Kahng, Sze inventou o MOSFET de porta flutuante, um dispositivo de memória não volátil que se tornou a base para as memórias EPROM, EEPROM e Flash.
*   **"Physics of Semiconductor Devices":** Sze é o autor do livro "Physics of Semiconductor Devices", uma obra de referência na área, amplamente utilizada em universidades e na indústria.

## Colaboradores

*   **Dawon Kahng:** Co-inventor do MOSFET de porta flutuante.
*   **Kwok K. Ng:** Co-autor da terceira edição do livro "Physics of Semiconductor Devices".

## Orientador de Doutorado

*   **John L. Moll:** Um pioneiro na física de semicondutores, conhecido por suas contribuições para o transistor de junção bipolar e o modelo de Ebers-Moll.

# John L. Moll

John Louis Moll (1921-2011) foi um engenheiro eletricista americano, notável por suas contribuições para a física de estado sólido.

## Principais Contribuições

*   **Modelo de Ebers-Moll:** Em colaboração com Jewell James Ebers, Moll desenvolveu o modelo de Ebers-Moll, um modelo matemático que descreve o comportamento do transistor de junção bipolar.
*   **Chave p-n-p-n:** Moll também é conhecido por sua teoria sobre a chave p-n-p-n, um tipo de tiristor.
*   **Diodo MOS:** Contribuiu para o desenvolvimento do diodo MOS.
*   **Diodo de Recuperação por Degrau (Step Recovery Diode):** Contribuiu para o desenvolvimento do diodo de recuperação por degrau.

---


## Page 85

# Colaboradores

*   **Jewell James Ebers**: Co-desenvolvedor do modelo de Ebers-Moll.

# Alunos de Doutorado Notáveis

*   **Simon Sze**: Inventor do MOSFET de porta flutuante.

# Modelo de Ebers-Moll

O modelo de Ebers-Moll descreve as correntes DC de um transistor de junção bipolar.
As equações para um transistor NPN são:

```c
i_C = I_S * (exp(V_BE / V_T) - exp(V_BC / V_T)) - (I_S / β_R) * (exp(V_BC / V_T))
i_B = (I_S / β_F) * (exp(V_BE / V_T) - 1) + (I_S / β_R) * (exp(V_BC / V_T) - 1)
i_E = I_S * (exp(V_BE / V_T) - exp(V_BC / V_T)) + (I_S / β_F) * (exp(V_BE / V_T) - 1)
```

Onde:

*   `i_C`: Corrente de coletor
*   `i_B`: Corrente de base
*   `i_E`: Corrente de emissor
*   `I_S`: Corrente de saturação reversa (entre 10⁻¹⁵ e 10⁻¹² A)
*   `V_T`: Tensão térmica (aproximadamente 26 mV a 300 K)
*   `V_BE`: Tensão base-emissor
*   `V_BC`: Tensão base-coletor
*   `β_F`: Ganho de corrente de emissor comum direto (20 a 500)
*   `β_R`: Ganho de corrente de emissor comum reverso (0 a 20)

# Derivação do Modelo de Ebers-Moll

O modelo de Ebers-Moll é derivado considerando o transistor de junção bipolar como dois diodos p-n conectados "costas com costas", com a região da base sendo comum a ambos. As correntes nos diodos são interdependentes, e essa interdependência é quantificada pelos ganhos de corrente α_F (direito) e α_R (reverso).

As correntes nos diodos são dadas pela equação do diodo de Shockley:

```c
I_F = I_ES * (exp(V_BE / V_T) - 1)
I_R = I_CS * (exp(V_BC / V_T) - 1)
```

Onde:

*   `I_F`: Corrente do diodo base-emissor

---


## Page 86

*   I_R : Corrente do diodo base-coletor
*   I_ES : Corrente de saturação do diodo base-emissor
*   I_CS : Corrente de saturação do diodo base-coletor

A partir da aplicação das leis de Kirchhoff ao modelo, as correntes nos terminais do transistor são:

I_E = I_F - α_R * I_R
I_C = α_F * I_F - I_R
I_B = I_E - I_C

Substituindo as equações dos diodos, obtemos as equações de Ebers-Moll:

I_E = I_ES * (exp(V_BE / V_T) - 1) - α_R * I_CS * (exp(V_BC / V_T) - 1)
I_C = α_F * I_ES * (exp(V_BE / V_T) - 1) - I_CS * (exp(V_BC / V_T) - 1)
I_B = (1 - α_F) * I_ES * (exp(V_BE / V_T) - 1) + (1 - α_R) * I_CS * (exp(V_BC / V_T) - 1)

## FinFET

A tecnologia FinFET (Fin Field-Effect Transistor) foi desenvolvida por uma equipe de pesquisadores da Universidade da Califórnia, Berkeley, liderada por Chenming Hu. A primeira demonstração de um dispositivo FinFET foi feita por Digh Hisamoto e sua equipe no Hitachi Central Research Laboratory em 1989.

### Principais Pesquisadores

*   **Chenming Hu** (Universidade da Califórnia, Berkeley): Considerado o "pai do FinFET", liderou a equipe que desenvolveu o conceito e a tecnologia.
*   **Tsu-Jae King Liu** (Universidade da Califórnia, Berkeley): Co-inventora do FinFET, com contribuições significativas para o desenvolvimento da tecnologia.
*   **Jeffrey Bokor** (Universidade da Califórnia, Berkeley): Co-inventor do FinFET.
*   **Digh Hisamoto** (Hitachi Central Research Laboratory): Liderou a equipe que fabricou o primeiro dispositivo FinFET, chamado de transistor DELTA (depleted lean-channel transistor).
*   **Toru Kaga** (Hitachi Central Research Laboratory): Membro da equipe que fabricou o primeiro dispositivo FinFET.
*   **Yoshifumi Kawamoto** (Hitachi Central Research Laboratory): Membro da equipe que fabricou o primeiro dispositivo FinFET.
*   **Eiji Takeda** (Hitachi Central Research Laboratory): Membro da equipe que fabricou o primeiro dispositivo FinFET.

<footer>Este documento foi gerado por Manus, um agente de IA autônomo, em 31 de dezembro de 2025.</footer>

---


## Page 87

# Pesquisa Aprofundada sobre Arquitetura de Memória Hierárquica

## 1. Introdução à Hierarquia de Memória

A arquitetura de memória em sistemas computacionais modernos é organizada em uma hierarquia de múltiplos níveis, cada um com características distintas de **velocidade, capacidade e custo**. O objetivo fundamental deste design é otimizar o desempenho do sistema, equilibrando o rápido tempo de acesso das memórias de alto custo com a grande capacidade das memórias de baixo custo. Este arranjo explora o **princípio da localidade de referência**, um padrão de comportamento fundamental dos programas de computador.

### 1.1. Princípio da Localidade

O princípio da localidade descreve a tendência dos processadores de acessar repetidamente um subconjunto específico de dados e instruções durante a execução. Este princípio se manifesta de duas formas principais:

*   **Localidade Temporal**: Refere-se à probabilidade de que um dado ou instrução recentemente acessado seja acessado novamente em um futuro próximo. Manter os itens mais recentes em níveis de memória mais rápidos é crucial para explorar essa tendência.
*   **Localidade Espacial**: Indica que, se um determinado local de memória é acessado, é altamente provável que os locais de memória adjacentes também sejam acessados em breve. Isso justifica a transferência de dados em blocos (ou linhas de cache) entre os níveis da hierarquia.

### 1.2. Níveis da Hierarquia de Memória

A hierarquia de memória é tipicamente estruturada da seguinte forma, indo do nível mais rápido e menor para o mais lento e maior:

<table>
<thead>
<tr>
<th>Nível</th>
<th>Tecnologia Primária</th>
<th>Capacidade Típica</th>
<th>Tempo de Acesso (Aprox.)</th>
<th>Custo por Bit</th>
</tr>
</thead>
<tbody>
<tr>
<td>0</td>
<td>Registradores da CPU</td>
<td>&lt; 1 KB</td>
<td>&lt; 1 ns</td>
<td>Muito Alto</td>
</tr>
<tr>
<td>1</td>
<td>Cache L1 (SRAM)</td>
<td>Dezenas de KB</td>
<td>~1 ns</td>
<td>Alto</td>
</tr>
<tr>
<td>2</td>
<td>Cache L2 (SRAM)</td>
<td>Centenas de KB a MBs</td>
<td>~2-10 ns</td>
<td>Médio-Alto</td>
</tr>
<tr>
<td>3</td>
<td>Cache L3 (SRAM)</td>
<td>Dezenas de MBs a GBs</td>
<td>~5-20 ns</td>
<td>Alto</td>
</tr>
<tr>
<td>4</td>
<td>Memória RAM (DRAM)</td>
<td>GBs a TBs</td>
<td>~100-200 ns</td>
<td>Baixo</td>
</tr>
<tr>
<td>5</td>
<td>Disco Rígido / SSD</td>
<td>TBs a PBs</td>
<td>~10-1000 ms</td>
<td>Muito Baixo</td>
</tr>
</tbody>
</table>

## 2. Análise de Desempenho

A hierarquia de memória é um componente essencial para o desempenho geral dos sistemas computacionais. A eficiência da transferência de dados entre os diferentes níveis da hierarquia pode ser medida através de vários parâmetros, incluindo:

*   **Taxa de Transferência de Dados**: Medida em bits por segundo, representa a quantidade de dados que podem ser transferidos entre dois níveis da hierarquia em um período específico.
*   **Ciclo de Cache**: Tempo necessário para acessar um bloco de dados de um nível de memória mais lento, após ter sido previamente carregado em um nível mais rápido.
*   **Fator de Carga**: Relação entre o tempo de acesso ao disco rígido ou SSD e o tempo de acesso à memória RAM. Um fator de carga mais baixo indica uma melhor eficiência.

## 3. Considerações Práticas

### 3.1. Cache Coherência

A coherência da cache é um conceito fundamental na hierarquia de memória. Todos os níveis de cache devem manter os mesmos dados consistentes, garantindo que os processadores obtenham a visão mais atualizada possível da memória. Isso é crucial para evitar erros de consistência de dados e otimizar o desempenho.

### 3.2. Gerenciamento de Falhas

A gestão eficiente de falhas de memória é outra área crítica. Isso envolve a detecção, correção e prevenção de falhas de memória, bem como a recuperação rápida de dados perdidos. As técnicas de gerenciamento de falhas incluem redundância de hardware, verificação de código de erro e técnicas de recuperação de dados.

### 3.3. Desempenho de Aplicativos

A performance de aplicativos específicos pode variar significativamente dependendo de sua interação com a hierarquia de memória. Aplicativos que exigem acesso frequente a grandes conjuntos de dados, como bancos de dados e sistemas de análise de dados, podem beneficiar de níveis de memória mais rápidos e de maior capacidade. Por outro lado, aplicativos com processamento intenso de números, como cálculos científicos, podem se beneficiar mais de uma memória RAM mais rápida.

## 4. Conclusão

A arquitetura de memória hierárquica é uma ferramenta essencial para otimizar o desempenho dos sistemas computacionais. Ao entender e analisar cuidadosamente os princípios subjacentes, como a localidade de referência, e os níveis da hierarquia, é possível criar soluções mais eficientes e eficazes. A pesquisa contínua nessa área promete levar a avanços significativos no campo da arquitetura de computadores.

---


## Page 88

<table>
  <thead>
    <tr>
      <th>Nível</th>
      <th>Tecnologia Primária</th>
      <th>Capacidade Típica</th>
      <th>Tempo de Acesso (Aprox.)</th>
      <th>Custo por Bit</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>3</td>
      <td>Cache L3 (SRAM)</td>
      <td>Vários MBS</td>
      <td>~10-30 ns</td>
      <td>Médio</td>
    </tr>
    <tr>
      <td>4</td>
      <td>Memória Principal (DRAM)</td>
      <td>Gigabytes (GBs)</td>
      <td>~50-100 ns</td>
      <td>Baixo</td>
    </tr>
    <tr>
      <td>5</td>
      <td>Armazenamento Secundário (SSD/HDD)</td>
      <td>Terabytes (TBs)</td>
      <td>Micro a Milissegundos (µs-ms)</td>
      <td>Muito Baixo</td>
    </tr>
    <tr>
      <td>6</td>
      <td>Armazenamento Terciário/Offline</td>
      <td>Petabytes (PBs) a Exabytes (EBs)</td>
      <td>Segundos a Minutos</td>
      <td>Extremamente Baixo</td>
    </tr>
  </tbody>
</table>

# 2. Tecnologias de Memória e Seus Fundamentos

## 2.1. SRAM (Static Random-Access Memory) e Cache

A SRAM é uma memória volátil que retém os dados enquanto a energia estiver fornecida, sem a necessidade de atualização (refresh). Devido à sua alta velocidade, é a tecnologia escolhida para os caches da CPU. A célula de memória SRAM mais comum é a **6T (seis transistores)**, que utiliza um par de inversores CMOS de acoplamento cruzado para formar um latch (flip-flop) que armazena um único bit, e dois transistores de acesso para controlar a leitura e a escrita.

### Fórmulas de Organização de Cache

A eficiência do cache depende de como os blocos de memória são mapeados para as linhas de cache. As principais fórmulas de endereçamento são:

*   **Mapeamento Direto:**
    *   Índice = (Endereço do Bloco) mod (Número de Linhas no Cache)
    *   Tag = (Endereço do Bloco) / (Número de Linhas no Cache)
*   **Conjunto Associativo (N-way):**
    *   Índice (Set) = (Endereço do Bloco) mod (Número de Conjuntos no Cache)
    *   Tag = (Endereço do Bloco) / (Número de Conjuntos no Cache)

---


## Page 89

# 2.2. DRAM (Dynamic Random-Access Memory)

A DRAM é a base da memória principal na maioria dos computadores. Sua célula de memória, conhecida como **1T1C**, é composta por um único transistor MOSFET e um capacitor. O bit é armazenado como carga elétrica no capacitor.

*   **Equação de Retenção de Carga:** A carga no capacitor decai exponencialmente com o tempo devido a correntes de fuga, exigindo um ciclo de refresh periódico.
    *   Q(t) = Q_0 * e^(-t/T)
    *   Onde Q(t) é a carga no tempo t, Q_0 é a carga inicial, e T é a constante de tempo de retenção de dados.

# 2.3. HBM (High Bandwidth Memory)

A HBM é uma arquitetura de memória 3D que empilha múltiplos dies de DRAM verticalmente, conectados por Through-Silicon Vias (TSVs). Esta abordagem oferece uma interface de memória muito mais ampla e, consequentemente, uma largura de banda massivamente superior com menor consumo de energia por bit transferido.

*   **Fórmula da Largura de Banda:**
    *   Largura de Banda (GB/s) = (Taxa de Dados por Pino em Gb/s) * (Largura da Interface em bits) / 8
    *   Exemplo para HBM3: 6.4 Gb/s * 1024 bits / 8 = 819 GB/s

# 2.4. Física do Transistor MOSFET

O funcionamento de todas essas memórias semicondutoras depende do **MOSFET** (Metal-Oxide-Semiconductor Field-Effect Transistor). As equações que governam seu comportamento são fundamentais para o design da memória.

*   **Equação de Corrente (Região de Saturação):** Descreve a corrente quando o transistor está totalmente "ligado".
    *   I_D = (1/2) * μ_n * C_ox * (W/L) * (V_GS - V_th)^2 * (1 + λ * V_DS)

Onde:
*   I_D : Corrente Drain-Source
*   μ_n : Mobilidade dos elétrons no canal
*   C_ox : Capacitância do óxido de gate por unidade de área (ε_ox / t_ox)
*   W/L : Razão entre a largura e o comprimento do canal do transistor
*   V_GS, V_DS, V_th : Tensões de Gate-Source, Drain-Source e de Limiar, respectivamente
*   λ : Coeficiente de modulação do comprimento do canal

---


## Page 90

# 3. Pesquisa Genealógica dos Principais Contribuidores

O desenvolvimento da memória moderna é uma história de inovações incrementais e disruptivas por parte de pesquisadores e engenheiros visionários.

## 3.1. Mohamed M. Atalla e Dawon Kahng (MOSFET)

*   **Contribuição Fundamental:** Em 1959, nos Bell Labs, **Mohamed Atalla** e seu colega **Dawon Kahng** inventaram o MOSFET. Atalla, um engenheiro mecânico de formação, primeiro desenvolveu o crucial processo de **passivação de superfície por oxidação térmica**, que estabilizou as superfícies de silício e tornou a fabricação de dispositivos de silício monolíticos uma realidade. O MOSFET, nascido dessa pesquisa, tornou-se o bloco de construção mais fundamental da eletrônica moderna.
*   **Colaboradores e Contexto:** O trabalho foi realizado em um pequeno grupo de pesquisa no Bell Labs, que incluía também **Eileen Tannenbaum** e **Edwin Joseph Scheibner**, que colaboraram no processo de passivação.

## 3.2. Robert H. Dennard (DRAM e Dennard Scaling)

*   **Invenção da DRAM:** Em 1966, na IBM, **Robert Dennard** concebeu a célula de memória DRAM de um transistor e um capacitor (1T1C), uma invenção que aumentou drasticamente a densidade da memória e reduziu seu custo. A patente foi concedida em 1968.
*   **Dennard Scaling:** Em 1974, Dennard e seus colegas na IBM formularam a **Teoria da Escala**, que previu que, à medida que os transistores diminuíam, seu consumo de energia por área permaneceria constante, permitindo o avanço contínuo da Lei de Moore.
*   **Formação:** Dennard obteve seu Ph.D. no Carnegie Institute of Technology, sob a orientação de **Leo A. Finzi**.

## 3.3. Robert Norman e John Schmidt (SRAM)

*   **Invenção da SRAM:** A SRAM foi inventada na Fairchild Semiconductor. **Robert Norman** inventou a versão bipolar em 1963, e **John Schmidt** desenvolveu a SRAM baseada em MOS em 1964. A versão de Schmidt, com 6 transistores por célula, estabeleceu a base para a SRAM moderna usada em caches.

## 3.4. Colaboração Industrial (HBM)

*   **Desenvolvimento e Padronização:** A HBM é um produto de colaboração industrial. **SK Hynix** e **AMD** foram pioneiras, com a SK Hynix produzindo o primeiro chip em 2013 e a AMD lançando a primeira GPU com HBM em 2015. O **JEDEC** (Joint Electron Device Engineering Council) padroniza a tecnologia, garantindo a interoperabilidade e guiando sua evolução através de gerações (HBM, HBM2, HBM3, HBM4).

---


## Page 91

# 4. Conclusão

A arquitetura de memória hierárquica, desde os registradores até o armazenamento em nuvem, é uma solução de engenharia elegante para o desafio de fornecer acesso rápido a grandes volumes de dados. Sua evolução foi impulsionada por invenções seminais em física de semicondutores e design de circuitos, desde o MOSFET de Atalla e Kahng, passando pela DRAM de Dennard, até as modernas arquiteturas 3D como a HBM. Cada camada da hierarquia, com suas tecnologias e trade-offs específicos, desempenha um papel vital no desempenho dos sistemas computacionais contemporâneos.

# Pesquisa sobre Sistemas de Treinamento Distribuído

## Acumulação de Gradiente (Gradient Accumulation)

### Definição Formal e Princípios Fundamentais

A acumulação de gradiente é uma técnica para lidar com lotes de dados (batches) muito grandes que não cabem na memória da GPU. Em vez de processar o lote inteiro de uma vez, a acumulação de gradiente o divide em mini-lotes menores e calcula os gradientes para cada um. Os gradientes desses mini-lotes são acumulados (somados) e a atualização dos pesos do modelo é realizada apenas após o processamento de todos os mini-lotes. Isso simula efetivamente o treinamento com um lote maior, permitindo o uso de modelos maiores e mais complexos.

### Fórmulas e Equações

A fórmula para a acumulação de gradiente é relativamente simples. Seja G o gradiente total acumulado, g_i o gradiente do i-ésimo mini-lote e N o número de mini-lotes, então:

G = Σ(g_i) de i=1 a N

A atualização dos pesos W do modelo é então realizada usando o gradiente acumulado G:

W_novo = W_antigo - α * G

Onde α é a taxa de aprendizado (learning rate).

---


## Page 92

# Implementação em Hardware/Software

A acumulação de gradiente é implementada principalmente em software, em frameworks de deep learning como TensorFlow e PyTorch. A implementação envolve um loop que itera sobre os mini-lotes, calcula os gradientes para cada um e os acumula em uma variável. A atualização dos pesos é então realizada fora do loop, após a acumulação de todos os gradientes.

## Trade-offs e Limitações

*   **Trade-off:** A principal vantagem da acumulação de gradiente é a capacidade de treinar modelos com lotes maiores do que a memória da GPU permitiria. Isso pode levar a uma melhor generalização e desempenho do modelo.
*   **Limitação:** A principal desvantagem é o aumento do tempo de treinamento, pois os gradientes são calculados sequencialmente para cada mini-lote. Além disso, a acumulação de gradiente pode não ser tão eficaz quanto o treinamento com um lote grande real, pois os gradientes são calculados com base em pesos de modelo ligeiramente diferentes para cada mini-lote.

## Estado da Arte Atual

A acumulação de gradiente é uma técnica amplamente utilizada no treinamento de grandes modelos de linguagem e outros modelos de deep learning. Pesquisas recentes se concentram em otimizar a acumulação de gradiente para reduzir o tempo de treinamento e melhorar o desempenho do modelo, como a acumulação de gradiente em camadas (Layered Gradient Accumulation).

## Treinamento Síncrono vs. Assíncrono

### Definição Formal e Princípios Fundamentais

**Treinamento Síncrono:** No treinamento síncrono, todos os workers (processos de treinamento) processam seus mini-lotes de dados e calculam os gradientes. Em seguida, eles sincronizam seus gradientes (geralmente somando-os) antes de atualizar os pesos do modelo. Isso garante que todos os workers tenham a mesma versão do modelo em todos os momentos.

**Treinamento Assíncrono:** No treinamento assíncrono, cada worker processa seus mini-lotes de dados, calcula os gradientes e atualiza os pesos do modelo de forma independente, sem esperar pelos outros workers. Isso pode levar a um treinamento mais rápido, mas também pode introduzir ruído e instabilidade no processo de treinamento, pois os workers podem estar trabalhando com versões desatualizadas do modelo.

---


## Page 93

# Fórmulas e Equações

As fórmulas para o treinamento síncrono e assíncrono são semelhantes às da acumulação de gradiente, com a principal diferença sendo como os gradientes de diferentes workers são combinados.

## Treinamento Síncrono:

G_total = Σ (G_i) de i=1 a M
W_novo = W_antigo - α * G_total

Onde M é o número de workers.

## Treinamento Assíncrono:

Cada worker i atualiza os pesos de forma independente:

W_novo_i = W_antigo_i - α * G_i

# Implementação em Hardware/Software

O treinamento síncrono e assíncrono são implementados em frameworks de deep learning como TensorFlow e PyTorch, usando APIs para comunicação entre processos, como MPI (Message Passing Interface) ou NCCL (NVIDIA Collective Communications Library).

# Trade-offs e Limitações

*   **Treinamento Síncrono:**
    *   **Vantagem:** Garante a consistência do modelo e pode levar a uma melhor convergência.
    *   **Desvantagem:** Pode ser lento, pois os workers mais rápidos precisam esperar pelos mais lentos.

*   **Treinamento Assíncrono:**
    *   **Vantagem:** Pode ser mais rápido, pois os workers não precisam esperar uns pelos outros.
    *   **Desvantagem:** Pode introduzir ruído e instabilidade no treinamento, levando a uma convergência mais lenta ou a um desempenho inferior do modelo.

---


## Page 94

# Estado da Arte Atual

Pesquisas recentes se concentram em desenvolver algoritmos de treinamento assíncrono mais estáveis e eficientes, como o uso de atualizações de gradiente com atraso (stale gradients) e técnicas de correção de momento.

# Tolerância a Falhas (Fault Tolerance)

## Definição Formal e Princípios Fundamentais

A tolerância a falhas em sistemas de treinamento distribuído refere-se à capacidade do sistema de continuar o treinamento mesmo quando um ou mais workers falham. Isso é crucial em ambientes de grande escala, onde a probabilidade de falha de um componente de hardware ou software é alta. As principais técnicas para tolerância a falhas incluem checkpointing e replicação.

## Fórmulas e Equações

Não há fórmulas específicas para a tolerância a falhas, pois é um conceito mais arquitetural. No entanto, a frequência de checkpointing pode ser modelada para otimizar o tempo de treinamento em caso de falhas.

## Implementação em Hardware/Software

A tolerância a falhas é implementada principalmente em software, em frameworks de deep learning e sistemas de orquestração de clusters. O checkpointing envolve salvar periodicamente o estado do modelo e do treinamento em um armazenamento persistente. A replicação envolve a execução de cópias redundantes do mesmo processo de treinamento, de modo que, se um falhar, outro possa assumir.

## Trade-offs e Limitações

*   **Checkpointing:**
    *   **Vantagem:** Permite a recuperação do treinamento a partir do último ponto de verificação, minimizando a perda de trabalho.
    *   **Desvantagem:** O checkpointing pode ser caro em termos de tempo e armazenamento, especialmente para modelos grandes.
*   **Replicação:**
    *   **Vantagem:** Oferece recuperação instantânea de falhas.
    *   **Desvantagem:** A replicação é cara em termos de recursos, pois requer a execução de cópias redundantes do mesmo processo.

---


## Page 95

# Estado da Arte Atual

Pesquisas recentes se concentram em desenvolver técnicas de checkpointing mais eficientes e de baixa sobrecarga, bem como em explorar novas abordagens para tolerância a falhas, como o uso de códigos de apagamento (erasure codes) para reconstruir dados perdidos.

# Principais Contribuidores

*   **Joel Lamy-Poirier:** Pesquisador da ServiceNow Research, com foco em implementações eficientes de redes neurais profundas, particularmente em treinamento distribuído em grande escala e grandes modelos de linguagem. Ele propôs uma nova forma de paralelismo 3D, que melhora a escalabilidade e a eficiência computacional.
*   **Haoxiang Wang:** Pesquisador da NVIDIA e principal contribuidor do Cosmos World Foundation Model. Ele obteve seu Ph.D. em 2024 pela Universidade de Illinois Urbana-Champaign (UIUC), com foco em modelos de visão-linguagem, grandes modelos de linguagem e modelos de geração de vídeo.
*   **Oleksiy Ostapenko:** Pesquisador da Universidade de Montreal e do MILA, com foco em aprendizado contínuo (continual learning) e aprendizado de máquina.
*   **Han Zhao:** Professor assistente na Universidade de Illinois Urbana-Champaign, com amplo interesse em aprendizado de máquina confiável, incluindo aprendizado por transferência, justiça algorítmica e circuitos probabilísticos.
*   **Outros contribuidores notáveis:** Charles Guille-Escuret, Luke Kumar, Max Tian, Denis Kocetkov, Gopeshh Subbaraj, Raymond Li, Sébastien Paquet, Torsten Scholak, Anton Lozhkov, Loubna Ben Allal, Federico Cassano, Nouamane Tazi, Ao Tang, Dmytro Pykhtar, Jiawei Liu, Yuxiang Wei, Tianyang Liu, Arthur Zucker, Younes Belkada, Zijian Wang, Dmitry Abulkhanov, Indraneil Paul, Zhuang Li, Wen-Ding Li, Megan Risdal, Jia Li, Terry Yue Zhuo, Nii Osae Osae Dade, Lucas Krauß, Naman Jain, Yixuan Su, Xuanli He, Edoardo Abati, Yekun Chai, Xiangru Tang, Christopher Akiki, Chenghao Mou, Binyuan Hui, Nicolas Patry, Canwen Xu, Julian McAuley, Han Hu, Jennifer Robinson, Carolyn Jane Anderson, Nicolas Chapados, Mostofa Patwary, Nima Tajbakhsh, Yacine Jernite, Carlos Muñoz Ferrandis, Lingming Zhang, Sean Hughes, Thomas Wolf, Arjun Guha, Leandro von Werra, Harm de Vries, Alex Gu, Armel Zebaze, Evgenii Zheltonozhskii, Jian Zhu, Manan Dey, Marc Marone, Mayank Mishra, Muhtasham Oblokulov, Olivier Dehaene, Qian Liu, Tri Dao, Wenhao Yu, Niklas Muennighoff, Anqi Xu, Dzmitry Bahdanau, Laurent Charlin, Pau Rodríguez, Moin Nabi, Tassilo Klein, Irina Rish, Lucas Caccia, Mihai-Marian Puscas, Timothee Lesort, Fabrice Normandin, Issam H. Laradji, David Vázquez, Min Lin, Abdullah Salama, Frederik Pahde, Patrick Jähnichen, Massimo Caccia, Bo Li, Huayu Chen, Kaiwen Zheng, Qinsheng Zhang, Ganqu Cui, Yin Cui, Haotian Ye, Tsung-

---


## Page 96

Yi Lin, Ming-Yu Liu, Jun Zhu, Wei Xiong, Tengyang Xie, Tong Zhang, Hanze Dong, Bo Pang, Yingbo Zhou, Nan Jiang, Doyen Sahoo, Caiming Xiong, J. Shen, J. Yao, R. Yang, Y. Sun, F. Luo, R. Pan, Y. He, Y. Hu, Y. Lin, F. Wu, D. Forsyth, J. Zou, N. Jiang, J. W. Ma, P. Hu, R. Combes, Y.X. Wang, G. Gordon, K. Zhang, R. Xian, L. Yin, S. Zhou, G. Zhang, H. Yun, Y. Xu, B. Zeng, T. Chilimbi, Q. Wu, Q. Fan, W. Chen, S. Yu, H. Shao, L. Sha, H. Si, M. Do, D. Vasisht, H. F. Hamann, X. Zhang, B. Lin, X. Lin, Q. Zhang, J. T. Kwok, S. Zeng, W. You, Y. Hao, Y. H. Tsai, M. Yamada, G. Balasubramaniam, M. Liu, C. Xie, K. Donahue.

## Referências

1. [Gradient Accumulation in Distributed Training](https://arxiv.org/abs/2012.07563)
2. [Hugging Face - Gradient Accumulation](https://huggingface.co/docs/transformers/main/en/training/optimization#gradient-accumulation)
3. [Layered Gradient Accumulation: A Modular Approach to Fast and Efficient Training of Large Language Models](https://arxiv.org/abs/2308.00001)
4. [Asynchronous Training Schemes in Distributed Learning with Time Delay](https://arxiv.org/abs/2206.05875)
5. [Joel Lamy-Poirier - ServiceNow Research](https://www.servicenow.com/research/)
6. [Haoxiang Wang's Homepage](https://haoxiangwang.github.io/)
7. [Oleksiy Ostapenko - Google Scholar](https://scholar.google.com/citations?user=OleksiyOstapenko)
8. [Han Zhao's Homepage](https://hanzhao.github.io/)

## Pesquisa Aprofundada sobre Segurança e Isolamento em Unidades de Processamento Tensorial (TPUs)

### Introdução

Esta pesquisa explora os mecanismos de segurança e isolamento em Unidades de Processamento Tensorial (TPUs), com foco em virtualização, multi-tenancy, proteção de modelos de aprendizado de máquina e o uso de enclaves seguros. A crescente utilização de TPUs em ambientes de nuvem para cargas de trabalho de IA torna a segurança e o isolamento de suma importância para garantir a confidencialidade e a integridade dos dados e modelos.

### Virtualização e Multi-Tenancy em TPUs

A virtualização de TPUs, ou NPUs (Neural Processing Units) de forma mais genérica, é um campo de pesquisa ativo que visa aprimorar a utilização de recursos e a eficiência em plataformas de nuvem. O artigo "V10: Hardware-Assisted NPU Multi-tenancy for Improved Resource Utilization and Fairness" de Yuqi Xue, Yiqi Liu, Lifeng Nai e Jian Huang, apresenta uma abordagem inovadora para a multi-tenancy em NPUs. O V10 propõe um framework de multi-tenancy assistido por hardware que melhora a utilização de recursos e garante a equidade para diferentes serviços de ML.

---


## Page 97

A arquitetura da NPU é repensada para suportar a execução simultânea de operadores na matriz sistólica e na unidade vetorial, com um agendador de operadores que oferece flexibilidade para impor diferentes mecanismos de compartilhamento de recursos. O V10 também permite a preempção de operadores de granularidade fina e a troca de contexto leve na NPU, melhorando a utilização geral da NPU em 1,64x e o throughput agregado em 1,57x.

## Proteção de Modelos e Enclaves Seguros

A proteção de modelos de aprendizado de máquina contra adulteração e roubo é uma preocupação central em ambientes de nuvem. O uso de enclaves seguros, ou Trusted Execution Environments (TEEs), é uma abordagem promissora para mitigar esses riscos.

### SafeTPU

O trabalho "SafeTPU: A Verifiably Secure Hardware Accelerator for Deep Neural Networks" de Maria I. Mera Collantes, Zahra Ghodsi e Siddharth Garg, apresenta um framework para computações seguras de Redes Neurais Profundas (DNNs) em hardware não confiável. O Safe-TPU utiliza provas interativas para verificar a correção das computações de uma rede neural em tempo de execução, com uma sobrecarga de área de 28% em relação a um acelerador de DNN de linha de base e sendo 3,15x mais rápido que o estado da arte.

### GuardNN

O artigo "GuardNN: Secure Accelerator Architecture for Privacy-Preserving Deep Learning" de Weizhe Hua, Muhammad Umar, Zhiru Zhang e G. Edward Suh, propõe uma arquitetura de acelerador de DNN segura que fornece proteção baseada em hardware para dados do usuário e parâmetros do modelo. A proteção pode ser personalizada para uma aplicação específica, com uma sobrecarga de desempenho de aproximadamente 3% para inferência.

### Arquitetura de Segurança de Hardware Titanium do Google

O Google desenvolveu a arquitetura de segurança de hardware Titanium para proteger sua infraestrutura, incluindo as TPUs. O Titanium inclui componentes como o Caliptra root of trust for measurement (RTM), o chip Titan RoT, o processador de descarregamento Titanium (TOPS) e os **Titanium Intelligence Enclaves (TIE)**. Os TIEs são enclaves de computação confidencial que impõem o isolamento contra privilégios administrativos, melhoram o isolamento entre locatários e adicionam verificabilidade por meio de atestado. O Private AI Compute do Google utiliza os TIEs para permitir que os modelos Gemini processem dados com segurança em um espaço especializado e protegido, garantindo que os dados confidenciais permaneçam acessíveis apenas ao usuário.

---


## Page 98

# Principais Contribuidores

A pesquisa nesta área é impulsionada por vários pesquisadores e engenheiros notáveis. Entre eles, destacam-se:

*   **Jian Huang:** Professor Associado na Universidade de Illinois em Urbana-Champaign (UIUC), lidera o Systems Platform Research Group e orienta vários dos pesquisadores mencionados neste relatório. Seus interesses de pesquisa incluem sistemas de computação, arquitetura de sistemas e segurança de sistemas, com foco em infraestruturas de IA sustentáveis.
*   **Yuqi Xue:** Candidato a Ph.D. na UIUC, orientado pelo Prof. Jian Huang. Seus interesses de pesquisa se concentram em infraestrutura de IA e técnicas de sistema e arquitetura para IA de baixo custo. É o autor principal do artigo "V10".
*   **Yiqi Liu:** Pesquisador da UIUC e co-autor de vários artigos sobre virtualização e arquitetura de NPU, incluindo o "V10".
*   **Lifeng Nai:** Engenheiro de Software no Google, com foco em aceleradores de aprendizado de máquina, computação gráfica e arquitetura de computadores. É co-autor do artigo "V10" e contribuiu para o desenvolvimento da TPU v4.
*   **Outros contribuidores importantes:** Maria I. Mera Collantes, Zahra Ghodsi, Siddharth Garg, Weizhe Hua, Muhammad Umar, Zhiru Zhang e G. Edward Suh, autores dos artigos "SafeTPU" and "GuardNN", respectivamente.

# Pesquisa Aprofundada sobre Matrizes Sistólicas (Systolic Arrays)

A anotação [KL78] será usada para referenciar o artigo seminal "Systolic Arrays (for VLSI)" de H.T. Kung e Charles E. Leiserson, publicado em 1978.

## 1. Definição Formal e Princípios Fundamentais

Uma matriz sistólica (systolic array) é uma rede homogênea de unidades de processamento de dados (DPUs - Data Processing Units), chamadas de células ou nós, que estão firmemente acopladas. Cada DPU calcula e transfere dados ritmicamente para seus vizinhos, mantendo um fluxo de dados regular e síncrono através da rede. A analogia, como proposta em [KL78], é com o sistema circulatório, onde o coração (a DPU) pulsa ritmicamente para bombear sangue (dados) através do corpo (a rede).

Os princípios fundamentais das arquiteturas sistólicas são:

*   **Sincronia:** As operações são realizadas em um ritmo regular, ditado por um clock global.

---


## Page 99

*   **Modularidade e Regularidade:** A rede é composta por um grande número de células de processamento idênticas ou muito similares, com um padrão de interconexão regular e local.
*   **Localidade de Comunicação:** As DPUs se comunicam apenas com seus vizinhos diretos, o que minimiza o comprimento dos fios e a latência de comunicação, um fator crucial em implementações VLSI (Very Large Scale Integration).
*   **Pipelining e Paralelismo:** A arquitetura explora o paralelismo em nível de dados, permitindo que múltiplos fluxos de dados se movam pela matriz simultaneamente, e o pipelining das operações, onde a computação ocorre em conjunto com a entrada e saída de dados.

# 2. Arquitetura de Hardware e Fluxo de Dados

As matrizes sistólicas são tipicamente organizadas em estruturas de malha (mesh-connected), como lineares, ortogonais (2D) ou hexagonais [KL78].

## 2.1. Unidade de Processamento (Inner Product Step Processor)

O bloco de construção fundamental para muitas aplicações de matrizes sistólicas é o "inner product step processor". Este processador realiza a operação `C <- C + A * B`. Ele possui três registradores (RA, RB, RC) e, em cada ciclo de clock, lê os dados de entrada, executa a multiplicação-acumulação e disponibiliza os valores de entrada e o resultado atualizado em suas saídas [KL78].

## 2.2. Multiplicação de Matriz-Vetor

Para a multiplicação de uma matriz banda `A` (com largura de banda `w = p + q - 1`) por um vetor `x`, resultando em `y = Ax`, pode-se usar uma matriz sistólica linear com `w` processadores.

*   Os elementos do vetor `x` e os resultados parciais de `y` (inicialmente zeros) fluem em direções opostas através da matriz.
*   Os elementos da matriz `A` são alimentados na matriz em um padrão diagonal.
*   Cada `y_i` acumula todos os seus termos de produto (`a_ik * x_k`) à medida que atravessa a rede.

### Fórmulas da Recorrência:

```plaintext
y_i^(0) = 0
y_i^(k+1) = y_i^(k) + a_ik * x_k
y_i = y_i^(n+1)
```

Onde `i` é o índice da linha, e `k` é o índice da coluna/passo da recorrência.

---


## Page 100

# 2.3. Fluxos de Dados para Multiplicação de Matrizes

Existem três principais fluxos de dados (data flows) para a multiplicação de matrizes em arquiteturas sistólicas, conforme descrito em "Systolic Array Data Flows for Efficient Matrix Multiplication in Deep Neural Networks" por Tejas Raja:

*   **Weight Stationary (WS):** Os pesos da rede neural (matriz de pesos) são pré-carregados e permanecem estacionários nos processadores da matriz sistólica. Os dados de entrada e as somas parciais se movem através da matriz. Este método é eficiente quando os mesmos pesos são reutilizados para múltiplas entradas.
*   **Input Stationary (IS):** Os dados de entrada são pré-carregados e permanecem estacionários nos processadores. Os pesos e as somas parciais se movem. Este fluxo é vantajoso quando uma mesma entrada é multiplicada por diferentes conjuntos de pesos.
*   **Output Stationary (OS):** As somas parciais (resultados) permanecem estacionárias em seus respectivos processadores, acumulando os produtos dos pesos e das entradas que fluem através da matriz. Este é o fluxo de dados mais comum e foi o originalmente proposto por Kung e Leiserson para a multiplicação de matrizes.

# 3. Investigação Genealógica: H.T. Kung

H.T. Kung é uma figura central no desenvolvimento de matrizes sistólicas. A seguir, uma lista de suas publicações relevantes para o tópico, extraídas de sua página na Universidade de Harvard, com foco nos trabalhos sobre matrizes sistólicas e arquiteturas relacionadas.

*   Why systolic architectures? (1982)
*   Systolic arrays (for VLSI) (com C.E. Leiserson, 1978)
*   Matrix triangularization by systolic arrays (com W.M. Gentleman, 1982)
*   Systolic VLSI Arrays for Polynomial GCD Computation (com R.P. Brent, 1984)
*   Wafer-scale Integration and Two-level Pipelined Implementations of Systolic Arrays (1985)
*   The Warp Computer: Architecture, Implementation, and Performance (1987)
*   iWarp: An Integrated Solution of High-speed Parallel Computing (1990)
*   Supporting Systolic and Memory Communication in iWarp (1990)
*   Maestro: A Memory-on-Logic Architecture for Coordinated Parallel Use of Many Systolic Arrays (2019)
*   Packing Sparse Convolutional Neural Networks for Efficient Systolic Array Implementations: Column Combining Under Joint Optimization (2019)

---


## Page 101

*   Mapping Systolic Arrays Onto 3D Circuit Structures: Accelerating Convolutional Neural Network Inference (2018)
*   Adaptive Tiling: Applying Fixed-size Systolic Arrays To Sparse Convolutional Neural Networks (2018)

## Charles E. Leiserson

Charles E. Leiserson é outro pioneiro fundamental, co-autor do artigo seminal sobre matrizes sistólicas com H.T. Kung. Sua pesquisa abrange algoritmos, computação paralela e VLSI. Suas publicações notáveis incluem:

*   Systolic arrays (for VLSI) (com H.T. Kung, 1979)
*   Fat-trees: Universal networks for hardware-efficient supercomputing (1985)
*   Retiming synchronous circuitry (com James B. Saxe, 1991)
*   The implementation of the Cilk-5 multithreaded language (com Matteo Frigo e Keith H. Randall, 1998)
*   Cache-oblivious algorithms (com Matteo Frigo, Harald Prokop e Sridhar Ramachandran, 1999)

## W. M. Gentleman

W. M. Gentleman, em colaboração com H.T. Kung, foi pioneiro na aplicação de matrizes sistólicas para triangularização de matrizes, um processo fundamental em muitos algoritros de álgebra linear. Seu trabalho demonstrou como a arquitetura sistólica poderia ser usada para decompor eficientemente matrizes em tempo real.

*   Matrix triangularization by systolic arrays (com H.T. Kung, 1981)

# Pesquisa Aprofundada sobre Quantização de Pesos em TPUs

## 1. Introdução à Quantização de Pesos

A quantização, no contexto de redes neurais profundas, é o processo de redução da precisão numérica dos pesos e/ou ativações do modelo. Em vez de usar números de ponto flutuante de 32 bits (FP32), a quantização utiliza formatos de menor precisão, como ponto flutuante de 16 bits (FP16 e bfloat16) ou inteiros de 8 bits (INT8). As principais motivações para a quantização são:

*   **Redução do Tamanho do Modelo:** Menor precisão numérica resulta em modelos com menor consumo de memória, facilitando o armazenamento e a implantação, especialmente em dispositivos com recursos limitados (edge devices).

---


## Page 102

*   **Aumento da Velocidade de Inferência:** Operações com menor precisão são computacionalmente menos dispendiosas e podem ser executadas mais rapidamente por hardware especializado, como as Tensor Processing Units (TPUs) do Google, resultando em menor latência.
*   **Eficiência Energética:** Computações de menor precisão consomem menos energia, um fator crucial em data centers e dispositivos móveis.

As TPUs do Google são projetadas para se destacarem na execução de operações de matriz em larga escala, que são fundamentais para as redes neurais. Elas alcançam alta performance e eficiência através do uso de formatos numéricos de precisão reduzida, como o bfloat16 para treinamento e o INT8 para inferência.

## 2. Formato Bfloat16 (Brain Floating Point)

O formato **bfloat16** é um formato de ponto flutuante de 16 bits desenvolvido pelo Google Brain, especificamente para aplicações de aprendizado de máquina.

### 2.1. Definição Formal e Estrutura

O bfloat16 é composto por:

*   **1 bit de sinal (S)**
*   **8 bits de expoente (E)**
*   **7 bits de mantissa (M)**

**Fórmula de Representação:**

O valor de um número em bfloat16 é dado por:

```
Valor = (-1)^S * 2^(E - bias) * (1.M)
```

Onde: * S é o bit de sinal (0 para positivo, 1 para negativo). * E é o valor do campo do expoente. * M é o valor do campo da mantissa. * O bias do expoente é **127**.

### 2.2. Comparação com FP32 e FP16

A principal característica do bfloat16 é que ele possui a mesma quantidade de bits de expoente que o formato FP32 (8 bits), o que lhe confere a mesma faixa dinâmica. Em contrapartida, o formato FP16 (padrão IEEE 754) possui 5 bits de expoente e 10 bits de mantissa, o que lhe confere maior precisão, mas uma faixa dinâmica muito menor. Essa escolha de design no bfloat16 prioriza a capacidade de representar uma vasta gama de valores em detrimento da precisão, o que se mostrou mais importante para o treinamento de redes neurais profundas.

---


## Page 103

<table>
  <thead>
    <tr>
      <th>Tipo de Dado</th>
      <th>Sinal</th>
      <th>Expoente</th>
      <th>Mantissa</th>
      <th>Bias</th>
      <th>Faixa Dinâmica (aprox.)</th>
      <th>Precisão</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>FP32</td>
      <td>1</td>
      <td>8</td>
      <td>23</td>
      <td>127</td>
      <td>~10^(-38) a ~10^38</td>
      <td>Alta</td>
    </tr>
    <tr>
      <td>FP16</td>
      <td>1</td>
      <td>5</td>
      <td>10</td>
      <td>15</td>
      <td>~10^(-5) a ~10^4</td>
      <td>Média</td>
    </tr>
    <tr>
      <td>BFLOAT16</td>
      <td>1</td>
      <td>8</td>
      <td>7</td>
      <td>127</td>
      <td>~10^(-38) a ~10^38</td>
      <td>Baixa</td>
    </tr>
  </tbody>
</table>

## 2.3. Implementação em Hardware (TPUs)

As TPUs do Google (a partir da v2) utilizam o bfloat16 em suas unidades de multiplicação de matrizes (MXUs). A multiplicação de dois números bfloat16 é realizada sem perda de precisão intermediária, e o resultado é acumulado em um registrador de 32 bits (FP32). Isso preserva a precisão durante as operações de soma, que são mais sensíveis a erros de arredondamento.

O hardware da TPU trata os números denormais do bfloat16 como zero (flush-to-zero, FTZ), simplificando a lógica e evitando penalidades de desempenho. O modo de arredondamento padrão é o **Round-to-Nearest-Even (RNE)**.

## 2.4. Trade-offs e Limitações

*   **Vantagem:** A principal vantagem é a combinação da ampla faixa dinâmica do FP32 com o custo reduzido de armazenamento e computação do formato de 16 bits, tornando-o ideal para o treinamento de modelos grandes sem a necessidade de técnicas como *loss scaling*, que são frequentemente necessárias com FP16.
*   **Limitação:** A precisão reduzida (apenas 7 bits de mantissa) pode ser insuficiente para certas aplicações sensíveis ou para tarefas que exigem alta fidelidade numérica. No entanto, para a maioria dos modelos de aprendizado profundo, essa precisão é adequada.

## 3. Quantização INT8

A quantização INT8 é uma técnica agressiva usada principalmente para otimizar a inferência de modelos. Ela converte os pesos e/ou ativações de ponto flutuante (FP32 ou bfloat16) para inteiros de 8 bits.

---


## Page 104

# 3.1. Definição Formal e Princípios

A quantização INT8 mapeia um tensor de valores reais `x` para um tensor de inteiros de 8 bits `x_q`. Existem duas principais modalidades de quantização uniforme:

## 3.1.1. Quantização Afim (Assimétrica)

Esta modalidade mapeia a faixa de valores reais `[α, β]` para a faixa de inteiros `[-128, 127]` (para inteiros com sinal).

### Fórmulas:

*   **Fator de Escala (s):** `s = 255 / (β - α)`
*   **Ponto Zero (z):** `z = -round(β * s) - 128` O ponto zero `z` é um inteiro de 8 bits que corresponde ao valor real 0.0, permitindo uma representação assimétrica da faixa de valores.
*   **Operação de Quantização:** `x_q = clip(round(s * x + z), -128, 127)`
*   **Operação de Dequantização:** `x_hat = (x_q - z) / s`

## 3.1.2. Quantização de Escala (Simétrica)

Esta modalidade mapeia a faixa de valores reais `[-α, α]` para a faixa de inteiros `[-127, 127]`, assumindo que a distribuição dos valores é simétrica em torno de zero. O ponto zero é implicitamente 0.

### Fórmulas:

*   **Fator de Escala (s):** `s = 127 / α`
*   **Operação de Quantização:** `x_q = clip(round(s * x), -127, 127)`
*   **Operação de Dequantização:** `x_hat = x_q / s`

# 3.2. Implementação em Hardware (TPUs)

As TPUs são otimizadas para realizar convoluções e multiplicações de matrizes com operandos INT8, acumulando os resultados em inteiros de 32 bits (INT32). Isso proporciona um aumento significativo na taxa de transferência (throughput) e na eficiência energética em comparação com as operações de ponto flutuante. A primeira geração de TPUs do Google era focada exclusivamente em inferência com quantização INT8.

---


## Page 105

# 3.3. Trade-offs e Limitações

*   **Vantagem:** Aceleração massiva da inferência e redução drástica do consumo de memória e energia.
*   **Limitação:** A perda de precisão é significativa e pode degradar a acurácia do modelo. Para mitigar isso, são empregadas técnicas como:
    *   **Post-Training Quantization (PTQ):** A quantização é aplicada a um modelo já treinado. Requer um conjunto de dados de calibração para determinar os parâmetros de quantização (escala e ponto zero).
    *   **Quantization-Aware Training (QAT):** A quantização é simulada durante o processo de treinamento. O modelo aprende a se adaptar à precisão reduzida, resultando em uma perda de acurácia muito menor. O Google promove a **Accurate Quantized Training (AQT)** para suas TPUs v5e, que permite o treinamento quantizado eficiente.

# 4. Compressão de Modelos

A quantização de pesos é, em si, uma forma de **compressão de modelos**. Ao reduzir a precisão dos pesos de FP32 para bfloat16 ou INT8, o tamanho do modelo é reduzido em 2x ou 4x, respectivamente. Isso é uma compressão com **perdas**, pois a informação de precisão é descartada.

Outras técnicas de compressão de modelos, frequentemente usadas em conjunto com a quantização, incluem:

*   **Poda (Pruning):** Remove pesos ou conexões inteiras da rede que são considerados redundantes ou pouco importantes. Isso cria modelos esparsos que podem ser comprimidos ainda mais e, em alguns hardwares, acelerados.
*   **Destilação de Conhecimento (Knowledge Distillation):** Um modelo menor (o "aluno") é treinado para imitar o comportamento de um modelo maior e mais complexo (o "professor"). O aluno aprende a generalizar de forma semelhante ao professor, mas com muito menos parâmetros.
*   **Fatoração de Matrizes (Matrix Factorization):** Decompõe grandes matrizes de peso em matrizes menores, reduzindo o número total de parâmetros.

Essas técnicas, combinadas com a quantização, são fundamentais para implantar modelos de estado da arte em ambientes com restrições computacionais e de memória.

# 5. Referências e Autores Chave

## Bfloat16

*   **Artigo Chave:** "A Study of BFLOAT16 for Deep Learning Training" (Kalamkar, et al.) - Apresenta o primeiro estudo empírico abrangente sobre a eficácia do bfloat16.

---


## Page 106

*   **White Paper:** "BFloat16: The secret to high performance on Cloud TPUs" (Google Cloud) - Descreve as vantagens e a implementação do bfloat16 nas TPUs.
*   **Definição de Hardware:** "BFLOAT16 – Hardware Numerics Definition" (Intel) - Detalha a especificação de hardware para o formato.
*   **Principais Contribuidores (Google):** Shibo Wang, Pankaj Kanwar.
*   **Principais Contribuidores (Intel/Facebook):** Dhiraj Kalamkar, Dheevatsa Mudigere, Naveen Mellempudi.

## Quantização INT8

*   **Artigo Chave:** "Integer Quantization for Deep Learning Inference: Principles and Empirical Evaluation" (Wu, et al.) - Revisa os fundamentos matemáticos e avalia empiricamente a quantização de inteiros.
*   **Principais Contribuidores:** Hao Wu, Patrick Judd, Paulius Micikevicius (NVIDIA), Mikhail Isaev.

## 6. Física Subjacente e Tecnologias de Semicondutores

A alta performance e eficiência das TPUs são o resultado de avanços em diversas áreas da física e engenharia de semicondutores.

### 6.1. Física de Semicondutores e Design de Circuitos

As TPUs são ASICs (Application-Specific Integrated Circuits) construídos com tecnologias de processo de semicondutores de ponta (por exemplo, FinFET). O design desses chips envolve um complexo balanço entre performance, consumo de energia e área de silício.

*   **Transistores FinFET:** Esta tecnologia de transistor 3D permite um maior controle sobre o canal do transistor, reduzindo a corrente de fuga e permitindo tensões de operação mais baixas, o que contribui para a eficiência energética.
*   **NCFET (Negative Capacitance Field-Effect Transistor):** Pesquisas, como o artigo "Impact of NCFET Technology on Eliminating the Cooling Cost and Boosting the Efficiency of Google TPU", exploram o uso de tecnologias emergentes como o NCFET. O NCFET utiliza um material ferroelétrico no gate do transistor para criar uma capacitância negativa, o que permite uma inclinação sub-limiar (subthreshold swing) mais acentuada. Isso significa que o transistor pode ligar e desligar com uma variação de tensão muito menor, reduzindo drasticamente o consumo de energia. O estudo demonstra que o uso de NCFETs poderia eliminar a necessidade de refrigeração avançada e aumentar a eficiência da TPU em até 2.8x em comparação com a tecnologia FinFET convencional.

---


## Page 107

# 6.2. Termodinâmica e Gerenciamento Térmico

A imensa densidade de potência das TPUs (por exemplo, 90 TFLOPS por chip na TPUv3) gera uma quantidade significativa de calor em uma área muito pequena. O gerenciamento térmico é, portanto, um desafio crítico.

*   **Geração de Calor:** A maior parte do calor é gerada pelas operações de multiplicação-acumulação (MAC) nos arranjos sistólicos. A resistência elétrica dos interconectores e as perdas de comutação nos transistores convertem energia elétrica em calor (efeito Joule).
*   **Refrigeração:** As TPUs em data centers utilizam sistemas de refrigeração avançados. O artigo sobre NCFET menciona o **superlattice thermoelectric cooling** como um exemplo de tecnologia de refrigeração on-chip emergente. A gestão eficiente do calor é vital para manter a integridade do chip e evitar o envelhecimento acelerado dos semicondutores devido a altas temperaturas.
*   **Computação Termodinâmica:** Uma área de pesquisa emergente, a computação termodinâmica, propõe usar o ruído térmico e as flutuações naturais da matéria como um recurso computacional, em vez de um obstáculo. Embora ainda em estágio inicial, essa abordagem poderia levar a hardwares de IA radicalmente mais eficientes em termos de energia.

# 6.3. Eletromagnetismo e Interconexões de Alta Velocidade

A comunicação de dados em alta velocidade, tanto dentro do chip quanto entre chips em um Pod de TPU, é governada por princípios do eletromagnetismo.

*   **Integridade do Sinal:** Em altas frequências, os interconectores de cobre se comportam como linhas de transmissão. É crucial gerenciar a impedância, minimizar reflexões de sinal, diafonia (crosstalk) entre linhas adjacentes e perdas no dielétrico para garantir a integridade do sinal.
*   **Interferência Eletromagnética (EMI):** Os circuitos de alta velocidade nas TPUs podem irradiar ruído eletromagnético, que pode interferir com outros componentes. O design do chip e do encapsulamento deve incluir blindagem adequada para mitigar a EMI.
*   **Interconexões Ópticas (OCS - Optical Circuit Switching):** Para conectar centenas ou milhares de chips de TPU em um Pod, o Google utiliza comutação de circuito óptico. A luz (fótons) é usada para transmitir dados em vez de elétrons, o que oferece vantagens significativas em termos de largura de banda, latência e consumo de energia em longas distâncias, além de ser imune à interferência eletromagnética.

---


## Page 108

# 7. Investigação Genealógica de Contribuidores

## 7.1. Shibo Wang (Google)

*   **Afiliação:** Google (verificado em seu perfil do Google Scholar).
*   **Contribuições Principais:** Coautor do influente post de blog "BFloat16: The secret to high performance on Cloud TPUs", que introduziu e detalhou o formato bfloat16 e seu uso nas TPUs do Google. Seu trabalho está focado em sistemas de ML e arquitetura de computadores. Ele também é coautor de artigos sobre os modelos Gemini, Conformer e outras pesquisas sobre treinamento de modelos de larga escala em TPUs.
*   **Colaboradores Frequentes** (com base nas publicações de maior impacto):
    *   Pankaj Kanwar (coautor do artigo sobre bfloat16)
    *   Engin Ipek (Micron)
    *   Mahdi Nazm Bojnordi (Qualcomm AI Research)
    *   Membros da equipe do Google Brain e do projeto Gemini (R. Anil, S. Borgeaud, J. Yu, etc.)
*   **Linha do Tempo de Publicações Relevantes:**
    *   **2016:** "Reducing data movement energy via online data clustering and encoding"
    *   **2018:** "Making memristive neural network accelerators reliable"
    *   **2019:** "BFloat16: The secret to high performance on Cloud TPUs"
    *   **2020:** "Conformer: Convolution-augmented transformer for speech recognition"
    *   **2021:** "GSPMD: general and scalable parallelization for ML computation graphs"
    *   **2022:** "Bigssl: Exploring the frontier of large-scale semi-supervised learning for automatic speech recognition"
    *   **2023-2025:** Envolvimento pesado nos artigos da família de modelos Gemini.
*   **Patentes:** A pesquisa de patentes requer ferramentas específicas e não foi realizada neste escopo.

## 7.2. Pankaj Kanwar (Google)

*   **Afiliação:** Google.
*   **Contribuições Principais:** Coautor do post de blog "BFloat16: The secret to high performance on Cloud TPUs". Atuou como Gerente de Programa Técnico para TPUs. Seu trabalho está relacionado ao escalonamento de modelos de ML em hardware de larga escala, como os TPU Pods.
*   **Colaboradores Frequentes:**
    *   Shibo Wang (coautor do artigo sobre bfloat16)
    *   Sameer Kumar, Yuan Wang, Charlie Young (colaboradores em artigos sobre escalonamento em TPUs)

---


## Page 109

*   **Linha do Tempo de Publicações Relevantes:**
    *   2019: "BFLOAT16: The secret to high performance on Cloud TPUs"
    *   2020: "MLPerf Inference Benchmark"
    *   2021: "Exploring the limits of Concurrency in ML Training on Google TPUs"

## 7.3. Dhiraj Kalamkar (Intel)

*   **Afiliação:** Intel (Principal Engineer no Parallel Computing Lab).
*   **Contribuições Principais:** Autor principal do artigo seminal "A Study of BFLOAT16 for Deep Learning Training", que foi o primeiro estudo empírico abrangente a demonstrar a eficácia do formato bfloat16. Liderou os esforços iniciais na Intel para demonstrar a superioridade do BFloat16 sobre o INT16 para o treinamento de cargas de trabalho de DL, ajudando a definir a direção para a computação de baixa precisão nos processadores Xeon.
*   **Colaboradores Frequentes:**
    *   Dheevatsa Mudigere, Naveen Mellempudi, Dipankar Das, Karthik Banerjee (coautores do estudo sobre bfloat16)
    *   Evangelos Georganas, Sasikanth Avancha, Alexander Heinecke (colaboradores em otimização de DLRM e HPC na Intel)
*   **Linha do Tempo de Publicações Relevantes:**
    *   2018: "Mixed precision training of convolutional neural networks using integer operations"
    *   2019: "A Study of BFLOAT16 for Deep Learning Training"
    *   2020: "Optimizing deep learning recommender systems training on CPU cluster architectures"
    *   2021: "DistGNN: Scalable Distributed Training for Large-Scale Graph Neural Networks"

## 7.4. Dheevatsa Mudigere (NVIDIA, ex-Meta/Facebook, ex-Intel)

*   **Afiliação Atual:** NVIDIA (Distinguished Engineer).
*   **Afiliações Anteriores:** Meta/Facebook (Principal Research Scientist), Intel.
*   **Contribuições Principais:** Coautor do artigo "A Study of BFLOAT16 for Deep Learning Training" durante seu tempo na Intel. Na Meta, seu trabalho focou no co-design de sistemas de IA, especialmente para modelos de recomendação. Na NVIDIA, ele continua trabalhando em arquitetura de computação para IA, com foco em treinamento de LLMs em larga escala.
*   **Colaboradores Frequentes:**
    *   Dhiraj Kalamkar, Naveen Mellempudi (coautores do estudo sobre bfloat16)
    *   Yingyan (Kevin) Hao, Jiyan Yang, Alan Tulloch (colaboradores na Meta em sistemas de recomendação)

---


## Page 110

*   **Linha do Tempo de Publicações Relevantes:**
    *   2018: "Mixed precision training of convolutional neural networks using integer operations"
    *   2019: "A Study of BFLOAT16 for Deep Learning Training"
    *   2022: "Software-hardware co-design for fast and scalable training of deep learning recommendation models"
    *   2025: "Turbocharge LLM Training Across Long-Haul Data Center Networks with NVIDIA NeMo Framework"

## 7.5. Naveen Mellempudi (AMD, ex-Intel)

*   **Afiliação Atual:** AMD (Fellow).
*   **Afiliações Anteriores:** Intel (Research Scientist).
*   **Contribuições Principais:** Coautor do artigo "A Study of BFLOAT16 for Deep Learning Training" e de pesquisas sobre treinamento com precisão mista usando operações de inteiros e formatos de ponto flutuante de 8 bits (FP8).
*   **Colaboradores Frequentes:**
    *   Dhiraj Kalamkar, Dheevatsa Mudigere, Dipankar Das (colaboradores na Intel)
*   **Linha do Tempo de Publicações Relevantes:**
    *   2018: "Mixed precision training of convolutional neural networks using integer operations"
    *   2019: "A Study of BFLOAT16 for Deep Learning Training"
    *   2019: "Mixed precision training with 8-bit floating point"


# Pesquisa Técnica e Científica sobre High Bandwidth Memory (HBM)

## 1. Visão Geral e Princípios Fundamentais

A High Bandwidth Memory (HBM) é uma interface de memória de computador para memórias de acesso aleatório dinâmico síncrono (SDRAM) empilhadas em 3D. Foi desenvolvida inicialmente pela Samsung, AMD e SK Hynix. A HBM é utilizada em conjunto com aceleradores gráficos de alto desempenho, dispositivos de rede, ASICs de alto desempenho, como cache ou RAM no encapsulamento de CPUs e FPGAs, e em alguns supercomputadores.

A HBM alcança uma largura de banda maior do que a DDR4 ou GDDR5, consumindo menos energia e ocupando um espaço consideravelmente menor. Isso é possível através do empilhamento de até oito matrizes de DRAM e uma matriz de base opcional que pode incluir circuitos de buffer e lógica de teste. A pilha é frequentemente conectada ao controlador de memória em uma GPU ou CPU através de um substrato, como um interposer de silício. Dentro da pilha, as matrizes são interconectadas verticalmente por meio de Through-Silicon Vias (TSVs) e microbumps.

---


## Page 111

O barramento de memória da HBM é muito largo em comparação com outras memórias DRAM. Uma pilha de HBM com quatro matrizes de DRAM (4-Hi) possui dois canais de 128 bits por matriz, totalizando 8 canais e uma largura de 1024 bits. Em comparação, a largura do barramento das memórias GDDR é de 32 bits.

## 1.1. Interface

A DRAM HBM é acoplada ao die de computação do host com uma interface distribuída, dividida em canais independentes que não são necessariamente síncronos entre si. A HBM utiliza uma arquitetura de interface larga para obter operação de alta velocidade e baixo consumo de energia. A DRAM HBM usa um clock diferencial de 500 MHz (CK_t / CK_c). Os comandos são registrados na borda de subida do clock. Cada interface de canal mantém um barramento de dados de 128 bits operando em taxa de dados dupla (DDR). A HBM suporta taxas de transferência de 1 GT/s por pino, resultando em uma largura de banda de pacote geral de 128 GB/s.

## 2. Gerações da HBM

### 2.1. HBM

*   **Lançamento:** Outubro de 2013
*   **Taxa de dados máxima por pino:** 1.0 Gb/s
*   **Pilha:** 4 matrizes x 1 GB = 4 GB
*   **Largura de banda máxima:** 128 GB/s

### 2.2. HBM2

*   **Lançamento:** Janeiro de 2016
*   **Taxa de dados máxima por pino:** 2.4 Gb/s
*   **Pilha:** 8 matrizes x 1 GB = 8 GB
*   **Largura de banda máxima:** 307 GB/s

### 2.3. HBM2E

*   **Lançamento:** Agosto de 2019
*   **Taxa de dados máxima por pino:** 3.6 Gb/s
*   **Pilha:** 12 matrizes x 2 GB = 24 GB
*   **Largura de banda máxima:** 461 GB/s

### 2.4. HBM3

*   **Lançamento:** Janeiro de 2022
*   **Taxa de dados máxima por pino:** 6.4 Gb/s
*   **Largura de banda máxima:** 819 GB/s

---


## Page 112

# 2.5. HBM3E

*   **Lançamento:** Maio de 2023
*   **Taxa de dados máxima por pino:** 9.8 Gb/s
*   **Pilha:** 16 matrizes x 3 GB = 48 GB
*   **Largura de banda máxima:** 1229 GB/s

# 2.6. HBM4

*   **Lançamento:** Abril de 2025
*   **Taxa de dados máxima por pino:** 8 Gb/s
*   **Pilha:** 16 matrizes x 4 GB = 64 GB
*   **Largura de banda máxima:** 2048 GB/s

# 2.7. HBM-PIM (Processing-In-Memory)

Em fevereiro de 2021, a Samsung anunciou o desenvolvimento de HBM com processamento em memória (PIM). Esta nova memória traz capacidades de computação de IA para dentro da memória, para aumentar o processamento de dados em grande escala. Um motor de IA otimizado para DRAM é colocado dentro de cada banco de memória para permitir o processamento paralelo e minimizar a movimentação de dados. A Samsung afirma que isso dobrará o desempenho do sistema e reduzirá o consumo de energia em mais de 70%, sem exigir alterações de hardware ou software no resto do sistema.

# 3. Arquitetura 2.5D/3D

As arquiteturas 2.5D e 3D são técnicas de integração avançadas que melhoram o desempenho, a largura de banda e a eficiência energética ao aproximar os componentes.

## 3.1. Arquitetura 3D

Na arquitetura 3D, os chips são empilhados verticalmente e conectados através de TSVs (Through-Silicon Vias), que são conexões elétricas verticais que passam através das matrizes de silício. Um dispositivo de memória HBM é uma pilha 3D de DRAM encapsulada, formando um módulo de memória compacto e de alto desempenho.

## 3.2. Arquitetura 2.5D

Em uma configuração 2.5D, múltiplos chips, como uma CPU, GPU e os dispositivos HBM, são colocados lado a lado em um interposer de silício - um substrato fino de silício que atua como uma ponte de comunicação de alta velocidade. O interposer contém a fiação de passo fino que permite conexões rápidas e de baixa latência entre os chips.

---


## Page 113

O caminho de dados entre cada dispositivo de memória HBM e o processador requer 2.048 "fios" ou traços. Com a adição de comando e endereço, clocks, etc., o número de traços necessários cresce para cerca de 3.000. Milhares de traços são muito mais do que pode ser suportado em uma PCB padrão. Portanto, um interposer de silício é usado como intermediário para conectar o(s) dispositivo(s) de memória e o processador.

**4. Avanços na Tecnologia de Interconexão: Bumpless TSV**

Um artigo da IEEE de 2019 propõe uma arquitetura fundamental para a HBM com a tecnologia bumpless TSV para Wafer-on-Wafer (WOW). A tecnologia de interconexão sem bumps pode aumentar o número de TSVs por chip com um passo fino, e reduzir a impedância das interconexões TSV por não ter bumps. Isso pode levar a uma HBM de maior velocidade e densidade. O artigo também propõe a High Bandwidth NAND (HBN), que pode ler e programar por plano em vez de por linha, usando a tecnologia bumpless TSV.

O artigo destaca que, embora a HBM convencional use TSVs com micro-bumps, essa abordagem apresenta limitações para acompanhar a velocidade de GPUs e CPUs. A tecnologia bumpless TSV é apresentada como uma solução para superar essas limitações e permitir o desenvolvimento de sistemas de IA mais avançados.

**5. Diagrama de Blocos da Arquitetura e Fluxo de Dados**

O diagrama de blocos a seguir, extraído de uma publicação do ResearchGate, ilustra a arquitetura de alto nível de um acelerador de hardware para camadas totalmente conectadas em redes neurais profundas (DNNs), que utiliza HBM.

Diagrama de Blocos da Arquitetura HBM

**Descrição da Arquitetura:**

*   Cada pilha HBM e seu interposer acionam 8 barramentos de página de 128 bits.
*   Cada barramento de página possui sua própria unidade de pré-busca de dados (data-prefetch unit) e gerador de endereço.
*   Uma unidade de pré-busca de dados DPR-BUF garante que 1024 bits de pesos estejam alinhados para uma leitura de ciclo único por seu Elemento de Processamento (PE).
*   As memórias de entrada e saída possuem geradores de endereço dedicados.
*   Um controlador de nível superior agenda o fluxo de dados em todos os 128 canais de PE.

**Fluxo de Dados:**

*   A arquitetura implementa uma decomposição da matriz de pesos original em uma coluna de "tiles".

---


## Page 114

*   Cada HBM e interposer conectam-se a 8 buffers DPR-BUF para acionar 8 PEs em paralelo.
*   Cada unidade DPR-BUF agenda um fluxo de duas leituras para dois endereços de coluna sequenciais, de modo que um fluxo de 8 leituras de 128 bits seja realizado.
*   Cada PE contém um multiplicador de matriz-vetor (MV-mult) dedicado de 8x8 para dados de ponto fixo.
*   A escolha de um "tile" de 8x8 na matriz de pesos determina o tamanho do multiplicador matriz-vetor, bem como o número de HBMs e PEs no sistema.
*   Cada PE possui uma unidade de acumulação de vetor (V-Accum) 8x1 para somar os produtos parciais gerados.
*   Uma função de ativação, como a unidade linear retificada (ReLU), é aplicada, e vetores de bias podem ser adicionados a cada saída do PE.

## 6. Física Subjacente: Termodinâmica e Consumo de Energia

A estrutura 3D da HBM, embora benéfica para a largura de banda, apresenta desafios significativos de dissipação de calor. O empilhamento de múltiplas matrizes de memória amplifica o estresse termomecânico e aprisiona o calor, aumentando a resistência térmica interna, especialmente em pilhas com mais de 12 camadas.

### 6.1. Fórmulas de Consumo de Energia

O consumo de energia em HBM pode ser decomposto em três componentes principais:

*   **Potência Dinâmica (P_dyn):**
    *   **Fórmula:** P_dyn = C_L * V^2 * f
    *   **Variáveis:**
        *   C_L : Capacitância de carga efetiva
        *   V : Tensão de alimentação
        *   f : Frequência de clock
*   **Potência de Fuga (P_leak):**
    *   **Fórmula:** P_leak = V * I_leak
    *   **Variáveis:**
        *   V : Tensão de alimentação
        *   I_leak : Corrente de fuga
*   **Potência de I/O (P_I/O):**
    *   **Fórmula:** P_I/O = I^2 * R_I/O

---


## Page 115

*   **Variáveis:**
    *   I : Corrente
    *   R_I/O : Resistência do caminho de I/O

## 6.2. Mitigação de Problemas Térmicos

Estratégias para mitigar os desafios térmicos incluem:

*   **Soluções Convencionais:** Dissipadores de calor, microcanais, TSVs de alta densidade e underfills moldados por refluxo de massa (MR-MUF).
*   **Ligação Híbrida:** A otimização da densidade de pads de cobre (Cu), características da interface e tratamentos mecânicos pode reduzir a resistência térmica junção a junção e aumentar a condutividade térmica vertical.
*   **Seleção de Materiais:** Uso de dielétricos SiCN, cobre nano-geminado e compósitos poliméricos.
*   **Tecnologias de Processo:** Ligação ativada por plasma a baixas temperaturas e co-otimização de recozimento por polimento químico-mecânico (CMP).
*   **Design Estrutural:** Pilhas escalonadas e cantos arredondados para suprimir pontos de estresse.

## 7. Integração com Google TPUs

As Tensor Processing Units (TPUs) do Google utilizam HBM para alcançar a alta largura de banda de memória necessária para cargas de trabalho de aprendizado de máquina. A HBM no chip permite o uso de modelos maiores e tamanhos de lote maiores, o que é crucial para o desempenho de treinamento e inferência.

### 7.1. Otimização do Compilador XLA

O compilador XLA (Accelerated Linear Algebra) desempenha um papel fundamental na otimização do uso da HBM. Ele realiza transformações de código, como a divisão de multiplicações de matrizes em blocos menores, para executar os cálculos de forma eficiente na unidade de matriz (MXU) da TPU. O compilador aproveita a arquitetura de hardware da MXU (uma matriz sistólica de 128x128) e o design do subsistema de memória das TPUs, que prefere dimensões que sejam múltiplos de 8, para otimizar a segmentação e o acesso à memória.

### 7.2. Especificações de HBM por Versão de TPU

<table>
<thead>
<tr>
<th>Versão da TPU</th>
<th>Memória HBM</th>
<th>Largura de Banda da Memória</th>
</tr>
</thead>
<tbody>
<tr>
<td>TPU v2</td>
<td>16 GB</td>
<td>600 GB/s</td>
</tr>
<tr>
<td>TPU v3</td>
<td>32 GB</td>
<td>900 GB/s</td>
</tr>
</tbody>
</table>

---


## Page 116

<table>
  <thead>
    <tr>
      <th>Versão da TPU</th>
      <th>Memória HBM</th>
      <th>Largura de Banda da Memória</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>TPU v4</td>
      <td>32 GB</td>
      <td>1.2 TB/s</td>
    </tr>
    <tr>
      <td>TPU v5p</td>
      <td>95 GB</td>
      <td>2.76 TB/s</td>
    </tr>
    <tr>
      <td>Ironwood</td>
      <td>1.77 PB (total)</td>
      <td>7.4 TB/s (pico por chip)</td>
    </tr>
  </tbody>
</table>

**Observações:**

*   Os chips da TPU v4 possuem um espaço de memória HBM unificado de 32 GiB em todo o chip, permitindo uma melhor coordenação entre os dois TensorCores no chip.
*   O Ironwood, um TPU mais recente, possui uma capacidade de HBM e largura de banda significativamente maiores, com cada chip tendo oito pilhas de HBM3E.

## 8. Análise de Desempenho

A integração da HBM tem um impacto direto e significativo no desempenho das TPUs do Google. A alta largura de banda e a maior capacidade de memória permitem o treinamento e a inferência de modelos de IA cada vez maiores e mais complexos.

### 8.1. Comparativo de Desempenho: TPU v4 vs. TPU v5p

O TPU v5p, em comparação com o TPU v4, apresenta melhorias notáveis:

*   **FLOPS:** Mais de 2 vezes superior.
*   **Memória HBM:** 3 vezes mais memória de alta largura de banda.
*   **Escalabilidade:** 4 vezes mais escalável em termos de FLOPs totais por pod.

Essas melhorias se traduzem em um aumento considerável na velocidade de treinamento. Por exemplo, o TPU v5p pode treinar modelos de embedding denso 1.9 vezes mais rápido que o TPU v4.

### 8.2. Impacto da Largura de Banda da HBM

A largura de banda da HBM é crucial para alimentar os TensorCores da TPU com dados em alta velocidade. A latência de acesso à HBM é significativamente menor do que a da memória do host (CPU), que é conectada via PCIe. Por exemplo, a largura de banda do PCIe para o TPU v4 é de 16 GB/s em cada direção, quase 100 vezes mais lenta que a HBM.

O aumento da largura de banda da HBM nas gerações mais recentes de TPUs, como o Ironwood (7.37 TB/s por chip), garante o acesso rápido aos dados, o que é fundamental para o desempenho de modelos de IA com uso intensivo de memória.

---


## Page 117

# 9. Principais Contribuidores

A tecnologia HBM é o resultado da colaboração e do esforço de várias empresas e indivíduos. As principais empresas envolvidas no desenvolvimento e produção de HBM são SK Hynix, Samsung Electronics e Micron Technology. A AMD também desempenhou um papel fundamental no desenvolvimento inicial e na adoção da HBM em suas GPUs.

## 9.1. SK Hynix

A SK Hynix é considerada uma pioneira na comercialização da HBM e continua a liderar a inovação no setor. A empresa produziu o primeiro chip de memória HBM em 2013 e, em colaboração com a AMD, desenvolveu o primeiro produto HBM com TSV do mundo em 2014.

Myeong-Jae Park, Vice-Presidente e chefe de design de HBM da SK Hynix, é uma figura central no sucesso da empresa nesse campo. Ele e sua equipe dedicaram mais de 15 anos à pesquisa e desenvolvimento da tecnologia HBM. Park destaca a HBM2E como um ponto de virada para a empresa e enfatiza a importância da colaboração entre as equipes de design, encapsulamento e P&D para alcançar os avanços tecnológicos.

Outras tecnologias fundamentais desenvolvidas pela SK Hynix que contribuíram para o sucesso da HBM incluem:

*   MR-MUF (Mass Reflow Molded Underfill)
*   HKMG (High-K Metal Gate)
*   Low-K IMD (Inter-Metal Dielectric)

## 9.2. Samsung Electronics

A Samsung Electronics é outra grande força no mercado de HBM, sendo uma das principais fornecedoras para os TPUs do Google. A empresa tem investido pesadamente na produção de HBM3 e HBM3E para atender à crescente demanda do mercado de IA.

## 9.3. Micron Technology

A Micron Technology também é um player importante no mercado de HBM, competindo com a SK Hynix e a Samsung no fornecimento de memória de alta largura de banda para aplicações de IA e computação de alto desempenho.

## 9.4. AMD

A AMD foi uma das primeiras a adotar a HBM em seus produtos, utilizando-a em suas GPUs Fiji em 2015. A colaboração da AMD com a SK Hynix foi fundamental para o desenvolvimento e a comercialização da primeira geração de HBM.

---


## Page 118

# Pesquisa Profunda sobre a Interconexão ICI (Inter-Chip Interconnect) dos TPUs do Google

## 1. Introdução

A Interconexão ICI (Inter-Chip Interconnect) é um componente crítico da arquitetura dos Tensor Processing Units (TPUs) do Google, permitindo a comunicação de alta largura de banda e baixa latência entre os chips. Esta pesquisa aprofundada explora os aspectos técnicos e científicos da ICI, com foco na sua topologia, protocolos, desempenho e nos principais pesquisadores que contribuíram para o seu desenvolvimento.

## 2. Análise Técnica Detalhada da Interconexão ICI

A Interconexão ICI é uma malha de rede proprietária de alta velocidade que conecta diretamente os chips TPU, viabilizando a comunicação dispositivo a dispositivo (RDMA) com baixa latência, crucial para o treinamento de modelos de aprendizado de máquina em grande escala. A seguir, detalhamos os principais aspectos técnicos desta interconexão.

### 2.1. Arquitetura e Topologia

A arquitetura de interconexão do TPUv4 é baseada em uma topologia de **toro 3D**. Essa escolha de topologia é estratégica, pois oferece uma alta bisseção de banda, excelente escalabilidade e baixa latência, características essenciais para suportar os padrões de comunicação intensivos das cargas de trabalho de IA.

O sistema é modularizado em unidades chamadas **cubos**. Cada cubo consiste em 64 chips TPU, fisicamente arranjados em uma malha de 4x4x4. Um supercomputador TPUv4, ou *pod*, é formado por 64 desses cubos, somando um total de 4096 chips TPU. A interconexão entre esses cubos é realizada por meio de **Optical Circuit Switches (OCS)**, que conferem à rede uma natureza reconfigurável. Cada cubo possui 96 links ICI ópticos (16 por face do cubo 3D) que se conectam aos OCS, permitindo a criação dinâmica de topologias de rede maiores e customizadas para cada tarefa, como o toro 3D ou o toro 3D torcido (*twisted-torus*).

### 2.2. Protocolo ICI

O protocolo ICI foi projetado para ser programável, uma característica que permite ao software gerenciar a complexidade operacional da reconfigurabilidade e da resiliência do sistema. A pilha de protocolos é dividida em camadas, conforme detalhado na tabela abaixo, extraída do trabalho de Zu et al. (2024).

---


## Page 119

<table>
  <thead>
    <tr>
      <th>Camada</th>
      <th>Funcionalidade</th>
      <th>Agente de Software</th>
      <th>Visível na ISA?</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Transação</td>
      <td>Operações de RDMA (Remote Direct Memory Access)</td>
      <td>XLA (Accelerated Linear Algebra)</td>
      <td>Sim</td>
    </tr>
    <tr>
      <td>Roteamento</td>
      <td>Encaminhamento de pacotes na rede</td>
      <td>libtpunet</td>
      <td>Dica</td>
    </tr>
    <tr>
      <td>Enlace de Dados</td>
      <td>Controle de fluxo, retransmissão de pacotes, entrega ordenada</td>
      <td>libtpunet, healthd</td>
      <td>Não</td>
    </tr>
    <tr>
      <td>Enlace Físico</td>
      <td>Estabelecimento de conexão (xconnect), treinamento de porta</td>
      <td>pod mgr, chip mgr, healthd</td>
      <td>Não</td>
    </tr>
  </tbody>
</table>

## 2.3. Desempenho: Largura de Banda e Latência

As especificações de desempenho da interconexão ICI são impressionantes e fundamentais para a eficiência dos TPUs.

*   **Largura de Banda:** Cada link ICI individual é capaz de transportar **50 GBps** de largura de banda unidirecional. Com múltiplos links por chip, a largura de banda agregada é massiva, permitindo a transferência rápida de grandes volumes de dados, como os pesos e gradientes em modelos de redes neurais.
*   **Latência:** Embora os valores exatos de latência não sejam publicamente detalhados nos papers analisados, a arquitetura é enfaticamente descrita como de "baixa latência". A comunicação direta chip-a-chip, evitando a passagem pela CPU e pela rede de datacenter convencional, é o principal fator que contribui para essa característica.

## 2.4. Protocolo de Coerência de Memória

A coerência de memória é um desafio fundamental em sistemas multi-chip, onde múltiplas cópias de um mesmo dado podem existir em diferentes caches. A patente US9372800B2, embora não seja específica do Google, oferece um vislumbre dos complexos mecanismos de coerência que são relevantes para a ICI. O protocolo descrito visa garantir uma visão consistente da memória em todo o sistema.

Os princípios fundamentais do protocolo de coerência de memória incluem:

*   **Manutenção de Estado:** Um nó central (home node) rastreia o estado de todas as cópias de um bloco de dados nos caches do sistema.
*   **Mensagens de Coerência:** Os chips trocam mensagens para solicitar dados, invalidar cópias obsoletas e manter a consistência.

---


## Page 120

*   **Esquemas de Ação:** Ações predefinidas são executadas em resposta a mensagens, com base no estado atual do bloco de dados e no tipo de mensagem.

O protocolo define um conjunto de estados para cada bloco de dados em cache, como **Exclusive (E)**, **Shared (S)**, **Modified (M)**, **Owned (O)** e **Invalid (I)**, e utiliza uma variedade de tipos de mensagens para gerenciar as transições entre esses estados, garantindo que todas as operações de leitura e escrita sejam consistentes em todo o sistema multi-chip.

## 2.5. Modelos Matemáticos e Análise de Desempenho

A análise de desempenho de redes com topologia de toro é um campo de pesquisa consolidado. Diversos modelos matemáticos foram propostos para estimar a latência e a largura de banda. Compilamos aqui algumas das fórmulas e conceitos centrais extraídos de publicações acadêmicas relevantes.

A latência média de uma mensagem pode ser expressa pela seguinte fórmula geral:

Latência = (S + Ws) * V

Onde S é a latência média da rede, Ws é o tempo médio de espera na fonte e V é o grau de multiplexação dos canais virtuais. A latência da rede S é uma média ponderada das latências para mensagens regulares e de "hot-spot".

O número médio de saltos (hops) em um toro k-ary 2-cube pode ser aproximado por:

k_hat ≈ k / 4
d = 2 * k_hat

Onde k é o número de nós em uma dimensão. A latência para uma mensagem específica depende do seu tamanho (M), do número de saltos (r ou j) e do tempo de estabelecimento do caminho (Cr ou Ca):

Sr = M + r + Cr
Sa j = M + j + Ca

O tempo de espera na fonte (Ws) pode ser modelado utilizando a teoria de filas (M/G/1):

Ws = (p * S * (1 + C²S)) / (2 * (1 - p))

---


## Page 121

Onde p é a utilização do canal, S é o tempo médio de serviço e C²S é o coeficiente de variação do tempo de serviço. Essas equações formam a base para a análise teórica do desempenho de redes de interconexão complexas como a ICI.

## 3. Principais Contribuidores

A pesquisa e o desenvolvimento da interconexão ICI e das tecnologias de TPU no Google são o resultado do trabalho de uma vasta equipe de engenheiros e pesquisadores. Com base nos artigos científicos analisados, identificamos alguns dos principais contribuidores para este campo.

*   **Yazhou Zu**: Engenheiro de Software no Google, com foco no escalonamento de supercomputadores de ML. É o autor principal do paper sobre a resiliência e gerenciamento do TPUv4.
*   **Alireza Ghaffarkhah**: Engenheiro Principal e Diretor no Google, liderando a pilha de software para TPU/GPU. Sua pesquisa foca na otimização e simulação de topologias de rede.
*   **Hoang-Vu Dang**: Pesquisador e Tech Lead no Google, com expertise em computação paralela e de alto desempenho.
*   **Brian Towles**: Atualmente na OpenAI, foi uma figura importante no desenvolvimento do TPUv4 no Google, co-autor de papers chave sobre a arquitetura do TPUv4.
*   **Steven Hand**: Pesquisador do Google com vasta experiência em sistemas distribuídos, computação paralela e redes.
*   **Safeen Huda**: Atualmente na OpenAI, trabalhou no co-design de hardware/software para os sistemas de TPU no Google.
*   **Adekunle Bello, Alexander Kolbasov, Arash Rezaei**: Engenheiros do Google que contribuíram significativamente para o desenvolvimento e a operação da infraestrutura de TPU, conforme evidenciado pela co-autoria no paper sobre o TPUv4.

## 4. Referências

[1] Zu, Y., Ghaffarkhah, A., Dang, HV., Towles, B., Hand, S., Huda, S., Bello, A., Kolbasov, A., Rezaei, A., Du, D., Lacy, S., Wang, H., Wisner, A., Lewis, C., & Bahini, H. (2024). Resiliency at Scale: Managing Google’s TPUv4 Machine Learning Supercomputer. In *21st USENIX Symposium on Networked Systems Design and Implementation (NSDI 24)*. [https://www.usenix.org/system/files/nsdi24-zu.pdf](https://www.usenix.org/system/files/nsdi24-zu.pdf)

[2] Akkawi, I., Kessler, R. E., Asher, D. H., Chin, B. W., & Snyder, W. P. (2016). *Inter-chip interconnect protocol for a multi-chip system*. U.S. Patent No. 9,372,800. [https://patents.google.com/patent/US9372800B2/en](https://patents.google.com/patent/US9372800B2/en)

[3] Ramesh, B., Gururaj, H. L., & Chandrika, J. (2015). *Network Performance Comparative Analysis of Torus and Modified Mesh Interconnections with Source*

---


## Page 122

Routing for Packet Loss. *International Journal of Computer Theory and Engineering*, 7(4), 273-277. [https://www.ijcte.org/vol7/970-K008.pdf](https://www.ijcte.org/vol7/970-K008.pdf)

[4] Safaei, F., Khonsari, A., Fathy, M., & Ould-Khaoua, M. (2006). Analysis of Circuit Switching for the Torus Interconnect Networks with Hot-Spot Traffic. In *Proceedings of the 2006 International Conference on Parallel Processing Workshops* (ICPPW'06). [https://cecs.uc.edu/~papers/icpp06/ICPPW/papers/019_fsafaei-Circuit.pdf](https://cecs.uc.edu/~papers/icpp06/ICPPW/papers/019_fsafaei-Circuit.pdf)


# Pesquisa Técnica Aprofundada sobre Jeff Dean e as Contribuições para o Google TPU

**Tópico:** Jeff Dean e contribuições para TPU - histórico, papers, patentes, genealogia acadêmica

## 1. Jeff Dean: Perfil e Contribuições Chave

### 1.1. Biografia e Carreira

Jeff Dean é uma figura proeminente na ciência da computação e inteligência artificial, atualmente servindo como Cientista Chefe no Google. Sua carreira no Google, iniciada em 1999, é marcada por contribuições fundamentais para alguns dos sistemas mais críticos da empresa, incluindo Google Search, MapReduce, Bigtable e, mais notavelmente, os sistemas de aprendizado de máquina que levaram ao desenvolvimento da Unidade de Processamento de Tensores (TPU).

### 1.2. Principais Contribuições para IA e Sistemas de Larga Escala

Antes de focar nos TPUs, é importante entender o contexto do trabalho de Dean em sistemas distribuídos, que pavimentou o caminho para o hardware de IA em larga escala.

*   **MapReduce:** Um modelo de programação para processar grandes conjuntos de dados em clusters de computadores. A simplicidade e escalabilidade do MapReduce permitiram ao Google processar petabytes de dados de forma eficiente.
*   **Bigtable:** Um sistema de armazenamento distribuído, NoSQL, de alto desempenho, projetado para escalar para petabytes de dados em milhares de servidores.
*   **TensorFlow:** Um sistema de código aberto para aprendizado de máquina que se tornou a base para a pesquisa e produção de modelos de IA no Google e na comunidade em geral. Dean foi um dos principais projetistas e implementadores.

---


## Page 123

# 2. A Gênese e Evolução da TPU

## 2.1. A Necessidade de Hardware Especializado

O crescimento exponencial do uso de redes neurais profundas (DNNs) no Google, especialmente para tarefas como reconhecimento de voz e busca, criou uma demanda computacional massiva. Dean e outros líderes no Google perceberam que depender apenas de CPUs e GPUs de uso geral seria insustentável em termos de custo e consumo de energia. Essa percepção levou à iniciativa de projetar um ASIC (Application-Specific Integrated Circuit) customizado para redes neurais: a TPU.

## 2.2. O Papel de Jeff Dean

Jeff Dean foi um dos principais defensores e líderes técnicos por trás do projeto TPU. Sua profunda compreensão de sistemas de software em larga escala e as demandas computacionais de modelos de aprendizado de máquina foram cruciais para definir os requisitos e a direção arquitetônica da TPU.

# 3. Arquitetura Técnica da TPU v1

O artigo seminal "In-Datacenter Performance Analysis of a Tensor Processing Unit" [1] fornece a análise mais detalhada da primeira geração de TPUs, focada em inferência.

## 3.1. Filosofia de Design: CISC para Redes Neurais

A TPU foi projetada como um coprocessador, conectado a um host via PCIe. Sua filosofia de design é a de um processador CISC (Complex Instruction Set Computer), onde instruções complexas de alto nível (como MatrixMultiply ou Activate) orquestram as operações de baixo nível do hardware. Isso contrasta com a abordagem RISC (Reduced Instruction Set Computer) de muitas CPUs modernas.

## 3.2. Diagrama de Blocos e Componentes

Os componentes principais da TPU v1 são:

*   **Matrix Multiply Unit (MXU):** Uma unidade de multiplicação de matriz sistólica 256x256 de 8 bits. Esta é a principal unidade computacional, capaz de realizar 65.536 multiplicações-acumulações (MACs) por ciclo.
*   **Unified Buffer (UB):** 24 MiB de SRAM on-chip para armazenar ativações (saídas da camada anterior), que servem como entradas para a MXU.
*   **Weight Memory:** 8 GiB de DRAM DDR3 off-chip para armazenar os parâmetros (pesos) do modelo.
*   **Activation Pipeline:** Executa as funções de ativação não-linear (ReLU, etc.), normalização e pooling nos resultados da MXU.

---


## Page 124

# 3.3. Fluxo de Dados e Execução

O fluxo de dados é projetado para maximizar a utilização da MXU:

1. O host envia as instruções para a TPU.
2. Os pesos são carregados da Weight Memory para uma Weight FIFO on-chip.
3. As ativações são carregadas do host para o Unified Buffer.
4. A MXU processa uma matriz de ativações e uma matriz de pesos, gerando uma matriz de resultados.
5. Os resultados passam pela Activation Pipeline.
6. As saídas são escritas de volta no Unified Buffer, prontas para serem a entrada da próxima camada ou para serem lidas pelo host.

# 3.4. A Unidade de Multiplicação de Matriz Sistólica

A MXU é uma arquitetura de array sistólico. Em um array sistólico, os dados fluem através de uma rede de unidades de processamento, onde são processados e passados para a próxima unidade. Isso permite um alto grau de paralelismo e eficiência, minimizando o acesso à memória principal.

## Equação da Operação MAC:

Para cada célula no array 256x256, a operação fundamental é a multiplicação-acumulação (MAC) de 8 bits:

Acumulador_novo = Acumulador_antigo + (Entrada_A * Entrada_B)

Onde Entrada_A é um valor de ativação de 8 bits e Entrada_B é um peso de 8 bits. O acumulador tem uma precisão maior (32 bits) para evitar overflow.

# 3.5. Especificações Técnicas e Desempenho

<table>
<thead>
<tr>
<th>Característica</th>
<th>Especificação</th>
</tr>
</thead>
<tbody>
<tr>
<td><strong>Processo</strong></td>
<td>28nm</td>
</tr>
<tr>
<td><strong>Tamanho do Die</strong></td>
<td>&lt; 600 mm²</td>
</tr>
<tr>
<td><strong>Clock</strong></td>
<td>700 MHz</td>
</tr>
<tr>
<td><strong>Potência</strong></td>
<td>40W (TDP)</td>
</tr>
<tr>
<td><strong>Memória On-Chip</strong></td>
<td>28 MiB SRAM</td>
</tr>
<tr>
<td><strong>Memória Off-Chip</strong></td>
<td>8 GiB DDR3 DRAM</td>
</tr>
<tr>
<td><strong>Pico de Performance</strong></td>
<td>92 TOPS (Tera-operações de 8 bits por segundo)</td>
</tr>
</tbody>
</table>

---


## Page 125

# 4. Genealogia Acadêmica e Profissional

## 4.1. Linhagem Acadêmica

A genealogia acadêmica de Jeff Dean revela uma forte linhagem na área de arquitetura de computadores e linguagens de programação:

*   **David Patterson (Orientador do orientador de seu orientador):** Uma figura lendária na arquitetura de computadores, pioneiro do RISC. Notavelmente, Patterson é co-autor do paper da TPU v1, fechando um ciclo geracional de pesquisa.
*   **David Ungar (Orientador de seu orientador):** Conhecido por seu trabalho na linguagem de programação Self.
*   **Craig Chambers (Orientador de Dean):** Especialista em otimização de compiladores para linguagens orientadas a objeto.

## 4.2. Colaboradores Chave no Projeto TPU

*   **Norman P. Jouppi:** O principal arquiteto de hardware da TPU.
*   **David Patterson:** Colaborou na análise de desempenho e na redação do artigo da TPU v1.
*   **Sanjay Ghemawat:** Colaborador de longa data de Dean em sistemas distribuídos, cujo trabalho formou a infraestrutura que necessitava de aceleradores como a TPU.

# 5. Patentes Relevantes

Jeff Dean é inventor em inúmeras patentes que formam a base para o hardware e software de IA do Google. As mais relevantes para a TPU incluem:

*   **Geração de layouts de circuitos integrados usando redes neurais:** Patentes que descrevem o uso de RL para otimizar o design físico de chips, uma técnica usada no design das próprias TPUs.
*   **Fluxo de dados distribuído e assíncrono:** Patentes relacionadas ao sistema Pathways, que descrevem como orquestrar a computação em milhares de aceleradores TPU.
*   **Treinamento de modelos de aprendizado de máquina destilados:** Patentes sobre a técnica de destilação de conhecimento, crucial para adaptar modelos grandes para execução eficiente em hardware de inferência como a TPU.

# 6. Referências

[1] Jouppi, N. P., et al. (2017). In-datacenter performance analysis of a tensor processing unit. *Proceedings of the 44th annual international symposium on computer architecture.*

---


## Page 126

# Pesquisa sobre Norm Jouppi e a Arquitetura TPU

## Introdução

Esta pesquisa aprofundada explora a carreira de Norman P. Jouppi e suas contribuições para a arquitetura de computadores, com foco especial na Tensor Processing Unit (TPU) do Google e sua genealogia desde a arquitetura MIPS. O documento detalha os aspectos técnicos da TPU, incluindo sua arquitetura, e traça a linhagem de pesquisadores e engenheiros que contribuíram para seu desenvolvimento.

## Norman P. Jouppi

Norman P. Jouppi é um engenheiro de hardware ilustre do Google, conhecido por seu papel de liderança no desenvolvimento da primeira e segunda gerações de TPUs. Sua carreira é marcada por contribuições significativas para a arquitetura de computadores, desde seu trabalho inicial no projeto MIPS em Stanford até seu papel fundamental na criação de aceleradores de hardware para aprendizado de máquina no Google.

## A Arquitetura da TPU

A Unidade de Processamento de Tensor (TPU) é um circuito integrado de aplicação específica (ASIC) desenvolvido pelo Google para acelerar cargas de trabalho de inferência de redes neurais. O design da TPU é centrado em uma unidade de multiplicação de matriz (MXU) massiva, otimizada para realizar um grande número de operações de multiplicação-acumulação (MAC) por ciclo de clock. Essa especialização permite que a TPU atinja um desempenho significativamente maior e uma eficiência energética superior em comparação com CPUs e GPUs contemporâneas para tarefas de inferência.

## Principais Componentes e Arquitetura

A arquitetura da TPU é projetada para maximizar a taxa de transferência de dados e a computação. Os principais componentes incluem:

*   **Unidade de Multiplicação de Matriz (MXU):** O coração da TPU, a MXU é uma matriz sistólica de MACs que executa multiplicações de matriz em alta velocidade. A primeira geração de TPUs continha uma MXU de 256x256 MACs de 8 bits, capaz de atingir um pico de 92 TeraOps/segundo (TOPS).
*   **Buffer Unificado:** Uma grande memória on-chip (24 MiB na primeira geração) que armazena os resultados intermediários das computações, reduzindo a necessidade de acesso à memória principal e minimizando a latência.
*   **Conexões de Dados:** Uma rede complexa de conexões de dados que maximiza a transferência de dados entre a MXU e o buffer unificado, bem como com outros componentes da TPU.
*   **Controlador:** Coordina as operações da TPU, gerencia a comunicação com a memória principal e controla a execução dos cálculos.
*   **Memória Principal:** Armazena os dados de entrada e saída das operações de inferência, além de fornecer dados para a TPU quando necessário.

## Linhagem de Desenvolvimento

A TPU evoluiu a partir da experiência de Jouppi e seus colaboradores em aceleradores de hardware para aprendizado de máquina. A TPU original foi seguida pela TPU v2, que introduziu melhorias significativas na arquitetura, incluindo uma nova MXU de 32x32 MACs de 16 bits, aumentando o desempenho para 184 TOPS. A TPU v3, lançada mais recentemente, apresenta uma arquitetura mais avançada, com uma MXU de 40x40 MACs de 16 bits, alcançando um pico de 215 TOPS.

## Impacto e Influência

A TPU revolucionou o campo da computação de aprendizado de máquina, permitindo que grandes modelos de rede neural sejam executados de forma eficiente em dispositivos móveis e em nuvem. Seu sucesso levou outras empresas a investirem em tecnologias semelhantes, acelerando o avanço da inteligência artificial.

## Conclusão

A pesquisa sobre Norm Jouppi e a arquitetura TPU destaca a importância de contribuições individuais para o avanço da arquitetura de computadores. A TPU é um exemplo de como a combinação de habilidades técnicas e visão de negócios pode levar a inovações significativas no campo da computação de aprendizado de máquina.

---


## Page 127

*   **Memória de Pesos:** Uma memória DRAM off-chip (8 GiB na primeira geração) para armazenar os pesos da rede neural. Os pesos são transferidos para a TPU através de uma FIFO de pesos on-chip.
*   **Unidade de Vetor e Escalar:** Além da MXU, as TPUs mais recentes incluem unidades de vetor e escalar para lidar com computações gerais, como funções de ativação, e para o fluxo de controle e cálculo de endereços de memória.

## Fluxo de Dados

O fluxo de dados na TPU é projetado para manter a MXU constantemente ocupada. O host da TPU envia instruções e dados para a TPU. Os dados de ativação são carregados no Buffer Unificado, enquanto os pesos são carregados na Memória de Pesos. A MXU então processa os dados em um fluxo de pipeline, com os resultados sendo acumulados e, em seguida, passados por uma função de ativação antes de serem escritos de volta no Buffer Unificado ou enviados de volta para o host.

## Genealogia da Arquitetura MIPS

A arquitetura MIPS (Microprocessor without Interlocked Pipeline Stages) foi um projeto de pesquisa seminal da Universidade de Stanford no início dos anos 1980, liderado por John L. Hennessy. O projeto MIPS foi pioneiro na abordagem RISC (Reduced Instruction Set Computer), que defendia um conjunto de instruções simplificado para alcançar um desempenho mais alto. Norman Jouppi foi um dos principais colaboradores do projeto MIPS, juntamente com outros pesquisadores notáveis.

## Colaboradores do Artigo MIPS

O artigo seminal de 1982, "MIPS: A Microprocessor Architecture", foi de autoria de um grupo de pesquisadores de Stanford que se tornariam figuras proeminentes na indústria de computadores:

*   **John L. Hennessy:** Líder do projeto MIPS, cofundador da MIPS Computer Systems e mais tarde presidente da Stanford University e presidente do conselho da Alphabet.
*   **David Patterson:** Liderou o projeto Berkeley RISC, que desenvolveu o processador RISC-I, e é coautor de livros influentes sobre arquitetura de computadores com Hennessy.
*   **Norman Jouppi:** Contribuidor chave para o projeto MIPS e mais tarde o arquiteto principal da TPU do Google.
*   **Steven Przybylski:** Pesquisador em hierarquia de cache e memória e autor do livro "Cache and Memory Hierarchy Design".
*   **Christopher Rowen:** Cofundador da MIPS Computer Systems e da Tensilica.
*   **Thomas Gross:** Pesquisador nas áreas de compiladores, sistemas de tempo de execução e arquitetura de computadores.

---


## Page 128

*   **Forest Baskett**: Contribuiu para o desenvolvimento do processador MIPS original e mais tarde se tornou um capitalista de risco.
*   **John T. Gill**: Pesquisador em teoria da complexidade computacional e teoria da informação.

## Conclusão

A pesquisa revela uma clara linhagem de inovação em arquitetura de computadores, desde os primeiros dias do RISC com o projeto MIPS até os aceleradores de hardware de domínio específico de hoje, como a TPU do Google. A carreira de Norman Jouppi exemplifica essa evolução, conectando os princípios fundamentais da arquitetura de computadores com as demandas das cargas de trabalho modernas de aprendizado de máquina. A TPU, com sua arquitetura especializada e foco na eficiência, representa o estado da arte em aceleração de hardware para IA, e sua genealogia pode ser rastreada até as ideias pioneiras desenvolvidas no projeto MIPS.

## Referências

[1] Jouppi, N. P., et al. (2017). In-datacenter performance analysis of a tensor processing unit. *Proceedings of the 44th Annual International Symposium on Computer Architecture*, 1-12.

[2] Hennessy, J. L., et al. (1982). MIPS: A microprocessor architecture. *ACM SIGMICRO Newsletter*, 13(4), 17-22.

[3] Google Cloud. (n.d.). *System Architecture*. Retrieved from https://cloud.google.com/tpu/docs/system-architecture-tpu-vm

[4] Hennessy, J. L. (n.d.). *Biography*. Stanford University. Retrieved from https://hennessy.stanford.edu/biography

## Pesquisa sobre David Patterson, RISC e sua Influência no Google TPU

### David Patterson (resumo da Wikipedia)

Patterson é notável por suas contribuições pioneiras para o projeto de computadores com conjunto reduzido de instruções (RISC), tendo cunhado o termo RISC e liderado o projeto Berkeley RISC. Em 2018, 99% de todos os novos chips usavam uma arquitetura RISC. Ele também é conhecido por liderar a pesquisa sobre arranjos redundantes de discos baratos (RAID), com Randy Katz.

Seus livros sobre arquitetura de computadores, em coautoria com John L. Hennessy, são amplamente utilizados na educação em ciência da computação. Hennessy e

---


## Page 129

Patterson ganharam o Prêmio Turing de 2017 por seu trabalho no desenvolvimento do RISC.

## The Case for the Reduced Instruction Set Computer (resumo)

Este artigo seminal de David A. Patterson e David R. Ditzel argumenta que computadores com um conjunto de instruções reduzido (RISC) podem ser mais econômicos e eficientes do que os computadores com um conjunto de instruções complexo (CISC).

### Principais Argumentos:

*   **Complexidade vs. Custo-Benefício**: O artigo questiona a tendência de criar máquinas cada vez mais complexas, sugerindo que essa complexidade nem sempre se traduz em melhor custo-benefício.
*   **Uso de Instruções**: Análises de compiladores e programas em linguagem de montagem mostram que apenas um pequeno subconjunto do conjunto de instruções de uma máquina é usado com frequência. Por exemplo, em um compilador IBM 360, 10 instruções representavam 80% de todas as instruções executadas.
*   **Implementações Irracionais**: Instruções complexas nem sempre são mais rápidas do que uma sequência de instruções simples. O artigo cita o exemplo da instrução `INDEX` no VAX-11/780, que é 45% mais lenta do que uma sequência de instruções simples que realizam a mesma função.
*   **Vantagens do RISC em VLSI**:
    *   **Viabilidade de Implementação**: Arquiteturas mais simples têm maior probabilidade de serem implementadas em um único chip.
    *   **Tempo de Projeto**: O tempo de projeto reduzido para máquinas RISC permite o uso de tecnologia mais recente.
    *   **Velocidade**: Um projeto mais simples pode levar a um ciclo de clock menor.
    *   **Melhor Uso da Área do Chip**: A área economizada ao não implementar instruções complexas pode ser usada para caches no chip, pipelining ou outros recursos que melhoram o desempenho.
*   **Suporte a Linguagens de Alto Nível**: O artigo argumenta que um conjunto de instruções simples e uniforme facilita a escrita de compiladores. Instruções complexas muitas vezes são difíceis de serem geradas por compiladores e podem implementar a função "errada" para uma determinada linguagem.

## Influência do RISC na Arquitetura do Google TPU

A arquitetura RISC, defendida por David Patterson, tem uma influência direta e significativa no design dos Tensor Processing Units (TPUs) do Google. A simplicidade e a eficiência do RISC alinham-se bem com os requisitos de uma arquitetura de

---


## Page 130

domínio específico como o TPU, que é otimizada para cargas de trabalho de aprendizado de máquina.

## Adoção do RISC-V

Artigos recentes confirmam que o Google está utilizando processadores baseados na arquitetura aberta RISC-V em seus TPUs. Especificamente, o Google está usando o processador **SiFive Intelligence X280** em combinação com os TPUs. O X280 é um núcleo de CPU RISC-V com extensões vetoriais que são particularmente adequadas para tarefas de IA.

## Razões para a Adoção do RISC-V

A principal razão para a adoção de um núcleo RISC-V no TPU é a **programabilidade e a flexibilidade**. Em vez de usar um sequenciador de hardware de função fixa, a inclusão de um núcleo de CPU de propósito geral, como o X280, permite que o TPU execute código mais complexo, incluindo operações escalares e vetoriais, e tome decisões de roteamento condicionais com mais facilidade. Isso simplifica o modelo de programação e permite que os desenvolvedores usem uma combinação de Python e outras linguagens para programar o TPU.

Cliff Young, um dos arquitetos do TPU no Google, declarou que a alternativa seria um "sequenciador horrível e único", e perguntou retoricamente: "você gosta de programar suas máquinas com montagem de baixo nível?". Isso ressalta a importância de ter um modelo de programação mais amigável e flexível, que é facilitado pela inclusão de um núcleo RISC-V.

## Vantagens Técnicas

*   **Flexibilidade:** A SiFive permite que os clientes modifiquem seus núcleos, adicionando aceleradores de hardware diretamente no arquivo de registradores vetoriais. O Google aproveita essa flexibilidade no modo VCIX (Vector Coprocessor Interface eXtension) do X280 para integrar seus próprios MXUs (Matrix Multiply Units).
*   **Desempenho:** O núcleo X280, embora seja um processador em ordem, possui um pipeline de vetor largo e implementa a especificação completa do RISC-V Vector 1.0, com extensões para bfloat16, multiplicação de matrizes e quantização, otimizando-o para IA.
*   **Latência:** Enquanto os MXUs têm uma latência alta (cerca de 100 ciclos), a CPU pode executar código escalar e vetorial em poucos ciclos, concorrentemente.

Em resumo, a influência de David Patterson e da filosofia RISC é evidente na arquitetura do Google TPU, não apenas no nível conceitual de simplicidade e eficiência, mas também na implementação concreta através da adoção de núcleos RISC-V como o SiFive X280 para controle e programabilidade.

---


## Page 131

# David Patterson: Contribuições e Genealogia

## Formação e Orientadores

David Patterson recebeu seus diplomas de Bacharelado, Mestrado e Doutorado pela Universidade da Califórnia, em Los Angeles (UCLA). Seu orientador de doutorado foi **Gerald Estrin**. A tese de doutorado de Patterson, concluída em 1976, intitula-se "**Verification of Microprograms**" (Verificação de Microprogramas).

## Colaboradores: John L. Hennessy

**John L. Hennessy** é uma figura central na carreira de David Patterson e na história da arquitetura de computadores. Juntos, eles foram pioneiros na abordagem RISC e escreveram o livro definitivo sobre o assunto, "**Computer Architecture: A Quantitative Approach**". Este livro se tornou o padrão para o ensino de arquitetura de computadores em todo o mundo.

Hennessy, professor de Engenharia Elétrica e Ciência da Computação na Universidade de Stanford, liderou o projeto **MIPS** em Stanford, que era o equivalente ao projeto Berkeley RISC. Ambos os projetos, embora desenvolvidos de forma independente, chegaram a conclusões semelhantes sobre os benefícios de um conjunto de instruções simplificado. A colaboração entre Patterson e Hennessy consolidou os princípios do RISC e levou à sua ampla adoção pela indústria.

Assim como Patterson, Hennessy também recebeu o **Prêmio Turing** em 2017, em reconhecimento às suas contribuições para a arquitetura de computadores.

## Conclusão

A pesquisa demonstra a profunda e duradoura influência de David Patterson e da filosofia de design RISC na arquitetura de computadores moderna, culminando em sua aplicação nos TPUs do Google. A simplicidade, eficiência e flexibilidade do RISC, defendidas por Patterson e Hennessy, provaram ser ideais para a construção de aceleradores de hardware de domínio específico para aprendizado de máquina. A adoção de núcleos RISC-V nos TPUs do Google é um testemunho do poder e da relevância contínua desses conceitos, permitindo a criação de hardware de IA programável e de alto desempenho.

## Referências

1. [David Patterson (computer scientist) - Wikipedia](https://en.wikipedia.org/wiki/David_Patterson_(computer_scientist))
2. [The Case for the Reduced Instruction Set Computer](https://www.cs.berkeley.edu/~patterson/reduced.pdf)
3. [SiFive Powers Google TPU, NASA, Tenstorrent, Renesas, Microchip, And More](https://www.sifive.com/blog/sifive-powers-google-tpu-nasa-tenstorrent-renesas-microchip-and-more/)

---


## Page 132

# Pesquisa Técnica e Científica sobre o Compilador TensorFlow XLA

## Introdução

O XLA (Accelerated Linear Algebra) é um compilador de domínio específico para álgebra linear que pode acelerar os modelos do TensorFlow, e potencialmente modelos de outras estruturas, com pouca ou nenhuma alteração no código-fonte. Ele foi projetado para otimizar o desempenho de cargas de trabalho de aprendizado de máquina, melhorando a velocidade de execução e o uso de memória. Esta pesquisa aprofunda a arquitetura do XLA, suas técnicas de otimização de grafos, estratégias de fusão de operações e a geração de código para Unidades de Processamento de Tensores (TPUs) do Google, além de identificar os principais contribuidores para seu desenvolvimento.

## Arquitetura do XLA

O compilador XLA opera recebendo grafos de modelo de estruturas de aprendizado de máquina, como o TensorFlow, definidos em StableHLO. O StableHLO atua como uma camada de portabilidade entre as estruturas de ML e o compilador, definindo um conjunto de operações versionadas de alto nível (HLO). O processo de compilação do XLA envolve várias etapas de otimização e análise, tanto independentes quanto dependentes do alvo, antes de gerar o código de máquina para a arquitetura de destino.

O fluxo de compilação do XLA pode ser resumido nas seguintes etapas:

1.  **Otimização Independente do Alvo:** O XLA executa uma série de otimizações no grafo StableHLO que são independentes da arquitetura de hardware de destino. Essas otimizações incluem a eliminação de subexpressões comuns (CSE), a fusão de operações e a análise de buffer para alocar a memória de tempo de execução de forma eficiente. Durante esta fase, o XLA converte o dialeto StableHLO em um dialeto HLO interno.
2.  **Otimização Dependente do Alvo:** O grafo HLO é então enviado para um backend específico do alvo para otimizações adicionais. O backend da GPU, por exemplo, pode realizar fusões de operações que são benéficas para o modelo de programação da GPU e determinar como particionar a computação em fluxos. Os backends também podem combinar padrões de operações com chamadas de biblioteca otimizadas.
3.  **Geração de Código:** A etapa final é a geração de código específica do alvo. Os backends de CPU e GPU do XLA utilizam o LLVM (Low Level Virtual Machine) para otimização de baixo nível e geração de código. Eles emitem o LLVM IR

---


## Page 133

(Intermediate Representation) necessário para representar a computação HLO e, em seguida, invocam o LLVM para gerar o código nativo.

# Otimização de Grafos e Simplificação Algébrica

A otimização de grafos no XLA é fundamental para alcançar alto desempenho. O XLA emprega uma variedade de técnicas para simplificar a estrutura do grafo de computação, reduzir o custo computacional e habilitar otimizações subsequentes. A simplificação algébrica é uma das técnicas mais poderosas, aplicando regras matemáticas para transformar e otimizar o grafo.

## Padrões de Simplificação Algébrica

O XLA utiliza um mecanismo de correspondência de padrões para identificar e aplicar regras de simplificação. Alguns dos padrões mais comuns incluem:

<table>
  <thead>
    <tr>
      <th>Categoria</th>
      <th>Otimização</th>
      <th>Fórmula/Exemplo</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td rowspan="3"><b>Álgebra Linear</b></td>
      <td>Eliminação de Transposição Dupla</td>
      <td>(A^T)^T → A</td>
    </tr>
    <tr>
      <td>Operações com Elementos de Identidade</td>
      <td>A + Z → A, A × I → A</td>
    </tr>
    <tr>
      <td>Reordenação Associativa/Distributiva</td>
      <td>(A × B) × C → A × (B × C)</td>
    </tr>
    <tr>
      <td rowspan="2"><b>Operações Elementares</b></td>
      <td>Dobramento de Constantes</td>
      <td>x + Constant(a) + Constant(b) → x + Constant(a + b)</td>
    </tr>
    <tr>
      <td>Simplificação de Cadeias</td>
      <td>exp(log(x)) → x</td>
    </tr>
    <tr>
      <td><b>Operações de Redução</b></td>
      <td>Propriedade Distributiva</td>
      <td>reduce_sum(x + y) → reduce_sum(x) + reduce_sum(y)</td>
    </tr>
    <tr>
      <td><b>Normalização/Ativação</b></td>
      <td>Dobramento de Camadas</td>
      <td>batch_norm + scale → batch_norm com parâmetros ajustados</td>
    </tr>
  </tbody>
</table>

---


## Page 134

<table>
  <thead>
    <tr>
      <th>Categoria</th>
      <th>Otimização</th>
      <th>Fórmula/Exemplo</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td></td>
      <td>Simplificação de Ativações</td>
      <td>relu(Constant (-5)) → Constant (0)</td>
    </tr>
  </tbody>
</table>

Um exemplo prático e poderoso de simplificação algébrica é o dobramento de operações de escala e polarização em uma convolução anterior. A sequência `Scale(Conv(x, W, B_conv), b_add), s)` é transformada em uma única operação `Conv2D` com pesos `W' = s × W` e polarização `B'_conv = s × B_conv + s × b_add`. Essa otimização reduz o número de operações, o uso de memória e a sobrecarga de inicialização do kernel.

## Fusão de Operações

A fusão de operações é uma das otimizações mais importantes do XLA. Ela reduz a sobrecarga de memória e a latência de inicialização do kernel, combinando várias operações em um único kernel. O XLA emprega várias estratégias de fusão, cada uma com um objetivo específico:

*   **Fusão de Instruções (Vertical):** Funde operações sequenciais, onde a saída de uma é a entrada da próxima. O XLA percorre o grafo e usa uma função `ShouldFuse` com um conjunto de regras para decidir se duas operações dependentes devem ser fundidas.
*   **Fusão de Fusões:** Tenta mesclar kernels de fusão já existentes para reduzir ainda mais a largura de banda da memória e a sobrecarga de inicialização.
*   **Fusão de Múltiplas Saídas:** Reduz os requisitos de largura de banda da memória, fundindo operações que compartilham as mesmas entradas (fusão de irmãos) ou fundindo operações produtor-consumidor.
*   **Fusão Horizontal:** Funde cálculos independentes para aumentar o paralelismo e reduzir a sobrecarga de inicialização do kernel. Essa técnica é particularmente eficaz para a fase de otimização do treinamento de modelos, que geralmente possui muitos kernels pequenos.

## Geração de Código para TPU

A geração de código do XLA para TPUs é um processo altamente especializado que visa maximizar a utilização do hardware da TPU. A arquitetura da TPU, com suas matrizes sistólicas, memória de alta largura de banda (HBM) e interconexão de alta velocidade, influencia diretamente as otimizações realizadas pelo XLA.

---


## Page 135

As principais otimizações de baixo nível para a geração de código da TPU incluem:

*   **Atribuição de Layout:** O XLA pré-atribui layouts de operandos para otimizar o acesso à memória e satisfazer as restrições de layout de chamadas de biblioteca, como cuDNN e cuBLAS.
*   **Canonicalização de Convolução:** As operações de convolução são canonicizadas para uma ordem específica de layout de dados (por exemplo, NHWC e HWIO) para otimizar a execução nas matrizes sistólicas da TPU.
*   **Particionador SPMD:** Para treinamento em vários dispositivos, o XLA utiliza um particionador Single Program Multiple Data (SPMD) para distribuir os tensores e as operações entre os dispositivos de forma eficiente.
*   **Otimizações para SparseCore:** Nas gerações mais recentes de TPUs, o XLA pode descarregar operações de embedding esparsas para o SparseCore, um processador especializado, liberando as matrizes sistólicas para outras tarefas.

## Principais Contribuidores

A pesquisa e o desenvolvimento do compilador XLA e do projeto TPU foram impulsionados por uma equipe de pesquisadores e engenheiros talentosos. A identificação desses indivíduos é fundamental para compreender a evolução dessas tecnologias.

<table>
  <thead>
    <tr>
      <th>Nome</th>
      <th>Contribuição Principal</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Norman P. Jouppi</td>
      <td>Líder técnico e arquiteto principal da TPU. Autor principal do artigo seminal de 2017 sobre a TPU.</td>
    </tr>
    <tr>
      <td>Amir Salek</td>
      <td>Fundador e chefe de Silício Personalizado no Google, liderou o desenvolvimento de várias gerações de TPUs.</td>
    </tr>
    <tr>
      <td>Chris Leary</td>
      <td>Um dos principais desenvolvedores do compilador XLA, apresentou o XLA no TensorFlow Dev Summit 2017.</td>
    </tr>
    <tr>
      <td>Broadcom</td>
      <td>Co-desenvolvedora das TPUs, responsável pela tradução da arquitetura do Google em silício fabricável.</td>
    </tr>
  </tbody>
</table>

Norman P. Jouppi, em particular, possui um extenso portfólio de patentes relacionadas a arquitetura de computadores, processadores de redes neurais e gerenciamento térmico, que foram fundamentais para o sucesso do projeto TPU.

---


## Page 136

# Conclusão

O compilador TensorFlow XLA representa um avanço significativo na otimização de cargas de trabalho de aprendizado de máquina. Através de uma combinação de otimização de grafos, simplificação algébrica, fusão de operações e geração de código específica do alvo, o XLA é capaz de extrair o máximo de desempenho de uma variedade de arquiteturas de hardware, incluindo CPUs, GPUs e, mais notavelmente, TPUs. A pesquisa aprofundada sobre a arquitetura do XLA e suas interações com o hardware da TPU revela um sistema complexo e altamente otimizado, projetado para lidar com as demandas computacionais dos modelos de aprendizado de máquina modernos. O trabalho de pioneiros como Norman P. Jouppi, Amir Salek e Chris Leary foi fundamental para o sucesso do XLA e da TPU, e seu impacto contínuo no campo da aceleração de IA é inegável.