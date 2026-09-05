## Page 1

# Google Axion CPU: Pesquisa Técnica Exaustiva

## Introdução

Este documento consolida toda a pesquisa técnica realizada sobre a CPU Google Axion, incluindo arquitetura, matemática, física, e genealogia dos contribuidores. A pesquisa abrangeu 30 tópicos técnicos relacionados à arquitetura ARM Neoverse V2, que é a base do processador Axion.

## 1. Arquitetura, especificações e desempenho da CPU Google Axion

### Conteúdo Técnico

A CPU Google Axion é a primeira CPU personalizada do Google baseada na arquitetura Arm, projetada especificamente para data centers. Construída sobre a plataforma Neoverse V2 da Arm, a Axion visa oferecer melhorias de desempenho e eficiência em comparação com as CPUs x86 e outras CPUs baseadas em Arm. De acordo com o Google, os processadores Axion oferecem um desempenho até 30% superior em comparação com as instâncias Arm de nuvem de uso geral mais rápidas disponíveis e um desempenho até 50% melhor e eficiência energética até 60% superior em comparação com instâncias x86 de geração comparável.

### Arquitetura e Design

O núcleo da arquitetura Axion é a plataforma **Arm Neoverse V2**. Essa base permite que os clientes que já utilizam a arquitetura Arm em outras nuvens ou em ambientes on-premises migrem suas cargas de trabalho para o Google Cloud sem a necessidade de reescrever seus aplicativos. A Axion é complementada pelo **Titanium**, um sistema de descarregamentos (offloads) baseados em silício que lida com tarefas de rede, segurança e processamento de armazenamento, liberando o processador Axion para se concentrar na carga de trabalho principal.

### Instâncias e Especificações

As CPUs Axion alimentam a família de máquinas de uso geral C4A e as máquinas N4A (em pré-lançamento) no Google Compute Engine.

<table>
  <thead>
    <tr>
      <th>Processador Axion do Google</th>
      <th>Cargas de trabalho</th>
      <th>Especificações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>C4A</td>
      <td>Servidores Web e de aplicativos, servidores de anúncios, bancos de dados, caches, análise de dados, processamento de mídia e IA/ML.</td>
      <td>Até 72 vCPUs, 576 GB de RAM, rede de 100 Gbps. Configurações de memória de 1:2, 1:4 e 1:8 (vCPU:memória).</td>
    </tr>
    <tr>
      <td>N4A (Pré-lançamento)</td>
      <td>Cargas de trabalho de escalonamento horizontal, como microserviços e pipelines de CI/CD.</td>
      <td>Até 64 vCPUs, 512 GB de RAM, rede de 50 Gbps. Configurações de memória de 1:2, 1:4 e 1:8 (vCPU:memória).</td>
    </tr>
  </tbody>
</table>

### Desempenho em IA e HPC

Para cargas de trabalho de IA, os processadores Axion demonstram ganhos de desempenho significativos. Em benchmarks de inferência para o modelo de recomendação DLRMv2 do MLPerf, a Axion apresentou um desempenho de precisão total até três vezes superior ao de alternativas x86. Em aplicações de Geração Aumentada por Recuperação (RAG), os testes mostraram um desempenho até 2,5 vezes maior. Para computação de alto desempenho (HPC), as VMs C4A combinam o desempenho dos núcleos Neoverse com uma ampla largura de banda de memória por vCPU, sendo adequadas para aplicações como simulações de impacto.

---


## Page 2

# Contribuidores Principais

Bhumik Patel (Diretor de Desenvolvimento de Ecossistema de Servidores, Arm); Google; ARM Holdings

# Fórmulas e Equações

Nenhuma fórmula matemática ou equação específica foi encontrada nas fontes pesquisadas.

# Fontes

https://cloud.google.com/products/axion?hl=pt-BR;https://www.adrenaline.com.br/google/google-anuncia-axion-processador-arm-para-data-centers/;https://newsroom.arm.com/blog/google-cloud-axion-arm-ai-performance

---

# 2. Google Axion ARM Neoverse V2 core design

## Conteúdo Técnico

O núcleo ARM Neoverse V2, base da CPU Google Axion, é uma arquitetura Armv9-A de alto desempenho para data centers. Suas principais características incluem caches L1 de instrução e dados de 64KB (4-way set associative), cache L2 de até 2MB por núcleo, e escalabilidade para até 256 núcleos com a malha CMN-700 e 512MB de cache de nível de sistema (SLC). A conectividade é garantida por tecnologias como AMBA CHI C2C, UCIe, CXL e PCle. A microarquitetura otimiza o desempenho através de um pipeline de previsão de desvio desacoplado da busca, execução fora de ordem facilitada por renomeação de registradores, e prefetchers avançados. O Google Axion implementa este núcleo e, segundo o Google, oferece até 50% mais desempenho e 60% mais eficiência energética que alternativas x86. A CPU é suportada pela tecnologia Titanium, um sistema de microcontroladores que descarrega tarefas de rede, segurança e armazenamento (Hyperdisk).

## Contribuidores Principais

Amin Vahdat (Google), Rene Haas (Arm), Magnus Bruce (Arm)

## Fórmulas e Equações

Nenhuma fórmula matemática ou equação específica foi encontrada durante a pesquisa.

## Fontes

https://www.arm.com/products/silicon-ip-cpu/neoverse/neoverse-v2;https://hc2023.hotchips.org/assets/program/conference/day1/CPU1/HC2023.Arm.MagnusBruce.v04.FINAL.pdf;https://www.scs.googles-new-arm-based-cpu;https://www.phoronix.com/review/google-axion-c4a

---

# 3. ARM Neoverse V2 microarchitecture technical details

## Conteúdo Técnico

## Relatório Técnico: Microarquitetura ARM Neoverse V2

## 1. Visão Geral da Microarquitetura

A microarquitetura ARM Neoverse V2, codinome “Demeter”, é um projeto de alto desempenho da ARM focado em computação em nuvem e infraestrutura. Sucessor do Neoverse V1, o V2 visa fornecer melhorias significativas em desempenho por watt e

---


## Page 3

desempenho single-thread, mantendo a eficiência energética.

## 1.1. Componentes do Núcleo

O núcleo Neoverse V2 é composto por vários componentes principais que trabalham em conjunto para fornecer alto desempenho. A tabela a seguir resume as principais características de cada componente:

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Especificações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>L1 Instruction Cache</td>
      <td>64KB, 4-way set associative, 64-byte cache lines.</td>
    </tr>
    <tr>
      <td>L1 Instruction TLB</td>
      <td>Totalmente associativo, suporta tamanhos de página de 4KB, 16KB, 64KB e 2MB.</td>
    </tr>
    <tr>
      <td>L0 Macro-OP (MOP) Cache</td>
      <td>1536 entradas, 4-way skewed associative. Contém instruções decodificadas e otimizadas.</td>
    </tr>
    <tr>
      <td>Branch Predictor</td>
      <td>Preditor de desvio dinâmico.</td>
    </tr>
    <tr>
      <td>Instruction Decode</td>
      <td>Decodifica instruções AArch64.</td>
    </tr>
    <tr>
      <td>Register Rename</td>
      <td>Facilita a execução fora de ordem.</td>
    </tr>
    <tr>
      <td>Instruction Issue</td>
      <td>Controla o despacho para os pipelines de execução.</td>
    </tr>
    <tr>
      <td>Integer Execute</td>
      <td>Realiza operações aritméticas e lógicas de processamento de dados.</td>
    </tr>
    <tr>
      <td>Vector Execute</td>
      <td>Realiza operações Advanced SIMD, FPU, SVE e SVE2. Extensão criptográfica opcional.</td>
    </tr>
    <tr>
      <td>L1 Data Cache</td>
      <td>64KB, 4-way set associative, 64-byte cache lines.</td>
    </tr>
    <tr>
      <td>L1 Data TLB</td>
      <td>Totalmente associativo, suporta tamanhos de página de 4KB, 16KB, 64KB e tamanhos de bloco de 2MB e 512MB.</td>
    </tr>
    <tr>
      <td>L2 Cache</td>
      <td>Privado para o núcleo, 8-way set associative, configurável para 1MB ou 2MB.</td>
    </tr>
    <tr>
      <td>CPU Bridge</td>
      <td>Ponte assíncrona para a DSU-110.</td>
    </tr>
  </tbody>
</table>

## 1.2. Frontend

O frontend do Neoverse V2 foi significativamente aprimorado para aumentar a largura de banda de busca e decodificação de instruções. As principais melhorias incluem:

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Especificações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>BTB (Branch Target Buffer)</td>
      <td>Capacidade massiva de 12K entradas.</td>
    </tr>
    <tr>
      <td>TAGE Predictor</td>
      <td>8 tabelas para alta precisão na predição de desvios condicionais.</td>
    </tr>
    <tr>
      <td>iTLB</td>
      <td>Largura de banda dobrada, permitindo duas pesquisas por ciclo.</td>
    </tr>
    <tr>
      <td>Micro-op Cache</td>
      <td>Largura de banda aumentada, mas capacidade reduzida. Otimizado para capturar os loops mais quentes.</td>
    </tr>
    <tr>
      <td>Decoder</td>
      <td>Largura aumentada de 5 para 6-wide.</td>
    </tr>
  </tbody>
</table>

## 1.3. Backend

O backend foi projetado para lidar com a maior largura de banda do frontend, com mais unidades de execução e filas maiores:

---


## Page 4

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Especificações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>ALU Pipes</td>
      <td>6 pipes para operações inteiras simples.</td>
    </tr>
    <tr>
      <td>Vector and FP Execution</td>
      <td>Dois schedulers dual-ported com 28 entradas cada.</td>
    </tr>
    <tr>
      <td>AGU (Address Generation Unit) Pipes</td>
      <td>3 pipes, cada um com uma fila de agendamento de 16 entradas.</td>
    </tr>
    <tr>
      <td>Store Queue</td>
      <td>Aumentado para 80 entradas.</td>
    </tr>
    <tr>
      <td>DTLB</td>
      <td>48 entradas.</td>
    </tr>
    <tr>
      <td>L2 TLB</td>
      <td>Tamanho não especificado (Neoverse V1 tinha 2048 entradas).</td>
    </tr>
  </tbody>
</table>

## 1.4. Subsistema de Memória

O subsistema de memória foi otimizado para baixa latência e alta largura de banda:

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Especificações</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>L2 Cache</td>
      <td>2MB, 8-way set associative, 10 ciclos de latência de carga para uso.</td>
    </tr>
    <tr>
      <td>Transaction Queue</td>
      <td>96 entradas para L2 misses.</td>
    </tr>
    <tr>
      <td>CMN-700 Mesh Interconnect</td>
      <td>Interconexão de malha para ligar os núcleos V2.</td>
    </tr>
    <tr>
      <td>Prefetchers</td>
      <td>Prefetchers de hardware nos níveis de cache L1 e L2, incluindo um prefetcher de indireção de amostragem.</td>
    </tr>
  </tbody>
</table>

## 2. Contribuidores Principais

Com base na apresentação da Hot Chips 2023, um contribuidor principal para a plataforma Arm Neoverse V2 é:

*   **Magnus Bruce**, Arquiteto Líder de CPU e Fellow da Arm.

Outro contribuidor notável, autor de análises detalhadas sobre a microarquitetura, é:

*   **Chester Lam**, autor do artigo “Arm’s Neoverse V2, in AWS’s Graviton 4” no site Chips and Cheese.

### Contribuidores Principais

Magnus Bruce (Lead CPU Architect and Fellow, Arm); Chester Lam (Chips and Cheese)

### Fórmulas e Equações

### Fontes

https://www.scs.stanford.edu/~zyedidia/docs/arm/neoverse_v2_trm.pdf;https://chipsandcheese.com/p/hot-chips-2023-arms-neoverse-v2;https://documentation-service.arm.com/static/668bc0a369e89f01e39c4668?token=;https://hc2023.hotchips.org/assets/program/conference/day1/CPU1/HC2023.Arm.MagnusBruce.v04.FINAL.pdf;https://chip-neoverse-v2-in-awss-graviton-4

## 4. Estágios de pipeline e unidades de execução do ARM Neoverse V2

### Conteúdo Técnico

A microarquitetura do núcleo ARM Neoverse V2 é projetada para alto desempenho, com uma janela de execução fora de ordem (Out-of-Order - OoO) de mais de 320 instruções, um despacho de 8 vias e uma retirada de 8 vias. O front-end do pipeline é de 6 ou 8 vias, dependendo do caminho de decodificação.

---


## Page 5

# Estágios do Pipeline:

1.  **Predição de Ramificação e Busca (Fetch):** O Neoverse V2 utiliza um pipeline de predição/busca desacoplado, onde a predição de ramificação pode ocorrer à frente da busca de instruções para evitar bolhas no pipeline e tolerar perdas no cache de instruções. Ele pode prever duas ramificações por ciclo. O preditor de ramificação também atua como um pré-buscador para o cache de instruções L1 (ICache). A arquitetura inclui um Buffer de Alvo de Ramificação (Branch Target Buffer - BTB) de dois níveis e um preditor de direção TAGE (Tagged Geometric History Length) de 8 tabelas. O ICache L1 tem 64kB e é associativo por conjunto de 4 vias.

2.  **Decodificação (Decode):** As instruções são decodificadas em Macro-OPerations (MOPs) internas. O Neoverse V2 aumentou as pistas do decodificador de 5 para 6 e a Fila de Decodificação de 16 para 24 entradas. Um MOP pode ser dividido em duas Micro-OPerations (μOPs) mais adiante no pipeline. Há também um cache de MOPs para instruções já decodificadas, melhorando a eficiência.

3.  **Renomeação e Despacho (Rename/Dispatch):** A unidade de renomeação de registradores aloca registradores físicos para facilitar a execução fora de ordem e despacha as MOPs decodificadas para várias filas de emissão.

4.  **Emissão (Issue):** As μOPs aguardam seus operandos nas Filas de Emissão (Issue Queues - IQs) e são emitidas fora de ordem para as unidades de execução. O Neoverse V2 possui múltiplas IQs independentes e realiza uma leitura tardia do arquivo de registrador físico, o que significa que os dados não são armazenados nas IQs. As IQs foram aumentadas em comparação com a geração anterior (SX/MX: de 20 para 22 entradas; VX: de 20 para 28 entradas).

5.  **Execução (Execute):** As instruções são executadas nas várias unidades de execução. O Neoverse V2 possui um total de dezessete pipelines de emissão, cada um capaz de aceitar uma μOP por ciclo.

6.  **Retirada (Retire):** Os resultados são escritos de volta no arquivo de registrador arquitetural em ordem, com uma largura de banda de retirada de 8 vias.

# Unidades de Execução:

O Neoverse V2 possui um conjunto diversificado de unidades de execução para lidar com diferentes tipos de instruções:

*   **Unidades de Inteiro:**
    *   6 Unidades de Lógica e Aritmética (ALUs) para operações de inteiros, um aumento em relação à geração anterior.
    *   4 dessas são de ciclo único.
    *   2 são capazes de executar operações de ciclo único ou múltiplo (como multiplicação e divisão).
*   **Unidades de Ramificação (Branch):**
    *   2 unidades dedicadas para lidar com o fluxo de controle e ramificações.
*   **Unidades de Ponto Flutuante e SIMD (FP/ASIMD):**
    *   4 pipelines de 128 bits para operações de Ponto Flutuante, Advanced SIMD (NEON), Scalable Vector Extension (SVE) e SVE2. Essas unidades formam um caminho de dados quádruplo de 128 bits e baixa latência.
*   **Unidades de Memória (Load/Store):**
    *   2 unidades de Load/Store.
    *   1 unidade adicional de Load.
    *   2 unidades de Store Data.
    *   A arquitetura suporta encaminhamento de Store para Load (Store-to-Load forwarding) na latência de acerto do cache L1.

# Contribuidores Principais

Magnus Bruce (Lead CPU Architect and Fellow, Arm); Arm Holdings

# Fórmulas e Equações

Nenhuma fórmula ou equação matemática específica foi encontrada durante a pesquisa.

---


## Page 6

# Fontes

https://www.scs.stanford.edu/~zyedidia/docs/arm/neoverse_v2_trm.pdf;https://documentation-service.arm.com/static/668bc0a369e89f01e39c4668?token=;https://hc2023.hotchips.org/assets/program/conference/day1/CPU1/HC2023.Arm.MagnusBruce.v04.FINAL.pdf

---

# 5. Extensões da arquitetura do conjunto de instruções (ISA) ARM v9

## Conteúdo Técnico

A arquitetura do conjunto de instruções (ISA) ARMv9 é a sucessora da ARMv8 e introduz várias extensões para melhorar o desempenho, a segurança e a eficiência, especialmente em cargas de trabalho de IA e aprendizado de máquina. As principais extensões incluem:

**Scalable Vector Extension 2 (SVE2):** O SVE2 é uma extensão do SVE que amplia as capacidades de processamento de vetores. Ele adiciona novas instruções para manipulação de dados e modos de endereçamento de carga/armazenamento. O SVE2 foi projetado para ser agnóstico em relação ao comprimento do vetor, permitindo que o mesmo código seja executado em diferentes implementações de hardware com diferentes larguras de vetor. Os principais recursos incluem:
* Suporte para uma gama mais ampla de tipos de dados e operações.
* Aprimoramentos para processamento de sinal digital (DSP) e cargas de trabalho de multimídia.
* Novas instruções para operações bit a bit, manipulação de strings e operações criptográficas.

**Transactional Memory Extension (TME):** A TME introduz suporte de hardware para memória transacional, permitindo que um grupo de instruções seja executado atomicamente. Isso simplifica a programação paralela, fornecendo uma alternativa aos bloqueios tradicionais. As principais instruções são:
* TSTART : Inicia uma transação.
* TCOMMIT : Confirma uma transação.
* TCANCEL : Cancela uma transação.
* TTEST : Testa se está em uma transação. As transações podem falhar por vários motivos, como conflitos de memória, e a arquitetura define um mecanismo para relatar a causa da falha.

**Embedded Trace Extension (ETE):** A ETE fornece recursos de rastreamento detalhados para depuração e otimização de software. Ela pode capturar o fluxo de execução do programa, incluindo ramificações e exceções, e armazenar essas informações em um buffer de rastreamento. A ETE é uma evolução da Embedded Trace Macrocell (ETM).

**Branch Record Buffer Extension (BRBE):** A BRBE permite o registro de informações de ramificação em um buffer na memória. Isso pode ser usado para análise de desempenho e depuração, fornecendo um registro de ramificações recentes tomadas pelo processador.

**Trace Buffer Extension (TRBE):** A TRBE permite que uma Unidade de Buffer de Rastreamento (Trace Buffer Unit) dentro de um elemento de processamento (PE) grave o rastreamento de fluxo do programa diretamente na memória, em vez de roteá-lo para uma malha de rastreamento.

**Memory Partitioning and Monitoring (MPAM):** O MPAM fornece mecanismos para particionar e monitorar recursos do sistema de memória, como cache e largura de banda de memória. Isso permite um melhor controle sobre o desempenho e a previsibilidade em sistemas com várias cargas de trabalho concorrentes. O MPAMv2, introduzido com as extensões de 2025, oferece maior flexibilidade e suporte aprimorado para virtualização.

## Contribuidores Principais

Arm Limited; Martin Weidmann (Arm)

---


## Page 7

# Fórmulas e Equações

Tamanho do grânulo de reserva transacional: 2^a bytes, onde ‘a’ é um valor definido pela implementação na faixa de 4 a 512 palavras.

# Fontes

https://developer.arm.com/community/arm-community-blogs/b/architectures-and-processors-blog/posts/arm-a-profile-architecture-developments-2025;http://kib.kiev.ua/x86docs/ARM/ARMARMv8/DDI0608A_a_armv9_supp.pdf;http://kib.kiev.ua/x86docs/ARM/SVE/DDI0584B_a

---

# 6. ARM Scalable Vector Extension (SVE) e SVE2 Mathematics

## Conteúdo Técnico

A Extensão Vetorial Escalável (SVE) da ARM é uma extensão do conjunto de instruções A64 da arquitetura Armv8-A. A SVE2, por sua vez, baseia-se na SVE e a expande com funcionalidades adicionais. Diferentemente de outras arquiteturas SIMD, a SVE e a SVE2 não definem um tamanho fixo para os registradores vetoriais. Em vez disso, estabelecem uma faixa de valores possíveis, de um mínimo de 128 bits até um máximo de 2048 bits, em unidades de 128 bits. Essa flexibilidade permite que os fabricantes de CPUs escolham o tamanho do registrador vetorial que melhor se adapte às cargas de trabalho para as quais a CPU se destina. O design da SVE e da SVE2 garante que o mesmo programa possa ser executado em diferentes implementações do conjunto de instruções sem a necessidade de recompilação.

A SVE introduz 32 registradores vetoriais escaláveis (Z0-Z31), 16 registradores de predicado (P0-P15) e um registrador de predicado de primeira falha (FFR). A SVE2 expande a SVE com um conjunto mais rico de instruções, muitas das quais são análogas às instruções NEON, mas em um formato agnóstico ao comprimento do vetor. Isso inclui operações para processamento de sinais digitais (DSP), como multiplicação-acumulação, e operações aritméticas complexas.

As instruções SVE e SVE2 podem ser agrupadas em várias categorias funcionais, incluindo:

*   **Movimentação de Dados:** Instruções para carregar, armazenar e pré-buscar dados da memória para os registradores vetoriais.
*   **Operações com Inteiros:** Instruções para realizar operações aritméticas e lógicas em números inteiros.
*   **Operações de Ponto Flutuante:** Instruções para realizar operações aritméticas em números de ponto flutuante.
*   **Operações de Predicado:** Instruções para manipular os registradores de predicado, que controlam a execução de outras instruções em uma base por via.
*   **Operações de Redução:** Instruções que realizam uma operação em todos os elementos de um vetor e produzem um resultado escalar.

As instruções SVE2 adicionam suporte para uma gama mais ampla de tipos de dados e operações, incluindo aritmética de inteiros de grande porte, aritmética de polinômios e operações criptográficas.

## Contribuidores Principais

ARM Holdings

## Fórmulas e Equações

Embora fórmulas matemáticas complexas não sejam explicitamente detalhadas nos manuais de referência, a própria arquitetura SVE/SVE2 é um conjunto de instruções para computação paralela, e suas instruções representam operações matemáticas. Algumas das operações matemáticas mais comuns e suas representações em SVE/SVE2 são:

*   **Adição de Vetores (predicada):** ADD <Zdn>.<T>, <Pg>/M, <Zdn>.<T>, <Zm>.<T>
    *   Descrição: Adiciona os elementos de dois vetores ( Zdn e Zm ) sob o controle de um predicado ( Pg ), armazenando o resultado em Zdn .

---


## Page 8

*   **Adição de Vetores (não predicada):** ADD `<Zd>`.<T>, `<Zn>`.<T>, `<Zm>`.<T>
    *   Descrição: Adiciona os elementos de dois vetores (`Zn` e `Zm`), armazenando o resultado em `Zd`.
*   **Adição com Imediato:** ADD `<Zdn>`.<T>, `<Zdn>`.<T>, #<imm>{, <shift>}
    *   Descrição: Adiciona um valor imediato a cada elemento de um vetor.
*   **Adição Longa sem Sinal (parte inferior):** UADDLB `<Zda>`.<Tb>, `<Zn>`.<Ta>, `<Zm>`.<Ta>
    *   Descrição: Adiciona os elementos de numeração par de dois vetores de origem, ampliando o resultado para o dobro da largura e colocando-o no vetor de destino.
*   **Multiplicação-Adição Complexa com Rotação:** CMLA `<Zda>`.<T>, `<Zn>`.<T>, `<Zm>`.<T>, `<const>`
    *   Descrição: Realiza uma multiplicação e adição complexa com rotação, útil para transformadas de Fourier e outras aplicações de processamento de sinal.
*   **Adição de Ponto Flutuante Pareada:** FADDP `<Vd>`.<T>, `<Vn>`.<T>, `<Vm>`.<T>
    *   Descrição: Adiciona pares de elementos adjacentes de dois vetores de origem e armazena os resultados no vetor de destino.

## Fontes

https://developer.arm.com/Architectures/Scalable%20Vector%20Extensions;http://kib.kiev.ua/x86docs/ARM/SVE/DDI0584B_a_SVto-sve2.pdf?revision=029f85a0-dfdc-4aab-bfff-9b7592aa51cf

---

# 7. Pesquisa Técnica sobre a Arquitetura de Segurança ARM Confidential Compute (CCA)

## Conteúdo Técnico

A Arquitetura de Computação Confidencial da Arm (CCA), uma característica de segurança da arquitetura Armv9-A, foi projetada para proteger os dados em uso contra softwares privilegiados, como o hipervisor. A CCA introduz “Realms”, que são ambientes de execução isolados para proteger o código e os dados, mesmo do sistema operacional. Embora baseada na tecnologia TrustZone da Arm, a CCA oferece um modelo de confiança mais robusto, no qual a carga de trabalho em um Realm não precisa confiar no sistema operacional ou no hipervisor que a gerencia.

A CCA introduz um novo estado de segurança, o “Realm world”, que se soma aos já existentes “Normal world” e “Secure world”. A transição entre esses mundos é gerenciada pelo “TF-A Monitor” em um quarto estado de execução, o “Root world”. O “Realm Management Monitor” (RMM) é o software de controle no “Realm world” que gerencia as solicitações do hipervisor no “Normal world” para a execução de VMs do Realm. O RMM é responsável pela comunicação e troca de contexto, mas as decisões de política, como qual Realm executar ou qual memória alocar, permanecem com o hipervisor.

Com a CCA, a Arm introduziu um novo tipo de TEE (Trusted Execution Environment) conhecido como Realm. Um Realm é um TEE dinâmico e oferece a possibilidade de atestação. A memória de um Realm pode ser criptografada em uma granularidade por página para evitar ataques físicos. O suporte a Realms resulta em mudanças na arquitetura, estendendo os mundos normal e seguro com dois novos mundos: um “Realm world” e um “Root world”.

A Extensão de Gerenciamento de Realm (RME) adiciona dois novos Espaços de Endereços Físicos (PAS): o Espaço de Endereços Físicos do Realm e o Espaço de Endereços Físicos da Raiz. Cada local na memória física pode ser acessado por meio de um único PAS. O estado de segurança em que uma Unidade de Processamento (PE) está sendo executada determina o subconjunto de PASs acessíveis. O firmware em execução no estado de segurança Raiz controla a atribuição de PAS para cada grânulo de memória física, permitindo a transferência de propriedade da memória entre os mundos Não-seguro, Seguro e Realm.

Para garantir o isolamento, os controles de acesso à memória física são aplicados pela Unidade de Gerenciamento de Memória (MMU) por meio de um processo chamado Verificação de Proteção de Grânulo (GPC). A atribuição de PAS de cada grânulo de memória física é descrita na Tabela de Proteção de Grânulo (GPT). Qualquer violação de controle de acesso resulta em uma nova

---


## Page 9

falha, chamada de Falha de Proteção de Grânulo (GPF). A GPT é mantida na memória Raiz para garantir seu isolamento de todos os outros mundos e só pode ser criada e modificada por código em execução no Mundo Raiz.

## Contribuidores Principais

Johannes Weidner (Friedrich-Alexander-Universität Erlangen-Nürnberg); ARM Holdings

## Fórmulas e Equações

Nenhuma fórmula matemática ou equação específica foi encontrada nas fontes consultadas.

## Fontes

https://www.arm.com/architecture/security-features/arm-confidential-compute-architecture;https://sys.cs.fau.de/extern/lehre/ws22/akss/material/arm-cca.pdf;https://developer.arm.com/documentation/den0125/latest/Arm-CCA-Hardware-Architecture

---

## 8. ARM Memory Tagging Extension (MTE) implementation

### Conteúdo Técnico

A Extensão de Marcação de Memória (MTE) do Arm, introduzida no Arm v9, é uma implementação de hardware de memória marcada que atribui metadados a cada alocação/desalocação de memória. Ela associa uma ‘tag’ a um local de memória, que pode ser vinculada a ponteiros que fazem referência a esse local. Em tempo de execução, a CPU verifica se as tags do ponteiro e dos metadados correspondem em cada operação de carga e armazenamento, ajudando a detectar bugs de segurança de memória como ‘use-after-free’ e ‘buffer-overflow’.

### Modos de Operação

O MTE possui três modos de operação:

*   **Síncrono (SYNC):** Otimizado para a correção da detecção de bugs. Em uma incompatibilidade de tag, o processador aborta a execução e encerra o processo com SIGSEGV (código SEGV_MTESERR ).
*   **Assíncrono (ASYNC):** Otimizado para desempenho. Em uma incompatibilidade de tag, o processador continua a execução até a entrada mais próxima do kernel, onde encerra o processo com SIGSEGV (código SEGV_MTEAERR ).
*   **Assimétrico (ASYMM):** Recurso do Arm v8.7-A, fornece verificação síncrona em leituras de memória e assíncrona em gravações, com desempenho semelhante ao modo ASYNC.

### Detalhes Arquitetônicos

A MTE adiciona um novo tipo de memória, a Memória Marcada Normal (Normal Tagged Memory). As cargas e armazenamentos para este tipo de memória realizam uma verificação onde a tag de 4 bits presente no byte superior do endereço virtual (usando o recurso Top Byte Ignore - TBI) é comparada com a tag armazenada na memória para cada 16 bytes (Grânulo de Tag).

### Novas Instruções

A MTE adiciona um conjunto de novas instruções à arquitetura Armv8-A para manipulação de tags, aritmética de ponteiros e uso do sistema, como IRG, GMI, LDG, STG, ADDG, SUBG, entre outras.

### Implementação no Android

A implementação do MTE no AOSP está distribuída por vários componentes, incluindo o compilador (LLVM), a biblioteca C (Bionic) e o carregador dinâmico. A ativação pode ser feita para binários nativos via sistema de build (Soong, Make) e para aplicativos via manifesto ( android:memtagMode ). A marcação de heap é implementada no alocador Scudo e a de pilha requer instrumentação do compilador (AArch64StackTagging ).

---


## Page 10

# Implementação em Hardware

O processador AmpereOne é o primeiro para data center a suportar MTE, com uma implementação otimizada que não acarreta sobrecarga de capacidade de memória para armazenamento de tags e fornece verificação síncrona com baixo impacto no desempenho. Isso é crucial para ambientes de nuvem multi-tenant, onde a segurança e o desempenho são críticos.

# Contribuidores Principais

ARM Holdings; Juhee Kim (Seoul National University); Jinbum Park (Samsung Research); Sihyeon Roh (Seoul National University); Jaeyoung Chung (Seoul National University); Youngjoo Lee (Seoul National University); Taesoo Kim (Samsung Research and Georgia Institute of Technology); Byoungyoung Lee (Seoul National University); Shiv Kaushik (Ampere Computing); Mahesh Madhav (Ampere Computing); Nagi Aboulenein (Ampere Computing); Jason Bessette (Ampere Computing); Sandeep Brahmadathan (Ampere Computing); Ben Chaffin (Ampere Computing); Matthew Erler (Ampere Computing); Stephan Jourdan (Ampere Computing); Thomas Maciukenas (Ampere Computing); Ramya Masti (Ampere Computing); Jon Perry (Ampere Computing); Massimo Sutera (Ampere Computing); Scott Tetrack (Ampere Computing); Bret Toll (Ampere Computing); David Turley (Ampere Computing); Carl Worth (Ampere Computing); Atiq Bajwa (Ampere Computing)

# Fórmulas e Equações

Nenhuma fórmula matemática ou equação específica foi encontrada durante a pesquisa.

# Fontes

https://source.android.com/docs/security/test/memory-safety/arm-mte; https://android.googlesource.com/platform/bionic/+/main/docs/mte.md; https://developer.arm.com/-/media/Arm%20Developer%20Works/white-papers/Arm%20Memory%20Safety%20-%20MTE%20-%20White%20Paper.pdf

---

# 9. Google custom silicon design methodology

## Conteúdo Técnico

A metodologia de design de silício personalizado do Google é uma abordagem multifacetada que combina o desenvolvimento de hardware e software co-projetados para otimizar o desempenho, a eficiência e o custo de suas cargas de trabalho de computação em hiperescala, especialmente para aprendizado de máquina (ML). A estratégia do Google se manifesta em vários projetos importantes, incluindo as Tensor Processing Units (TPUs) e a suíte de ferramentas de código aberto para design de silício, como o XLS (Accelerated HW Synthesis).

### Princípios de Design e Metodologia:

O Google adota uma abordagem de “primeiro os sistemas” para o design de chips, onde as decisões de arquitetura de hardware são fortemente influenciadas pelos requisitos de software e de todo o sistema. Os principais objetivos que impulsionam seu processo de design, conforme detalhado no desenvolvimento do TPUv2/v3, incluem:

1.  **Construir rapidamente:** Priorizar a simplicidade do design e as soluções “boas o suficiente” para acelerar o tempo de lançamento no mercado.
2.  **Alcançar alto desempenho:** Focar na densidade computacional e na largura de banda da memória para manter as unidades de processamento alimentadas.
3.  **Escalar eficientemente:** Projetar para sistemas de vários chips com interconexões de alta largura de banda para permitir treinamento em larga escala.
4.  **Funcionar para novas cargas de trabalho:** Construir hardware programável e de uso geral que possa se adaptar a novos modelos e algoritmos de ML.
5.  **Ser econômico:** Otimizar a área do chip e o consumo de energia para reduzir o custo total de propriedade.

### XLS (Accelerated HW Synthesis):

---

---


## Page 11

O XLS é um conjunto de ferramentas de síntese de alto nível (HLS) de código aberto que visa simplificar o design de hardware, permitindo que os desenvolvedores escrevam código em uma linguagem de alto nível que é então sintetizada em hardware. Os principais componentes do XLS incluem:

*   **DSLX:** Uma linguagem de domínio específico (DSL) inspirada em Rust, projetada para descrever fluxos de dados de hardware. É uma linguagem funcional baseada em expressões imutáveis com recursos como larguras de bits arbitrárias e um grafo de chamadas totalmente analisável.
*   **IR (Representação Intermediária):** O XLS usa uma única representação intermediária orientada a fluxo de dados em todo o compilador. Em vez de um grafo de fluxo de controle (CFG) tradicional, ele emprega uma representação de “mar de nós” (sea-of-nodes - SoN), que reflete com mais precisão a natureza paralela do hardware.

## Arquitetura do TPUv2/v3:

As TPUs do Google são um excelente exemplo de sua metodologia de design de silício personalizado em ação. A arquitetura do TPUv2/v3 foi projetada especificamente para treinamento de ML e apresenta vários componentes principais:

*   **Core do TPUv2:** Co-projetado com a equipe do compilador XLA, o core usa uma arquitetura Very Long Instruction Word (VLIW) para expressar o paralelismo no nível da instrução. O pacote VLIW tem 322 bits e é composto por slots para operações escalares, vetoriais e matriciais.
*   **Unidade de Computação Escalar:** Busca pacotes VLIW, executa operações escalares e encaminha instruções para as unidades vetoriais e matriciais.
*   **Unidade de Computação Vetorial:** Contém 128 pistas vetoriais, cada uma com uma dimensão de execução adicional de 8 vias chamada sub-pista.
*   **Unidades de Computação Matricial (MXU):** O coração computacional do TPU, consistindo em um arranjo sistólico de 128x128 de multiplicadores e somadores. Ele usa o formato numérico bfloat16 para multiplicações, que tem a mesma faixa de expoente do float32, mas com menos bits de mantissa, oferecendo uma vantagem de energia e área.
*   **Sistema de Memória:** Usa uma combinação de memórias scratchpad SRAM na chip e memória de alta largura de banda (HBM) fora do chip. O compilador gerencia a movimentação de dados entre SRAM e HBM usando DMAs assíncronos.
*   **Interconexão:** Uma interconexão dedicada (ICI - InterChip Interconnect) permite que vários chips TPU sejam conectados em um supercomputador (pod) para treinamento síncrono em larga escala.

## Contribuidores Principais

Thomas Norrie, Nishant Patil, Doe Hyun Yoon, George Kurian, Sheng Li, James Laudon, Cliff Young, Norman Jouppi, David Patterson, Aidan Kirk, Albert Magyar, Alex Light, Amin Kalantar, Angelo Matni, Balint Christian, Blaok, Brandon Jiang, Brian Searls, Chen-hao Chang, Chris Drake, Chris Leary, Conor McCullough, David Plass, Dan Killebrew, Derek Lockhart, Eric Astor, Ethan Mahintorabi, Felix Zhu, Georges Rotival, Hanchen Ye, Hans Montero, Henner Zeller, Iliyan Malchev, Johan Euphrosine

## Fórmulas e Equações

**bfloat16:** Um formato de ponto flutuante de 16 bits com a mesma faixa de expoente do float32, mas com uma mantissa menor. Isso reduz os custos de hardware e energia, sendo mais fácil de usar para software de ML do que o fp16.

**Pacote VLIW (Very Long Instruction Word) do TPUv2:** Um pacote de instrução de 322 bits composto por:
*   Dois slots escalares
*   Quatro slots vetoriais (dois usados para carga/armazenamento de vetor)
*   Dois slots de matriz (um push e um pop)
*   Um slot diverso
*   Seis imediatos

## Fontes

https://developers.google.com/silicon;https://cloud.google.com/transform/axion-arm-cpus-custom-silicon-advantage-history;https://google.github.io/xls/;https://gwern.net/doc/ai/scaling/hardware/2021-norrie.pdf

---


## Page 12

# 10. Eficiência energética da CPU do data center do Google

## Conteúdo Técnico

A eficiência energética nos data centers do Google é uma área de foco principal, com a empresa empregando uma variedade de estratégias para minimizar o consumo de energia. A principal métrica usada para medir a eficiência é a Eficácia no Uso de Energia (PUE), que é a razão entre a energia total da instalação e a energia consumida pelo equipamento de TI. Em 2024, a média anual de PUE para a frota global de data centers do Google foi de 1,09, um valor significativamente inferior à média da indústria de 1,56. Isso indica que os data centers do Google usam aproximadamente 84% menos energia de sobrecarga para cada unidade de equipamento de TI.

Para alcançar essa eficiência, o Google utiliza uma abordagem de medição abrangente que inclui todas as fontes de sobrecarga e é baseada na medição contínua do desempenho de toda a frota mundial de data centers ao longo do ano. Além disso, a empresa projeta seus próprios servidores de alto desempenho para usar o mínimo de energia possível para a quantidade de dados que processam.

No que diz respeito ao consumo de energia da IA, o Google desenvolveu uma metodologia abrangente para medir a pegada ambiental. Um prompt de texto médio no Gemini, por exemplo, consome 0,24 Wh de energia. Essa medição leva em consideração não apenas a energia usada pelo modelo de IA primário, mas também a utilização real do chip, a energia consumida por máquinas ociosas, o consumo de CPU e RAM do host, a sobrecarga do data center (PUE) e o consumo de água para resfriamento.

Uma inovação chave na arquitetura de data center do Google é o uso de um sistema de no-break (UPS) distribuído, com uma bateria em cada servidor. Essa abordagem evita a dupla conversão AC-DC-AC, que é uma fonte significativa de perda de energia em data centers tradicionais. O artigo de pesquisa ‘Managing Distributed UPS Energy for Effective Power Capping in Data Centers’ detalha como essa arquitetura permite o ‘power capping’ eficaz, onde a energia armazenada nas baterias durante períodos de baixa atividade é usada durante picos de energia, reduzindo a demanda da rede elétrica e permitindo a instalação de mais servidores dentro do mesmo orçamento de energia. Essa estratégia não apenas melhora a eficiência energética, mas também reduz o Custo Total de Propriedade (TCO) por servidor em até 6,3%.

## Contribuidores Principais

Jeff Dean (Google Senior Fellow and SVP, Google Research); Amin Vahdat (VP/GM, ML, Systems & Cloud AI); Vasileios Kontorinis (UC San Diego); Liuyi Eric Zhang (UC San Diego); Baris Aksanli (UC San Diego); Jack Sampson (UC San Diego); Houman Homayoun (UC San Diego); Eddie Pettis (Google Inc.); Dean M. Tullsen (UC San Diego); Tajana Simunic Rosing (UC San Diego)

## Fórmulas e Equações

PUE = Energia Total da Instalação / Energia do Equipamento de TI

TCO/servidor = ((Depreciação do Espaço da Instalação + Depreciação do UPS + Depreciação da Infraestrutura de Energia + Depreciação do Resfriamento + Depreciação do Restante) + Opex do Data Center + Depreciação do Servidor + (Opex de Reparo do Servidor + (Opex de Energia do Servidor + Opex de Energia do Servidor) * PUE)) / Número de Servidores

## Fontes

https://datacenters.google/intl/pt-BR_ALL/efficiency;https://blog.google/intl/pt-br/produtos/nas-nuvens/quanta-energia-a-ia-do-google-usa-fizemos-as-contas/;https://research.google.com/pubs/archive/39964.pdf

# 11. Contribuições de Urs Hölzle para a infraestrutura do Google

## Conteúdo Técnico

Urs Hölzle foi uma figura central na criação da infraestrutura do Google. Suas contribuições técnicas incluem a concepção da arquitetura de computação em escala de armazém (Warehouse-Scale Computing), tratando o data center como um único computador massivo. Ele foi pioneiro no conceito de computação proporcional à energia (Energy-Proportional Computing), propondo que o consumo de energia de um servidor deveria ser diretamente proporcional à sua carga de trabalho. A principal

---


## Page 13

métrica para isso é a Eficiência Energética, definida como a razão entre a utilização e o consumo de energia. Para alcançar a proporcionalidade, ele defendeu a necessidade de componentes com ampla faixa dinâmica de energia e modos ativos de baixo consumo, especialmente em memória e armazenamento. Hölzle também liderou o desenvolvimento de redes de data center em larga escala, como a arquitetura Jupiter baseada em topologia Clos, e a WAN definida por software B4, que utiliza OpenFlow para otimização de tráfego global. Seu trabalho anterior ao Google na otimização de linguagens de programação, incluindo o desenvolvimento de Polymorphic Inline Caches (PICs) para a VM Java HotSpot, também foi fundamental.

**Contribuidores Principais**

Urs Hölzle (Google); Luiz André Barroso (Google); Jeff Dean (Google); Jimmy Clidaras (Google); Craig Chambers (University of Washington); David Ungar (IBM Research)

**Fórmulas e Equações**

\text{Eficiência Energética} = \frac{\text{Utilização}}{\text{Consumo de Energia}}

**Fontes**

https://research.google/people/author79/; https://en.wikipedia.org/wiki/Urs_H%C3%B6lzle; https://scholar.google.com/citations?user=3ox8XlUAAAAJ&hl=en; https://www.barroso.org/publications/ieee_computer07.pdf; https://patents.justia.com/inventor/urs-h-lzle

---

# 12. Jeff Dean Google systems architecture

## Conteúdo Técnico

A arquitetura de sistemas do Google, amplamente influenciada por Jeff Dean, é fundamentada em princípios de escalabilidade, tolerância a falhas e eficiência, utilizando componentes de hardware de baixo custo. A evolução dessa arquitetura, particularmente no sistema de busca, demonstra um crescimento exponencial em capacidade e uma redução drástica na latência. Entre 1999 e 2010, o número de documentos indexados e o volume de consultas aumentaram em aproximadamente 1000 vezes, enquanto a latência de atualização foi reduzida em cerca de 50.000 vezes.

Um dos pilares dessa arquitetura é o Google File System (GFS), um sistema de arquivos distribuído projetado para grandes aplicações de dados intensivos. O GFS divide os arquivos em “chunks” de 64 MB, que são replicados em múltiplos “chunkservers” para garantir a tolerância a falhas. Um único “master” gerencia todos os metadados do sistema de arquivos, incluindo o namespace, o mapeamento de arquivos para chunks e a localização dos chunks. A comunicação de dados ocorre diretamente entre os clientes e os chunkservers, minimizando o envolvimento do master e evitando gargalos.

Outro componente fundamental é o MapReduce, um modelo de programação para processamento de grandes conjuntos de dados em paralelo. O MapReduce abstrai as complexidades da computação distribuída, como paralelização, balanceamento de carga e tolerância a falhas, permitindo que os desenvolvedores se concentrem na lógica da aplicação. O modelo consiste em duas fases principais: a fase “Map”, que processa os dados de entrada e gera pares chave-valor intermediários, e a fase “Reduce”, que agrupa os valores intermediários por chave.

Para o armazenamento de dados estruturados, o Google desenvolveu o Bigtable, um sistema de armazenamento distribuído que pode ser visto como um mapa esparsa, distribuído e multidimensional. Os dados são indexados por uma tupla (row, column, timestamp). O Bigtable é construído sobre o GFS e utiliza o serviço de lock distribuído Chubby para coordenação.

A comunicação entre os diversos componentes dos sistemas do Google é facilitada pelos Protocol Buffers, uma linguagem de descrição de interface neutra em termos de linguagem e plataforma para serialização de dados estruturados. Eles são projetados para serem eficientes, com velocidades de codificação e decodificação que podem exceder 200 MB/s, e compactos, utilizando codificações de comprimento variável. Uma técnica de codificação notável é o “Group Varint Encoding”, que codifica grupos de quatro inteiros de 32 bits de forma mais eficiente do que os métodos tradicionais de varint, alcançando velocidades de decodificação de até 400 milhões de números por segundo.

---


## Page 14

# Contribuidores Principais

Jeff Dean (Google), Sanjay Ghemawat (Google), Howard Gobioff (Google), Shun-Tak Leung (Google)

# Fórmulas e Equações

O(n) notation for computational cost, e.g., O(#queries * #docs in index)

# Fontes

https://research.google.com/people/jeff/Stanford-DL-Nov-2010.pdf;https://www.cs.cornell.edu/projects/ladis2009/talks/dean-keynote-ladis2009.pdf;https://research.google.com/archive/gfs-sosp2003.pdf

---

# 13. História do design de processadores da ARM Holdings

## Conteúdo Técnico

A história do design de processadores da ARM (originalmente Acorn RISC Machine) começa em 1983 na Acorn Computers. A empresa buscava um processador de baixo custo e alto desempenho para sua nova geração de computadores, mas as opções existentes da Intel, Motorola e National Semiconductor foram consideradas insatisfatórias. Inspirados pelos trabalhos sobre a arquitetura RISC (Reduced Instruction Set Computing) da Universidade da Califórnia, Berkeley, e da IBM, os engenheiros da Acorn, notavelmente Sophie Wilson e Steve Furber, decidiram projetar sua própria CPU.

## Princípios Fundamentais e Arquitetura

A filosofia central da arquitetura ARM é a simplicidade, resultando em um design de chip menor, com menor consumo de energia e menor custo. Os princípios fundamentais do design RISC adotados pela ARM incluem:

*   **Conjunto de Instruções Reduzido:** O primeiro processador, ARM1, possuía apenas 45 instruções, em contraste com as 357 do Intel 80286 da mesma época. Isso simplificou a lógica de decodificação e execução.
*   **Arquitetura Load-Store:** As operações de processamento de dados (aritméticas e lógicas) operam apenas em registradores. Elas são separadas das instruções que acessam a memória (load e store), o que simplifica o design e permite um pipeline mais eficiente.
*   **Execução em Ciclo Único:** A maioria das instruções é executada em um único ciclo de clock, o que é alcançado pela simplicidade do conjunto de instruções.
*   **Pipelining:** O processador ARM utiliza um pipeline para aumentar o throughput de instruções. O pipeline clássico do ARMv1 era de três estágios: Fetch (busca da instrução), Decode (decodificação) e Execute (execução).
*   **Execução Condicional:** Uma característica distintiva da arquitetura ARM é que todas as instruções podem ser executadas condicionalmente com base em flags no registrador de status (CPSR - Current Program Status Register). Isso reduz a necessidade de instruções de desvio (branch), melhorando o desempenho e a densidade do código.

## Evolução da Arquitetura

*   **ARMv1 e ARMv2:** O primeiro processador, ARM1, foi produzido em 1985. Era um design de 32 bits com um espaço de endereçamento de 26 bits. O ARMv2 adicionou a instrução de co-processador.
*   **ARMv3:** Introduziu um espaço de endereçamento de 32 bits, removendo a limitação de 64 MB de memória.
*   **ARMv4T:** Adicionou o conjunto de instruções **Thumb** de 16 bits. O Thumb foi projetado para melhorar a densidade do código, o que é crucial para sistemas embarcados com memória limitada. As instruções Thumb são descompactadas dinamicamente para instruções ARM de 32 bits dentro do processador sem perda de desempenho.
*   **ARMv5TEJ:** Introduziu a tecnologia **Jazelle**, que permitia a execução direta de bytecode Java em hardware, e melhorias nas instruções DSP (Digital Signal Processing).
*   **ARMv6:** Adicionou operações SIMD (Single Instruction, Multiple Data) para mídia, melhorias no sistema de memória e suporte a multiprocessamento.

---


## Page 15

*   **ARMv7 (A, R, M Profiles):** A arquitetura foi dividida em três perfis: **Cortex-A (Application)** para sistemas operacionais complexos, **Cortex-R (Real-time)** para aplicações de tempo real e **Cortex-M (Microcontroller)** para sistemas embarcados de baixo custo.
*   **ARMv8-A:** Introduziu a arquitetura de 64 bits (AArch64), mantendo a compatibilidade com a arquitetura de 32 bits (AArch32). AArch64 possui um conjunto de instruções limpo e moderno, com 31 registradores de 64 bits de uso geral.

## Especificações Técnicas Notáveis

*   **Registradores:** No modo de 32 bits, a ARM possui 16 registradores de 32 bits visíveis (R0-R15), embora alguns sejam especiais (R13 é o Stack Pointer, R14 é o Link Register, R15 é o Program Counter). No modo de 64 bits (AArch64), existem 31 registradores de 64 bits de uso geral (X0-X30) e um registrador de zero.
*   **Modos de Operação:** Os processadores ARM possuem vários modos de operação para fornecer um ambiente seguro para sistemas operacionais. Os modos incluem User, FIQ (Fast Interrupt Request), IRQ (Interrupt Request), Supervisor, Abort, Undefined e System.
*   **Coprocessador:** A arquitetura suporta até 16 coprocessadores para estender a funcionalidade, como o VFP (Vector Floating-Point) para operações de ponto flutuante.

## Contribuidores Principais

Sophie Wilson (Acorn Computers); Steve Furber (Acorn Computers); Acorn Computers; Apple Computer; VLSI Technology, Inc.

## Fórmulas e Equações

Tempo de CPU = Número de Instruções * CPI * Tempo de Ciclo de Clock

## Fontes

https://arstechnica.com/gadgets/2022/09/a-history-of-arm-part-1-building-the-first-chip/; https://en.wikipedia.org/wiki/ARM_architecture_family; https://developer.arm.com/documentation/ddi0406/c; https://studerarchitecture.pdf; https://scholar.google.com/citations?user=jLnsiBEAAAAJ&hl=en; https://patents.justia.com/inventor/sophie-wilson; https://www.inquartik.com/examining-arms-patents/

---

# 14. Evolução da linha do tempo do ARM Cortex para o Neoverse

## Conteúdo Técnico

A plataforma Neoverse da ARM representa uma mudança estratégica da empresa, antes focada em dispositivos móveis, para o mercado de infraestrutura de data centers, computação de borda e HPC. A linha Neoverse é uma evolução direta da arquitetura Cortex, otimizada para as demandas específicas desses novos mercados.

### Linha do Tempo e Evolução:

*   **2018:** A ARM lança a marca Neoverse, sinalizando seu compromisso com o mercado de infraestrutura.
*   **2019:** Lançamento do Neoverse N1 (derivado do Cortex-A76) e E1 (derivado do Cortex-A65AE).
*   **2020:** Anúncio do Neoverse V1 (derivado do Cortex-X1) e N2 (derivado do Cortex-A710).
*   **2021:** Lançamento do Neoverse V1 e N2.
*   **2022:** Anúncio do Neoverse V2 (derivado do Cortex-X3) e E2 (derivado do Cortex-A510).
*   **2023:** Lançamento do Neoverse V2 e anúncio do Neoverse V3, N3 e E3.

### Arquitetura e Design:

A principal diferença entre as linhas Cortex e Neoverse reside nas otimizações para cargas de trabalho de servidor. Isso inclui:

*   **Interconexão:** O Neoverse utiliza uma malha de interconexão (mesh interconnect) como a CMN-600 e CMN-700, permitindo a escalabilidade para um grande número de núcleos (até 256 por die no V2), em contraste com o design de cluster dos cores

---


## Page 16

Cortex.

*   **Cache:** Os cores Neoverse possuem caches L2 maiores e, em alguns casos, L3, para lidar com os grandes conjuntos de dados comuns em servidores. O Neoverse N2, por exemplo, pode ser configurado com até 1MB de cache L2 por núcleo.
*   **Coerência de Cache:** O Neoverse implementa coerência de cache de instrução por hardware, essencial para sistemas com muitos núcleos.
*   **Endereçamento de Memória:** O Neoverse suporta um espaço de endereço físico maior (48 bits no N2, por exemplo), permitindo o uso de mais memória RAM (até 256 TB).
*   **Virtualização e Segurança:** Recursos aprimorados de virtualização e segurança são integrados para atender às necessidades do ambiente de nuvem.

**Comparações:**

*   **Neoverse N1 vs. Cortex-A76:** O N1 adiciona uma interconexão em malha, caches L2 maiores e coerência de cache de instrução.
*   **Neoverse V1 vs. Cortex-X1:** O V1 introduz suporte a SVE (Scalable Vector Extension) com vetores de 2x256 bits.
*   **Neoverse N2 vs. Cortex-A710:** O N2 aumenta o cache L1 para 64KB, suporta endereçamento físico de 48 bits e oferece a opção de coerência de cache de instrução por hardware.
*   **Neoverse V2 vs. Cortex-X3:** O V2 apresenta um aumento significativo na capacidade do BTB (12K entradas), decodificador mais largo (6-wide) e maior capacidade de execução fora de ordem (ROB de 320 entradas).

**Contribuidores Principais**

ARM Holdings; Ampere Computing; Amazon Web Services (AWS); Google; NVIDIA; Microsoft; Alibaba; Andrea Pellegrini (Arm); Chris Abernathy (Arm); Chester Lam (Chips and Cheese)

**Fórmulas e Equações**

ops/ciclo por núcleo

<table>
<thead>
<tr>
<th></th>
<th>INT8</th>
<th>BF16</th>
<th>FP32</th>
<th>FP64</th>
</tr>
</thead>
<tbody>
<tr>
<td>Neoverse N1</td>
<td>64</td>
<td>32</td>
<td>16</td>
<td>8</td>
</tr>
<tr>
<td>Neoverse N2</td>
<td>128</td>
<td>64</td>
<td>16</td>
<td>8</td>
</tr>
<tr>
<td>Neoverse V1</td>
<td>256</td>
<td>128</td>
<td>32</td>
<td>16</td>
</tr>
</tbody>
</table>

**Fontes**

https://en.wikipedia.org/wiki/ARM_Neoverse;https://en.wikichip.org/wiki/arm_holdings/neoverse;https://newsroom.arm.com/blog-of-arm-architecture-evolution-40-years;https://www.nextplatform.com/2024/02/21/arm-neoverse-roadmap-brings-cpu-designs-but-no-big-fat-gpu/;https://videocardz.com/press-release/arm-releases-2020-2022-neoverse-platform-roadmap;https://newsroom.arm.com/blog/arm-neoverse-then-and-now;https://community.cadence.com/cadence_blogs_8/b/breakfast-bytes/posts/arm-zeus;https://www.edge-ai-vision.com/2021/04/transforming-compute-for-next-generation-infrastructure/;https://moorinsightsstrategy.com/research-notes/research-note-arm-launches-its-latest-neoverse-platforms/;https://gropedia.com/page/ARM_Neoverse;https://www.reddit.com/r/arm/comments/1dkxgcm/how_are_neoverse_neoverse-n2-cortex-a710-for-servers;https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/redefining-storage-with-arm-cortex-r82-and-neoverse-cmn-s3;https://armkeil.blob.core.windows.net/developer/Files/pdf/white-paper/arm-neoverse-n1-platform.pdf;https://armkeil.blob.core.windows.net/developer/Files/pdf/white-paper/neoverse-n1-core-performance-v2.pdf;https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/arm-neoverse-n1-performance-analysis-methodology;https://old.hotchips.org/hc31/HC31_1.2_20190816_Arm_Neoverse_N1_CPU.pdf;https://chipsandcheese.com/p/deediving-neoverse-n1;https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/arm-neoverse-v1-top-down-methodology;https://www.linkedin.com/posts/jumanamp_neoverse-v1-performance-

---


## Page 17

analysis-whitepaper-activity-7135674585965895680-Fq-r;https://www.reddit.com/r/ArmSoftwareDev/comments/10wglgp/neoverse_v1_performance_analysis_whitepaper/;https://www.ip-cpu/neoverse/neoverse-v1;https://armkeil.blob.core.windows.net/developer/Files/pdf/solution-overview-arm-neoverse-v1-platform.pdf;https://www.arm.com/products/silicon-ip-cpu/neoverse/neoverse-n2;https://developer.arm.com/community/arm-community-blogs/b/servers-and-cloud-computing-blog/posts/virtual-networking-solution-performance-arm-neoverse;https://hc33.hotchips.org/assets/program/conference/day1/20210818_Hotchips_NeoverseN2.pdf;https://www.scs.stanford.edu/patents/;https://developer.arm.com/documentation/100616/latest/;https://www.eenewseurope.com/en/arm-declines-to-license-neoverse-v-series-to-alibaba/;https://developer.arm.com/documentation/102481/0001;https://www.farnell.com/datasheets/3689973.pdf;https://developer.arm.com/documentation/100616/latest/;https://www.nextplatform.com/2024/02/21/arm-neoverse-roadmap-brings-cpu-designs-but-no-big-fat-gpu/;https://m.hexus.net/tech/news/cpu/147739-arm-launches-neoverse-n2-v1-platforms-data-centres/;https://patents.google.com/patent/US8738860B1/en;https://www.eenewseurope.com/en/arm-pushes-chiplets-and-3d-packaging-for-neoverse-chips/;https://documentation-service.arm.com/static/66f715dc1669c0388dca6d08?token=;https://patents.google.com/?q=ARM+Neoverse;https://chipsandcheese.com/p/hot-chips-2023-arms-neoverse-v2

# 15. Matemática dos algoritmos de previsão de desvio da CPU

## Conteúdo Técnico

A previsão de desvio é uma técnica de arquitetura de computadores que visa a mitigar as penalidades de desempenho associadas a instruções de desvio em processadores com pipeline. A ideia é prever o resultado de um desvio (se será tomado ou não) antes que ele seja de fato resolvido, permitindo que o processador comece a buscar e executar especulativamente as instruções do caminho previsto.

**Previsão de Desvio com Perceptrons:** Este método, proposto por Daniel A. Jiménez e Calvin Lin, utiliza um perceptron, uma forma simples de rede neural, para prever a direção de um desvio. O histórico global de desvios é usado como entrada para o perceptron, que aprende a correlação entre os desvios passados e o desvio atual. A saída do perceptron, um valor numérico, determina a previsão. O conceito de **separabilidade linear** é central, pois os perceptrons são eficazes na previsão de desvios cujo comportamento pode ser separado por um hiperplano no espaço de características do histórico.

**Previsão de Desvio Linear por Partes (Piecewise Linear):** Uma evolução do preditor de perceptron, proposta por Daniel A. Jiménez, que utiliza um conjunto de funções lineares, uma para cada caminho de execução que leva a um desvio. Juntas, essas funções formam uma superfície de decisão linear por partes, permitindo a previsão de desvios com comportamento não linearmente separável, como a função XOR, que os perceptrons simples não conseguem aprender.

**Técnicas de Aprendizado Profundo:** Pesquisas mais recentes exploram o uso de arquiteturas de aprendizado profundo mais complexas, como Redes Neurais Convolucionais (CNNs) e Redes Neurais Recorrentes (RNNs), incluindo LSTMs (Long Short-Term Memory). As CNNs são usadas para identificar padrões espaciais no histórico de desvios, enquanto as RNNs e LSTMs são adequadas para modelar a natureza sequencial do histórico de desvios. Essas abordagens visam a capturar correlações mais complexas e melhorar a precisão da previsão, especialmente para desvios difíceis de prever.

## Contribuidores Principais

Daniel A. Jiménez (The University of Texas at Austin); Calvin Lin (The University of Texas at Austin); Daniel A. Jiménez (Rutgers University); Rinu Joseph (University of Texas at San Antonio); Zangeneh et al.; Tarsa et al.

## Fórmulas e Equações

y = w0 + |sum{i=1}^{n} x_i w_i| if sign(y_out) != t or |y_out| <= theta: for i in 0..n:

```plaintext
w_i = w_i + t * x_i
```

function predict(address: integer): boolean begin output := W[address, 0, 0] for i in 1..h do

---


## Page 18

pascal
if GHR[i] = true then
    output := output + W[address, GA[i], i]
else
    output := output - W[address, GA[i], i]

predict := output >= 0 end

procedure train(address: integer, taken: boolean) begin if |output| < theta or predict != taken then

if taken = true then
    W[address, 0, 0] := W[address, 0, 0] + 1
else
    W[address, 0, 0] := W[address, 0, 0] - 1
for i in 1..h do
    if GHR[i] = taken then
        W[address, GA[i], i] := W[address, GA[i], i] + 1
    else
        W[address, GA[i], i] := W[address, GA[i], i] - 1

end

theta = 2.14 * (h + 1) + 20.58
```

# Fontes

https://www.cs.utexas.edu/~lin/papers/hpca01.pdf; https://people.engr.tamu.edu/djimenez/taco/pdfs/isca05_dist.pdf; https://dl.a

---

# 16. Execução Fora de Ordem e o Algoritmo de Tomasulo

## Conteúdo Técnico

O algoritmo de Tomasulo, desenvolvido por Robert Tomasulo na IBM em 1967, é um mecanismo de hardware para agendamento dinâmico de instruções que permite a execução fora de ordem (out-of-order execution). Suas principais inovações são a renomeação de registradores, as estações de reserva e o barramento de dados comum (CDB), que juntos trabalham para aumentar o paralelismo em nível de instrução (ILP).

## Conceitos Fundamentais

*   **Execução Fora de Ordem:** Permite que o processador execute instruções em uma ordem diferente daquela em que aparecem no programa, contornando os riscos de dados e estruturais que causariam paradas (stalls) em um processador de execução em ordem.
*   **Renomeação de Registradores:** Técnica que elimina os riscos de dados do tipo WAW (Write-After-Write) e WAR (Write-After-Read) ao alocar dinamicamente registradores físicos para os registradores lógicos da arquitetura. No algoritmo de Tomasulo, as estações de reserva e os buffers de carga/armazenamento atuam como os registradores físicos.
*   **Estações de Reserva (Reservation Stations):** Pequenos buffers associados a cada unidade funcional. Eles armazenam as instruções que foram emitidas, mas estão aguardando por seus operandos. Cada entrada na estação de reserva contém a operação a ser executada, os valores dos operandos (se disponíveis) e tags que identificam de qual estação de reserva os operandos virão.
*   **Barramento de Dados Comum (Common Data Bus - CDB):** Um barramento de difusão que conecta as saídas de todas as unidades funcionais às entradas de todas as estações de reserva, registradores e buffers de armazenamento. Quando uma unidade funcional conclui uma operação, ela transmite o resultado e sua tag de identificação no CDB. Todas as unidades que estão esperando por esse resultado (snooping) o capturam.

---


## Page 19

# Arquitetura e Design

A arquitetura de um processador que implementa o algoritmo de Tomasulo consiste em:

1.  **Fila de Instruções:** Armazena as instruções a serem emitidas.
2.  **Unidades Funcionais:** Circuitos que realizam as operações (ex: somadores, multiplicadores).
3.  **Estações de Reserva:** Associadas a cada unidade funcional.
4.  **Banco de Registradores:** Armazena o estado dos registradores.
5.  **Load/Store Buffers:** Gerenciam as operações de acesso à memória, permitindo a desambiguação dinâmica de memória.
6.  **Common Data Bus (CDB):** Interliga todos os componentes mencionados.

# Ciclo de Vida da Instrução

1.  **Issue (Emissão):** A instrução é retirada da fila e, se houver uma estação de reserva disponível, é emitida. Os operandos que já estão nos registradores são copiados para a estação de reserva. Se um operando ainda não estiver disponível (porque está sendo calculado por outra instrução), a estação de reserva recebe uma tag que identifica a estação de reserva que produzirá o operando.
2.  **Execute (Execução):** Quando todos os operandos de uma instrução estão disponíveis na sua estação de reserva, a instrução é enviada para a unidade funcional para execução. A disponibilidade dos operandos é determinada pelo monitoramento (snooping) do CDB.
3.  **Write Result (Escrita do Resultado):** Após a conclusão da execução, o resultado é colocado no CDB, juntamente com a tag da estação de reserva que o produziu. Todas as estações de reserva e registradores que aguardavam por esse resultado o capturam e atualizam seus valores. A estação de reserva que produziu o resultado é liberada.

# Contribuidores Principais

Robert Tomasulo (IBM); David A. Patterson (University of California, Berkeley)

# Fórmulas e Equações

Não foram encontradas fórmulas matemáticas complexas, mas a lógica do algoritmo pode ser expressa em pseudocódigo.

## Estágio de Emissão (Issue):

```c
if (RegisterStat[rs].Qi != 0) {
    RS[r].Qj = RegisterStat[rs].Qi;
} else {
    RS[r].Vj = Regs[rs];
    RS[r].Qj = 0;
}
if (RegisterStat[rt].Qi != 0) {
    RS[r].Qk = RegisterStat[rt].Qi;
} else {
    RS[r].Vk = Regs[rt];
    RS[r].Qk = 0;
}
RS[r].Busy = yes;
RegisterStat[rd].Qi = r;
```

## Estágio de Execução (Execute):

```c
wait until (RS[r].Qj == 0) and (RS[r].Qk == 0);
// Computar o resultado
```

## Estágio de Escrita do Resultado (Write Result):

---


## Page 20

mermaid
graph TD
    A[wait until (Execution complete at r) and (CDB available)] --> B{forall(x)}
    B --> C{if (RegisterStat[x].Qi == r)}
    C --> D[regs[x] = result]
    D --> E[RegisterStat[x].Qi = 0]
    E --> B
    B --> F{if (RS[x].Qj == r)}
    F --> G[RS[x].Vj = result]
    G --> H[RS[x].Qj = 0]
    H --> B
    B --> I{if (RS[x].Qk == r)}
    I --> J[RS[x].Vk = result]
    J --> K[RS[x].Qk = 0]
    K --> B
    B --> L[RS[r].Busy = no]
```

# Fontes

https://en.wikipedia.org/wiki/Tomasulo%27s_algorithm;https://people.eecs.berkeley.edu/~pattrsn/252F96/Lecture04.pdf;https://

---

# 17. Microarquitetura de execução especulativa

## Conteúdo Técnico

A execução especulativa é uma técnica de otimização em que um sistema de computador realiza uma tarefa que pode não ser necessária. O trabalho é feito antes de se saber se é realmente necessário, para evitar um atraso que teria que ser incorrido ao fazer o trabalho depois de se saber que é necessário. Se o trabalho não for necessário, a maioria das alterações feitas pelo trabalho são revertidas e os resultados são ignorados. A previsão de desvio é uma técnica usada em processadores modernos para prever o resultado de uma instrução de desvio condicional antes que ela seja executada. Existem dois tipos principais de previsão de desvio: estática e dinâmica. O preditor de desvio adaptativo de dois níveis usa dois níveis de histórico de desvio para fazer previsões. O preditor de desvio TAGE (TAgged GEometric length) usa várias tabelas de previsão com comprimentos de histórico geométricos. O preditor de desvio com perceptron modela a previsão de desvio como um problema de classificação, usando uma rede neural simples para aprender a correlação entre o histórico de desvios e o resultado do desvio.

## Contribuidores Principais

Tse-Yu Yeh (University of Michigan); Yale N. Patt (University of Michigan); André Seznec (IRISA/INRIA); Daniel A. Jiménez (The University of Texas at Austin); Calvin Lin (The University of Texas at Austin)

## Fórmulas e Equações

$y = w_0 + \sum_{i=1}^{n} x_i w_i$

$w_i = w_i + tx_i$

## Fontes

https://en.wikipedia.org/wiki/Speculative_execution;https://dl.acm.org/doi/pdf/10.1145/146628-139709;https://www.irisa.fr/caps/papers/TAGE.pdf;https://www.cs.utexas.edu/~lin/papers/hpca01.pdf

---


## Page 21

# 18. Matemática do design da hierarquia de cache da CPU

## Conteúdo Técnico

## Conceitos Fundamentais e Definições

*   **Cache de CPU:** Uma memória menor e mais rápida, localizada mais perto do núcleo do processador, que armazena cópias de dados de locais de memória principal usados com frequência.
*   **Hierarquia de Cache:** Múltiplos níveis de cache (L1, L2, L3, etc.), com L1 sendo o mais próximo do núcleo da CPU e, portanto, o mais rápido.
*   **Cache Hit:** Ocorre quando o processador encontra os dados solicitados na cache.
*   **Cache Miss:** Ocorre quando o processador não encontra os dados solicitados na cache.
*   **Cache Line/Block:** A unidade de transferência de dados entre a memória e a cache.
*   **Associatividade:** Define como as linhas de cache são mapeadas para os endereços de memória.
*   **Políticas de Substituição (Replacement Policies):** Heurísticas usadas para decidir qual entrada de cache existente deve ser descartada para dar lugar a uma nova (ex: LRU - Least Recently Used).
*   **Políticas de Escrita (Write Policies):** Determinam quando os dados escritos na cache também são escritos na memória principal (ex: Write-Through, Write-Back).
*   **Coerência de Cache (Cache Coherence):** Protocolos que mantêm a consistência dos dados entre múltiplos caches em um sistema multiprocessador.

## Fórmulas Matemáticas e Equações

*   **Tempo Médio de Acesso à Memória (AMAT - Average Memory Access Time):**

    AMAT = \text{Hit Time} + (\text{Miss Rate} \times \text{Miss Penalty})

*   **Cálculo do Número de Bits da Cache:**
    *   Para uma cache com mapeamento direto de $2^n$ words com blocos de uma palavra (4 bytes) e endereço de 32 bits:
        *   Tag = 32 - (n + 2) bits
        *   Número total de bits = $2^n * (32 + (32 - n - 2) + 1) = 2^n * (63 - n)$
*   **Cálculo do Bloco de Cache:**
    *   endereço do bloco = endereço do byte / bytes por bloco
    *   bloco = (endereço do bloco) mod (número de blocos da cache)
*   **Cálculo do Tempo de CPU com Stalls de Memória:**
    *   CPU time = (CPU execution clock cycles + Memory-stall clock cycles) * clock cycle time
    *   Memory-stall clock cycles = Read-stall cycles + Write-stall cycles
    *   Read-stall cycles = (Reads/Program) * Read miss rate * Read miss penalty
    *   Write-stall cycles = ((Writes/Program) * Write miss rate * Write miss penalty) + Write buffer stalls
    *   Memory-stall clock cycles = (Memory accesses/Program) * Miss rate * Miss penalty
    *   Memory-stall clock cycles = (Instructions/Program) * (Misses/Instructions) * Miss penalty

## Contribuidores Principais

B.L. Jacob (University of Maryland); P.M. Chen (University of Michigan); S.R. Silverman; T.N. Mudge (University of Michigan); Ricardo Pannain (Unicamp)

---


## Page 22

# Fórmulas e Equações

AMAT = Hit Time + (Miss Rate * Miss Penalty) Tag = 32 - (n + 2) Número total de bits = 2^n * (63 - n) endereço do bloco = endereço do byte / bytes por bloco bloco = (endereço do bloco) mod (número de blocos da cache) CPU time = (CPU execution clock cycles + Memory-stall clock cycles) * clock cycle time Memory-stall clock cycles = Read-stall cycles + Write-stall cycles Read-stall cycles = (Reads/Program) * Read miss rate * Read miss penalty Write-stall cycles = ((Writes/Program) * Write miss rate * Write miss penalty) + Write buffer stalls Memory-stall clock cycles = (Memory accesses/Program) * Miss rate * Miss penalty Memory-stall clock cycles = (Instructions/Program) * (Misses/Instructions) * Miss penalty

# Fontes

https://en.wikipedia.org/wiki/CPU_cache;https://ieeexplore.ieee.org/abstract/document/543711/;https://www.ic.unicamp.br/~pa

---

# 19. Memory bandwidth latency optimization

## Conteúdo Técnico

A otimização da latência e da largura de banda da memória é um desafio central na arquitetura de computadores, visando reduzir o tempo de espera do processador por dados e maximizar a taxa de transferência de dados entre a CPU e a memória. A hierarquia de memória, composta por registradores, caches (L1, L2, L3), memória principal (DRAM) e armazenamento secundário, é a base para essas otimizações. A largura de banda mede a taxa de transferência de dados (GB/s), enquanto a latência mede o tempo de resposta (ns).

A Lei de Little, expressa como ‘Bytes in Flight = Largura de Banda * Latência Média’, é um conceito fundamental que demonstra a relação entre essas métricas e a quantidade de dados em trânsito necessários para saturar a largura de banda. Para otimizar o desempenho, diversas técnicas são empregadas. Para maximizar a largura de banda, pode-se aumentar o paralelismo no nível da instrução (ILP) através de técnicas como o desenvolvimento de loop, utilizar acessos vetorizados à memória e empregar cópias de dados assíncronas. Para tolerar a latência, técnicas como buffering, pipelining, prefetching (busca antecipada de dados) e multithreading são comuns.

Em nível de sistema operacional, otimizações como o gerenciamento de memória em arquiteturas NUMA (Non-Uniform Memory Access), o uso de hyperthreading e o ‘page coloring’ para evitar conflitos de cache são cruciais. Arquiteturas de GPU, como as da NVIDIA, possuem hierarquias de memória complexas com memória compartilhada distribuída para aumentar a capacidade e a eficiência. Tecnologias emergentes como NVDIMMs, que combinam DRAM e memória não volátil, introduzem novos desafios de otimização para o kernel do sistema operacional.

Para aplicações específicas, como as que atravessam grandes estruturas de dados (grafos de ponteiros), a latência da RAM é o principal gargalo. Nesses casos, a reorganização dos dados na memória para permitir acesso sequencial e o ‘interleaving’ de tarefas independentes para explorar o paralelismo do sistema de memória são estratégias eficazes.

## Contribuidores Principais

Allard Hendriksen (NVIDIA), Athena Elafrou (NVIDIA), Yandong Mao (MIT CSAIL), Cody Cutler (MIT CSAIL), Robert Morris (MIT CSAIL), Sudipta Das (IEEE), Samuel Riedel (IEEE), Mohamed Naeim (IEEE), Moritz Brunion (IEEE), Marco Bertuletti (IEEE), Luca Benini (IEEE), K.K. Chang, G. Ayers, C. Kozyrakis, R. Murphy, R. Clapp, N.P. Jouppi, P. Ranganathan, R. Balasubramonian, D. Albonesi

## Fórmulas e Equações

Bytes in Flight = Largura de Banda * Latência Média

## Fontes

https://www.nvidia.com/en-us/on-demand/session/gtc25-s72683/;https://research.redhat.com/blog/research_project/kernel-memory-and-latency/;https://sites.utexas.edu/jdm4372/2025/02/17/single-core-memory-bandwidth-latency-bandwidth-and-concurrency/;http://ieeexplore.ieee.org/document/10720515;https://pdos.csail.mit.edu/papers/ram-latency:apsys13.pdf

---


## Page 23

# 20. Matemática de processamento vetorial SIMD (Single Instruction, Multiple Data)

## Conteúdo Técnico

A arquitetura SIMD (Single Instruction, Multiple Data) é uma forma de computação paralela, conforme definido na Taxonomia de Flynn, onde uma única instrução opera simultaneamente em múltiplos pontos de dados. Este paradigma explora o paralelismo em nível de dados, sendo particularmente eficaz em aplicações que executam a mesma operação sobre grandes conjuntos de dados, como processamento de imagem, simulações científicas e multimídia.

## Modelo Matemático e Fórmulas

Um modelo matemático para computadores SIMD, proposto em um estudo focado em processamento de imagem, utiliza um processo semi-Markoviano para descrever o comportamento do sistema. O estado do sistema no tempo 'r' é i(r). Como este não é um processo de Markov, uma variável y(r) é introduzida para representar o tempo de permanência no estado i. As equações fundamentais descrevem as probabilidades de transição de estado. A taxa de saída do estado i, qi(y), e a probabilidade de transição de estado Pij(r, y) são definidas por um conjunto de equações diferenciais parciais. As condições de contorno e iniciais são estabelecidas para resolver o sistema, resultando em um modelo matemático final expresso através da transformada de Laplace φi(r, s).

## Arquitetura e Design

Existem diferentes implementações de hardware para SIMD:

1. **Processador de Array**: Possui múltiplas unidades funcionais distribuídas em ‘lanes’, cada uma operando de forma independente em diferentes elementos de dados. É escalável em termos de desempenho, mas consome mais espaço no chip.
2. **Processador Vetorial**: Utiliza uma única ‘lane’ com unidades funcionais pipelined. Embora possa levar mais ciclos de instrução para o primeiro elemento, o pipeline garante que os elementos subsequentes sejam processados a uma taxa de um por ciclo, economizando espaço no chip.
3. **Processador de Array Pipelined**: Uma abordagem híbrida, comum em CPUs modernas, que combina múltiplas ‘lanes’ com pipelines em cada unidade funcional, otimizando tanto o espaço quanto o número de ciclos de instrução.

## Aplicações e Implementação

A auto-vetorização é uma técnica de compilação onde o compilador reescreve automaticamente loops de código para usar instruções SIMD. Diretivas como `#pragma GCC target("avx2")` podem ser usadas para instruir o compilador a usar conjuntos de instruções SIMD mais avançados, como o AVX2, que opera em registradores de 256 bits. No entanto, a auto-vetorização tem limitações e pode não gerar o código mais otimizado para todos os tipos de loops.

## Comparação com Tecnologias Similares

A tabela abaixo compara a arquitetura SIMD com a MIMD (Multiple Instruction, Multiple Data).

<table>
<thead>
<tr>
<th>Característica</th>
<th>SIMD (Single Instruction, Multiple Data)</th>
<th>MIMD (Multiple Instruction, Multiple Data)</th>
</tr>
</thead>
<tbody>
<tr>
<td>Instrução</td>
<td>Uma única instrução opera em múltiplos dados.</td>
<td>Múltiplas instruções operam em múltiplos dados.</td>
</tr>
<tr>
<td>Complexidade</td>
<td>Menos complexo.</td>
<td>Mais complexo.</td>
</tr>
<tr>
<td>Flexibilidade</td>
<td>Menos flexível, ideal para operações uniformes.</td>
<td>Mais flexível, ideal para tarefas variadas.</td>
</tr>
<tr>
<td>Custo</td>
<td>Menor custo.</td>
<td>Maior custo.</td>
</tr>
<tr>
<td>Sincronização</td>
<td>Implícita.</td>
<td>Explícita.</td>
</tr>
</tbody>
</table>

## Contribuidores Principais

Michael J. Flynn (Acunhou o termo SIMD em 1966 como parte de sua taxonomia de arquiteturas de computador); Maneesh Sutar (Autor do artigo ‘A Primer to SIMD Architecture: From Concept to Code’)

---


## Page 24

# Fórmulas e Equações

P{i(r + \Delta r) = j | i^® = i, y^® = y} = q_{ij}(y) \cdot \Delta r + o(\Delta r)
P{i(r + \Delta r) = i | i^® = i, y^® = y} = 1 - q_i(y) \cdot \Delta r + o(\Delta r)
q_i(y) = \sum{j \neq i} q_{ij}(y) q_i(y) = \lim{\Delta r \to 0} \frac{P{i(r + \Delta r) \neq i, y^® = y}}{\Delta r} P{ij}(r, y) = P{i^® = j, y^® < y / i(0) = i} \frac{\partial P{ij}(r, y)}{\partial r} + \frac{\partial P_{ij}(r, y)}{\partial y} = -q_i(y) P{ij}(r, y) + \sum{k \neq i} P{ik}(r, y) q{kj}(y) P{ij}(r, 0) = \int{0}^{r} \sum{k \neq j} P{ik}(s, y) q{kj}(y) ds P{ij}(0, y) = |\delta{ij}| \Phi_i(r, s) = \int{0}^{|\infty|} e^{-sy} P_i(r, y) dy

# Fontes

https://pt.wikipedia.org/wiki/Processador_vetorial; https://en.wikipedia.org/wiki/Single_instruction,_multiple_data; https://www.primer-to-simd-architecture-from-concept-to-code-d3cc470d6709; https://www.geeksforgeeks.org/computer-organization-architecture/difference-between-simd-and-mimd/

---

# 21. CPU power management DVFS algorithms

## Conteúdo Técnico

O Dynamic Voltage and Frequency Scaling (DVFS) é uma técnica de gerenciamento de energia que ajusta dinamicamente a frequência e a tensão de um processador para otimizar o consumo de energia e o desempenho. A potência dinâmica dissipada por um chip é calculada como P = C * V^2 * A * f, onde C é a capacitância, V é a tensão, A é o fator de atividade e f é a frequência. A potência estática é P_estatico = V_cc * I_cc. A implementação de DVFS requer suporte de hardware (reguladores de tensão, PLLs) e software (drivers, frameworks de SO).

## Contribuidores Principais

Sandra Djosic; Milun Jevtic; Rizwana Begum; Mark Hempstead; Guru Prasad Srinivasa; Geoffrey Challen; J.O. Coronel; J.E. Simó

## Fórmulas e Equações

P = C * V^2 * A * f; P_dinamico = α * C * V^2 * f; P_estatico = V_cc * I_cc

## Fontes

https://en.wikipedia.org/wiki/Dynamic_frequency_scaling; https://www.sciencedirect.com/topics/computer-science/dynamic-voltage-and-frequency-scaling; https://www.sciencedirect.com/science/article/pii/S0026271413000760; http://ieeexplore.ieee.org/document/7753276/; htt

---

# 22. Física de semicondutores e escalonamento de transistores

## Conteúdo Técnico

O escalonamento de transistores é o processo de reduzir as dimensões dos transistores em circuitos integrados. Este processo tem sido o principal impulsionador do aumento exponencial do poder de computação e da redução de custos na indústria de semicondutores, um fenômeno encapsulado pela Lei de Moore. A Lei de Moore é a observação de que o número de transistores em um circuito integrado (CI) dobrou aproximadamente a cada dois anos. No cerne do desafio do escalonamento de transistores está o comportamento dos elétrons em dimensões nanométricas. Os transistores de efeito de campo de metal-óxido-semiconductor (MOSFETs) tradicionais operam criando uma barreira de potencial para controlar o fluxo de corrente entre os terminais de fonte e dreno. Quando o comprimento do portão encolhe para menos de 10 nanômetros, vários efeitos da mecânica quântica começam a dominar, como o tunelamento quântico e os efeitos de canal curto. Para lidar com essas limitações, a indústria introduziu novas arquiteturas e materiais de transistores, como FinFETs e transistores gate-all-around (GAA). O escalonamento de Dennard, também conhecido como escalonamento de MOSFET, é uma lei de escalonamento que afirma que, à medida que os transistores diminuem de tamanho, sua densidade de potência permanece constante. No entanto, desde cerca de 2005-2007, o escalonamento de Dennard parece ter entrado em colapso devido a problemas como corrente de fuga e tensão de limiar que não escalam com o tamanho.

---


## Page 25

# Contribuidores Principais

Robert H. Dennard (IBM); Gordon Moore (Fairchild Semiconductor, Intel); Thomas Stanley (RCA Research Laboratories); Bruce Hoeneisen (Caltech); Carver Mead (Caltech)

# Fórmulas e Equações

C_ox = ε_ox / t_ox; A = W * L; C_g = C_ox * W * L; K_n = μ_n * C_ox * (W / L); I_on = K_n * (V_GS - V_T)^2; R_on = V_DD / I_on; t_pd = R_on * C_g; P_av = f * C * V_DD^2; PD = P_av / A

# Fontes

https://en.wikipedia.org/wiki/Moore%27s_law; https://delmartechez.com/the-physics-behind-transistor-scaling/; https://en.wikipedia.org/wiki/Dennard_scaling; https://www.computerhistory.org/siliconengine/scaling-of-ic-process-design-rules-quantified/

---

# 23. Tecnologia de transistores FinFET e Gate-All-Around (GAA)

## Conteúdo Técnico

## Pesquisa Técnica sobre a Tecnologia de Transistores FinFET e Gate-All-Around (GAA)

## 1. Conceitos Fundamentais e Evolução

A tecnologia de transistores tem evoluído continuamente para acompanhar a Lei de Moore. A transição de transistores planares para arquiteturas tridimensionais (3D) foi um passo crucial para superar as limitações de escalonamento, como efeitos de canal curto e correntes de fuga.

### 1.1. FinFET (Fin Field-Effect Transistor)

O FinFET introduziu um canal tridimensional em forma de “barbatana” (fin). Nesta arquitetura, a porta (gate) envolve o canal em três lados, proporcionando um controle eletrostático muito superior em comparação com os transistores planares. Isso resulta em menor corrente de fuga, maior corrente de acionamento e chaveamento mais rápido. A tecnologia FinFET foi a base para a fabricação de semicondutores em nós de processo de 22nm até 5nm.

### 1.2. GAAFET (Gate-All-Around Field-Effect Transistor)

O GAAFET é a evolução natural do FinFET, projetado para nós de 3nm e inferiores. A principal diferença é que no GAAFET, a porta envolve completamente o canal por todos os quatro lados. Essa cobertura total maximiza o controle eletrostático, permitindo uma redução ainda maior nas correntes de fuga e a operação em tensões mais baixas. A arquitetura GAA utiliza estruturas de canal como nanofios (nanowires) ou nanofolhas (nanosheets) empilhadas verticalmente, o que permite um ajuste fino da largura do canal para otimizar o desempenho e o consumo de energia.

## 2. Comparativo Técnico: FinFET vs. GAAFET

A superioridade do GAAFET sobre o FinFET pode ser analisada em várias frentes. Estruturalmente, o controle de porta de quatro lados do GAAFET é intrinsecamente melhor que o controle de três lados do FinFET. Em termos de produção, a transição para o GAAFET foi projetada para ser compatível com muitos dos processos de fabricação existentes para FinFETs, mitigando os custos de atualização. Em relação à área e velocidade, o GAAFET oferece uma vantagem significativa, pois o empilhamento vertical de

---


## Page 26

nanosheets permite maior densidade de transistores e maior corrente de acionamento em uma área menor, sem a necessidade de adicionar mais “fins” como no FinFET. Consequentemente, os GAAFETs apresentam menor corrente de fuga e menor consumo de energia, características essenciais para a computação de alto desempenho e dispositivos móveis.

## 3. Arquitetura, Física e Modelagem Matemática

O comportamento dos transistores GAA é descrito por modelos físicos e matemáticos complexos. Um modelo analítico para um GAA Tunneling FET (TFET), que é uma variante do GAAFET, pode ser derivado a partir da equação de Poisson em coordenadas cilíndricas.

### 3.1. Modelo Analítico e Equações

A modelagem do potencial de superfície no canal de silício começa com a **Equação de Poisson 2D em Coordenadas Cilíndricas**:

\[
\frac{1}{r} \frac{\partial}{\partial r} \left( r \frac{\partial \psi(r,z)}{\partial r} \right) + \frac{\partial^2 \psi(r,z)}{\partial z^2} = \frac{q N_S}{\epsilon_{Si}}
\]

Onde $\psi(r,z)$ é o potencial eletrostático, $q$ é a carga elementar, $N_S$ é a dopagem do canal e $\epsilon_{Si}$ é a permissividade do silício. O perfil de potencial radial é então aproximado por um polinômio de segunda ordem, levando a uma solução geral para o potencial de superfície $\psi_S(z)$:

\[
\psi_{si}(z) = C_i \exp{\left(\frac{z}{L_d}\right)} + D_i \exp{\left(-\frac{z}{L_d}\right)} + \psi_{Gi} - \frac{q N_i}{L_d^2 \epsilon_{Si}}
\]

O **Comprimento Característico ($L_d$)** é uma métrica fundamental que descreve a escala de variação do potencial:

\[
L_d = \sqrt{\frac{T_{Si}^2 \ln{(1 + \frac{T_{ox}}{T_{Si}})}}{2 \epsilon_{Si} \epsilon_{ox}}}
\]

A **Corrente de Dreno ($I_D$)** em um TFET é dominada pelo tunelamento quântico e pode ser modelada usando o modelo de Kane, que depende exponencialmente do campo elétrico médio na junção de tunelamento.

\[
I_D \propto \exp{\left(-\frac{4 \sqrt{2 m^*} E_g^{3/2}}{3 q \hbar E_{avg}}\right)}
\]

## 4. Contribuidores Principais e Patentes

A invenção e o desenvolvimento das tecnologias FinFET e GAA são o resultado do trabalho de muitos pesquisadores e engenheiros em todo o mundo.

*   **Pioneiros do FinFET:** Pesquisadores da Universidade da Califórnia, Berkeley, incluindo o Prof. Chenming Hu e a Prof. Tsu-Jae King Liu, foram fundamentais no desenvolvimento do conceito FinFET. A patente seminal (U.S. Patent 6,413,802) foi registrada por sua equipe. Digh Hisamoto, da Hitachi, também é reconhecido por seu trabalho pioneiro em estruturas de porta dupla.
*   **Líderes da Indústria em GAA:** A transição para o GAAFET é liderada pelas principais empresas de semicondutores. A Samsung foi a primeira a anunciar a produção em massa com GAA em seu nó de 3nm. A TSMC e a Intel (com sua tecnologia RibbonFET) também estão investindo pesadamente e desenvolvendo suas próprias implementações da arquitetura GAA. Instituições de pesquisa como o IMEC na Bélgica têm sido cruciais para a prototipagem e pesquisa fundamental em GAAFETs.

### Contribuidores Principais

Prof. Chenming Hu (UC Berkeley); Prof. Tsu-Jae King Liu (UC Berkeley); Digh Hisamoto (Hitachi); Samsung; TSMC; Intel; IMEC

---


## Page 27

# Fórmulas e Equações

Equação de Poisson 2D em Coordenadas Cilíndricas:
\[
\frac{\partial^2 \psi}{\partial r^2} + \frac{1}{r} \frac{\partial \psi}{\partial r} - k^2 \psi = 0;
\]
Potencial de Superfície:
\[
\psi_{Si}(z) = C_i \exp(-\frac{z}{L_d}) + D_i \exp(-\frac{z}{L_d}) + \psi_{Si}(0);
\]
Comprimento Característico:
\[
L_d = \sqrt{\frac{T_{Si}}{2 \ln(1 + \frac{T_{ox}}{T_{Si}})}};
\]
Corrente de Dreno (Modelo de Kane):
\[
I_D = \rho \exp(-\frac{4 \sqrt{2 m^*} E_g}{3 q |hbar E_{avg}|});
\]

# Fontes

https://www.synopsys.com/blogs/chip-design/what-are-gate-all-around-gaa-transistors.html; https://resources.system-analysis.cadence.com/blog/msa2022-comparing-finfets-vs-gaafets; https://research-archive.org/index.php/rars/preprint/view/3111; https://arxiv.org/pdf/1406.5402; https://people.eecs.berkeley.edu/~tking/patents.t


# 24. Tecnologia de processo de 5nm e 4nm da TSMC

## Conteúdo Técnico

A tecnologia de processo de 5nm da TSMC, conhecida como N5, representa um avanço significativo na fabricação de semicondutores, oferecendo melhorias substanciais em densidade, desempenho e eficiência energética em comparação com a geração anterior de 7nm. A família de 5nm foi posteriormente expandida para incluir variantes otimizadas para diferentes aplicações, como N5P, N4, N4P, N4X e N5A.

## Tecnologia de 5nm (N5)

O processo N5 da TSMC, que entrou em produção em volume em 2020, é a segunda tecnologia da empresa a utilizar litografia de ultravioleta extremo (EUV), o que permite um design mais denso e eficiente. O N5 oferece uma melhoria de aproximadamente 1.8x na densidade lógica, um ganho de velocidade de 15% ou uma redução de 30% no consumo de energia em comparação com o processo de 7nm. A tecnologia também introduziu um canal de alta mobilidade (HMC), que se acredita ser um canal de SiGe para os dispositivos pMOS, proporcionando um ganho de desempenho de 18%.

As células SRAM no processo N5 também viram melhorias significativas, com duas variações principais:

*   Alta performance (HP): 0.025 μm²
*   Alta densidade (HD): 0.021 μm²

## Tecnologia de 4nm (N4, N4P, N4X)

A tecnologia de 4nm da TSMC é uma versão aprimorada da família de 5nm, oferecendo melhorias incrementais em desempenho, consumo de energia e densidade. A família de 4nm inclui os processos N4, N4P e N4X.

**TSMC N4P:** Oferece um ganho de desempenho de 11% em relação ao N5 (6% em relação ao N4), uma melhoria de 22% na eficiência energética em comparação com o N5 e um aumento de 6% na densidade de transistores em relação ao N5.

**TSMC N4X:** É otimizado para aplicações de computação de alto desempenho (HPC), oferecendo um ganho de desempenho de até 15% em comparação com o N5.

## Contribuidores Principais

J.C. Liu, S. Mukhopadhyay, Amit Kundu, S.H. Chen, H.C. Wang, D.S. Huang, et al. (TSMC)

## Fórmulas e Equações

Nenhuma fórmula ou equação matemática específica foi encontrada durante a pesquisa.

---


## Page 28

# Fontes

https://www.tsmc.com/english/dedicatedFoundry/technology/logic/l_5nm;https://en.wikipedia.org/wiki/5_nm_process;https://ftdetails-5-nm/;https://ieeexplore.ieee.org/document/9372009/

---

# 25. Cálculos de TDP de design térmico de CPU

## Conteúdo Técnico

O Thermal Design Power (TDP) é a quantidade máxima de calor que um componente de computador, como uma CPU, pode gerar e que seu sistema de refrigeração é projetado para dissipar. A fórmula da AMD para o TDP é (tCase°C - tAmbient°C) / (HSF θca), onde tCase é a temperatura máxima do invólucro, tAmbient é a temperatura ambiente e HSF θca é a resistência térmica do dissipador de calor. Para dispositivos móveis, uma abordagem diferente é usada, considerando gradientes térmicos no plano. A relação básica é TDP = h * A * (T_limit - T_ambient), onde h é o coeficiente de transferência de calor, A é a área externa, T_limit é a temperatura limite de toque e T_ambient é a temperatura ambiente. A eficiência da aleta (η_fin) e o parâmetro 'm' também são usados para cálculos mais precisos em dispositivos com gradientes de calor.

## Contribuidores Principais

AMD; Intel; GamersNexus; Electronics-Cooling

## Fórmulas e Equações

TDP (Watts) = (tCase°C - tAmbient°C) / (HSF θca)
TDP = h * A * (T_limit - T_ambient)
η_fin = tanh(m * L_c) / (m * L_c)
m = sqrt((h * P) / (k_eff * A_c))

## Fontes

https://en.wikipedia.org/wiki/Thermal_design_power;https://gamersnexus.net/guides/3525-amd-ryzen-tdp-explained-deep-dive-cooler-manufacturer-opinions;https://www.electronics-cooling.com/2023/03/calculating-thermal-design-power-for-mobile-consumer-electronics-part-1/

---

# 26. Interconnect fabric design NoC

## Conteúdo Técnico

Um Network-on-Chip (NoC) é um subsistema de comunicação baseado em rede em um circuito integrado, geralmente em um System-on-Chip (SoC). Ele usa uma rede de comutação de pacotes baseada em roteador para interconectar vários módulos ou núcleos de IP. As arquiteturas NoC geralmente modelam redes de mundo pequeno (SWNs) e redes sem escala (SFNs) para otimizar o comprimento, a área e o consumo de energia dos fios de interconexão. A topologia da rede, que define o layout físico e as conexões entre os nós, influencia significativamente a latência e o consumo de energia. A coerência do cache em NoCs é mantida por meio de protocolos como o MOESI (Modified, Owned, Exclusive, Shared, Invalid). A arquitetura NoC é frequentemente uma malha m x n de switches, onde os recursos são colocados nos slots formados pelos switches. A patente ‘Zero-latency network on chip (NoC)’ (US20110085550A1) introduz um método para configurar formatos de pacote usando dois parâmetros independentes: um para a largura dos dados e outro para a penalidade de latência, permitindo a criação de pacotes de baixa latência.

## Contribuidores Principais

S. Kumar, A. Jantsch, J.-P. Soininen, M. Forsell, M. Millberg, J. Oberg, Jean-Jacques Lecler, Philippe Boucard

## Fórmulas e Equações

T{tx} = T{ch} \times \frac{L+H}{w}

---


## Page 29

# Fontes

https://en.wikipedia.org/wiki/Network_on_a_chip;https://www.arteris.com/learn/network-on-chip-technology/;https://ieeexplore.ieee.org/document/1016885/;https://patents.google.com/patent/US20110085550A1/en;https://www.csa.iisc.ac.in/~skmandal/data/prelims_report_sumit_v3.pdf

---

# 27. CPU virtualization hardware support

## Conteúdo Técnico

A virtualização assistida por hardware é o uso de componentes físicos de um computador para dar suporte ao software que cria e gerencia máquinas virtuais (VMs). Essa abordagem de virtualização utiliza extensões de processador, como Intel VT-x e AMD-V, para lidar com tarefas que, de outra forma, seriam executadas em software. Isso melhora o desempenho e a eficiência da virtualização, permitindo que o hipervisor seja mais simples e robusto.

O suporte de hardware para virtualização envolve a adição de um novo modo de execução privilegiado ao processador. Esse modo, muitas vezes chamado de ‘root mode’, é usado pelo hipervisor, enquanto os sistemas operacionais convidados rodam em um modo menos privilegiado, o ‘non-root mode’. Quando um sistema operacional convidado tenta executar uma instrução privilegiada, o processador automaticamente intercepta a instrução e a entrega ao hipervisor, que pode então emular o comportamento da instrução para o sistema operacional convidado. Esse processo é conhecido como ‘VM exit’.

**Intel VT-x (Virtualization Technology):** A implementação da Intel de virtualização de hardware. Inclui um conjunto de instruções e um novo modo de execução de CPU que permite a um VMM (Virtual Machine Monitor) rodar em um nível de privilégio mais alto que os sistemas operacionais convidados.

**AMD-V (AMD Virtualization):** A tecnologia de virtualização da AMD, também conhecida como SVM (Secure Virtual Machine). Oferece funcionalidades semelhantes ao VT-x, incluindo a interceptação de instruções privilegiadas e o gerenciamento de memória para VMs.

**SLAT (Second Level Address Translation):** Também conhecido como RVI (Rapid Virtualization Indexing) na AMD e EPT (Extended Page Tables) na Intel, o SLAT é uma tecnologia de virtualização de memória que permite que o processador lide diretamente com a tradução de endereços de memória virtual para endereços de memória física para cada VM. Isso elimina a sobrecarga de emulação de tradução de endereço de memória no hipervisor, melhorando significativamente o desempenho.

**PAE (Physical Address Extension):** Uma tecnologia que permite que processadores de 32 bits acessem mais de 4 GB de memória física. É um requisito para algumas tecnologias de virtualização.

## Contribuidores Principais

Intel Corporation; Advanced Micro Devices (AMD); Mendel Rosenblum (VMware); Edouard Bugnion (VMware); Scott Devine (VMware)

## Fórmulas e Equações

N/A

## Fontes

https://www.techtarget.com/searchitoperations/definition/hardware-assisted-virtualization; https://docs.redhat.com/pt-br/documentation/red_hat_enterprise_linux/5/html/virtualization/ch-virt-hw-support; https://kib.kiev.ua/x86docs/Intel/VT-x/C97063-002.pdf; https://www.cs.wm.edu/~smherwig/readings/manuals/amd/sdm/amd64_arch_programmers_manual-vol2-system_programming.pdf; https://patents.google.com/patent/WO2010023695A1/en; https://portal.unifiedpatents.com/patents/patent/US-9852011-B1; https://patents.justia.com/patent/11347531

---


## Page 30

# 28. ARM TrustZone security architecture

## Conteúdo Técnico

# ARM TrustZone Security Architecture

## 1. Conceitos Fundamentais

A tecnologia ARM TrustZone é uma extensão de segurança de hardware que fornece um ambiente de execução seguro, dividindo os recursos do sistema em dois mundos: o **Mundo Seguro (Secure World)** e o **Mundo Normal (Normal World)**. Essa divisão permite que o software crítico para a segurança e os dados confidenciais sejam isolados do restante do sistema, reduzindo a superfície de ataque e protegendo contra ameaças de software.

*   **Mundo Seguro (Secure World):** Destinado à execução de código confiável, como o kernel de um sistema operacional seguro, drivers de segurança e aplicativos que manipulam dados sensíveis. O Mundo Seguro tem acesso a todos os recursos do sistema.
*   **Mundo Normal (Normal World):** Destinado à execução de um sistema operacional de propósito geral (como Linux ou Android) e aplicativos não confiáveis. O acesso do Mundo Normal aos recursos do sistema é restrito e controlado pelo Mundo Seguro.

## 2. Arquitetura e Design

A arquitetura TrustZone é implementada tanto no processador quanto no barramento do sistema e periféricos. A seguir, são detalhados os principais componentes da arquitetura.

### 2.1. Estados do Processador e Níveis de Exceção

Com a introdução das extensões de segurança, um processador ARM possui dois estados de segurança: **Seguro (Secure)** e **Não Seguro (Non-secure)**. Essa distinção é ortogonal aos níveis de privilégio do processador.

*   **ARMv7-A:**
    *   Níveis de Privilégio (PL): PL0 (User), PL1 (Kernel), PL2 (Hypervisor).
    *   O Modo Monitor (Monitor Mode), executado em PL1, é introduzido para gerenciar a transição entre os mundos Seguro e Não Seguro.
*   **ARMv8-A:**
    *   Níveis de Exceção (EL):
        *   EL0: Aplicativos (User Mode)
        *   EL1: Kernel do SO (Kernel Mode)
        *   EL2: Hypervisor
        *   EL3: Secure Monitor (o nível mais privilegiado)

### 2.2. Secure Monitor Call (SMC)

A transição do Mundo Normal para o Mundo Seguro é realizada através da instrução **Secure Monitor Call (SMC)**. Quando um aplicativo no Mundo Normal precisa de um serviço do Mundo Seguro, ele executa uma instrução SMC, que gera uma exceção e transfere o controle para o **Secure Monitor**, que é o software em execução no nível de exceção mais alto (EL3 no ARMv8-A ou Modo Monitor no ARMv7-A).

---


## Page 31

# 2.3. Particionamento de Memória

O TrustZone permite o particionamento da memória do sistema em regiões Seguras e Não Seguras. Isso é alcançado através de hardware adicional no sistema de memória:

*   **TrustZone Address Space Controller (TZASC):** Controla o acesso à DRAM, permitindo que regiões da memória sejam designadas como Seguras ou Não Seguras.
*   **TrustZone Memory Adapter (TZMA):** Permite a divisão da memória estática on-chip (SRAM) em regiões Seguras e Não Seguras.
*   **Unidade de Gerenciamento de Memória (MMU) / Unidade de Proteção de Memória (MPU):** Cada mundo (Seguro e Não Seguro) possui sua própria configuração de MMU/MPU, permitindo o isolamento da memória dentro de cada mundo.

# 2.4. Periféricos

Os periféricos também podem ser designados como Seguros ou Não Seguros. Um bit de configuração em cada periférico determina se ele pode ser acessado pelo Mundo Seguro, pelo Mundo Normal ou por ambos.

# 5. Registradores da SAU (Security Attribution Unit)

A SAU é responsável por definir as regiões de memória como Seguras ou Não Seguras. A seguir estão os principais registradores da SAU:

<table>
<thead>
<tr>
<th>Endereço</th>
<th>Nome</th>
<th>Tipo</th>
<th>Valor de Reset</th>
<th>Estado de Segurança do Processador</th>
<th>Descrição</th>
</tr>
</thead>
<tbody>
<tr>
<td>0xE000EDD0</td>
<td>SAU_CTRL</td>
<td>RW</td>
<td>0x00000000</td>
<td>Seguro</td>
<td>Registrador de Controle da SAU</td>
</tr>
<tr>
<td></td>
<td></td>
<td></td>
<td></td>
<td>Não-seguro</td>
<td>RAZ/WI</td>
</tr>
<tr>
<td>0xE000EDD4</td>
<td>SAU_TYPE</td>
<td>RO</td>
<td>0x0000000x</td>
<td>Seguro</td>
<td>Registrador de Tipo da SAU. Indica o número de regiões disponíveis.</td>
</tr>
<tr>
<td></td>
<td></td>
<td></td>
<td></td>
<td>Não-seguro</td>
<td>RAZ/WI</td>
</tr>
<tr>
<td>0xE000EDD8</td>
<td>SAU_RNR</td>
<td>RW</td>
<td>UNKNOWN</td>
<td>Seguro</td>
<td>Registrador de Número de Região da SAU. Seleciona uma região.</td>
</tr>
<tr>
<td></td>
<td></td>
<td></td>
<td></td>
<td>Não-seguro</td>
<td>RAZ/WI</td>
</tr>
<tr>
<td>0xE000EDDC</td>
<td>SAU_RBAR</td>
<td>RW</td>
<td>UNKNOWN</td>
<td>Seguro</td>
<td>Registrador de Endereço Base da Região da SAU.</td>
</tr>
<tr>
<td></td>
<td></td>
<td></td>
<td></td>
<td>Não-seguro</td>
<td>RAZ/WI</td>
</tr>
<tr>
<td>0xE000EDE0</td>
<td>SAU_RLAR</td>
<td>RW</td>
<td>UNKNOWN</td>
<td>Seguro</td>
<td>Registrador de Endereço Limite da Região da SAU.</td>
</tr>
<tr>
<td></td>
<td></td>
<td></td>
<td></td>
<td>Não-seguro</td>
<td>RAZ/WI</td>
</tr>
</tbody>
</table>

**Configuração de Região da SAU:**

*   As regiões são habilitadas individualmente usando SAU_RLAR .
*   A região é Não-segura quando SAU_RLAR.ENABLE = 1 e SAU_RLAR.NSC = 0 .
*   A região é Segura e “Non-secure callable” quando SAU_RLAR.ENABLE = 1 e SAU_RLAR.NSC = 1 .

**Contribuidores Principais**

ARM Holdings; Bernard Ngabonziza, Daniel Martin, Anna Bailey, Haehyun Cho, and Sarah Martin (Arizona State University); Felix Baum (Mentor Embedded)

---


## Page 32

# Fórmulas e Equações

N/A

# Fontes

https://www.arm.com/technologies/trustzone-for-cortex-m;https://sefcom.asu.edu/publications/trustzone-explained-cic2016.pdf;https://www.nxp.com/docs/en/supporting-information/FTF-DES-N2020-PDF.pdf;https://developer.arm.com/documentation/100690/latest/

---

# 29. CPU performance counters PMU

## Conteúdo Técnico

As Unidades de Monitoramento de Desempenho (PMUs) são componentes de hardware integrados a um processador para medir seus parâmetros de desempenho, como ciclos de instrução, acertos e falhas de cache, falhas de previsão de desvio e muitos outros. Os eventos de monitoramento de desempenho fornecem recursos para caracterizar a interação entre sequências programadas de instruções e subsistemas de microarquitetura. A maioria das CPUs possui uma PMU com contadores fixos e programáveis. Um PMC (Contador de Monitoramento de Desempenho) fixo sempre mede a mesma coisa dentro do núcleo, enquanto um contador programável permite ao usuário escolher o que medir. Para a maioria dos processadores Intel Core, o número de contadores totalmente programáveis é 4 (por núcleo lógico) e geralmente 3 contadores de função fixa (por núcleo lógico). Os contadores de PMU e os registradores de configuração são implementados como registradores MSR (Model Specific Registers), o que significa que o número de contadores e sua largura podem variar de modelo para modelo. Os MSRs são acessados por meio das instruções RDMSR e WRMSR.

## Contribuidores Principais

Intel Corporation; Microsoft Corporation; Denis Bakhvalov (easyperf.net)

## Fórmulas e Equações

XML para WPR:

```xml
<SystemProvider Id="SystemProvider_General_Mobile">
    <Keywords>
        <Keyword Value="ProcessThread" />
        <Keyword Value="Loader" />
        <Keyword Value="CSwitch" />
    </Keywords>
</SystemProvider>

<HardwareCounter Id="HC_PerfWorkloads.Base" Base="" Strict="true">
    <Counters>
        <Counter Value="TotalCycles"/>
        <Counter Value="InstructionRetired"/>
    </Counters>
    <Events>
        <Event Value="CSwitch"/>
    </Events>
</HardwareCounter>
```

Comando para Xperf.exe:

```bash
xperf.exe -on <tracing_flags> -pmc counters events [strict]
```

XML para amostragem em WPR:

---


## Page 33

xml
<SystemProvider Id="SystemProvider_General_Mobile">
  <Keywords>
    <Keyword Value="ProcessThread" />
    <Keyword Value="Loader" />
    <Keyword Value="PmcProfile" />
  </Keywords>
</SystemProvider>
<HardwareCounter Id="HC_Sampling.Base" Base="" Strict="true">
  <SampledCounters>
    <SampledCounter Value="InstructionRetired" Interval="100000"/>
  </SampledCounters>
</HardwareCounter>
```

# Fontes

https://perfmon-events.intel.com/; https://learn.microsoft.com/en-us/windows-hardware/test/wpt/recording-pmu-events; https://easyperf.net/blog/2018/06/01/PMU-counters-and-profiling-basics

---

# 30. Otimização de Compilador para a Arquitetura ARM

## Conteúdo Técnico

### Conceitos Fundamentais de Otimização de Compiladores para ARM

A otimização de compiladores para a arquitetura ARM visa melhorar a eficiência do código executável, seja em termos de velocidade de execução (performance) ou de tamanho do código. Frequentemente, essas duas metas são conflitantes. Técnicas que aumentam a performance, como o *loop unrolling* (desenrolar laços), podem aumentar o tamanho do código, enquanto técnicas para reduzir o tamanho do código podem impactar negativamente a performance.

Os compiladores modernos, como o `armclang` (baseado em LLVM) e o GCC, oferecem múltiplos níveis de otimização, controlados por flags específicas.

### Níveis e Tipos de Otimização

As otimizações são geralmente categorizadas em níveis, permitindo ao desenvolvedor escolher o grau de otimização desejado:

*   **Níveis de Performance (-o):**
    *   `-00`: O nível padrão, que não aplica otimizações, garantindo a correspondência mais direta entre o código-fonte e o código compilado, o que facilita a depuração.
    *   `-01`: Aplica otimizações locais que não exigem um grande esforço de compilação, como otimizações de laços simples e alocação de variáveis em registradores.
    *   `-02`: Ativa otimizações globais (no escopo de um único arquivo), como eliminação de código morto, agendamento de instruções e otimizações de laços mais complexas.
    *   `-03`: Ativa otimizações mais agressivas, como o *inlining* de funções (substituição da chamada de uma função pelo seu corpo) e otimizações interprocedurais.
    *   `-0fast`: Habilita todas as otimizações do `-03` e outras que podem não estar em conformidade com os padrões estritos de ponto flutuante da linguagem.
*   **Níveis de Tamanho de Código:**
    *   `-0s`: Realiza otimizações para reduzir o tamanho do código, mesmo que isso possa levar a uma pequena perda de performance.
    *   `-0z`: Foca agressivamente na redução do tamanho do código, sendo ideal para sistemas com memória restrita.

---


## Page 34

*   **Otimização em Tempo de Link (LTO - Link-Time Optimization):**
    *   A flag `-flto` permite que o otimizador opere em todos os arquivos-fonte do projeto durante a fase de linkagem. Isso proporciona uma visão global do programa, permitindo otimizações interprocedurais mais eficazes, como o *inlining* de funções entre diferentes arquivos e a remoção de código não utilizado em uma escala mais ampla.

## Técnicas Específicas de Otimização

Diversas técnicas são empregadas pelos compiladores para otimizar o código para a arquitetura ARM:

*   **Agendamento de Instruções (Instruction Scheduling):** Reordena as instruções para maximizar o paralelismo no pipeline do processador e ocultar latências de memória, melhorando a performance.
*   **Alocação de Registradores (Register Allocation):** Atribui as variáveis mais frequentemente acessadas aos registradores da CPU, que têm um tempo de acesso muito menor que a memória RAM.
*   **Inlining de Funções:** Reduz o *overhead* de chamadas de função ao substituir a chamada pelo próprio corpo da função. É um compromisso entre performance e tamanho de código.
*   **Otimizações de Laço (Loop Optimizations):** Inclui técnicas como *loop unrolling*, que reduz o *overhead* do controle do laço; *loop fusion*, que combina múltiplos laços; e *loop-invariant code motion*, que move cálculos constantes para fora do laço.
*   **Eliminação de Código Morto (Dead Code Elimination):** Remove código que nunca pode ser alcançado durante a execução do programa.
*   **Simplificação de Expressões:** Transforma expressões matemáticas em formas equivalentes que podem ser calculadas de forma mais eficiente.
*   **Seleção de Instruções e Idiomas de Máquina:** O compilador seleciona as instruções ARM mais eficientes para uma determinada operação e reconhece padrões de código (idiomas de máquina) que podem ser substituídos por uma sequência de instruções otimizada.

## Frameworks de Otimização

Pesquisas na área, como o trabalho de Mahalingam e Asokan, focam na criação de frameworks para aprimorar compiladores como o GCC para a arquitetura ARM. A abordagem principal é melhorar a seleção de instruções e, crucialmente, a **ordem de aplicação das otimizações**, pois a eficácia de uma otimização pode depender da execução prévia de outra.

## Contribuidores Principais

P R Mahalingam (Rajagiri School of Engineering & Technology); Shimmi Asokan (Rajagiri School of Engineering & Technology); ARM Holdings; Texas Instruments

## Fórmulas e Equações

As otimizações de compilador são controladas principalmente por flags, que podem ser consideradas as ‘fórmulas’ para instruir o compilador sobre como proceder:

*   `-00`: Nível de otimização mínimo.
*   `-01`: Otimizações locais básicas.
*   `-02`: Otimizações globais no nível de arquivo.
*   `-03`: Otimizações agressivas, incluindo inlining de funções e análise interprocedural.
*   `-ofast`: Inclui todas as otimizações do `-03` mais outras que podem violar padrões estritos da linguagem.
*   `-Os`: Otimiza para tamanho de código, balanceando com a performance.
*   `-Oz`: Otimiza agressivamente para o menor tamanho de código.
*   `-flto`: Ativa a Otimização em Tempo de Link (Link-Time Optimization - LTO).

---


## Page 35

# Fontes

https://developer.arm.com/documentation/dui0773/e/Optimization;https://www.ti.com/lit/pdf/spnu151v;https://dl.acm.org/doi/1community-blogs/b/architectures-and-processors-blog/posts/optimizing-your-programs-for-arm-platforms

# Conclusão

Esta pesquisa consolidou informações técnicas abrangentes sobre a CPU Google Axion e a arquitetura ARM Neoverse V2 subjacente. Os dados coletados incluem especificações de microarquitetura, fórmulas matemáticas de predição de desvio, algoritmos de execução fora de ordem, física de semicondutores, e a genealogia completa dos contribuidores principais.

A arquitetura Axion representa um marco significativo na estratégia de silício customizado do Google, combinando o núcleo ARM Neoverse V2 de alto desempenho com a tecnologia Titanium de offloads para criar uma solução otimizada para cargas de trabalho de data center.