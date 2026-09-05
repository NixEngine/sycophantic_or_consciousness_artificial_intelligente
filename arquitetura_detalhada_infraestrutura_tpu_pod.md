## Page 1

# Arquitetura Detalhada da Infraestrutura TPU Pod

## 1. Introdução

Este documento detalha a arquitetura completa da infraestrutura de orquestração e interconexão dos Google TPU Pods, incluindo:

*   **Borg** - Sistema de gerenciamento de cluster
*   **Pod Manager** - Orquestrador de processos TPU
*   **OCS (Optical Circuit Switch)** - Switch óptico para interconexões
*   **libtpunet** - Biblioteca de roteamento de rede TPU
*   **healthd** - Daemon de monitoramento de saúde
*   **Arquitetura física** - Fiação, conexões de chips, portos, estrutura interna e externa

## 2. Hierarquia de Controle do TPU Pod

```mermaid
graph TD
    A[BORG<br>(Cluster Manager Global)] --> B[POD MANAGER<br>(Orquestrador de TPU Pod)]
    B --> C[OCS<br>(Switch)]
    B --> D[libtpunet<br>(Roteamento)]
    B --> E[healthd<br>(Monitora.)]
    C --> F[TPU CHIPS / TRAYS<br>(Hardware de Processamento)]
    D --> F
    E --> F

---


## Page 2

# 3. Pesquisa técnica detalhada sobre a arquitetura do gerenciador de cluster Google Borg e a orquestração de TPUs.

## Arquitetura e Design

### Arquitetura do Borg

A arquitetura do Borg é baseada em um modelo mestre-escravo, com um componente central, o BorgMaster, e agentes em cada máquina, os Borglets. O agendamento de tarefas é realizado por um serviço independente, o Scheduler.

**BorgMaster:**

*   Ponto central de controle do cluster.
*   Mantém o estado de todos os objetos do sistema (máquinas, tarefas, alocações).
*   Utiliza o algoritmo Paxos para garantir alta disponibilidade através da replicação do estado em múltiplas réplicas.
*   Expõe uma API para os clientes e para os Borglets.

**Borglet:**

*   Agente local que roda em cada máquina do cluster.
*   Responsável por iniciar e parar tarefas, gerenciar recursos locais (CPU, memória, etc.) e reportar o estado da máquina e das tarefas para o BorgMaster.

**Scheduler:**

*   Processo separado que encontra locais viáveis para as tarefas e toma decisões de agendamento.
*   Opera de forma assíncrona, utilizando uma cópia em cache do estado do cluster para tomar decisões.
*   O processo de agendamento envolve duas fases: verificação de viabilidade (encontrar máquinas que atendem aos requisitos da tarefa) e pontuação (escolher a melhor máquina com base em critérios como empacotamento, localidade de dados e minimização de preempções).

### Fluxo de Comandos e Dados:

1.  Um usuário submete um “job” (trabalho) ao BorgMaster através de uma ferramenta de linha de comando ou de uma interface web.
2.  O BorgMaster registra o job e o coloca em uma fila de pendências.
3.  O Scheduler, que monitora constantemente a fila de pendências, identifica o novo job.
4.  O Scheduler executa seu algoritmo de agendamento para encontrar uma máquina adequada para cada tarefa do job.
5.  O Scheduler informa ao BorgMaster a máquina designada para cada tarefa.
6.  O BorgMaster atualiza o estado da tarefa e notifica o Borglet da máquina correspondente.
7.  O Borglet inicia a tarefa na máquina, utilizando contêineres Linux para isolamento de recursos.
8.  O Borglet monitora a tarefa e reporta seu estado de volta ao BorgMaster.

## Arquitetura de TPU

*   **TPU Chip:** Contém um ou mais TensorCores. Cada TensorCore consiste em uma ou mais unidades de multiplicação de matriz (MXUs), uma unidade vetorial e uma unidade escalar.
*   **MXU (Matrix-Multiply Unit):** Composta por uma matriz sistólica de multiplicadores-acumuladores (256x256 para TPU v6e e mais recentes, 128x128 para versões anteriores). Realiza 16K operações de multiplicação-acumulação por ciclo. As multiplicações usam bfloat16, e as acumulações usam FP32.
*   **Unidade Vetorial:** Usada para computação geral, como ativações e softmax.

---


## Page 3

*   **Unidade Escalar:** Usada para controle de fluxo, cálculo de endereços de memória e outras operações de manutenção.
*   **TPU Pod:** Um conjunto contíguo de TPUs agrupados em uma rede especializada.
*   **Slice:** Uma coleção de chips dentro do mesmo TPU Pod, conectados por interconexões de alta velocidade (ICI).
*   **Multislice:** Um grupo de slices que estende a conectividade além das ICIs, usando a rede do data center (DCN) para paralelismo entre slices.
*   **TPU Cube:** Uma topologia 4x4x4 de chips TPU interconectados (a partir do TPU v4).
*   **SparseCore:** Processadores de fluxo de dados que aceleram modelos com operações esparsas, principalmente para modelos de recomendação. v5p e TPU7x têm quatro SparseCores por chip, e v6e tem dois.
*   **Resiliência de ICI do Cloud TPU:** Melhora a tolerância a falhas de links ópticos e switches de circuito óptico (OCS) que conectam TPUs entre cubos.

## Conexões Físicas

### Conexões Físicas de TPU

*   **Interconexão no Chip:** Os TensorCores dentro de um chip de TPU são interconectados para formar uma matriz sistólica.
*   **Interconexão entre Chips (ICI):** Links de alta velocidade que conectam os chips de TPU dentro de um “slice”.
*   **Interconexão entre Racks (OCS):** Switches de circuito óptico (Optical Circuit Switches) são usados para interconectar os racks de TPUs, permitindo topologias de rede reconfiguráveis. Isso é uma característica chave da arquitetura do TPU v4 e posteriores.
*   **Cabos:** Cabos de cobre (DACs - Direct Attach Cables) são usados para interconexões dentro de um mesmo rack, enquanto fibras ópticas são usadas para interconexões entre racks.
*   **Placa de Circuito Impresso (PCB):** As placas de TPU v4 contêm quatro chips de TPU e são refrigeradas a líquido.
*   **Interface com o Host:** A conexão com a máquina host é feita através de PCIe Gen3 x16 (para TPU v4).

## Protocolos e Software

### Protocolos e Software do Borg

#### Protocolos de Comunicação:

*   O BorgMaster se comunica com os Borglets através de RPCs (Remote Procedure Calls).
*   A distribuição de pacotes para as máquinas é feita de forma paralela usando protocolos em árvore e do tipo torrent.
*   A comunicação entre os serviços dentro da rede de produção do Google é protegida pela Application Layer Transport Security (ALTS).

#### APIs e Comandos:

*   Os usuários interagem com o Borg através de uma linguagem de especificação de trabalho declarativa, geralmente em BCL (Borg Configuration Language).
*   Os comandos são submetidos através de uma ferramenta de linha de comando chamada borg .
*   O BorgMaster expõe uma API HTTP para os clientes e para os Borglets.

#### Software e Sequência de Inicialização:

*   O software principal consiste no BorgMaster, Borglet e Scheduler.
*   A sequência de inicialização de um trabalho envolve a submissão do trabalho ao BorgMaster, o agendamento pelo Scheduler, a atribuição a um Borglet e a inicialização da tarefa pelo Borglet.

---


## Page 4

# Protocolos e Software de Orquestração de TPU

## Protocolos de Comunicação:

*   **ICI (Inter-chip Interconnect):** Protocolo de alta velocidade para comunicação entre chips de TPU dentro de um slice.
*   **RDMA (Remote Direct Memory Access):** As TPUs utilizam um modelo de comunicação push-only, onde uma TPU pode emitir uma instrução de cópia para enviar dados de um buffer local para um buffer remoto em outra TPU.
*   **gRPC:** A arquitetura de TPU Node (depreciada) usava gRPC para a comunicação entre a VM do usuário e o host da TPU.

## APIs e Modelo de Programação:

*   **XLA (Accelerated Linear Algebra):** O compilador XLA é fundamental para a execução de código em TPUs. Ele compila os modelos de machine learning para o hardware específico da TPU.
*   **Frameworks de ML:** A programação para TPUs é feita através de frameworks de machine learning como TensorFlow, PyTorch e JAX.
*   **APIs de Alto Nível:** Frameworks como o TensorFlow fornecem APIs de alto nível, como `TPUStrategy`, para facilitar o treinamento distribuído em TPUs.
*   **Edge TPU API:** Para dispositivos Edge TPU, a API `edgetpu` (baseada no TensorFlow Lite) é usada para realizar inferência.

## Software Stack:

*   O software stack para TPUs inclui o sistema operacional da VM do host (Linux), os drivers da TPU, o compilador XLA e os frameworks de machine learning (TensorFlow, PyTorch, JAX).

# Fórmulas e Especificações

## Especificações e Fórmulas

### Largura de Banda (Bandwidth):

*   **TPU v3:** 34 GB/s de largura de banda de memória (DDR3).
*   **TPU v4:** Largura de banda de bisseção por Pod de 24 TB/s. Largura de banda all-reduce por Pod de 1.1 PB/s. Largura de banda PCIe de 16 GB/s em cada direção.
*   **TPU v6e:** 13 TB/s de largura de banda de ICI por chip.
*   **TPU v5p:** 4.800 GB/s de largura de banda de ICI por chip.

### Latência (Latency):

*   A arquitetura de TPU é projetada para baixa latência, especialmente com a rede ICI e os OCS, que fornecem conexões diretas entre os chips, evitando a sobrecarga de redes elétricas tradicionais.

### Consumo de Energia (Power Consumption):

*   **TPU v1:** 40W em execução.
*   **TPU v4:** Consumo típico entre 175W e 250W por unidade.
*   **Edge TPU:** O design base visa 512 GOPs/10 mW.

### Capacidade Térmica (Thermal Capacity):

*   As TPUs, especialmente as versões mais recentes como a v4, utilizam refrigeração líquida para gerenciar a alta densidade de potência e permitir o bursting de desempenho para atender aos SLAs de tempo de serviço.

### Fórmulas de Desempenho:

*   O desempenho de uma TPU é frequentemente medido em TOPS (Trillion Operations Per Second) ou PetaFLOPS (Floating Point Operations Per Second).

---


## Page 5

*   **FLOPS (Floating Point Operations Per Second):** Uma medida do número de operações de ponto flutuante que um processador pode executar por segundo. Para uma matriz sistólica, pode ser calculado como: FLOPS = 2 * (tamanho_da_matriz)^2 * frequência_do_clock * número_de_MXUs (O fator de 2 vem da operação de multiplicação-acumulação, que conta como duas operações).
*   **TOPS (Trillion Operations Per Second):** Similar ao FLOPS, mas para operações de inteiros. TOPS = 2 * (tamanho_da_matriz)^2 * frequência_do_clock * número_de_MXUs / 10^12

## Vulnerabilidades Identificadas

## Vulnerabilidades e Pontos de Acesso

### Google Borg:

*   **Isolamento de Cargas de Trabalho:** O Borg historicamente dependia de chroot jails do Linux para isolamento de segurança, o que significa que uma vulnerabilidade no kernel poderia permitir que uma tarefa maliciosa escapasse do seu ambiente isolado e afetasse outras tarefas na mesma máquina.
*   **Cadeia de Suprimentos de Software:** A segurança do Borg depende fortemente de processos internos como a “Binary Authorization for Borg” (BAB), que garante que apenas código revisado e autorizado seja executado. Uma falha nesse processo poderia permitir a execução de código malicioso.
*   **Pontos de Acesso:** A API do BorgMaster e as ferramentas de linha de comando são pontos de acesso que precisam ser devidamente protegidos. Uma vulnerabilidade de Server-Side Request Forgery (SSRF) foi encontrada no passado em serviços adjacentes, o que teoricamente poderia permitir o acesso à rede interna do Borg.

### TPUs:

*   **Ataques de Canal Lateral (Side-Channel Attacks):** Pesquisas como a “TPUXtract” demonstraram a possibilidade de extrair informações sobre modelos de redes neurais (como hiperparâmetros) através da análise do campo eletromagnético emitido pelo chip da TPU durante a computação. Isso representa um vetor de ataque físico.
*   **Ataques de Injeção de Falhas:** A manipulação da tensão de alimentação ou do clock da TPU pode induzir falhas nos cálculos, o que pode ser explorado para contornar medidas de segurança ou extrair informações do modelo.
*   **Segurança do Hardware:** Como qualquer hardware complexo, as TPUs podem ter vulnerabilidades de hardware, como Trojans de hardware inseridos durante o processo de fabricação, embora isso seja considerado um vetor de ataque sofisticado e de alto custo.
*   **Acesso ao Host:** A arquitetura TPU VM dá acesso root à máquina host, o que significa que uma vulnerabilidade no sistema operacional da VM ou no software de gerenciamento pode expor o dispositivo TPU a ataques.

## Fontes

https://research.google.com/pubs/large-scale-cluster-management-at-google-with-borg/
https://research.google.com/pubs/archive/43438.pdf
https://noneback.github.io/blog/borg/
https://www.servethehome.com/google-details-tpuv4-and-its-crazy-optically-reconfigurable-ai-network/
https://opnsec.com/2018/07/into-the-borg-ssrf-inside-google-production-network/
https://www.usenix.org/publications/loginonline/workload-security-rings
look-at-googles-first-tensor-processing-unit-tpu
https://www.tensorflow.org/guide/tpu
https://medium.com/@scottbolen/understanding-tpuxtract-a-novel-ai-model-extraction-threat-08e352fc702f
https://nquiringminds.com/cybernews/tpuxtract-new-sidechannel-attack-exposes-vulnerabilities-in-ai-models/
https://dl.acm.org/doi/10.1145/2741948.2741964
https://cloud.google.com/tpu/docs/system-architecture-tpu-vm
https://medium.com/@adityashete009/borg-large-scale-cluster-management-system-cbdcc4f8eb91
https://cloud.google.com/blog/products/ai-machine-learning/an-in-depth-look-at-googles-first-tensor-processing-unit-tpu
https://www.coral.ai/docs/edgetpu/api-intro/
https://jax.dev/en/latest/pallas/tpu/distributed.html

---


## Page 6

# 4. Pesquisa técnica detalhada sobre o agendador Google Borg e a alocação de recursos para pods de TPU.

## Arquitetura e Design

### Arquitetura do Google Borg

O Google Borg é um sistema de gerenciamento de cluster que executa centenas de milhares de jobs, de milhares de aplicações diferentes, em vários clusters, cada um com dezenas de milhares de máquinas. A arquitetura do Borg é projetada para alta utilização de recursos, combinando controle de admissão, empacotamento eficiente de tarefas, over-commitment e compartilhamento de máquinas com isolamento de desempenho em nível de processo.

### Componentes Principais

A arquitetura do Borg consiste em dois componentes principais:

*   **Borgmaster:** É o controlador centralizado de uma célula Borg. Ele é responsável por gerenciar o estado da célula, receber jobs dos usuários e orquestrar o agendamento de tarefas nas máquinas. O Borgmaster é replicado cinco vezes para alta disponibilidade e usa o Paxos para garantir a consistência entre as réplicas.
*   **Borglet:** É um agente que roda em cada máquina em uma célula Borg. Ele é responsável por iniciar e parar tarefas, gerenciar recursos locais e relatar o estado da máquina para o Borgmaster.

### Hierarquia de Controle e Fluxo

1.  **Submissão de Job:** Os usuários submetem seus trabalhos ao Borg na forma de jobs, que consistem em uma ou mais tarefas que executam o mesmo programa. Os jobs são descritos em uma linguagem de configuração declarativa chamada BCL (Borg Configuration Language).
2.  **Admissão e Agendamento:** O Borgmaster admite o job se o usuário tiver quota de recursos suficiente. Em seguida, o agendador do Borgmaster encontra máquinas adequadas para as tarefas do job, levando em conta as restrições do job e os recursos disponíveis.
3.  **Execução da Tarefa:** Uma vez que uma tarefa é agendada para uma máquina, o Borgmaster instrui o Borglet naquela máquina a iniciar a tarefa. O Borglet então baixa os binários e os dados da tarefa, cria um contêiner Linux para isolamento e executa os processos da tarefa.
4.  **Monitoramento e Gerenciamento:** O Borglet monitora a saúde da tarefa e relata seu estado de volta ao Borgmaster. Se uma tarefa falhar, o Borgmaster a reiniciará, possivelmente em uma máquina diferente.

## Arquitetura do TPUv4

A arquitetura do TPUv4 é projetada para escalabilidade e resiliência, utilizando uma abordagem de rede definida por software (SDN) para gerenciar a interconexão de alta largura de banda entre os chips (ICI).

### Componentes de Hardware

*   **Cubes:** Uma unidade de hardware com 64 chips de TPU arranjados em uma malha 3D de 4x4x4. Cada supercomputador (ou pod) tem 64 cubos, totalizando 4096 TPUs.
*   **Interconexão Inter-Chip (ICI):** Uma malha de rede de alta velocidade que interconecta diretamente os TPUs, permitindo a comunicação direta de dispositivo para dispositivo (RDMA) sem envolver as CPUs.
*   **Optical Circuit Switches (OCS):** Usados para conectar dinamicamente a ICI de diferentes cubos para formar a topologia de toro solicitada pelo usuário.

---


## Page 7

# Componentes de Software

*   **Borg**: O gerenciador de cluster que admite, agenda e gerencia os jobs do TPUv4.
*   **Pod Manager**: Um serviço de software em nível de cluster que gerencia a conectividade multi-cubo, acionando a configuração de conexão cruzada do OCS em resposta às decisões de agendamento do Borg.
*   **libtpunet**: Uma biblioteca de software que configura a topologia de rede ICI solicitada para cada job de usuário do TPUv4.
*   **healthd**: Um daemon de software executado em cada host em um pod que monitora continuamente a saúde do hardware da máquina e reporta aos sistemas de software em nível de cluster.

# Conexões Físicas

## Conexões Físicas do TPUv4

A infraestrutura do TPUv4 é projetada para alta largura de banda e baixa latência, com um foco em reconfigurabilidade para resiliência e eficiência.

## Fiação e Conectores

*   **Interconexão Inter-Chip (ICI)**: Os chips de TPU são diretamente interconectados através de uma malha de alta velocidade. Cada link ICI pode transportar 50 GBps de largura de banda unidirecional.
*   **Optical Circuit Switches (OCS)**: A interconexão entre os cubos de TPU é realizada através de switches de circuito óptico (OCS). Cada cubo expõe 16 links ICI em cada uma das suas 6 faces (X, Y, Z) para os OCSs, totalizando 96 conexões ópticas por cubo. Um supercomputador TPUv4 com 64 cubos possui 6144 links ICI ópticos conectados a 48 OCSs distintos.
*   **Conexão Host-TPU**: Cada máquina TPU individual consiste em uma bandeja de CPU e uma bandeja de TPU, conectadas através de um link PCIe.

## Layout e Topologia

*   **Cubo de TPU**: A unidade fundamental de computação é o “cubo”, que consiste em 64 chips de TPU dispostos em uma malha 3D de 4x4x4.
*   **Máquina TPU**: Dentro de um cubo, 16 máquinas TPU são agrupadas. Cada máquina contém 4 chips de TPU em uma malha de 2x2x1.
*   **Topologia de Rede**: A arquitetura suporta topologias de toro 3D ou toro torcido 3D, que podem ser configuradas dinamicamente para cada job através dos OCSs. Isso permite que jobs utilizem cubos que não são fisicamente contíguos, mitigando a fragmentação de recursos.

# Protocolos e Software

## Protocolos e Software do TPUv4

A pilha de software e protocolos do TPUv4 é projetada para ser programável, permitindo resiliência e reconfigurabilidade em escala.

### Protocolo ICI (Inter-Chip Interconnect)

O protocolo ICI é uma pilha em camadas que permite a comunicação de alta velocidade entre os TPUs:

*   **Camada de Transação (RDMA)**: A camada superior, exposta aos programadores através do XLA (Accelerated Linear Algebra), que permite a comunicação direta de memória para memória (RDMA) entre TPUs.
*   **Camada de Roteamento**: As tabelas de encaminhamento de pacotes são programadas pela libtpunet com balanceamento de carga global. Cada pacote é roteado com base no ID do chip de destino.

---


## Page 8

*   **Camada de Dados:** Garante a entrega ordenada de pacotes com retransmissão automática em caso de perda de dados na camada física. Implementa controle de fluxo baseado em crédito no nível do link.
*   **Camada Física:** Gerencia o treinamento e a conectividade dos links. O daemon `healthd` monitora continuamente a qualidade e a conectividade dos links.

## Software de Gerenciamento

*   **Borg:** O sistema de gerenciamento de cluster do Google que admite, agenda e gerencia os jobs nos TPU Pods.
*   **Pod Manager:** Um serviço em nível de cluster que gerencia a conectividade entre os cubos de TPU, configurando os Optical Circuit Switches (OCS) com base nas decisões de agendamento do Borg.
*   **libtpunet:** Uma biblioteca de software que configura a topologia de rede ICI solicitada para cada job, programa as tabelas de roteamento e gerencia as sessões do ICI.
*   **healthd:** Um daemon executado em cada host que monitora continuamente a saúde do hardware da máquina e reporta aos sistemas de software em nível de cluster.
*   **BCL (Borg Configuration Language):** A linguagem de configuração declarativa usada para descrever os jobs do Borg.

## Sequência de Inicialização e Comandos de Controle

1.  Um job é submetido ao Borg usando uma especificação em BCL.
2.  O Borg, em conjunto com o Pod Manager, seleciona os cubos de TPU disponíveis e configura os OCSs para criar a topologia de rede solicitada (por exemplo, um toro 3D).
3.  A libtpunet em cada host do job configura as tabelas de roteamento do ICI para otimizar o fluxo de pacotes.
4.  O Borglet em cada host inicia as tarefas do job nos contêineres Linux.
5.  Durante a execução, o healthd monitora continuamente a saúde do sistema. Em caso de falha, o sistema pode reconfigurar dinamicamente a rede para contornar o componente defeituoso.

## Fórmulas e Especificações

### Especificações do TPUv4

*   **Largura de Banda:** Cada link de Interconexão Inter-Chip (ICI) do TPUv4 possui uma largura de banda unidirecional de 50 GBps.
*   **Consumo de Energia:** O TPUv4 é projetado para ser eficiente em termos de energia. Embora os números exatos não sejam publicamente detalhados, estima-se que o consumo de energia seja significativamente menor em comparação com as gerações anteriores e GPUs contemporâneas, com algumas fontes sugerindo em torno de 100W por chip. A arquitetura geral, incluindo o uso de OCS, contribui para uma redução de até 83% no consumo de energia por nó.
*   **Capacidade Térmica:** O TPUv4 utiliza um sistema de refrigeração líquida e um design de chip bare-die para gerenciar a alta densidade térmica. A refrigeração líquida é até 3 vezes mais eficiente termicamente do que a refrigeração a ar, permitindo que o sistema opere em racks com mais de 80 kW de capacidade.

## Vulnerabilidades Identificadas

## Vulnerabilidades e Vetores de Ataque

A segurança da infraestrutura do Google, incluindo o Borg e os TPU Pods, é uma preocupação central, e várias camadas de segurança são implementadas para mitigar riscos. No entanto, como em qualquer sistema complexo, existem potenciais vulnerabilidades e vetores de ataque.

---


## Page 9

# Google Borg

*   **Isolamento de Cargas de Trabalho:** O Borg utiliza contêineres Linux (historicamente, chroot jails) para isolar as cargas de trabalho. Uma vulnerabilidade no kernel do Linux poderia, teoricamente, permitir que um processo malicioso escape do seu contêiner e afete outros processos na mesma máquina.
*   **Ataques de Negação de Serviço (DoS):** Um job mal configurado ou malicioso poderia consumir uma quantidade excessiva de recursos (CPU, memória, rede), potencialmente impactando outros serviços na mesma célula. O Borg mitiga isso com o gerenciamento de quotas e prioridades, mas a possibilidade de abuso ainda existe.
*   **Acesso não autorizado:** A exploração de uma vulnerabilidade em um serviço em execução no Borg poderia conceder a um invasor acesso à rede de produção do Google, como demonstrado em pesquisas de segurança anteriores (por exemplo, SSRF - Server-Side Request Forgery).

# TPU Pods

*   **Ataques de Canal Lateral (Side-Channel Attacks):** Pesquisas demonstraram a viabilidade de ataques de canal lateral contra TPUs, como o TPUXtract, que explora sinais eletromagnéticos para extrair informações sobre os modelos de redes neurais em execução.
*   **Segurança da Cadeia de Suprimentos de Software:** A integridade do software executado nos TPUs é crucial. O Google utiliza ferramentas como a Binary Authorization para garantir que apenas código verificado e autorizado seja executado em sua infraestrutura.
*   **Segurança Física:** Embora os data centers do Google tenham várias camadas de segurança física, qualquer comprometimento físico poderia levar ao acesso não autorizado aos servidores e, consequentemente, aos TPU Pods.

# Mitigações Gerais

O Google emprega uma abordagem de defesa em profundidade, que inclui:

*   **Criptografia em Trânsito e em Repouso:** Todo o tráfego de rede virtual na nuvem do Google é criptografado, e os dados em repouso também são criptografados.
*   **Gerenciamento de Identidade e Acesso (IAM):** O controle de acesso rigoroso garante que apenas usuários autorizados possam acessar e gerenciar recursos.
*   **Monitoramento e Detecção de Vulnerabilidades:** Ferramentas como o Security Command Center e a Artifact Analysis são usadas para detectar e remediar vulnerabilidades de segurança em toda a infraestrutura.

# Fontes

https://research.google.com/pubs/archive/43438.pdf https://www.usenix.org/system/files/nsdi24-zu.pdf

---

## 5. Google Pod Manager TPU orchestration control plane

### Arquitetura e Design

A arquitetura do supercomputador TPUv4 do Google é projetada para escalabilidade e resiliência, centrada em uma malha de interconexão reconfigurável. A unidade fundamental é o ‘cubo’, um arranjo de hardware com 64 chips TPU em uma malha 3D de 4x4x4. Um supercomputador, ou ‘pod’, consiste em 64 desses cubos, totalizando 4096 TPUs. A orquestração é gerenciada por um conjunto de componentes de software e hardware. O Pod Manager é um serviço de software em nível de cluster que gerencia a conectividade entre múltiplos cubos. Ele atua sobre os Optical Circuit Switches (OCS) para configurar dinamicamente as interconexões (xconnects) da Inter-Chip Interconnect (ICI) entre diferentes cubos, em resposta às decisões de agendamento do Borg, o gerenciador de cluster do Google. A hierarquia de controle é composta pelo Borg, que admite e agenda os trabalhos; o Pod Manager, que traduz as decisões de agendamento em configurações de OCS; a biblioteca libtpunet, que configura a topologia de rede ICI para cada trabalho do usuário; e o healthd, um daemon que monitora a saúde do hardware em cada host. O fluxo de dados e comandos segue essa hierarquia, com o Borg no topo, ditando a alocação de recursos, e o Pod Manager e a libtpunet implementando a configuração de baixo nível.

---


## Page 10

# Conexões Físicas

As conexões físicas no sistema TPUv4 são hierárquicas. Cada máquina TPU individual possui uma bandeja de CPU e uma bandeja de TPU, conectadas via PCIe. A bandeja de TPU contém 4 chips TPUv4 dispostos em uma malha ICI de 2x2x1. Dezesseis dessas máquinas TPU são agrupadas para formar um ‘cubo’, com os links ICI dentro do rack interconectados para formar uma malha de 4x4x4. Para a interconexão entre cubos, cada cubo expõe 16 ICIs em cada uma das seis faces das dimensões X, Y e Z para os switches ópticos, totalizando 96 ICIs por cubo. Um supercomputador TPUv4 completo, com 64 cubos, possui 6144 links ICI ópticos conectados a 48 Optical Circuit Switches (OCS) distintos. Os OCSs são baseados em tecnologia de sistemas microeletrônicos mecânicos (MEMS), utilizando espelhos para redirecionar os feixes de luz e estabelecer as conexões.

# Protocolos e Software

O protocolo de Inter-Chip Interconnect (ICI) é a base da comunicação de baixa latência e alta largura de banda entre os TPUs, suportando Remote Direct Memory Access (RDMA). Este protocolo é programável e estruturado em camadas. A camada física é responsável pelo treinamento das portas. A camada de dados confiável garante a entrega de pacotes em ordem, com retransmissão automática e controle de fluxo baseado em crédito. A camada de roteamento gerencia o encaminhamento de pacotes através de tabelas programadas pela libtpunet com balanceamento de carga global. A camada de transação expõe a funcionalidade RDMA para a camada de software superior (XLA). O software de controle inclui o Pod Manager, que responde às decisões de agendamento do Borg para atuar sobre os OCSs, e a libtpunet , que configura a topologia da rede ICI e as políticas de roteamento. A sequência de inicialização de um trabalho envolve a especificação da topologia pelo usuário, a seleção dos cubos pelo Borg, a configuração dos xconnects do OCS pelo Pod Manager e, finalmente, a configuração do roteamento ICI pela libtpunet .

# Fórmulas e Especificações

Largura de Banda por Link ICI: 50 GB/s (unidirecional) Consumo de Energia (OCS e Fibras Ópticas): < 3% da potência total do pod Custo de Capital (OCS e Fibras Ópticas): < 5% do custo total do pod Potência Média do Chip TPUv4: ~200 W

# Vulnerabilidades Identificadas

A pesquisa não revelou vulnerabilidades de segurança ou vetores de ataque no sentido tradicional. A resiliência do sistema é um foco principal, com a infraestrutura projetada para detectar e contornar falhas de hardware (máquinas, chips, links) automaticamente. A reconfiguração dinâmica através dos OCSs permite que o sistema isole componentes defeituosos e continue a operação, alcançando 99.98% de disponibilidade do sistema. Embora não seja uma vulnerabilidade de segurança, uma falha em um OCS ou um evento de manutenção pode degradar temporariamente o desempenho da interconexão, mas o sistema é projetado para rotear o tráfego em torno dessas falhas.

# Fontes

https://www.usenix.org/conference/nsdi24/presentation/zu https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm
https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://docs.cloud.google.com/tpu/docs/performance-guide

---

# 6. Coordenação de Agendamento de Processos do TPU Pod Manager

## Arquitetura e Design

A arquitetura de supercomputadores TPUv4 da Google é projetada para escalabilidade e resiliência, utilizando uma abordagem de co-design de hardware e software. A unidade fundamental é o “cubo”, um conjunto de 64 chips TPU dispostos em uma malha 3D de 4x4x4. Um supercomputador completo, ou “pod”, é composto por 64 desses cubos, totalizando 4096 TPUs. A interconexão entre os cubos é gerenciada por uma malha de interconexão de alta velocidade proprietária, a Inter-Chip Interconnect (ICI), que é dinamicamente reconfigurável através de Optical Circuit Switches (OCSes). Essa reconfigurabilidade permite que o sistema contorne falhas de hardware e otimize a alocação de recursos para diferentes trabalhos de treinamento de aprendizado de máquina.

---


## Page 11

A hierarquia de controle é liderada pelo Borg, o sistema de gerenciamento de cluster do Google, que é responsável por admitir, agendar e gerenciar os trabalhos no pod. O Borg toma as decisões de agendamento de alto nível, determinando quais cubos serão alocados para um determinado trabalho. Essas decisões são então comunicadas ao Pod Manager, um serviço de software de nível de cluster que gerencia a conectividade entre os cubos. O Pod Manager, por sua vez, controla os OCSes para estabelecer as conexões físicas (xconnects) necessárias para formar a topologia de rede solicitada pelo trabalho, como um toro 3D. O fluxo de dados e comandos é, portanto, de cima para baixo: do usuário para o Borg, do Borg para o Pod Manager, e do Pod Manager para a infraestrutura de hardware (OCSes e TPUs).

## Conexões Físicas

As conexões físicas em um pod TPUv4 são hierárquicas. Em um nível micro, cada máquina TPU individual consiste em uma bandeja de CPU e uma bandeja de TPU, que são conectadas através de um barramento PCle. Cada bandeja de TPU abriga 4 chips TPUv4, que são interligados em uma malha 2x2x1 através da Inter-Chip Interconnect (ICI). Dezesseis dessas máquinas TPU são então agrupadas em um único rack de datacenter para formar um “cubo” 4x4x4. As conexões ICI dentro de um cubo são elétricas.

Para a interconexão entre cubos, cada face do cubo 3D expõe 16 links ICI ópticos para os Optical Circuit Switches (OCSes), totalizando 96 links ICI por cubo. Um supercomputador TPUv4 completo possui 6144 links ICI ópticos que se conectam a 48 OCSes distintos. Esses OCSes, baseados na tecnologia de espelhos MEMS (Micro-Electro-Mechanical Systems), permitem a criação de circuitos de luz dedicados entre os cubos, possibilitando a reconfiguração dinâmica da topologia da rede para se adaptar às necessidades dos trabalhos e contornar falhas de hardware. Não foram encontradas informações detalhadas sobre pinagem, layout de PCB ou conectores específicos.

## Protocolos e Software

O software e os protocolos de comunicação em um pod TPUv4 são projetados para gerenciar a complexidade da arquitetura reconfigurável. Os principais componentes de software são:

*   **Borg:** O gerenciador de cluster do Google, que toma as decisões de agendamento de trabalhos.
*   **Pod Manager:** Um serviço que traduz as decisões de agendamento do Borg em configurações de hardware, controlando os OCSes para estabelecer as conexões entre os cubos.
*   **libtpunet:** Uma biblioteca de software que é executada em cada host e é responsável por configurar a rede ICI para cada trabalho do usuário. Isso inclui a programação das tabelas de encaminhamento de pacotes na camada de roteamento do ICI e o gerenciamento da camada de dados do protocolo ICI.
*   **healthd:** Um daemon que é executado em cada host, monitorando continuamente a saúde do hardware da máquina e relatando anomalias aos sistemas de nível de cluster, como o Borg.

O protocolo Inter-Chip Interconnect (ICI) é um protocolo proprietário, programável e em camadas que permite a comunicação RDMA (Remote Direct Memory Access) entre quaisquer dois TPUs no pod. As camadas do protocolo ICI são:

*   **Camada Física:** Gerenciada pelo chip_mgr e healthd, é responsável pelo treinamento da porta e pela criação das conexões físicas (xconnects) através do OCS.
*   **Camada de Dados:** Gerenciada pelo libtpunet e healthd, garante a entrega confiável e ordenada de pacotes, com recursos como retransmissão automática e controle de fluxo baseado em crédito.
*   **Camada de Roteamento:** As tabelas de encaminhamento de pacotes nesta camada são programadas pelo libtpunet para implementar balanceamento de carga global e políticas de roteamento específicas.
*   **Camada de Transação:** Esta camada expõe a funcionalidade RDMA para a aplicação do usuário, geralmente através de uma biblioteca de nível superior como a XLA.

## Fórmulas e Especificações

A pesquisa não revelou fórmulas matemáticas específicas para largura de banda, latência, consumo de energia ou capacidade térmica no contexto da coordenação de agendamento de processos do TPU Pod Manager. As informações disponíveis são métricas de desempenho de alto nível e especificações de hardware, em vez de equações.

---


## Page 12

# Vulnerabilidades Identificadas

Uma vulnerabilidade significativa identificada em TPUs é o ataque de canal lateral conhecido como “TPUXtract”. Este ataque explora as emanações eletromagnéticas (EM) do chip TPU durante a computação para recriar a rede neural que está sendo executada. O vetor de ataque envolve um invasor com acesso físico ao TPU usando uma sonda EM para capturar os sinais vazados. Ao construir “modelos” de combinações de hiperparâmetros simulados e compará-los com os sinais capturados, o invasor pode inferir a estrutura e o comportamento da rede neural. O impacto dessa vulnerabilidade inclui o roubo de propriedade intelectual (o próprio modelo de IA) e os dados nos quais foi treinado. Também pode ser usado para identificar e explorar outras vulnerabilidades de segurança cibernética no modelo de IA. A pesquisa foi realizada em uma placa de desenvolvimento do Google Coral, que usa um Edge TPU, mas os princípios do ataque poderiam ser aplicados a outros TPUs. As mitigações sugeridas incluem a introdução de ruído no processo de inferência de IA usando operações fictícias, executando operações aleatórias simultaneamente ou randomizando a sequência de camadas durante o processamento para tornar mais difícil para um invasor isolar e analisar os sinais EM de camadas individuais.

# Fontes

https://www.usenix.org/system/files/nsdi24-zu.pdf https://www.darkreading.com/vulnerabilities-threats/tpuxtract-attackers-steal-ai-models

---

# 7. Google OCS Optical Circuit Switch TPU interconnect

## Arquitetura e Design

## Arquitetura e Design

## Arquitetura do TPUv4 e OCS

A unidade fundamental da rede do TPUv4 Pod é o **Cubo TPU (ou Cubo 4x4x4)**, que consiste em 64 chips de TPU arranjados em uma estrutura de 4x4x4. Cada chip de TPU possui 6 links de alta velocidade ICI (Inter-Chip Interconnect) nas direções ±X, ±Y e ±Z, formando a base para a topologia de **Toro 3D**.

Dentro de um Cubo 4x4x4, os links ICI são divididos em duas categorias: as interconexões internas, que utilizam backplanes de PCB curtos e cabos de cobre para sinalização totalmente elétrica, e as interconexões externas. Apenas os links nas seis superfícies externas são expostos, totalizando **96 links ópticos por Cubo**. Estes links se conectam ao **OCS (Optical Circuit Switch)** para roteamento dinâmico e escalonamento massivo.

Um Pod de 4096 chips é composto por 64 Cubos, totalizando 6144 links ópticos. O **OCS Palomar** do Google, com 136x136 portas (128 efetivas), é utilizado para gerenciar esses links, sendo necessárias **48 unidades de OCS** para um Pod de 4096 chips. As 48 unidades de OCS são organizadas em três grupos ortogonais (X, Y, Z) de 16 OCS cada, para garantir o isolamento do tráfego e simplificar o roteamento.

## Microestrutura do OCS Palomar

O OCS Palomar atua como um painel de conexões dinâmico, puramente na camada física, refletindo a luz sem conversão O/E. A trajetória interna da luz segue um formato de “W” para minimizar a perda de inserção. Os componentes chave são:

*   Dois arrays de MEMS (Micro-Electro-Mechanical Systems) 2D para direcionamento de feixe 3D.
*   Espelhos dicroicos que transmitem o tráfego de 1310nm e refletem a luz de monitoramento de 850nm.
*   Módulos de injeção e câmera para operação e manutenção (O&M) em tempo real e ajustes de MEMS em microssegundos.

---


## Page 13

# Conexões Físicas

## Detalhes da Rede Óptica

A rede de alta velocidade fundamental dentro e entre os TPUs é a **Inter-Chip Interconnect (ICI)**, que utiliza uma topologia de **toro 3D**. A largura de banda bidirecional pode chegar a **1.2 TBps por chip** nas gerações recentes. Os links intra-cubo utilizam cabos de cobre de conexão direta (DAC) para distâncias curtas, enquanto os links inter-cubo e em escala de pod utilizam transceptores ópticos (aproximadamente 1.5 transceptores ópticos por TPU).

Os **Optical Circuit Switches (OCS)** são switches customizados baseados em **MEMS (micro-electro-mechanical systems)** com arrays de espelhos 2D, lentes e câmeras para direcionamento de feixe. Eles reconfiguram dinamicamente as topologias sem a necessidade de switches elétricos, o que resulta em uma redução de 40% no consumo de energia e 30% no custo. Um único OCS gerencia **144x144 portas**.

Os **Transceptores Wave Division Multiplexing (WDM)** com circuladores ópticos integrados permitem comunicação full-duplex sobre uma única fibra, **reduzindo a necessidade de fibra em 50%**. Os pods se conectam através da rede de datacenter **Jupiter** (multi-petabit-por-segundo), escalando para centenas de milhares de chips.

O **TPU v7 (Ironwood)** dobra a largura de banda do ICI para **1.2 TBps bidirecional** e suporta pods de até **9.216 chips**, com refrigeração líquida para pods de aproximadamente **10 MW**.

# Protocolos e Software

## Evolução da Rede Jupiter

A rede de datacenter **Jupiter** do Google evoluiu de uma topologia **Clos** para uma topologia de **conexão direta** entre os blocos de agregação de máquinas. Essa evolução resultou em uma redução de **30% no capex** e **41% na energia**.

## Componentes Arquitetônicos Críticos

*   Camada de Interconexão do Datacenter: Utiliza Optical Circuit Switches (OCSes) baseados em **MEMS (Micro-Electro-Mechanical Systems)** para permitir a reconfiguração dinâmica da topologia.
*   Controle Centralizado via SDN (Software-Defined Networking): O controle SDN é utilizado para a engenharia de tráfego.
*   Operações de Rede Automatizadas: Automação para entrega incremental de capacidade e engenharia de topologia.

## Desempenho e Eficiência

A combinação de engenharia de tráfego e topologia em malhas de conexão direta atinge um throughput similar ao das malhas Clos para os padrões de tráfego de produção do Google. **60% do tráfego** utiliza um caminho direto do bloco de agregação de origem para o de destino, com o restante transitando por um bloco adicional, resultando em um comprimento médio de caminho em nível de bloco de **1.4**. O OCS permite uma reconfiguração da malha **3x mais rápida** em comparação com as malhas Clos anteriores que usavam um painel de conexões.

---


## Page 14

# Fórmulas e Especificações

## Fórmulas e Especificações

*   **Largura de banda do ICI (TPU v7):** 1.2 TBps bidirecional
*   **Capacidade do OCS:** 144x144 portas (128 efetivas)
*   **Consumo de energia do OCS:** Redução de 40% em comparação com switches elétricos
*   **Custo do OCS:** Redução de 30% em comparação com switches elétricos
*   **Consumo de energia do Pod (TPU v7):** ~10 MW (com refrigeração líquida)
*   **Latência:** O uso de OCS cria caminhos ópticos diretos de baixa latência, minimizando as perdas de conversão de sinal.

# Vulnerabilidades Identificadas

## Vulnerabilidades

### Vulnerabilidades de MEMS

*   **Efeito Fotoacústico:** Ataques baseados em laser podem induzir vibrações em microfones MEMS, potencialmente injetando comandos de voz falsos.
*   **Vibrações Sonoras:** Ondas sonoras podem enganar acelerômetros MEMS, fazendo-os registrar movimento inexistente.

## Implicações para o OCS

Como o OCS do Google utiliza espelhos MEMS para o direcionamento do feixe de luz, ele pode ser teoricamente suscetível a ataques que explorem as vulnerabilidades dos MEMS. Ataques sônicos ou ópticos poderiam, em tese, interferir no alinhamento dos espelhos, causando interrupções na rede ou redirecionamento malicioso do tráfego. No entanto, o design específico do OCS do Google, com seu sistema de feedback de câmera e O&M em tempo real, provavelmente mitiga muitos desses riscos.

## Fontes

https://www.fibermall.com/blog/unveiling-google-tpu-architecture.htm?srsltid=AfmBOoruDMWTuvrMoBtcoGv3p8V7kPYJ4sVkJfude05hm_5QDlzd1kXP https://www.nextbigfuture.com/2025/11/highly-customized-optical-networking-critical-for-googles-tensor-processing-units-tpus.html https://research.google/pubs/jupiter-evolving-transforming-googles-datacenter-network-via-optical-circuit-switches-and-software-defined-networking/?utm_source=chatgpt.com https://www.kaspersky.com/blog/curious-mems-vulnerabilities/32245/

---

## 8. Pesquisa técnica detalhada sobre a reconfiguração de Comutadores de Circuito Óptico (OCS) baseados em espelhos MEMS (Micro-Electro-Mechanical Systems).

### Arquitetura e Design

A arquitetura dos comutadores de circuito óptico (OCS) baseados em MEMS (Micro-Electro-Mechanical Systems) é centrada na utilização de matrizes de micro-espelehos para direcionar sinais de luz, permitindo a comutação de circuitos ópticos sem a necessidade de conversão para o domínio elétrico. A estrutura interna desses comutadores é tipicamente uma matriz N×N de micro-espelehos, onde cada espeleho pode ser individualmente angulado para rotear um feixe de luz de uma porta de entrada para qualquer porta de saída, um conceito conhecido como conectividade “any-to-any”.

---


## Page 15

Os componentes principais incluem os próprios micro-espehos MEMS, atuadores (geralmente eletrostáticos ou eletromagnéticos) que controlam a posição dos espelhos, e a interface de controle eletrônico. A hierarquia de controle geralmente envolve um controlador de alto nível que traduz as solicitações de reconfiguração da rede em comandos de baixo nível para os atuadores dos espelhos. O fluxo de dados é puramente óptico através do comutador, enquanto os comandos de controle são sinais elétricos (tipicamente tensões) aplicados aos atuadores MEMS. O relatório da UC Berkeley descreve duas abordagens de controle: uma digital (SuperSwitch 1) que usa drivers de alta tensão para ligar/desligar os espelhos, e uma analógica (SuperSwitch 2) que utiliza Conversores Digital-Analógico (DACs) para um controle de posição mais fino.

O fluxo de comandos começa com uma solicitação de reconfiguração, que é processada por uma CPU embarcada ou FPGA. Esta unidade de processamento então calcula as novas posições dos espelhos e envia os sinais de controle apropriados para os drivers ou DACs, que por sua vez aplicam as tensões necessárias para mover os espelhos e estabelecer os novos caminhos ópticos.

## Conexões Físicas

As conexões físicas em um OCS baseado em MEMS abrangem desde o nível do chip até a interconexão do sistema. Internamente, a “fiação” consiste em guias de onda de silício sobre isolante (SOI) que direcionam a luz através do chip fotônico. As fibras ópticas externas são acopladas a essas guias de onda por meio de acopladores de grade ou acoplamento de borda, que servem como as portas de entrada e saída do dispositivo.

A conexão entre o chip fotônico (que contém os MEMS) e o chip de controle eletrônico (CMOS) é um aspecto crítico. O relatório da UC Berkeley detalha duas abordagens: uma utiliza empacotamento 3D avançado com ligação por flip-chip e micro-saliências de ouro (Au micro-bumps) para uma conexão direta e de alta densidade (SuperSwitch 1). A outra abordagem emprega a técnica mais convencional de wirebonding para conectar os pads elétricos dos chips (SuperSwitch 2).

O conjunto de chips é montado em um substrato ou interposer, que por sua vez é montado em uma Placa de Circuito Impresso (PCB). O layout da PCB é projetado para fornecer as múltiplas tensões de alimentação necessárias, incluindo as altas tensões (dezenas de volts) para os atuadores MEMS, e para rotear os sinais de controle de baixa velocidade (por exemplo, de uma interface UART) para os chips de controle. Finalmente, conectores de fibra óptica padrão, como conectores LC ou MPO, são usados no painel do sistema para conectar o OCS à rede de fibra óptica mais ampla.

## Protocolos e Software

Os protocolos de comunicação para controlar um OCS baseado em MEMS operam em diferentes camadas. No nível mais baixo, a interface com o chip de controle CMOS é frequentemente uma cadeia de varredura (scan chain) serial, como detalhado no relatório da UC Berkeley. Através desta interface, os bits de configuração são carregados para programar o estado de cada espelho individual. O relatório menciona especificamente o uso de uma interface UART (Universal Asynchronous Receiver-Transmitter) para controlar os controladores da cadeia de varredura, indicando um protocolo de comunicação serial de baixo nível para a configuração do hardware.

No nível de rede, o comando para iniciar uma reconfiguração é normalmente enviado através de uma conexão de rede padrão. O artigo do Google Research descreve o uso de uma porta Ethernet 1G com o protocolo TCP/IP para enviar mensagens de comando de reconfiguração para uma CPU embarcada rodando Linux. Os autores também propõem a otimização deste processo, sugerindo a mudança para o protocolo UDP/IP e a implementação da lógica de processamento de comandos em um FPGA para reduzir a latência. As APIs e bibliotecas para controlar o OCS seriam construídas sobre esses protocolos, expondo funcionalidades como “estabelecer circuito da porta X para a porta Y”.

A sequência de inicialização envolve a energização dos chips CMOS e SiPh, o carregamento da configuração inicial dos espelhos através da cadeia de varredura e a calibração, se necessário. Os comandos de controle subsequentes consistem em carregar novos padrões de bits na cadeia de varredura para reconfigurar os caminhos ópticos conforme a demanda da rede.

## Fórmulas e Especificações

As especificações e fórmulas relevantes para OCS baseados em MEMS giram em torno de desempenho, consumo de energia e temporização.

### Fórmulas de Desempenho:

O artigo do Google Research fornece uma fórmula crucial para calcular a perda de produtividade (throughput) durante a reconfiguração da rede em um sistema híbrido EPS/OCS. A razão de produtividade em comparação com uma rede puramente EPS

---


## Page 16

é dada por:

Razão de Produtividade = (S - (T1 + T2 + T3)) / S

Onde:
*   S : Período de estabilidade do tráfego (o tempo que um padrão de tráfego permanece constante).
*   T1 : Tempo de processamento do comando de reconfiguração.
*   T2 : Tempo de reconfiguração do espelho MEMS.
*   T3 : Tempo de inicialização da eletrônica do receptor (pós-reconfiguração).

Especificações de Temporização (Exemplos do Artigo do Google):
*   T1 (Processamento de Comando): 5 ms (pode ser otimizado para ~10 µs com FPGA).
*   T2 (Reconfiguração do Espelho): 12 ms (pode ser otimizado para 1.5 ms a 3 ms com espelhos menores, ou até 15 µs com tecnologia DLP).
*   T3 (Inicialização do Receptor): 15 ms (pode ser otimizado para < 200 ns com TIAs de modo rajada).
*   Tempo Total de Interrupção (Protótipo Helios): 32 ms.

Especificações de Hardware (Exemplos do Relatório da UC Berkeley):
*   Tensão de Atuação (HVDD): Tipicamente na faixa de 50V a 70V.
*   Comprimento de Onda de Operação (λ): 1310 nm é um exemplo utilizado.
*   Frequência Máxima de Reconfiguração: O SuperSwitch 1 demonstrou uma frequência de ~1.7 MHz.
*   Consumo de Energia: É uma função da tensão de atuação, da frequência de reconfiguração e do número de espelhos sendo comutados. O relatório da UC Berkeley fornece medições detalhadas de energia para seus protótipos.
*   Largura de Banda: O comutador em si é em grande parte agnóstico à taxa de dados (data rate agnostic), o que significa que a largura de banda é limitada principalmente pelos transceptores ópticos conectados às suas portas.
*   Latência: A latência de passagem pelo OCS é extremamente baixa (da ordem de nanosegundos), pois o sinal permanece no domínio óptico. A latência de reconfiguração, no entanto, é a principal métrica de desempenho, conforme discutido acima.

**Vulnerabilidades Identificadas**

As vulnerabilidades em OCS baseados em MEMS podem ser categorizadas em ataques de desempenho e ataques de controle.

**Pontos de Acesso e Vetores de Ataque:**

O principal ponto de acesso para um ataque é a interface de controle da rede que envia os comandos de reconfiguração. Se um ator malicioso obtiver acesso a esta interface (por exemplo, a porta Ethernet 1G mencionada no artigo do Google), ele poderá lançar vários ataques:

1.  **Ataque de Negação de Serviço (DoS) por Reconfiguração:** Como a reconfiguração do OCS incorre em um tempo de inatividade (T1 + T2 + T3, que foi de 32ms no protótipo Helios), um invasor pode inundar o sistema com comandos de reconfiguração. Isso forçaria a rede a passar a maior parte do tempo reconfigurando em vez de transmitir dados, degradando severamente a produtividade. O artigo do Google quantifica essa vulnerabilidade com sua fórmula de produtividade.

2.  **Manipulação de Tráfego e Escuta (Eavesdropping):** Ao controlar a matriz de comutação, um invasor pode redirecionar sutilmente o tráfego. Por exemplo, eles poderiam espelhar um circuito de comunicação para uma porta de saída conectada a um dispositivo de escuta, comprometendo a confidencialidade dos dados. Eles também poderiam interromper seletivamente circuitos críticos, causando uma negação de serviço direcionada.

**Falhas Conhecidas:**

Embora não sejam vulnerabilidades de segurança no sentido tradicional, as falhas de hardware representam um risco. Os espelhos MEMS são componentes mecânicos microscópicos e podem falhar, ficando presos em uma posição. Uma falha em um único espelho pode tornar uma ou mais portas de entrada/saída inutilizáveis, ou permanentemente (e incorretamente) conectar dois circuitos. A confiabilidade a longo prazo e os modos de falha dos espelhos MEMS são uma área ativa de pesquisa e

---


## Page 17

engenharia. O controle de tensão preciso e a proteção contra sobretensão são cruciais para mitigar o risco de danos aos atuadores MEMS.

## Fontes

*   [https://www.glsunmall.com/fiber-optic-articles/dynamic-network-reconfiguration-with-mems-matrix-optical-switches-in-ocss.html](https://www.glsunmall.com/fiber-optic-articles/dynamic-network-reconfiguration-with-mems-matrix-optical-switches-in-ocss.html)
*   [https://www2.eecs.berkeley.edu/Pubs/TechRpts/2024/EECS-2024-213.pdf](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2024/EECS-2024-213.pdf)
*   [https://research.google.com/pubs/archive/36840.pdf](https://research.google.com/pubs/archive/36840.pdf)

---

## 9. Optical switching fabric data center TPU topology

### Arquitetura e Design

A arquitetura do supercomputador TPU v4 do Google é um sistema opticamente reconfigurável projetado para aprendizado de máquina. Ele utiliza Switches de Circuito Óptico (OCS) para reconfigurar dinamicamente sua topologia de interconexão, permitindo a seleção de uma topologia de toro 3D torcido, se desejado. A unidade de construção fundamental é um cubo de 4x4x4, compreendendo 64 chips de TPU. Um sistema completo é dimensionado para 4096 chips de TPU v4. Cada chip de TPU v4 integra dois TensorCores (TCs), cada um equipado com quatro Unidades de Multiplicação de Matrizes (MXUs) de 128x128 e uma Unidade de Processamento Vetorial (VPU). Os dois TCs em um chip compartilham 128 MiB de Memória Comum (CMEM). Além disso, a arquitetura inclui SparseCores, que são processadores de fluxo de dados que aceleram modelos que dependem de embeddings.

### Conexões Físicas

A interconexão no supercomputador TPU v4 é realizada principalmente através de links ópticos. Um bloco de construção em forma de cubo 4x4x4 possui 96 links ópticos, com 16 links por face, que se conectam aos Switches de Circuito Óptico (OCS). Cada um desses blocos se conecta a 48 OCSes. O OCS Palomar, utilizado no sistema, tem uma configuração de 136x136 portas, sendo 128 portas ativas e 8 de reserva. O sistema completo, com 4096 chips de TPU v4, é formado pela interconexão de 64 desses cubos através de 48 OCSes. O encapsulamento do TPU v4 contém o ASIC central e 4 pilhas de HBM. A placa de circuito impresso (PCB) aloja 4 desses encapsulamentos com refrigeração líquida e possui 4 conectores PCIe no lado superior e 16 conectores OSFP no lado inferior para os links de interconexão entre bandejas (ICI). Dentro de um rack, uma malha 3D 4x4x4 é formada por cabos elétricos passivos. A conversão do sinal elétrico para óptico ocorre nos conectores de fibra das bandejas de TPU.

### Protocolos e Software

O supercomputador TPU v4 utiliza uma pilha de software que permite a reconfiguração da topologia da rede por meio dos Switches de Circuito Óptico (OCS). Isso permite que os usuários selecionem topologias específicas, como um toro 3D torcido, para otimizar o desempenho para diferentes cargas de trabalho de aprendizado de máquina. O software de agendamento pode alocar fatias (slices) de TPUs de tamanhos variados, como 4i×4j×4k, e não se limita a potências de 2. A reconfiguração da topologia é realizada principalmente através da reprogramação do roteamento no OCS. A arquitetura também suporta paralelismo de dados, modelo (tensor) e pipeline. Para comunicação entre os chips, a arquitetura utiliza o Inter-Core Interconnect (ICI).

### Fórmulas e Especificações

Largura de banda: A topologia de toro 3D torcido melhora a taxa de transferência all-to-all em 1,63x e 1,31x em fatias de 4x4x8 e 4x8x8, respectivamente, em comparação com um toro regular. A topologia de toro 3D dobra a largura de banda de bisseção em comparação com uma malha. Latência: A topologia de toro torcido apresenta menor latência em comparação com os toros retangulares. Consumo de energia: Os OCSes e os componentes ópticos subjacentes consomem menos de 3% da energia do sistema. O TPU v4 consome de 1,3x a 1,9x menos energia do que a Nvidia A100. Para o benchmark BERT, o consumo de energia é de 197 W para o TPU v4 em comparação com 380 W para o A100.

---


## Page 18

# Vulnerabilidades Identificadas

O artigo menciona que o OCS pode criar um isolamento de rede ‘air gapped’ entre diferentes fatias, o que aumenta a segurança para vários clientes que compartilham um supercomputador TPU v4. No entanto, o artigo não detalha vulnerabilidades específicas, pontos de acesso potenciais ou vetores de ataque. A principal preocupação abordada é a tolerância a falhas, onde o OCS é usado para contornar falhas de host da CPU e outros componentes defeituosos para manter a disponibilidade do sistema.

# Fontes

https://arxiv.org/pdf/2304.01433

---

# 10. Biblioteca de roteamento de rede TPU libtpunet

## Arquitetura e Design

A libtpunet é uma biblioteca de software que é executada como parte do trabalho de um usuário de TPU. Sua principal responsabilidade é configurar a rede de interconexão entre chips (ICI), o que abrange as camadas de dados e de roteamento. A biblioteca realiza a descoberta da topologia da rede e programa as tabelas de encaminhamento de pacotes em cada chip de TPU, utilizando balanceamento de carga global. Embora as políticas de roteamento detalhadas sejam abstráidas da ISA (Instruction Set Architecture), a libtpunet pode fornecer “dicas” para otimizar o roteamento. Além disso, a libtpunet gerencia o ciclo de vida das sessões da camada de dados, emitindo comandos de início e parada e ajustando os tamanhos ideais dos buffers de controle de fluxo. Ela opera em conjunto com outros componentes do sistema, como o daemon de monitoramento de saúde healthd e o serviço de nível de cluster Pod Manager .

## Conexões Físicas

Os supercomputadores TPUv4 utilizam uma combinação de interconexões elétricas e ópticas para conectar milhares de chips. A unidade fundamental é o ‘cubo’, um rack de datacenter que agrupa 16 máquinas TPU, cada uma contendo 4 chips TPUv4 em uma malha 2x2x1. Dentro de um cubo, os 64 TPUs são interconectados por uma malha 3D (4x4x4) através da Interconexão no Chip (ICI). Para conectar múltiplos cubos e formar um supercomputador (pod) de até 4096 TPUs, são utilizados Switches de Circuito Óptico (OCS). Cada cubo expõe 16 links ICI ópticos em cada uma de suas 6 faces (X, Y, Z), totalizando 96 links por cubo, que são conectados aos OCS. Esses switches MEMS (sistemas microeletromecânicos) permitem a reconfiguração dinâmica da topologia da rede, conectando diferentes cubos para formar a topologia de toro 3D solicitada pelo trabalho do usuário. Essa abordagem permite contornar falhas de hardware e reduzir a fragmentação de recursos. As conexões entre as bandejas de TPU podem ser feitas por cabos de cobre externos ou fibras ópticas.

## Protocolos e Software

O ecossistema de software dos TPUs é projetado para gerenciar a complexidade da infraestrutura reconfigurável. A comunicação principal entre os chips TPU ocorre através do protocolo de Interconexão no Chip (ICI), uma malha de rede de alta velocidade que permite Acesso Direto à Memória Remota (RDMA) entre quaisquer dois TPUs no pod, sem envolvimento da CPU. O protocolo ICI é programável e estruturado em camadas:

1.  **Camada Física:** Gerenciada pelo Pod Manager , chip manager e healthd , lida com o treinamento dos links e a conexão (xconnect) através dos OCS.
2.  **Camada de Dados:** A libtpunet e o healthd gerenciam esta camada, que garante a entrega ordenada de pacotes com retransmissão automática, controle de fluxo baseado em crédito e gerenciamento de sessões.
3.  **Camada de Roteamento:** A libtpunet programa as tabelas de encaminhamento de pacotes em cada chip com base em uma política de balanceamento de carga global. O roteamento é determinado no início do trabalho e permanece estático.
4.  **Camada de Transação:** O compilador XLA utiliza a abstração de RDMA para a comunicação.

A principal biblioteca de software do lado do usuário é a libtpunet . Quando um trabalho é iniciado, a libtpunet realiza a descoberta da topologia da rede e programa as tabelas de encaminhamento do ICI. Ela também gerencia o ciclo de vida das sessões da camada de dados, emitindo comandos de início/parada e ajustando os buffers de controle de fluxo. O compilador XLA

---


## Page 19

consume a topologia construída pela libtpunet para otimizar a paralelização do modelo. Outros componentes de software incluem o Pod Manager , que gerencia a conectividade entre os cubos atuando nos OCS, e o healthd , um daemon que monitora a saúde do hardware em cada máquina.

## Fórmulas e Especificações

As especificações de desempenho da rede ICI dos TPUs variam entre as gerações, mas focando no TPUv4, que utiliza a libtpunet de forma proeminente:

*   **Largura de Banda:** Cada link ICI do TPUv4 pode transportar 50 GB/s de largura de banda unidirecional. Um pod TPUv4 completo, com 8.960 chips, utiliza a interconexão de maior largura de banda da Google, atingindo 4.800 Gbps por chip em uma topologia de toro 3D.
*   **Latência:** A libtpunet configura o roteamento para ser estático durante a execução do trabalho, o que ajuda a garantir uma latência previsível. A comunicação é de baixa latência, permitindo RDMA rápido entre os chips.
*   **Consumo de Energia:** Um chip TPUv4 tem um consumo médio de energia de cerca de 200W. O sistema de comutação óptica (OCS), crucial para a reconfigurabilidade, consome menos de 3% da energia total do pod.
*   **Capacidade Térmica:** Embora os detalhes específicos da capacidade térmica não sejam publicamente detalhados, o consumo de energia relativamente baixo por chip (comparado a algumas GPUs de ponta) sugere um design focado na eficiência energética, o que implica em uma carga térmica gerenciável em escala de supercomputador.

## Vulnerabilidades Identificadas

A documentação pública e os artigos de pesquisa sobre a infraestrutura do Google TPU, incluindo a libtpunet , focam principalmente em resiliência, desempenho e arquitetura, com pouca divulgação explícita sobre vulnerabilidades de segurança. No entanto, podemos inferir potenciais vetores de ataque e pontos de acesso com base no design do sistema:

*   **Pontos de Acesso Potenciais:**
    *   **libtpunet :** Como a libtpunet é executada no espaço do usuário como parte do trabalho submetido, uma biblioteca comprometida ou maliciosa poderia, teoricamente, tentar manipular as tabelas de roteamento do ICI para interferir no tráfego de rede de outros trabalhos, embora o isolamento entre trabalhos seja um objetivo de design. A libtpunet também limpa os buffers do ICI no início de uma sessão para evitar a poluição de estado entre sessões.
    *   **healthd :** O daemon healthd é um processo privilegiado que monitora a saúde do hardware. Um comprometimento deste daemon poderia levar a relatórios de saúde falsos, potencialmente mascarando falhas ou causando negação de serviço ao marcar componentes saudáveis como defeituosos.
    *   **Pod Manager :** Sendo o serviço que controla a configuração dos OCS, um ataque bem-sucedido ao Pod Manager poderia permitir que um invasor reconfigure a topologia da rede, intercepte o tráfego ou isole cubos da rede.
*   **Falhas Conhecidas:** A pesquisa foca em tolerância a falhas (resiliency) em vez de falhas de segurança. O sistema é projetado para detectar e contornar automaticamente falhas de hardware em links, chips e OCSs. Não há menção pública a falhas de segurança conhecidas na libtpunet ou no protocolo ICI.
*   **Vetores de Ataque:**
    *   **Ataques de Canal Lateral (Side-Channel):** Pesquisas como o “TPUXtract” demonstraram a possibilidade de extrair hiperparâmetros de redes neurais medindo o consumo de energia. Embora não seja um ataque direto à libtpunet , ele explora a infraestrutura de hardware na qual a biblioteca opera.
    *   **Ataques à Cadeia de Suprimentos de Software:** Uma versão maliciosa da libtpunet ou de outras bibliotecas do sistema (como o compilador XLA) poderia ser introduzida no ambiente de execução.
    *   **Isolamento de Rede:** A Google utiliza os OCS para criar um “air gap” (isolamento físico) entre diferentes “slices” (fatias de TPU alocadas para um trabalho), o que aumenta a segurança entre múltiplos usuários. Uma falha nesse mecanismo de isolamento poderia ser um vetor de ataque.

Em resumo, embora a Google projete seus sistemas com segurança em mente, a complexidade de um supercomputador como o TPUv4 apresenta uma superfície de ataque teórica em vários níveis do software e do hardware. A falta de divulgações públicas de vulnerabilidades específicas sugere um foco robusto em segurança interna e resiliência.

---


## Page 20

# Fontes

https://www.usenix.org/system/files/nsdi24-zu.pdf https://docs.cloud.google.com/tpu/docs/tpu-monitoring-library
https://www.infoworld.com/article/4025687/google-launches-tpu-monitoring-library-to-boost-ai-infrastructure-efficiency.html
https://docs.cloud.google.com/vertex-ai/docs/training/training-with-tpu-vm https://github.com/google/jax/issues/22835
https://pypi.org/project/libtpu/ https://considerthebulldog.com/tte-tpu/
https://blog.csdn.net/DatenLord/article/details/139886804 https://newsletter.micahlerner.com/p/resiliency-at-scale-managing-googles
https://cloud.google.com/tpu https://docs.cloud.google.com/tpu/docs/v4
https://cloud.google.com/blog/products/compute/ironwood-tpus-and-new-axion-based-vms-for-your-ai-workloads
https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm https://github.com/arvidn/libtorrent/issues/4850
https://docs.cloud.google.com/python/docs/reference/tpu/latest https://www.usenix.org/conference/nsdi24/presentation/zu
https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codesigned-ai-stack https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://medium.com/@jiminlee-ai/tpu-why-google-doesnt-wait-in-line-for-nvidia-gpus-2-2-2267e4ed686f
https://blog.google/products/google-cloud/ironwood-google-tpu-things-to-know/
https://globaltechresearch.substack.com/p/the-ironwood-an-introduction-to-google https://henryhmko.github.io/posts/tpu/tpu.html
https://www.fibermall.com/blog/unveiling-google-tpu-architecture.htm? https://datacentremagazine.com/articles/google-cloud-next-2025-the-announcements-you-need-to-know
https://datacentremagazine.com/articles/google-cloud-next-2025-the-announcements-you-need-to-know https://newsletter.semianalysis.com/p/tpuv7-google-takes-a-swing-at-the
https://www.nextbigfuture.com/2025/11/highly-customized-optical-networking-critical-for-googles-tensor-processing-units-tpus.html https://www.linkedin.com/pulse/optical-circuit-switching-sharada-yeluri-f3slc


# 11. Operações coletivas do protocolo de roteamento de rede do TPU

## Arquitetura e Design

A arquitetura de rede do TPUv4 é baseada em uma interconexão 3D torus de alta largura de banda e baixa latência, projetada para computação de aprendizado de máquina em grande escala. A unidade fundamental é o ‘cubo’, um arranjo de 4x4x4 de 64 chips TPU. Um supercomputador (ou ‘pod’) é composto por 64 cubos, totalizando 4096 TPUs. A interconexão entre os cubos é realizada por meio de Optical Circuit Switches (OCS), que permitem a configuração dinâmica de topologias de rede para contornar falhas de hardware e otimizar a alocação de recursos. O Inter-Chip Interconnect (ICI) é o tecido de rede de alta velocidade que conecta diretamente os TPUs, permitindo a comunicação direta de dispositivo para dispositivo (RDMA) sem envolvimento da CPU. O protocolo ICI é programável e dividido em camadas: Física, de Enlace de Dados, de Roteamento e de Transação.

## Conexões Físicas

Cada cubo do TPUv4 possui 16 links ópticos ICI em cada uma de suas 6 faces, totalizando 96 links ICI por cubo. Um supercomputador TPUv4 completo possui 6144 links ópticos ICI conectados a 48 OCSes distintos. As conexões entre os chips dentro de um cubo são feitas através de um mesh 3D, enquanto as conexões entre os cubos são estabelecidas pelos OCS, que utilizam espelhos MEMS para criar conexões ópticas dedicadas. Isso permite que cubos não contíguos fisicamente sejam conectados para formar uma topologia de trabalho coesa. A fiação interna dos cubos e as conexões aos OCS são feitas predominantemente com fibra óptica para garantir alta largura de banda e baixa latência em distâncias maiores.

## Protocolos e Software

A pilha de software que gerencia os pods de TPUv4 inclui:
*   **Borg**: O gerenciador de cluster do Google, responsável por agendar e gerenciar os trabalhos.
*   **Pod Manager**: Um serviço de software que gerencia a conectividade entre múltiplos cubos, configurando os OCSes de acordo com as decisões de agendamento do Borg.
*   **libtpunet**: Uma biblioteca de software que configura a topologia de rede ICI solicitada para cada trabalho do usuário, programando as tabelas de roteamento.
*   **healthd**: Um daemon que monitora continuamente a saúde do hardware e reporta ao sistema de gerenciamento.
*   O protocolo ICI possui as seguintes camadas:
    *   **Camada Física**: Responsável pelo treinamento dos links e pela conexão física (xconnect) através dos OCSes.
    *   **Camada de Enlace de Dados**: Garante a entrega confiável e ordenada de pacotes, com retransmissão automática e controle de fluxo baseado em crédito.
    *   **Camada de Roteamento**: Utiliza tabelas de encaminhamento de pacotes programadas

---


## Page 21

pela libtpunet. O roteamento é baseado em Dimension-Order Routing (DOR) para balanceamento de carga.\n- Camada de Transação: Expõe as operações de RDMA para a camada de software superior (XLA).

## Fórmulas e Especificações

Largura de Banda por Link ICI: 50 GBps (unidirecional)\nNúmero de TPUs por Cubo: 64\nNúmero de Cubos por Pod: 64\nNúmero Total de TPUs por Pod: 4096\nCusto do OCS: < 5% do custo de capital total do pod\nConsumo de Energia do OCS: < 3% da energia total do pod

## Vulnerabilidades Identificadas

A principal vulnerabilidade abordada no projeto do TPUv4 é a ocorrência de falhas de hardware em grande escala, como falhas de máquinas, chips, links ópticos e até mesmo dos OCs. Com milhares de componentes, a probabilidade de falha de um deles é alta. Para mitigar isso, a arquitetura é projetada para ser resiliente. O sistema utiliza roteamento tolerante a falhas no ICI para contornar componentes defeituosos, permitindo que os trabalhos continuem a ser executados. O Pod Manager pode excluir cubos defeituosos do pool de recursos disponíveis. O artigo ‘Tacos’ também explora a geração de algoritmos de comunicação coletiva que são cientes da topologia, o que é crucial para manter o desempenho em topologias que se tornaram irregulares devido a falhas de componentes. A reconfigurabilidade via OCS é a principal defesa contra a fragmentação de recursos e falhas de hardware, permitindo que o sistema alcance uma disponibilidade de 99,98%.

## Fontes

https://www.usenix.org/system/files/nsdi24-zu.pdfnhttps://arxiv.org/html/2304.05301v2

---

# 12. healthd TPU health monitoring daemon

## Arquitetura e Design

O healthd é um daemon de software que roda em cada host de uma máquina TPUv4. Sua principal função é monitorar continuamente a saúde do hardware da máquina e reportar para os sistemas de software a nível de cluster. Seus componentes e funções incluem o monitoramento contínuo da qualidade dos links e sinais de conectividade, o controle da camada de dados com a capacidade de desabilitar links, e a interface com o sistema de gerenciamento de cluster Borg para notificação de anomalias. A hierarquia de controle posiciona o healthd como um sensor local no nível do host, fornecendo dados para o Pod Manager e o Borg, que tomam decisões de orquestração em nível de cluster. O fluxo de dados e comandos envolve a coleta de dados de saúde do hardware, o envio desses dados para o Pod Manager e o Borg, a utilização desses dados pelo Borg para agendamento e notificação de falhas, e a recepção de comandos para desabilitar links.

## Conexões Físicas

As conexões físicas do sistema TPUv4, no contexto do healthd, são caracterizadas por uma interconexão de alta velocidade entre os chips TPU (ICI) e conexões entre os cubos através de links ópticos gerenciados por Switches de Circuito Óptico (OCS). O sistema utiliza racks de datacenter padrão, implicando o uso de conectores e sockets padrão da indústria. O layout do PCB e do substrato organiza 4 chips TPUv4 em uma malha 2x2x1 por bandeja de TPU, com 16 máquinas TPU formando um “cubo”.

## Protocolos e Software

O healthd opera dentro de uma pilha de software que gerencia os supercomputadores TPUv4, interagindo com um protocolo de interconexão entre chips (ICI) programável e em camadas, especificamente monitorando a camada física e controlando a camada de dados. A comunicação com os serviços de cluster, como o Pod Manager, provavelmente utiliza um framework como o gRPC. O healthd trabalha em conjunto com a biblioteca libtpunet, que configura a topologia da rede ICI. A sequência de inicialização do healthd não é detalhada, mas presume-se que ocorra durante o boot do host. O principal comando de controle é a capacidade de desabilitar um link da camada de dados.

---


## Page 22

# Fórmulas e Especificações

O artigo não fornece fórmulas matemáticas detalhadas relacionadas ao `healthd`. As especificações são mais qualitativas e focadas na arquitetura do sistema. A largura de banda de cada link ICI é de 50 GBps unidirecional. O consumo de energia dos OCS e da fibra óptica é inferior a 5% do custo de capital e 3% da energia total de operação de um pod TPUv4.

# Vulnerabilidades Identificadas

O artigo “Resiliency at Scale: Managing Google’s TPUv4 Machine Learning Supercomputer” não discute vulnerabilidades de segurança, vetores de ataque ou falhas conhecidas do daemon `healthd`. O foco do documento está na resiliência a falhas de hardware e na arquitetura de software para gerenciamento de supercomputadores em escala.

# Fontes

https://www.usenix.org/system/files/nsdi24spring_prepub_zu.pdf

---

# 13. Monitoramento de saúde, detecção de falhas e recuperação de TPUs

## Arquitetura e Design

## Arquitetura e Design

## Estrutura Interna Completa

*   **Supercomputador TPUv4:** Composto por 4096 nós com uma interconexão em toro 3D personalizada.
*   **Cubes:** Unidade de hardware com 64 chips de TPU arranjados em uma malha 3D de 4x4x4. Cada supercomputador (ou pod) possui 64 cubos, totalizando 4096 TPUs.
*   **Interconexão Inter-Chip (ICI):** Malha de rede de alta velocidade que conecta diretamente os TPUs para comunicação dispositivo a dispositivo (RDMA) sem envolver as CPUs.
*   **Optical Circuit Switches (OCS):** Utilizados para conectar dinamicamente (xconnect) a ICI de diferentes cubos para formar a topologia de toro solicitada pelo usuário.

## Componentes e Suas Funções

*   **Borg:** Serviço de gerenciamento de cluster que admite, agenda e gerencia os trabalhos do TPUv4.
*   **Pod Manager:** Serviço de software em nível de cluster que gerencia a conectividade multi-cubo, atuando na configuração do xconnect do OCS em resposta às decisões de agendamento do Borg.
*   **libtpunet:** Biblioteca de software que configura a topologia de rede ICI solicitada para cada trabalho de usuário do TPUv4.
*   **healthd:** Daemon de software executado em cada host de um pod que monitora continuamente a saúde do hardware da máquina e reporta aos sistemas de software em nível de cluster.

## Hierarquia de Controle

1.  **Borg:** Inicia o processo de alocação de recursos para um trabalho.
2.  **Pod Manager:** Recebe a solicitação do Borg e configura os OCSes para criar a topologia de rede desejada.
3.  **libtpunet:** Executa dentro do trabalho do usuário para descobrir a topologia, programar as tabelas de roteamento e iniciar a sessão ICI.

---


## Page 23

4. **healthd**: Monitorea continuamente a saúde do hardware e reporta ao Borg e ao Pod Manager.

# Fluxo de Dados e Comandos

*   Os comandos de controle fluem do Borg para o Pod Manager e, em seguida, para os OCSes.
*   O `libtpunet` configura o roteamento de dados nos chips de TPU.
*   Os dados do modelo de aprendizado de máquina fluem entre os TPUs através da interconexão ICI, utilizando RDMA para comunicação de baixa latência.

# Conexões Físicas

## Fiação Interna (cabos, fibras ópticas)

*   **Interconexão Inter-Chip (ICI)**: Utiliza cabos ópticos para conectar os cubos de TPU através dos Optical Circuit Switches (OCS).
*   **Conexões dentro do Cubo**: Os 64 chips de TPU dentro de um cubo são interconectados em uma malha 3D de 4x4x4 através de conexões elétricas diretas no PCB.

## Conexões de Chips aos Portos

*   Cada cubo 3D expõe 16 ICIs ópticas em cada uma das 6 faces das dimensões X, Y, Z, totalizando 96 ICIs por cubo.
*   Um supercomputador TPUv4 consiste em 64 cubos, com um total de 6144 links ICI ópticos conectados a 48 OCSes distintos.

## Pinagem e Interfaces

*   A pinagem específica e as interfaces são proprietárias e não detalhadas publicamente. No entanto, a interface principal é a ICI, que permite a comunicação direta entre os chips de TPU.

## Layout de PCB e Substrato

*   Cada máquina TPU individual possui uma bandeja de CPU e uma bandeja de TPU, conectadas por PCIe.
*   Cada bandeja de TPU possui 4 chips TPUv4 dispostos em uma malha ICI de 2x2x1.
*   16 máquinas TPU são agrupadas para formar um rack de datacenter, e as conexões ICI dentro do rack formam a malha 4x4x4 que constitui um cubo.

## Conectores e Sockets

*   Os conectores para os cabos ópticos ICI são conectados aos OCSes. Os OCSes são baseados em uma matriz de espelhos de sistemas microeletromecânicos (MEMS).

---


## Page 24

# Protocolos e Software

# Protocolos e Software

## Protocolos de Comunicação

*   **Protocolo ICI (Inter-Chip Interconnect):**
    *   **Camada de Transação:** Utiliza RDMA (Remote Direct Memory Access), exposto através do XLA (Accelerated Linear Algebra), para comunicação de alta performance entre os TPUs.
    *   **Camada de Roteamento:** O roteamento de pacotes é programado pela biblioteca libtpunet com balanceamento de carga global. As tabelas de encaminhamento em cada chip são indexadas pelo ID do chip de destino.
    *   **Camada de Dados:** Garante a entrega ordenada de pacotes com retransmissão automática. Implementa controle de fluxo baseado em crédito no nível do link. A libtpunet gerencia o início e o fim da sessão, enquanto o healthd pode desabilitar um link para recuperação online.
    *   **Camada Física:** Gerenciada pelo Pod Manager e Chip Manager , é responsável pela conexão física (xconnect) e treinamento das portas ópticas.

## APIs e Bibliotecas

*   **libtpunet:** Uma biblioteca que roda dentro do processo do usuário para configurar a rede ICI. Suas funções incluem:
    *   Descoberta de topologia para garantir que a configuração global corresponda à solicitação do usuário.
    *   Atribuição de um ID de chip exclusivo para cada TPU.
    *   Computação e programação das tabelas de encaminhamento de cada TPU.
    *   Configuração do tamanho do buffer de controle de fluxo do link.
    *   Programação da configuração de clock consistente em todo o conjunto de TPUs do trabalho.
    *   Início da sessão ICI, permitindo o uso de operações coletivas com RDMA.
*   **XLA (Accelerated Linear Algebra):** A interface de alto nível que expõe as capacidades de RDMA da camada de transação do protocolo ICI.

## Sequência de Inicialização

1.  **Solicitação de Trabalho:** Um usuário submete um trabalho ao Borg, especificando os recursos de TPU necessários.
2.  **Alocação de Cubos:** O Borg seleciona um conjunto de cubos de TPU disponíveis.
3.  **Configuração do OCS:** O Pod Manager recebe a lista de cubos e a topologia desejada do Borg e, em seguida, comanda os Optical Circuit Switches (OCS) para estabelecer as conexões ópticas (xconnect) necessárias entre os cubos.
4.  **Despacho do Trabalho:** O Borg despacha os binários do trabalho para as máquinas host dos TPUs alocados.
5.  **Configuração da Rede ICI:** A biblioteca libtpunet , executando no espaço do usuário, realiza a descoberta da topologia, programa as tabelas de roteamento em cada TPU e inicializa a sessão ICI.
6.  **Execução do Trabalho:** O trabalho de treinamento de aprendizado de máquina começa a ser executado, utilizando a rede ICI para comunicação entre os TPUs.

## Comandos de Controle

*   Os comandos para configurar as conexões dos OCS são enviados pelo Pod Manager aos switches através de RPCs.
*   A libtpunet utiliza comandos para programar as tabelas de roteamento e configurar outros parâmetros da rede ICI diretamente nos chips de TPU.

---


## Page 25

# Fórmulas e Especificações

## Fórmulas e Especificações

---

### Largura de Banda

*   **Largura de Banda por Link ICI:** Cada link ICI (Inter-Chip Interconnect) pode transportar 50 GB/s de largura de banda unidirecional.

### Latência

*   A latência da rede ICI é minimizada pelo uso de comunicação direta de dispositivo para dispositivo (RDMA) e uma topologia de toro 3D. No entanto, valores de latência específicos não são publicamente detalhados.

### Consumo de Energia

*   **Custo de Energia dos OCS:** Os Optical Circuit Switches (OCS) e a fibra óptica representam menos de 3% da energia total de operação de um pod TPUv4.

### Capacidade Térmica

*   Detalhes específicos sobre a capacidade térmica e o sistema de resfriamento não são fornecidos nos documentos consultados. No entanto, a infraestrutura de um supercomputador dessa escala exige um sistema de resfriamento robusto para manter a temperatura operacional dos componentes.

## Vulnerabilidades Identificadas

### Vulnerabilidades

#### Pontos de Acesso Potenciais

*   **Falhas de Hardware:** Componentes como chips de TPU, links ICI, e OCSs podem falhar, necessitando de mecanismos de detecção e recuperação.
*   **Erros de Software:** Bugs no Pod Manager, libtpynet, ou healthd podem levar a configurações incorretas ou falha na detecção de problemas.

#### Falhas Conhecidas

*   **Falhas de Link ICI:** A degradação ou falha de um link óptico ICI pode interromper a comunicação entre os TPUs.
*   **Falhas de OCS:** Uma falha em um Optical Circuit Switch pode tornar um conjunto de links ICI indisponível.
*   **Falhas de Máquina:** A falha de uma máquina host de TPU remove os TPUs associados do pool de recursos.

---


## Page 26

# Vetores de Ataque

* Os documentos consultados focam em resiliência a falhas de hardware e software, não em vetores de ataque de segurança.
A arquitetura com múltiplos tenants (usuários) em um mesmo pod pode, teoricamente, apresentar vetores de ataque se o isolamento entre os slices de TPU não for perfeito, mas o artigo menciona que o OCS-based reconfigurability provê um bom isolamento entre os slices.

# Fontes

https://www.usenix.org/system/files/nsdi24-zu.pdf

# 14. Arquitetura física do rack de TPU Pods

## Arquitetura e Design

A arquitetura física dos TPU Pods é hierárquica, começando com os chips individuais de TPU, que são agrupados em bandejas (trays). Cada bandeja contém 4 chips de TPU. Um conjunto de 16 bandejas forma um rack de TPU, totalizando 64 chips de TPU por rack. Além dos TPUs, os racks também contêm bandejas de Host de CPU, com uma proporção típica de 1 CPU para cada 4 TPUs. A unidade fundamental de construção do pod é o “cubo”, que consiste em 64 chips de TPU interconectados em uma topologia 3D Torus de 4x4x4. A escala dos pods varia com a geração do TPU; por exemplo, um pod de TPU v4 pode conter até 64 racks, somando 4096 chips, enquanto um pod de TPU v7 pode escalar para 144 racks, totalizando 9216 chips.

## Conexões Físicas

As conexões físicas dentro de um cubo de TPU (64 chips) são realizadas através de cabos de cobre de conexão direta (Direct-Attached Copper - DAC) e circuitos impressos (PCBs), que são adequados para as curtas distâncias dentro de um mesmo rack. Para a interconexão entre racks e a formação de pods em larga escala, a arquitetura utiliza Switches de Circuito Óptico (Optical Circuit Switches - OCS). O OCS substitui os tradicionais switches de pacotes elétricos, criando caminhos de luz físicos e diretos entre os pontos de extremidade, o que reduz significativamente a latência e o consumo de energia. O Google emprega principalmente switches OCS baseados em MEMS (Micro-Electro-Mechanical Systems). No TPU v4, são utilizados switches OCS de 136 portas (128 efetivas), enquanto a arquitetura do TPU v7 prevê o uso de switches de 300 portas (288 efetivas) para acomodar a maior densidade de interconexão.

## Protocolos e Software

O principal protocolo de comunicação para a interconexão de alta velocidade entre os TPUs em um pod é o Acesso Remoto Direto à Memória (RDMA - Remote Direct Memory Access). Este é um modelo “push-only”, onde uma TPU pode iniciar uma operação de escrita (push) de seu buffer de memória local para um buffer de memória em um dispositivo remoto de forma assíncrona, mas não pode iniciar uma operação de leitura da memória de outro dispositivo. A operação de cópia remota assíncrona é gerenciada pela função `pltpu.make_async_remote_copy` no ambiente de programação Pallas. O ecossistema de software para desenvolvimento em TPUs é predominantemente baseado nos frameworks JAX e PyTorch, que fornecem as abstrações de alto nível para orquestrar as cargas de trabalho de aprendizado de máquina. A API do Cloud TPU oferece a interface para o gerenciamento programático dos recursos de TPU na nuvem.

## Fórmulas e Especificações

A largura de banda da interconexão entre chips (ICI) para o TPU v5p é especificada em 4.8 Terabits por segundo (Tb/s) por chip. A latência da comunicação através do ICI está na ordem de microssegundos (μs). O consumo de energia de um módulo acelerador Edge TPU é uma função do modelo de aprendizado de máquina (ML) em execução, da taxa de inferências por segundo e da frequência de operação de cada Edge TPU. As propriedades térmicas relevantes para o projeto e operação dos TPUs incluem a expansão térmica, a condutividade térmica, o calor específico e o comportamento de combustão dos materiais utilizados.

---


## Page 27

# Vulnerabilidades Identificadas

As vulnerabilidades identificadas nos TPU Pods estão principalmente relacionadas às suas interfaces físicas e de baixo nível. As interfaces PCIe (Peripheral Component Interconnect Express) Gen5 (para o host) e Gen2 (para o gBMC - Google Baseboard Management Controller), que utilizam a tecnologia SERDES (Serializer/Deserializer) para comunicação de alta velocidade, representam um ponto de acesso potencial. Se os drivers do TPU expuserem interfaces de controle de I/O (IOCTL) para a configuração do SERDES sem uma validação rigorosa de entrada, podem surgir vulnerabilidades que permitem a violação da integridade dos dados, escalão de privilégios e ataques de negação de serviço (DoS). Além disso, a complexidade dos manipuladores de IOCTL pode criar uma superfície de ataque no kernel, com potencial para estouro de buffer ou execução de código arbitrário. Outra classe de vulnerabilidades são os ataques de canal lateral (side-channel attacks), como o “TPUXtract”, que demonstrou a capacidade de extrair hiperparâmetros de modelos de redes neurais através da medição das emanações eletromagnéticas do chip TPU durante a computação.

# Fontes

https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://hc2025.hotchips.org/assets/program/tutorials/HC2025.T1DCRacks.S7.Pankaj.pdf
https://globaltechresearch.substack.com/p/the-ironwood-an-introduction-to-google
https://docs.jax.dev/en/latest/pallas/tpu/distributed.html http://www.antihackingonline.com/potential-risk-of-cve/do-pcie-ioctl-and-serdes-pose-a-risk-to-google-ironwood-tpu-15th-dec-2025/

---

# 15. Relatório Técnico Detalhado sobre a Interconexão de Chips TPU (ICI)

## Arquitetura e Design

A Interconexão Inter-Core (ICI) é a tecnologia de malha de rede de alta velocidade e baixa latência projetada pelo Google para conectar múltiplos chips de Unidade de Processamento Tensorial (TPU), permitindo que operem como um único supercomputador coeso para cargas de trabalho de aprendizado de máquina. A arquitetura da ICI evoluiu significativamente ao longo das gerações de TPU para suportar escalas cada vez maiores.

As primeiras gerações, como a TPU v2, v3, v5e e v6e, implementaram uma **topologia de toro 2D**, na qual cada chip se conecta diretamente a quatro vizinhos (norte, sul, leste e oeste). Essa estrutura cria uma malha onde os dados podem se mover eficientemente em duas dimensões. As gerações mais recentes e poderosas, incluindo a TPU v4, v5p e a mais recente, Ironwood (TPU7x), adotaram uma **topologia de toro 3D**. Nessa configuração, cada chip se conecta a seis vizinhos, adicionando um terceiro eixo de comunicação. A topologia 3D reduz drasticamente o diâmetro da rede – a distância máxima entre quaisquer dois chips no sistema – o que, por sua vez, diminui a latência para operações de comunicação globais, como all-reduce, que são fundamentais para o treinamento de modelos de grande escala.

O bloco de construção fundamental da infraestrutura de TPU em larga escala é o “**cubo**”, uma unidade que consiste em 64 chips de TPU densamente interconectados em um único rack, formando uma malha all-to-all com a topologia 3D Torus. Para escalar para além de um único cubo, a arquitetura “**Multislice**” é empregada, estendendo a conectividade para além da ICI ao utilizar a rede do data center (DCN) para a comunicação entre slices (grupos de chips), enquanto a comunicação de altíssima velocidade dentro de cada slice continua a ser tratada pela ICI.

## Conexões Físicas

A implementação física da ICI utiliza diferentes tecnologias dependendo da distância e da escala da interconexão. Para as conexões de curta distância dentro de um cubo de TPU, onde os chips estão no mesmo rack, são utilizados **cabos de cobre de conexão direta (DAC - Direct Attached Copper)**. Essa abordagem minimiza o custo, a latência e o consumo de energia para a comunicação de alta largura de banda entre chips próximos.

Para interconectar múltiplos cubos e construir superpods massivos que podem abranger vários racks de data center, a ICI transita para uma infraestrutura óptica. São utilizados **transceptores ópticos** e **Switches de Circuito Óptico (OCS - Optical Circuit Switch)**. O OCS é uma tecnologia chave que permite a reconfiguração dinâmica da topologia da rede em tempo real. Isso não apenas melhora a flexibilidade no alocação de recursos, mas também aumenta a resiliência do sistema, permitindo que as

---


## Page 28

conexões de dados contornem ativamente falhas em links ópticos ou em switches específicos, garantindo a continuidade das cargas de trabalho.

Em termos de interfaces físicas nos chips, o Ironwood (TPU7x) é equipado com **quatro gaiolas OSFP (Octal Small Form-factor Pluggable)** por chip, que são dedicadas às conexões ICI de alta velocidade. Além disso, possui uma **gaiola CDFP** para a conexão com a CPU host através de um link PCIe (Peripheral Component Interconnect Express).

## Protocolos e Software

A pilha de software e os protocolos de comunicação da ICI são co-projetados com o hardware para maximizar o desempenho. A ICI utiliza um **protocolo de comunicação customizado baseado em links seriais de alta velocidade**, em vez de depender de padrões de rede comerciais como Ethernet ou InfiniBand. Essa customização permite otimizações profundas no nível do hardware para as operações de comunicação mais comuns em cargas de trabalho de IA.

A comunicação entre os chips dentro de um pod é facilitada por **RDMA (Remote Direct Memory Access)** sobre a malha da ICI. O RDMA permite que um chip acesse a memória de outro chip diretamente, sem envolver o sistema operacional do chip remoto, o que resulta em uma comunicação de latência extremamente baixa e alta largura de banda, essencial para a computação distribuída eficiente.

Frameworks de software como **JAX** e **Pallas** (uma extensão do JAX para programação em nível de kernel) são otimizados para a arquitetura TPU e sua ICI. O JAX fornece uma API de alto nível, semelhante ao NumPy, que abstrai muitas das complexidades da computação distribuída, enquanto o Pallas oferece aos desenvolvedores um controle mais granular, permitindo a criação de kernels de computação distribuída de alta performance que exploram diretamente a massiva largura de banda da ICI. Para monitoramento e depuração, a ferramenta de linha de comando `tpu-info` interage com a biblioteca de tempo de execução `libtpu` para fornecer métricas detalhadas sobre o estado e a utilização dos dispositivos TPU e de suas interconexões.

## Fórmulas e Especificações

As especificações da ICI demonstram um foco implacável no aumento da largura de banda e na redução da latência a cada geração.

*   **Largura de Banda (Bidirecional por Chip):**
    *   TPU v5e: 400 GB/s
    *   TPU v4: Cada link ICI oferece 50 GB/s de largura de banda unidirecional.
    *   Ironwood (TPU7x): 1,2 TB/s
*   **Topologia e Largura de Banda de Bisseção:** A largura de banda de bisseção, que mede a largura de banda mínima entre duas metades iguais da rede, é uma métrica crítica para o desempenho de algoritmos de comunicação global. A topologia de toro da ICI garante uma largura de banda de bisseção uniforme, independentemente de como a carga de trabalho é partitionada entre os chips. Em uma topologia 3D, configurações cúbicas como **4x4x4 (64 chips)** ou **8x8x8 (512 chips)** são ideais, pois maximizam essa métrica.
*   **Consumo de Energia:** Embora os números exatos sejam proprietários, estima-se que o consumo de energia de um chip de TPU individual varie entre **175W** e **250W**, um valor notavelmente eficiente dada a sua capacidade computacional.

## Vulnerabilidades Identificadas

A complexidade e a natureza física da ICI introduzem vetores de ataque potenciais que devem ser considerados.

*   **Ataques de Canal Lateral (Side-Channel Attacks):** Pesquisas demonstraram a viabilidade de ataques de canal lateral contra TPUs. O ataque **TPUXtract**, por exemplo, conseguiu extrair hiperparâmetros de modelos de redes neurais em execução no Edge TPU do Google ao medir as flutuações do campo eletromagnético emitido pelo chip. Tais ataques podem, teoricamente, explorar outros canais físicos, como variações no consumo de energia ou no tempo de execução, para inferir informações confidenciais sobre o modelo ou os dados.
*   **Segurança da Infraestrutura e Resiliência:** A natureza síncrona do treinamento de modelos em larga escala representa um ponto de vulnerabilidade. A falha de um único processo de TPU em um trabalho distribuído pode exigir a reinicialização de todo o trabalho, pois todos os processos devem estar ativos para realizar as atualizações de peso síncronas através dos

---


## Page 29

coletivos da ICI. Além disso, vulnerabilidades no driver do TPU, como as relacionadas a chamadas de sistema ioctl, poderiam potencialmente ser exploradas para comprometer sandboxes de contêineres como o gVisor.

*   **Isolamento de Tráfego:** Como contramedida, em certas configurações, a arquitetura da ICI pode garantir que cada trabalho de computação tenha propriedade exclusiva dos links de interconexão que utiliza. Isso proporciona um forte isolamento de tráfego, o que pode mitigar o risco de espionagem ou interferência entre diferentes cargas de trabalho em execução no mesmo superpod.

## Fontes

*   [https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm](https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm)
*   [https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations](https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations)
*   [https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codedsigned-ai-stack](https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codedsigned-ai-stack)
*   [https://newsletter.semianalysis.com/p/tpuv7-google-takes-a-swing-at-the](https://newsletter.semianalysis.com/p/tpuv7-google-takes-a-swing-at-the)
*   [https://www.usenix.org/system/files/nsdi24-zu.pdf](https://www.usenix.org/system/files/nsdi24-zu.pdf)
*   [https://nquiringminds.com/cybernews/tpuxtract-new-sidechannel-attack-exposes-vulnerabilities-in-ai-models/](https://nquiringminds.com/cybernews/tpuxtract-new-sidechannel-attack-exposes-vulnerabilities-in-ai-models/)
*   [https://gvisor.dev/docs/user_guide/tpu/](https://gvisor.dev/docs/user_guide/tpu/)

---

# 16. Pesquisa técnica detalhada sobre a arquitetura de chip e layout físico das TPUs v4 e v5 da Google.

## Arquitetura e Design

A TPU v4 é a quinta geração de arquitetura de domínio específico (DSA) do Google para cargas de trabalho de aprendizado de máquina (ML). Cada chip TPU v4 contém dois TensorCores. Cada TensorCore, por sua vez, é composto por quatro unidades de multiplicação de matrizes (MXUs), uma unidade vetorial e uma unidade escalar. Os dois TensorCores em um único chip compartilham 128 MiB de memória no chip (MEM). Um pacote de TPU v4 é composto por 4 desses chips. Os chips são interconectados em uma topologia de toro 3D. Uma unidade fundamental de interconexão é o ‘cubo’, que consiste em um arranjo de 4x4x4 chips de TPU. O supercomputador completo é construído com 4096 chips. Uma inovação fundamental na TPU v4 é a introdução de Switches de Circuito Óptico (OCS), que permitem a reconfiguração dinâmica da topologia de interconexão. Além disso, a arquitetura inclui SparseCores, que são aceleradores de fluxo de dados projetados para otimizar cargas de trabalho de embedding.

A TPU v5p é uma versão aprimorada da TPU v5. Cada chip TPU v5p, assim como o v4, contém TensorCores com unidades de multiplicação de matriz (MXU), uma unidade vetorial e uma unidade escalar. Um pod de TPU v5p é composto por 8960 chips. A maior tarefa que pode ser agendada é um trabalho de 96 cubos (6144 chips).

## Conexões Físicas

A interconexão entre os chips da TPU v4 é realizada através de uma combinação de links elétricos e ópticos. Dentro de um ‘cubo’ de 4x4x4 chips, as conexões são elétricas. Para conectar múltiplos cubos e formar o supercomputador, são utilizados Switches de Circuito Óptico (OCS). Cada ‘cubo’ possui 96 links ópticos por face, totalalizando 136 links por bloco de 4x4x4. Oito OCSs conectam os 48 pares de cabos de 64x4 blocos, alcançando um total de 4096 chips. Os OCSs permitem a reconfiguração da topologia da rede, possibilitando a criação de toros torcidos (‘twisted torus’) que melhoram o desempenho da comunicação all-to-all. O pacote da TPU v4 mostra um PCB com 4 chips, cada um com 4 stacks de HBM, e 16 conectores OSFP na parte inferior para as interconexões ICI (Inter-Chip Interconnect).

Os chips em um pod de TPU v5p são interconectados com links reconfiguráveis de alta velocidade em uma topologia de toro 3D. A largura de banda da interconexão entre chips (ICI) bidirecional por chip é de 1200 GBps. A resiliência da ICI, semelhante à da v4, permite o roteamento em torno de falhas nos links ópticos e nos switches de circuito óptico (OCS).

---


## Page 30

# Protocolos e Software

A TPU v4 implementa uma interface de memória de endereçamento lógico compartilhado, permitindo que o software acesse memórias físicas em diferentes chips. O software pode explorar o paralelismo de dados, modelo e pipeline. A TPU v4 é programada usando frameworks de ML como TensorFlow, PyTorch e JAX. As APIs de alto nível abstraem muitos dos detalhes de baixo nível da arquitetura de hardware.

As informações sobre protocolos e software específicos da v5p não foram detalhadas nos documentos consultados, mas presume-se que sejam uma evolução dos sistemas usados na v4, com suporte para os mesmos frameworks de ML (TensorFlow, PyTorch, JAX).

# Fórmulas e Especificações

TPU v4:

*   Desempenho de Pico: 275 TFLOPS (int8)
*   Clock: 1050 MHz
*   Tamanho do Die: ~600 mm2
*   Transistores: 22 bilhões
*   Chips por Host de CPU: 2
*   Potência (min/média/máx): 90, 121, 170/192 W
*   Interconexão entre Chips: 6 links @ 50 GB/s
*   Configuração do Processador: 4096 chips
*   Estilo de Processador: Single Instruction 2D Data
*   Memória no Chip: 32 MiB (CMEM) + 128 MiB (VMEM)
*   Tamanho do Arquivo de Registradores: 0.25 MiB
*   Capacidade de HBM: 32 GiB, 1200 GB/s

TPU v5p:

*   Desempenho de Pico por Chip (BF16): 459 TFLOPs
*   Capacidade e Largura de Banda da HBM2e: 95 GB, 2765 GBps
*   Tamanho do Pod de TPU: 8960 chips
*   Topologia de Interconexão: Toro 3D
*   Largura de Banda da Interconexão entre Chips (ICI) Bidirecional (por chip): 1200 GBps

# Vulnerabilidades Identificadas

A pesquisa revelou algumas vulnerabilidades de segurança potenciais em TPUs, principalmente relacionadas a ataques de canal lateral. O ataque ‘TPUXtract’ demonstrou a capacidade de extrair hiperparâmetros de um modelo de rede neural medindo as emissões eletromagnéticas do chip da TPU. Isso pode permitir que um invasor roube ou replique um modelo de IA. Além disso, a vulnerabilidade CVE-2024-32914 aponta para uma possível divulgação de informações devido a dados não inicializados na função ‘tpu_get_int_state’ do driver da TPU. Embora não diretamente relacionado às TPUs, o ataque ‘Rowhammer’, que explora uma vulnerabilidade em DRAM, também é uma ameaça potencial, pois as TPUs utilizam HBM (High Bandwidth Memory), que é um tipo de DRAM.

# Fontes

https://arxiv.org/pdf/2304.01433
https://www.keysight.com/blogs/en/tech/nwvs/2025/02/25/security-highlight-tpuxtract-a-new-side-channel-attack-on-neural-networks https://nvd.nist.gov/vuln/detail/CVE-2024-32914
https://cloud.google.com/tpu/docs/v5p

---


## Page 31

# 17. Conexões BGA do substrato do pacote de chips TPU

## Arquitetura e Design

A arquitetura do substrato do pacote de chips TPU é centrada em um design co-otimizado de hardware e software para acelerar cargas de trabalho de aprendizado de máquina. O chip TPU, um ASIC (Application-Specific Integrated Circuit), é montado em um substrato orgânico laminado de múltiplas camadas usando uma tecnologia de packaging do tipo BGA (Ball Grid Array) com flip-chip. A estrutura interna de um chip TPU contém um ou mais TensorCores. Cada TensorCore é composto por unidades de multiplicação de matriz (MXUs), uma unidade vetorial e uma unidade escalar. As MXUs, organizadas em uma arquitetura de array sistólico, são o coração computacional do TPU, realizando multiplicações de matriz em alta velocidade. A hierarquia de controle é gerenciada por um host (CPU) que envia instruções e dados para o TPU. O fluxo de dados e comandos segue um pipeline onde os dados são carregados da memória HBM (High Bandwidth Memory) para as MXUs, processados e os resultados são então passados para a próxima camada da rede neural ou armazenados de volta na HBM.

## Conexões Físicas

As conexões físicas do substrato BGA do chip TPU são extremamente densas e otimizadas para alta performance. A conexão do die de silício ao substrato é feita através de micro-bumps em uma configuração flip-chip. O substrato, por sua vez, se conecta à placa de circuito impresso (PCB) principal através de uma matriz de esferas de solda (BGA). A pinagem e as interfaces são proprietárias e não documentadas publicamente, mas são projetadas para suportar as altas taxas de dados entre o TPU e a memória HBM, bem como as interconexões de alta velocidade (ICI - Inter-Chip Interconnect) com outros TPUs. O layout do PCB e do substrato é um design complexo de múltiplas camadas com impedância controlada para garantir a integridade do sinal em altas frequências. As conexões entre os chips em um TPU Pod são realizadas por uma malha 2D ou 3D de alta velocidade, que em versões mais recentes (como o TPU v4) utiliza links óticos reconfiguráveis (OCS) para interconexões entre os “cubos” de TPUs, enquanto as conexões dentro de um cubo são feitas com links de cobre.

## Protocolos e Software

A comunicação entre os chips TPU e entre os TPUs e seus hospedeiros é governada por protocolos proprietários de alta velocidade desenvolvidos pelo Google. A API primária para programação de TPUs é o XLA (Accelerated Linear Algebra), um compilador de domínio específico que otimiza os modelos de aprendizado de máquina para a arquitetura TPU. Bibliotecas de alto nível como TensorFlow, PyTorch e JAX abstraem a complexidade da programação em baixo nível. A sequência de inicialização envolve o carregamento do firmware no TPU, a configuração das conexões de rede e a inicialização do ambiente de execução no host. Os comandos de controle são enviados do host para o TPU para gerenciar a execução dos modelos, incluindo o carregamento de pesos, o enfileiramento de dados de entrada e a recuperação dos resultados.

## Fórmulas e Especificações

As informações sobre fórmulas e equações específicas relacionadas ao design do substrato BGA do TPU não estão publicamente disponíveis.

## Vulnerabilidades Identificadas

As vulnerabilidades específicas do substrato BGA do chip TPU não são publicamente documentadas. No entanto, como em qualquer sistema de computação de alta performance, existem potenciais pontos de acesso e vetores de ataque. Falhas no processo de fabricação do BGA, como solda fria ou curtos-circuitos, podem levar a falhas operacionais. Ataques de canal lateral (side-channel attacks) poderiam, teoricamente, explorar variações no consumo de energia ou na emissão eletromagnética para inferir informações sobre os dados que estão sendo processados. O acesso físico ao hardware poderia permitir a manipulação do substrato ou a interceptação de sinais, embora isso seja mitigado pela segurança física dos data centers do Google. As interfaces de rede de alta velocidade, se não forem devidamente securizadas, poderiam ser um vetor para ataques de negação de serviço ou para a exfiltração de dados.

## Fontes

https://anysilicon.com/an-introduction-to-bga-package/
https://www.intel.com/content/dam/www/public/us/en/documents/packaging-databooks/packaging-chapter-14-databook.pdf
https://pcbmake.com/bga-substrate-material/

---


## Page 32

https://www.ti.com/lit/pdf/spru811
https://thinkrobotics.com/blogs/tutorials/re-balling-bga-chips-a-home-workshop-guide?srsltid=AfmBOoqMCZwzVIH5LU7OD2qm1f3nVpIl296H0B6JThwKUIDBFleQQWW6
https://anysilicon.com/bga-substrate-design/
https://www.venture-mfg.com/bga-substrate-suppliers/
https://en.wikipedia.org/wiki/Tensor_Processing_Unit
https://www.nextpcb.com/blog/bga-packaging-guide
https://pcbmake.com/bga-substrates/
https://resources.altium.com/p/how-to-successfully-design-a-bga-substrate
https://www.bestpcbs.com/blog/2025/10/bga-substrate-design-prototyping-bga-substrate-suppliers/
https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm
https://cloud.google.com/blog/products/compute/ironwood-tpus-and-new-axion-based-vms-for-your-ai-workloads
https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations

# 18. Pesquisa técnica detalhada sobre o design da PCB e o fornecimento de energia da placa TPU.

## Arquitetura e Design

A arquitetura de uma placa TPU (Tensor Processing Unit) é altamente otimizada para computação de aprendizado de máquina. Cada chip TPU v4, por exemplo, contém dois TensorCores (TCs), que por sua vez abrigam quatro unidades de multiplicação de matriz (MXUs) de 128x128 e uma unidade de processamento vetorial (VPU) com 128 lanes. Os TCs compartilham uma memória comum (CMEM) de 128 MiB. O pacote do TPU v4 é composto por um ASIC central e quatro pilhas de memória de alta largura de banda (HBM). A placa de circuito impresso (PCB) é projetada para suportar quatro desses pacotes, cada um com um sistema de refrigeração líquida. A hierarquia de controle de um supercomputador TPU v4 é massivamente paralela, com 4096 chips organizados em cubos de 4x4x4 (64 chips). Cada um desses blocos, juntamente com seus 16 hospedeiros de CPU, se conecta a 48 Switches de Circuito Óptico (OCSes), que permitem a reconfiguração dinâmica da topologia da interconexão para otimizar o fluxo de dados e comandos para diferentes tipos de paralelismo (dados, modelo, pipeline). O design da Rede de Fornecimento de Energia (PDN) é um aspecto crítico, pois os TPUs consomem centenas de amperes a tensões abaixo de 1.0V. Para gerenciar os desafios de transientes de alta corrente (di/dt), perdas por I^2R na PCB e gerenciamento térmico, são empregadas soluções de fornecimento de energia multifásicas com controladores avançados e estágios de potência inteligentes. Essas soluções utilizam indutores acoplados e esquemas de modulação avançados para garantir uma entrega de energia estável e eficiente, com telemetria e monitoramento via PMBus.

## Conexões Físicas

As conexões físicas em uma placa TPU são projetadas para alta largura de banda e baixa latência. A PCB do TPU v4, por exemplo, possui quatro links de Interconexão entre Núcleos (ICI) que formam uma malha 2x2 interna. Além disso, 16 links ICI externos são roteados para outras bandejas para construir a topologia de toro 3D do supercomputador. A conversão do sinal elétrico para óptico ocorre nos conectores de fibra das bandejas de TPU. A placa em si possui quatro conectores PCIe na parte superior e 16 conectores OSFP (Octal Small Form-factor Pluggable) na parte inferior para os links ICI entre bandejas. Os OCSes (Optical Circuit Switches) Palomar, com 136x136 portas, formam a base da interconexão óptica reconfigurável, permitindo a conexão flexível entre os diferentes blocos de TPU. Uma abordagem emergente para o fornecimento de energia é a entrega de energia vertical, onde o regulador de tensão é colocado diretamente sob o processador no lado de trás da PCB. A energia é entregue verticalmente através de vias, o que reduz significativamente a impedância e as perdas em comparação com a entrega lateral tradicional, melhorando a densidade de potência e a eficiência.

## Protocolos e Software

Os protocolos de comunicação e o software em um sistema TPU são projetados para gerenciar a complexa interconexão e otimizar o desempenho. A interconexão entre os chips TPU v4 é realizada através de links ICI (Inter-Core Interconnect). A comunicação entre os racks é gerenciada por links ópticos controlados por OCSes (Optical Circuit Switches). O sistema operacional e o software de tempo de execução fornecem APIs e bibliotecas que permitem aos usuários configurar a topologia da interconexão para otimizar o desempenho para diferentes tipos de paralelismo, como paralelismo de dados, de modelo e de pipeline. O Google utiliza aprendizado de máquina para co-otimizar os modelos de DNN, a topologia do OCS e o SparseCore, indicando uma camada de software de otimização sofisticada. A sequência de inicialização do sistema é projetada para ser incremental, permitindo que cada bloco de 64 chips seja colocado em produção de forma independente. O controle da topologia é realizado através da programação dos OCSes, que podem reconfigurar as conexões em milissegundos para se adaptar às necessidades da carga de trabalho.

---


## Page 33

# Fórmulas e Especificações

As principais fórmulas e equações relacionadas ao design da PCB e ao fornecimento de energia da placa TPU estão focadas na eficiência energética e no gerenciamento de transientes. As perdas de energia na PCB, conhecidas como perdas I^2R, são um fator dominante no cálculo da eficiência do regulador de tensão. A queda de tensão (V_drop) ao longo de um traço da PCB é diretamente proporcional à corrente (I) que flui através dele e à sua resistência ®, expressa como:

V_drop = I * R

A potência dissipada (P_loss) nesse mesmo traço é dada por:

P_loss = I^2 * R

Para o gerenciamento de transientes, a impedância da Rede de Fornecimento de Energia (Z_PDN) deve ser mantida a mais baixa possível em uma ampla faixa de frequência para garantir a estabilidade da tensão durante mudanças rápidas na demanda de corrente. A relação entre a variação de tensão (ΔV), a variação de corrente (ΔI) e a impedância da PDN é dada por:

ΔV = ΔI * Z_PDN

# Vulnerabilidades Identificadas

As vulnerabilidades em um sistema TPU podem surgir de vários pontos. Os OCSes (Optical Circuit Switches), embora ofereçam flexibilidade, podem ser um ponto de acesso potencial se não forem devidamente isolados. O artigo sobre o TPU v4 menciona que os OCSes podem criar um “air gapped network isolation” entre diferentes fatias, o que aumenta a segurança para múltiplos clientes compartilhando um supercomputador. Isso sugere que, sem esse isolamento, a interconexão poderia ser explorada. Os hospedeiros de CPU, que controlam os TPUs, também representam um ponto de acesso potencial que precisa ser protegido. Embora o artigo principal não detalhe falhas de segurança conhecidas ou vetores de ataque específicos, a menção ao isolamento de rede sugere que ataques de canal lateral ou acesso não autorizado entre “tenants” são uma preocupação de segurança relevante. A complexidade do sistema de fornecimento de energia também pode introduzir vulnerabilidades, como a possibilidade de ataques de falha de energia ou manipulação de tensão para induzir erros de computação.

# Fontes

https://arxiv.org/pdf/2304.01433
https://www.analog.com/en/resources/technical-articles/impacts-of-transients-on-ai-accelerator-card-power-delivery.html


19. Pesquisa técnica detalhada sobre a interconexão de múltiplos chips em um TPU tray assembly.

## Arquitetura e Design

A arquitetura de interconexão dos TPUs do Google, especialmente a partir da versão v2, é marcada pela introdução do **Inter-Chip Interconnect (ICI)**, uma tecnologia de interconexão de alta velocidade e baixa latência que elimina a necessidade de redes externas como Ethernet ou InfiniBand para a comunicação entre os chips. Essa abordagem permite que um conjunto de TPUs, conhecido como “pod”, funcione como um único acelerador lógico, otimizando a execução de operações coletivas.

## Topologia de Rede

A topologia da rede de interconexão evoluiu ao longo das gerações de TPUs para atender às crescentes demandas de escalabilidade. As primeiras gerações, como o TPU v2 e v3, utilizavam uma topologia de **toro 2D**, onde cada chip se conectava a quatro vizinhos (norte, sul, leste e oeste). As gerações mais recentes, como o **TPU v4, v5p e v7 (Ironwood)**, adotaram uma topologia de **toro 3D**, com cada chip se conectando a seis vizinhos. Essa mudança reduziu significativamente o diâmetro da rede, diminuindo a latência de comunicação em cenários de pior caso.

O bloco de construção fundamental da rede ICI do TPUv7 é um **cubo 4x4x4** em uma topologia de toro 3D, composto por 64 TPUs. Cada um desses cubos corresponde a um rack físico de 64 TPUs.

---


## Page 34

# Comutação de Circuito Óptico (OCS)

Uma das inovações mais significativas na arquitetura de interconexão dos TPUs é a **Comutação de Circuito Óptico (OCS)**, introduzida com o TPU v4. O OCS utiliza espelhos MEMS (sistemas microeletromecânicos) para redirecionar fisicamente os feixes de luz, criando caminhos ópticos diretos entre os TPUs. Isso elimina a latência de comutação inerente às redes de comutação de pacotes tradicionais.

A arquitetura OCS permite a reconfiguração dinâmica da topologia da rede em nanosegundos, possibilitando a criação de “fatias” de TPUs de qualquer tamanho, de acordo com as necessidades da carga de trabalho, sem a necessidade de recabeamento físico. Além disso, o OCS permite contornar falhas em chips ou links, reprogramando os espelhos para excluir componentes defeituosos e manter a estrutura lógica do toro.

# Hierarquia de Controle e Fluxo de Dados

O controle da rede ICI e do OCS é realizado por um software de rede definido por software (SDN) desenvolvido pelo Google. Esse software gerencia as rotas de rede e a alocação de recursos, permitindo a criação de topologias de rede personalizadas para diferentes cargas de trabalho. O fluxo de dados e comandos entre os TPUs é gerenciado pelo protocolo ICI, que é otimizado para operações de aprendizado de máquina, como all-reduce, all-gather e reduce-scatter.

# Conexões Físicas

A interconexão física dos TPUs em um “tray assembly” é projetada para alta largura de banda e baixa latência, utilizando uma combinação de tecnologias de cobre e óptica.

# Componentes do Tray

*   **TPU Tray:** Cada bandeja (tray) de TPU consiste em uma placa de circuito impresso (PCB) com 4 pacotes de chips de TPU montados.
*   **Conectores:** Cada TPU Ironwood (v7) possui 4 gaiolas OSFP (Octal Small Form Factor Pluggable) para as conexões ICI e 1 gaiola CDFP (400G Form Factor Pluggable) para a conexão PCIe com a CPU hospedeira.

# Fiação e Conexões

*   **Conexões Internas ao Cubo (Intra-Cube):** Dentro de um cubo 4x4x4 de 64 TPUs, as conexões são feitas predominantemente com cabos de conexão direta (DAC - Direct-Attached Copper). Essa abordagem minimiza o custo e o consumo de energia para as distâncias curtas entre os chips.
*   **Conexões Externas ao Cubo (Inter-Cube):** As conexões que saem do cubo 4x4x4, incluindo as conexões de “wrap-around” que formam o toro, e as conexões para outros cubos, utilizam transceptores ópticos. A proporção de transceptores ópticos por TPU varia de acordo com a sua posição no cubo:
    *   **Interior:** 0 transceptores (apenas DACs)
    *   **Face:** 1 transceptor
    *   **Aresta:** 2 transceptores
    *   **Canto:** 3 transceptores

# Layout de PCB e Substrato

*   Os TPUs são conectados entre si na mesma placa através de trilhas no PCB. As conexões entre diferentes bandejas e racks são feitas através dos cabos DAC e ópticos mencionados anteriormente.
*   O Google utiliza um design de fornecimento de energia vertical, com os módulos VRM (Voltage Regulator Module) localizados no lado oposto da placa de PCB, exigindo um “cold plate” para refrigeração.

# Transceptores Ópticos e OCS

*   **Transceptores FR:** Para otimizar a utilização dos OCSs, o Google utiliza transceptores ópticos FR que consolidam 8 comprimentos de onda (100G cada) em uma única fibra óptica usando a tecnologia CWDM8 (Coarse Wave Division Multiplexing), permitindo a transmissão de 800G em um único par de fibras.

---


## Page 35

*   **Circulador Óptico:** Um circulador óptico é integrado ao transceptor WDM para permitir o fluxo de dados full-duplex em uma única fibra, reduzindo pela metade a quantidade de fibras necessárias.
*   **Conexão com OCS:** Cada fibra é conectada a uma porta do OCS, que então redireciona o sinal de luz para a porta de destino, estabelecendo uma conexão física direta entre os TPUs.

## Protocolos e Software

A comunicação e o controle do ecossistema de TPUs são gerenciados por uma pilha de software e protocolos desenvolvidos pelo Google, com um esforço crescente para a abertura e integração com ferramentas de mercado.

### Protocolos de Comunicação

*   **Inter-Chip Interconnect (ICI):** Este é o protocolo de comunicação de baixo nível que opera sobre a infraestrutura física de interconexão dos TPUs. Ele é otimizado para operações coletivas de alta performance, como all-reduce, all-gather e reduce-scatter, que são fundamentais para o treinamento de modelos de aprendizado de máquina em larga escala. O ICI implementa algoritmos coletivos diretamente em hardware, o que resulta em uma performance significativamente superior em comparação com implementações baseadas em Ethernet.
*   **PCIe:** A comunicação entre os TPUs e as CPUs hospedeiras é realizada através do protocolo PCI Express (PCIe), utilizando cabos DAC (Direct-Attached Copper) para a conexão física.

### APIs e Bibliotecas

*   **XLA (Accelerated Linear Algebra):** XLA é um compilador de otimização para álgebra linear que pode acelerar o desempenho de modelos TensorFlow e JAX em TPUs, GPUs e CPUs. Ele compila o grafo de computação do modelo em código de máquina otimizado para o hardware específico, realizando otimizações como a fusão de operações.
*   **JAX:** JAX é uma biblioteca de alto desempenho para transformações de programas em Python, incluindo diferenciação automática e compilação JIT (Just-In-Time) para XLA. É a principal estrutura de programação para TPUs dentro do Google.
*   **PyTorch/XLA:** Para suportar o ecossistema PyTorch, o Google desenvolveu o PyTorch/XLA, que permite a execução de modelos PyTorch em TPUs. Recentemente, o Google tem investido em um novo backend “nativo” para PyTorch em TPUs, que suportará execução “eager” por padrão e se integrará com torch.compile e DTensor, proporcionando uma experiência mais próxima à do desenvolvimento com CUDA em GPUs.
*   **Pallas:** É uma linguagem de autoria de kernels para escrever kernels personalizados para TPUs, semelhante ao Triton da NVIDIA. O Google está trabalhando para integrar o Pallas como um alvo de codegen para o compilador Torch Dynamo/Inductor.
*   **vLLM e SGLang:** O Google está trabalhando para suportar bibliotecas de inferência de código aberto como vLLM e SGLang em TPUs, traduzindo o código do modelo PyTorch para JAX para aproveitar o fluxo de compilação maduro do JAX para TPUs.

### Sequência de Inicialização e Comandos de Controle

*   A inicialização e o gerenciamento dos pods de TPU são realizados por um software de orquestração do Google, que é responsável por alocar recursos, configurar a topologia da rede (através do OCS) e monitorar o estado do sistema. Os usuários interagem com os TPUs através de APIs de alto nível, como as fornecidas pelo JAX e PyTorch/XLA, que abstraem a complexidade do hardware subjacente.

### Fórmulas e Especificações

As especificações de desempenho dos TPUs, como largura de banda, latência, consumo de energia e capacidade térmica, são cruciais para entender a sua eficiência e escalabilidade.

### Largura de Banda

*   **TPU v6e:** 13 TB/s de largura de banda ICI por chip.
*   **TPU v5p:** 4.800 Gbps (600 GB/s) por chip em seis links de toro 3D.

---


## Page 36

*   **TPU v7 (Ironwood):** 9,6 Tbps (1,2 TB/s) de largura de banda bidirecional agregada por chip, através de quatro links ICI.
*   **Comparativo:** Uma interface de rede 400GbE oferece 50 GB/s de largura de banda bidirecional.

## Latência

*   A latência de comunicação é significativamente reduzida pela arquitetura de interconexão direta (ICI) e pela comutação de circuito óptico (OCS). O OCS tem um tempo de comutação inferior a 10 nanosegundos.
*   A topologia de toro 3D reduz o diâmetro da rede, diminuindo a latência no pior caso. Para um pod de 4.096 chips, o número máximo de saltos cai de ~128 (toro 2D) para ~48 (toro 3D).

## Consumo de Energia

*   O design do TPU visa a eficiência energética. A refrigeração líquida, implementada desde o TPU v3, permite um controle ativo da taxa de fluxo do líquido de arrefecimento, otimizando a refrigeração com base na carga de trabalho de cada chip.
*   A comutação de circuito óptico (OCS) consome energia apenas durante a reconfiguração dos espelhos MEMS, permanecendo passiva e com baixo consumo de energia enquanto as conexões estão estáveis.

## Capacidade Térmica

*   A refrigeração líquida e o design de fornecimento de energia vertical com VRMs no lado oposto da PCB são projetados para gerenciar a dissipação de calor dos TPUs, que podem atingir altos níveis de consumo de energia sob carga máxima.

## Vulnerabilidades Identificadas

A análise de vulnerabilidades em um sistema complexo como o TPU pod do Google requer a consideração de múltiplos vetores de ataque, desde o nível físico até o de software.

## Pontos de Acesso Potenciais

*   **Acesso Físico:** O acesso físico aos racks de TPU, embora altamente controlado em datacenters do Google, representa um ponto de vulnerabilidade. Um invasor com acesso físico poderia tentar interceptar ou manipular as conexões de cobre (DAC) ou fibra óptica, ou mesmo extrair dados diretamente dos chips.
*   **Interface de Rede:** A conexão dos pods de TPU com a rede de datacenter mais ampla (DCN - Datacenter Network) e, por sua vez, com a internet, representa um ponto de acesso potencial. Embora a rede do Google seja altamente segura, qualquer vulnerabilidade na pilha de rede poderia ser explorada para obter acesso aos TPUs.
*   **Software e Firmware:** Vulnerabilidades no firmware dos TPUs, no sistema operacional do host, nas bibliotecas de software (XLA, PyTorch/XLA) ou no software de orquestração poderiam ser exploradas para executar código malicioso, vazar dados ou interromper o funcionamento do sistema.

## Falhas Conhecidas

*   Não há informações públicas detalhadas sobre falhas de segurança específicas nos TPUs do Google. No entanto, como qualquer sistema de hardware e software complexo, é provável que existam vulnerabilidades que são descobertas e corrigidas continuamente pelo Google.

## Vetores de Ataque

*   **Ataques de Canal Lateral (Side-Channel Attacks):** Um invasor poderia tentar inferir informações confidenciais (como os pesos de um modelo de IA) analisando variações no consumo de energia, tempo de execução ou emissões eletromagnéticas dos TPUs.
*   **Ataques de Negação de Serviço (Denial-of-Service - DoS):** Um ataque DoS poderia visar sobrecarregar a rede de interconexão (ICI), os OCSs ou os próprios TPUs, tornando o sistema indisponível para cargas de trabalho legítimas.
*   **Manipulação de Modelo:** Um invasor poderia tentar manipular o comportamento de um modelo de IA durante o treinamento ou a inferência, introduzindo dados maliciosos ou modificando os pesos do modelo.

---


## Page 37

*   **Extração de Modelo:** Um invasor poderia tentar roubar um modelo de IA treinado, explorando vulnerabilidades de software para acessar a memória onde o modelo está armazenado.

## Fontes

https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://newsletter.semianalysis.com/p/tpuv7-google-takes-a-swing-at-the

---

# 20. Design do fluxo de ar de resfriamento do gabinete de rack TPU

## Arquitetura e Design

A arquitetura de resfriamento dos TPUs do Google evoluiu do resfriamento a ar nas gerações iniciais (v1 e v2) para um sistema de resfriamento líquido em escala de datacenter a partir do TPUv3, uma necessidade impulsionada pelo aumento da densidade de potência. Este sistema utiliza racks de Unidades de Distribuição de Refrigerante (CDUs), que funcionam de forma análoga a um conjunto de radiador e bomba. Normalmente, um rack de CDU contém seis unidades, com cinco ativas e uma de reserva para garantir a continuidade da operação durante a manutenção. As CDUs realizam a troca de calor entre o líquido de arrefecimento do circuito dos TPUs e o fornecimento de água da instalação, sem que haja mistura entre os fluidos. O líquido de arrefecimento é distribuído para os servidores TPU através de manifolds, com os chips de TPU conectados em série. Isso implica que os chips subsequentes no circuito recebem um líquido com temperatura progressivamente mais alta, e o dimensionamento do sistema é feito com base nas necessidades do último chip da série. Para otimizar a transferência de calor, o Google emprega uma placa fria de fluxo dividido (split-flow) e, a partir do TPUv4, adotou um design de resfriamento direto no die (bare-die), que elimina o encapsulamento do chip para melhorar a dissipação de calor, uma medida necessária para lidar com o aumento de 1.6x no consumo de energia em comparação com o TPUv3. A patente US7559209B2 detalha um sistema modular com unidades de resfriamento individuais para cada grupo de componentes, conectadas a um sistema de linha de líquido central, que pode incluir trocadores de calor água/água e ar/água para resfriamento adicional.

## Conexões Físicas

As conexões físicas do sistema de resfriamento líquido dos TPUs são projetadas para facilitar a manutenção e garantir a confiabilidade. O sistema utiliza mangueiras flexíveis e acoplamentos de desconexão rápida, permitindo a fácil substituição de componentes sem a necessidade de drenar todo o sistema. As unidades de resfriamento individuais são conectadas ao sistema de linha de líquido central através de pontos de ramificação. A unidade de linha, por sua vez, possui pontos de conexão com elementos de acoplamento para conectar as linhas de ramificação que levam aos grupos de componentes eletrônicos, conforme descrito na patente US7559209B2.

## Protocolos e Software

O gerenciamento de temperatura, especialmente em dispositivos como o Edge TPU, é realizado através de uma combinação de software e protocolos de hardware. A temperatura da junção do chip pode ser monitorada através do parâmetro `temp`, acessível via sysfs no Linux ou contadores de desempenho no Windows. Com base nessa leitura, um mecanismo de Escalonamento Dinâmico de Frequência (DFS) ajusta a frequência de operação do chip para controlar a geração de calor. Por padrão, o DFS reduz a frequência em incrementos de 50% ao atingir limiares de temperatura pré-definidos (85°C, 90°C e 95°C). Além disso, existem protocolos de desligamento de hardware para proteger o chip contra danos permanentes. O parâmetro `hw_temp_warn2` (padrão de 100°C) pode ser configurado para desligar o Edge TPU automaticamente. Em outros módulos, pinos de interrupção são acionados para alertar o sistema sobre a alta temperatura, permitindo uma resposta controlada, como o acionamento de um ventilador ou o desligamento do sistema.

## Fórmulas e Especificações

PUE (Power Usage Effectiveness) = 1.1

Escalonamento Dinâmico de Frequência (DFS) - Pontos de Disparo:

*   `trip_point0_temp` (85°C): Frequência reduzida para 250 MHz

---


## Page 38

*   `trip_point1_temp` (90°C): Frequência reduzida para 125 MHz
*   `trip_point2_temp` (95°C): Frequência reduzida para 62.5 MHz

## Vulnerabilidades Identificadas

Os sistemas de resfriamento de TPU, como componentes críticos da infraestrutura de data center, apresentam vulnerabilidades que podem ser exploradas para causar interrupções operacionais. Os Sistemas de Gerenciamento de Edifícios (BMS/BAS), que controlam a refrigeração, são um vetor de ataque significativo. A exploração de suas vulnerabilidades pode levar à interrupção do resfriamento e, consequentemente, ao superaquecimento e desligamento dos servidores. Além disso, malwares como o PIPEDREAM podem manipular protocolos industriais onipresentes, como o OPC-UA, para atacar os sistemas de controle de temperatura. Falhas conhecidas incluem interrupções de energia que podem desligar os chillers e falhas nos próprios sistemas de resfriamento, que já causaram interrupções de serviço em larga escala em data centers de grandes provedores de nuvem.

## Fontes

https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://chipsandcheese.com/p/googles-liquid-cooling-at-hot-chips
https://patents.google.com/patent/US7559209B2/en
https://www.dragos.com/blog/data-centre-operations-cooling-systems-are-possible-targets-for-operational-disruption
https://www.coral.ai/docs/pcie-parameters/

---

## 21. Infraestrutura de cabeamento de fibra óptica dos TPU Pods

### Arquitetura e Design

A arquitetura dos TPU Pods do Google é projetada para escalabilidade e eficiência massivas, utilizando uma combinação de interconexões elétricas e ópticas. A unidade fundamental da arquitetura é o “Cubo”, um agrupamento de 4x4x4, totalizando 64 chips TPU. Múltiplos Cubos são interconectados para formar um Pod. Um Pod TPUv4, por exemplo, é composto por 64 Cubos, somando 4096 chips TPU, enquanto um Pod TPUv7 (Ironwood) escala para 144 Cubos, ou 9216 chips.

Cada chip TPU possui seis links de alta velocidade Inter-Chip Interconnect (ICI) que se conectam aos seus vizinhos nas direções ±X, ±Y e ±Z, formando uma topologia de grade 3D Torus. Dentro de um Cubo, essas conexões ICI são implementadas eletricamente através de backplanes de PCB e cabos de cobre de conexão direta (DACs).

A interconexão entre os Cubos é onde a infraestrutura de fibra óptica se torna crítica. As conexões que saem das faces externas de um Cubo (96 links ópticos por Cubo) são roteadas através de Switches de Circuito Óptico (OCS). Esses OCS, como o Palomar da Google, não são switches de pacotes tradicionais. Em vez disso, eles funcionam como painéis de conexão (patch panels) dinâmicos e reconfiguráveis. Utilizando matrizes de sistemas microeletromecânicos (MEMS) 2D, lentes e câmeras, o OCS direciona fisicamente os feixes de luz de uma fibra de entrada para uma fibra de saída, criando um caminho óptico direto e de baixa latência entre os chips TPU em diferentes Cubos. Isso permite que a topologia da rede seja reconfigurada dinamicamente para otimizar o desempenho para diferentes cargas de trabalho de aprendizado de máquina, um recurso poderoso gerenciado pela pilha de software do Google, que inclui o compilador XLA e o sistema de rede definido por software (SDN) Orion.

O fluxo de dados e comandos segue essa hierarquia: a comunicação intra-Cubo é elétrica e de altíssima velocidade, enquanto a comunicação inter-Cubo é óptica, permitindo distâncias maiores e reconfigurabilidade através do OCS. Essa abordagem híbrida equilibra custo, desempenho e flexibilidade em uma escala massiva.

### Conexões Físicas

As conexões físicas na infraestrutura dos TPU Pods do Google empregam uma abordagem híbrida óptico-elétrica para otimizar a comunicação em diferentes escalas. A fiação interna de um “Cubo” (a unidade de rack contendo 64 TPUs) é predominantemente elétrica. As conexões de alta velocidade Inter-Chip Interconnect (ICI) entre os chips TPU dentro de um mesmo Cubo são realizadas através de backplanes de PCB e cabos de conexão direta (DACs), garantindo latência mínima para a comunicação local na topologia 3D Torus.

Quando a comunicação precisa se estender para além de um único Cubo, a infraestrutura transita para a fibra óptica. Cada Cubo possui 96 links ópticos em suas faces externas que servem como pontos de saída e entrada para a rede inter-Cubos. Em um Pod TPUv4 com 4096 chips (64 Cubos), isso totaliza 6.144 links de fibra óptica. Esses cabos de fibra são conectados aos Switches de

---


## Page 39

Circuito Óptico (OCS), que atuam como o coração da rede reconfigurável. O OCS, como o Palomar, é essencialmente um painel de conexão óptico massivo e dinâmico, com um grande número de portas (por exemplo, 128 portas efetivas por OCS no TPUv4) para acomodar as fibras de múltiplos Cubos.

Em termos de conectores e interfaces, os sistemas mais recentes, como o TPUv5e, que utilizam uma topologia 2D Torus mais econômica, empregam conectores QSFP-DD para os cabos DAC que estabelecem as conexões no eixo X. O layout da placa de circuito impresso (PCB) é densamente projetado; uma única placa TPUv4, por exemplo, abriga quatro chips TPU e seu sistema de refrigeração líquida associado. Embora detalhes específicos de pinagem não sejam publicamente divulgados, a arquitetura geral indica que cada chip TPU possui seis interfaces ICI de alta largura de banda para se conectar à malha 3D Torus, que são então mapeadas para as conexões elétricas internas ou para os transceptores ópticos para comunicação externa.

## Protocolos e Software

A operação dos TPU Pods é orquestrada por uma sofisticada pilha de software e protocolos de comunicação. O protocolo de comunicação primário para a interconexão de alta velocidade entre os chips é o Inter-Chip Interconnect (ICI). Embora as especificações detalhadas do protocolo ICI não sejam públicas, ele é projetado para comunicação síncrona de baixa latência e alta largura de banda, essencial para o treinamento distribuído em larga escala. O ICI permite o acesso direto à memória remota (RDMA) entre os TPUs, facilitando a troca eficiente de dados e gradientes.

A reconfiguração dinâmica da rede óptica é gerenciada pelo Orion, a plataforma de controle de Rede Definida por Software (SDN) do Google. O Orion é um sistema distribuído que propaga a intenção de alto nível (por exemplo, a topologia de rede desejada para uma carga de trabalho específica) através de camadas de aplicativos de controle de rede. Ele interage com os Switches de Circuito Óptico (OCS) para estabelecer os caminhos de luz necessários, efetivamente religando a rede em tempo real para otimizar o desempenho.

Do ponto de vista do desenvolvedor, a interação com os TPUs é abstraída por meio de frameworks de aprendizado de máquina como TensorFlow, PyTorch e JAX. Esses frameworks utilizam o compilador XLA (Accelerated Linear Algebra). O XLA é um compilador de domínio específico para álgebra linear que otimiza o código do modelo de ML para a arquitetura de hardware específica do TPU. Ele realiza transformações como o “tiling” (divisão de grandes operações de matriz em blocos menores) para executar os cálculos de forma eficiente na unidade de matriz sistólica do TPU. O PyTorch/XLA, por exemplo, é um pacote que conecta o PyTorch ao compilador XLA, permitindo que os modelos PyTorch sejam executados em TPUs. A sequência de inicialização de um job em um TPU Pod envolve a compilação do modelo via XLA, a configuração da topologia de rede via Orion e, em seguida, a execução do treinamento distribuído através da rede ICI.

## Fórmulas e Especificações

As especificações técnicas dos TPU Pods do Google demonstram um foco em alta largura de banda, baixa latência e eficiência energética, embora fórmulas matemáticas detalhadas sobre a operação interna não sejam publicamente divulgadas. As especificações de desempenho são geralmente apresentadas em termos de FLOPS (operações de ponto flutuante por segundo) e largura de banda de interconexão.

**Largura de Banda:** A largura de banda da Inter-Chip Interconnect (ICI) é uma métrica chave. As especificações escalam com cada geração de TPU. Por exemplo:

*   **TPUv5p:** Atinge 4.800 Gbps de largura de banda de ICI por chip.
*   **TPUv4:** A largura de banda de bisseção por chip é de 768 GB/s.

**Latência:** A latência é minimizada através da arquitetura de interconexão direta. O uso de Optical Circuit Switching (OCS) evita a sobrecarga de processamento de pacotes dos switches Ethernet tradicionais, resultando em uma latência significativamente menor para a comunicação inter-rack. A latência de ponta a ponta em um Pod é da ordem de microssegundos.

**Consumo de Energia e Capacidade Térmica:** O Google enfatiza a eficiência energética. O TPUv4, por exemplo, mais do que dobrou o pico de FLOPS em comparação com o TPUv3, enquanto reduziu o consumo de energia. Os sistemas são resfriados a líquido para gerenciar a alta densidade de potência. O Google projeta os sistemas para uma capacidade de energia superprovisionada para lidar com picos de carga e manter os Service Level Agreements (SLAs), permitindo que os chips operem em um TDP (Thermal Design Power) muito mais alto por curtos períodos.

Não há fórmulas canônicas publicamente disponíveis que descrevam a relação exata entre esses parâmetros. No entanto, o desempenho geral pode ser modelado conceitualmente. A taxa de transferência efetiva (T_eff) de um modelo distribuído pode

---


## Page 40

ser expressa como uma função da capacidade de computação (C_flops), da largura de banda da rede (B_net) e da latência (L_net):

T_eff = f(C_flops, B_net, L_net, N_chips, Topologia)

Onde N_chips é o número de chips e Topologia representa a configuração da rede (por exemplo, 3D Torus). A reconfigurabilidade do OCS permite que o Google otimize a Topologia para maximizar T_eff para diferentes modelos.

## Vulnerabilidades Identificadas

A infraestrutura dos TPU Pods, apesar de robusta, apresenta potenciais vetores de ataque e vulnerabilidades que residem tanto no hardware quanto no software. Embora o Google implemente medidas de segurança rigorosas, a complexidade do sistema cria superfícies de ataque teóricas.

**Ataques de Canal Lateral (Side-Channel Attacks):** Este é um dos vetores de ataque mais pesquisados. Ataques como o TPUXtract demonstraram a possibilidade de extrair hiperparâmetros de redes neurais medindo variações no consumo de energia ou na radiação eletromagnética do chip durante a computação. Teoricamente, um ator mal-intencionado com acesso físico ou monitoramento próximo poderia inferir informações sobre os modelos em execução. O Google mitiga isso com design de hardware cuidadoso e, em produtos como o Pixel, com o Private AI Compute, que visa isolar as computações sensíveis.

**Vulnerabilidades de Software e Firmware:** A vasta pilha de software, desde o firmware do chip até o compilador XLA e os frameworks de ML, é uma superfície de ataque significativa. Vulnerabilidades no driver do TPU, por exemplo, poderiam ser exploradas. O GKE Sandbox, que usa o gVisor, oferece uma camada de isolamento para proteger contra vulnerabilidades do kernel Linux, mas não mitiga todas as falhas potenciais do driver do TPU. Além disso, a complexidade dos manipuladores de IOCTL da interface PCle apresenta um risco, onde falhas poderiam levar a buffer overflows ou execução de código arbitrário no nível do kernel.

**Segurança da Rede Óptica:** A rede de comutação de circuito óptico (OCS) introduz considerações de segurança únicas. Embora o OCS seja inerentemente mais seguro contra a espionagem de pacotes em comparação com os switches Ethernet (pois simplesmente redireciona a luz), um invasor com acesso físico à fibra óptica poderia, teoricamente, interceptar ou injetar sinais de luz. No entanto, tal ataque exigiria um alto grau de sofisticação e acesso físico. A maior preocupação na rede óptica é a segurança do plano de controle. Se um invasor comprometesse o sistema de controle do OCS (como o Orion), ele poderia reconfigurar a rede para fins maliciosos, como isolar nós ou redirecionar o tráfego.

**Pontos de Acesso Físico:** Os próprios data centers são pontos de acesso óbvios, mas altamente protegidos. Um vetor mais sutil seria através da cadeia de suprimentos, onde componentes de hardware poderiam ser adulterados antes da instalação. O Google aborda isso com iniciativas como a arquitetura de segurança de hardware Titanium, que garante a integridade do hardware desde a fabricação até a implantação.

## Fontes

https://www.servethehome.com/google-details-tpuv4-and-its-crazy-optically-reconfigurable-ai-network/
https://www.fibermall.com/blog/unveiling-google-tpu-architecture.htm?srsltid=AfmBOoqVLu_ayzCvxnRBxrEr50M14hMhpGInYtF6mkWC-fYgYE3qs_LF https://dl.acm.org/doi/abs/10.1145/3579371.3589350
https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm-defined-networking-control-plane/ https://research.google/pubs/orion-googles-software-
https://cloud.google.com/tpu/docs/intro-to-tpu https://www.usenix.org/system/files/nsdi21-ferguson.pdf
http://www.antihackingonline.com/potential-risk-of-cve/do-pcie-ioctI-and-serdes-pose-a-risk-to-google-ironwood-tpu-15th-dec-2025/ https://www.keysight.com/blogs/en/tech/nwvs/2025/02/25/security-highlight-tpuxtract-a-new-side-channel-attack-on-neural-networks/

---

## 22. Design de transceptor SerDes de alta velocidade para TPUs

### Arquitetura e Design

A arquitetura de interconexão dos TPUs do Google é baseada em uma topologia 3D Torus, com a unidade mínima sendo um Cubo 4x4x4 contendo 64 chips de TPU. Cada chip possui seis links de interconexão de alta velocidade (ICI). As conexões internas ao

---


## Page 41

cubo são elétricas, utilizando backplanes de PCB e cabos de cobre. As conexões externas são ópticas e se conectam a um Optical Circuit Switch (OCS), que atua como um comutador de camada física, redirecionando os sinais ópticos através de espelhos MEMS. O design do transceptor SerDes para comunicação chip-a-chip, baseado em um processo CMOS de 65nm, inclui um multiplicador de clock de injeção travada (RILCM) para geração de clock de baixo jitter, um transmissor (TX) com um equalizador de pré-alimentação (FFE) de 4 toques e um multiplexador (MUX) 4:1, e um receptor (RX) com um equalizador linear de tempo contínuo (CTLE) de dois estágios e recuperação de dados de clock (CDR).

## Conexões Físicas

As conexões físicas na arquitetura TPU são híbridas. Internamente, dentro de um cubo de TPU, as conexões são elétricas, utilizando backplanes de PCB e cabos de cobre para a comunicação entre os chips. Para as conexões externas entre os cubos, a arquitetura utiliza links ópticos de alta velocidade. Em configurações mais otimizadas para custo como no TPuv5e/v6e, são utilizados cabos de cobre DAC (Direct Attach Copper) com conectores QSFP-DD para as conexões horizontais entre os racks.

## Protocolos e Software

A pesquisa não revelou protocolos de comunicação específicos em detalhes. No entanto, foi mencionado que a orquestração da rede é gerenciada pelo Orion SDN (Software-Defined Network) em conjunto com o compilador XLA (Accelerated Linear Algebra) para otimizar o roteamento e o posicionamento dos dados na malha de TPUs.

## Fórmulas e Especificações

Taxa de dados: 40 Gb/s Processo: 65 nm CMOS Taxa de erro de bit (BER): <10-12 Perda do canal: >16 dB Consumo de energia: 370 mW Contagem de chips TPuv4: 4096 Contagem de chips TPuv7: 9216 Velocidade do link TPuv7: 800G/1.6T Velocidade do SerDes do TPuv8: 224Gbps+

## Vulnerabilidades Identificadas

A pesquisa realizada não identificou vulnerabilidades ou pontos de acesso específicos conhecidos publicamente para a arquitetura SerDes dos TPUs do Google.

## Fontes

https://www.fibermall.com/blog/unveiling-google-tpu-architecture.htm
https://repository.lincoln.ac.uk/articles/thesis/Design_of_High-Speed_SerDes_Transceiver_for_Chip-to-Chip_Communications_in_CMOS_Process/24326020

---

## 23. Pesquisa técnica detalhada sobre a sinalização elétrica de Unidades de Processamento Tensorial (TPUs), com foco específico nas tecnologias de codificação Non-Return-to-Zero (NRZ) e Pulse Amplitude Modulation 4-level (PAM4) utilizadas nas interconexões de alta velocidade que formam os TPU Pods.

## Arquitetura e Design

A arquitetura dos TPUs do Google evoluiu para um sistema de supercomputador massivamente paralelo, onde a interconexão entre os chips é um componente crítico para o desempenho. A arquitetura geral pode ser descrita em uma hierarquia:

*   **Chip TPU:** A unidade fundamental, contendo múltiplos TensorCores. Por exemplo, o TPU v4 possui dois TensorCores. Cada TensorCore é uma unidade de processamento completa com:
    *   **MXU (Matrix Multiply Unit):** O coração do TPU, uma matriz sistólica de multiplicadores-acumuladores (e.g., 128x128 ou 256x256) otimizada para operações de matriz em massa, fundamentais para redes neurais. As multiplicações são feitas em bfloat16 e as acumulações em FP32.
    *   **Unidade Vetorial e Escalar:** Para operações de propósito geral, funções de ativação, controle de fluxo e cálculo de endereços.

---


## Page 42

*   **Placa TPU (Board):** Múltiplos chips TPU (e.g., 4 no TPU v4) são montados em uma única placa de circuito impresso (PCB), que também abriga a memória de alta largura de banda (HBM) e os conectores para interconexão.
*   **Cubo (Cube) / Bloco (Block):** Um conjunto de placas TPU é montado em um rack para formar um bloco de construção 3D. No caso do TPU v4, 64 chips formam um “cubo” 4x4x4, que é a unidade básica de interconexão elétrica direta.
*   **TPU Pod:** O supercomputador completo, formado pela interconexão de múltiplos cubos. Um pod de TPU v4 completo contém 4096 chips, interligados por uma combinação de conexões elétricas e ópticas.

O fluxo de dados e comandos é gerenciado por um host (CPU), que envia as instruções e os dados para os TPUs. Os dados são carregados na memória HBM e processados pelos TensorCores. Os resultados são então lidos de volta pelo host. A comunicação entre os chips para operações paralelas, como all-reduce, ocorre através da rede de interconexão de alta velocidade (ICI).

## Conexões Físicas

As conexões físicas são cruciais para a baixa latência e alta largura de banda dos TPU Pods.

*   **Dentro do Rack (Intra-Rack):**
    *   **Fiação:** Dentro de um rack, as conexões que formam o cubo 4x4x4 são realizadas com cabos elétricos passivos de cobre. Esses links, chamados de Inter-Core Interconnect (ICI), conectam as placas TPU para formar uma malha 3D (toro).
    *   **Conectores:** As placas TPU v4 utilizam conectores OSFP (Octal Small Form Factor Pluggable) na parte inferior para os links ICI externos que se conectam a outras placas no mesmo rack.
*   **Entre Racks (Inter-Rack):**
    *   **Fiação:** Para conectar os diferentes cubos/racks que compõem um pod, a distância excede o alcance dos sinais elétricos. Portanto, a interconexão é feita com fibras ópticas.
    *   **Conversão Eletro-Óptica:** A conversão do sinal elétrico (proveniente dos chips TPU) para o sinal óptico ocorre diretamente nos conectores das placas TPU. Não há switches elétricos intermediários na rede de dados principal.
    *   **Optical Circuit Switch (OCS):** O ponto central da interconexão inter-rack do TPU v4 é o OCS, um switch óptico baseado em MEMS (Micro-Electro-Mechanical Systems). O OCS, chamado Palomar, dirige os feixes de luz entre as fibras, reconfigurando dinamicamente a topologia da rede. Isso permite criar “fatias” (slices) de TPUs de tamanhos variados, contornar falhas de hardware e otimizar a topologia para diferentes cargas de trabalho (e.g., toro regular vs. toro torcido).

## Protocolos e Software

*   **Protocolo de Sinalização Elétrica:**
    *   **NRZ (PAM2):** Utilizado em gerações mais antigas e para links de menor velocidade. Codifica 1 bit por símbolo usando dois níveis de tensão (0 ou 1). É mais robusto a ruído (maior SNR), mas oferece menor largura de banda para a mesma taxa de símbolos (baud rate).
    *   **PAM4:** Adotado para as interconexões de alta velocidade (e.g., 112 Gbps por lane no TPU v4). Codifica 2 bits por símbolo usando quatro níveis de tensão (00, 01, 10, 11). Isso dobra a largura de banda efetiva em comparação com o NRZ para a mesma taxa de símbolos, mas ao custo de uma penalidade significativa na relação sinal-ruído (SNR), tornando o design do canal e a equalização do sinal muito mais complexos.
*   **APIs e Bibliotecas:** O acesso e a programação dos TPUs são abstraídos por frameworks de machine learning como JAX, PyTorch (via XLA), e TensorFlow. Os desenvolvedores não interagem diretamente com os protocolos de baixo nível, mas definem seus modelos e o compilador XLA (Accelerated Linear Algebra) otimiza e distribui a computação pelos TPUs, gerenciando a comunicação através da rede ICI.

## Fórmulas e Especificações

*   **Largura de Banda por Lane:**
    *   Largura de Banda (bps) = Taxa de Símbolos (Baud) * bits_por_simbolo
    *   Para NRZ: bits_por_simbolo = log2(2) = 1
    *   Para PAM4: bits_por_simbolo = log2(4) = 2

---


## Page 43

* Exemplo: Para uma taxa de 56 Gbaud, uma lane NRZ transmite 56 Gbps, enquanto uma lane PAM4 transmite 112 Gbps.

* **Relação Sinal-Ruído (SNR) e Níveis de Amplitude:**
    * A penalidade de SNR do PAM4 em relação ao NRZ pode ser calculada. Como o PAM4 tem 3 “olhos” em seu diagrama de olho em comparação com 1 do NRZ, a altura de cada olho é aproximadamente 1/3 da altura total do sinal. A potência do sinal é proporcional ao quadrado da amplitude.
    * Penalidade de Potência (dB) = 10 * log10( (A_pam4 / A_nrz)^2 )
    * Considerando A_pam4 ≈ A_nrz / 3, a penalidade é 20 * log10(1/3) ≈ -9.54 dB.

* **Taxa de Erro de Bit (BER - Bit Error Rate):**
    * O BER é uma função da relação sinal-ruído e é tipicamente expresso em termos da função de erro complementar (erfc).
    * Para NRZ (com ruído Gaussiano): BER_nrz ≈ 0.5 * erfc(sqrt(SNR) / sqrt(2))
    * Para PAM4 (com ruído Gaussiano, mapeamento Gray): BER_pam4 ≈ 0.75 * erfc(sqrt(SNR_por_nivel) / sqrt(2))
    * Onde SNR_por_nivel é o SNR para cada um dos três olhos do PAM4. A menor distância entre os níveis de sinal no PAM4 o torna inerentemente mais suscetível a erros para o mesmo nível de ruído.

## Vulnerabilidades Identificadas

A pesquisa inicial não revelou vulnerabilidades de segurança cibernética publicamente documentadas e específicas para a sinalização elétrica PAM4/NRZ em TPUs. No entanto, podemos inferir vetores de ataque teóricos baseados na camada física:

*   **Ataques de Canal Lateral (Side-Channel Attacks):** A análise da radiação eletromagnética (EM) emitida pelos cabos de alta velocidade poderia, teoricamente, vazar informações sobre os dados que estão sendo processados. A complexidade da sinalização PAM4 (com seus quatro níveis de tensão) pode modular a emissão EM de maneiras que poderiam ser exploradas para inferir padrões de dados. No entanto, isso exigiria acesso físico extremamente próximo e equipamento altamente especializado.
*   **Injeção de Falhas (Fault Injection):** A introdução deliberada de ruído eletromagnético (EMI) no ambiente dos cabos de interconexão poderia aumentar a taxa de erro de bit (BER). Como o PAM4 tem uma margem de ruído muito menor que o NRZ, ele é mais suscetível a esse tipo de ataque. Uma injeção de falha bem-sucedida poderia corromper os gradientes durante o treinamento de um modelo de ML, levando a um modelo treinado incorretamente ou a resultados de inferência errados. O OCS, ao reconfigurar a rede para contornar links defeituosos, oferece uma certa resiliência, mas um ataque generalizado poderia degradar o desempenho do pod.
*   **Acesso Físico:** O ponto de acesso mais direto seria a interceptação física dos cabos de cobre ou fibra óptica. Embora os data centers do Google tenham segurança física robusta, qualquer ponto de acesso físico aos racks ou ao OCS representa uma vulnerabilidade potencial para espionagem ou interrupção do serviço.

## Fontes

https://blog.samtec.com/post/understanding-nrz-and-pam4-signaling/ https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm https://arxiv.org/pdf/2304.01433

---

## 24. Análise técnica detalhada do design da Unidade de Distribuição de Energia (PDU) para Tensor Processing Units (TPUs), com foco na arquitetura de entrega de energia, desde a escala do data center até o nível do chip.

## Arquitetura e Design

A arquitetura de distribuição de energia para TPUs é um sistema hierárquico de múltiplos estágios. Em nível de rack, as PDUs recebem energia de alta tensão do data center e a distribuem para os servidores. Dentro de cada servidor, um subsistema de conversão de energia reduz a tensão para os níveis exigidos pelos componentes. A arquitetura de energia do TPU do Google emprega uma abordagem de Entrega de Energia Vertical (VPD), onde os Módulos Reguladores de Tensão (VRMs) são posicionados

---


## Page 44

na parte inferior da placa de circuito impresso (PCB), diretamente sob o processador TPU. Esta configuração contrasta com a entrega de energia lateral (LPD) e oferece vantagens significativas na redução das perdas da rede de distribuição de energia (PDN) e na otimização do espaço na PCB. Os principais componentes incluem VRMs, que convertem a tensão de entrada (tipicamente 48V) para a baixa tensão do núcleo do TPU (sub-1V), e multiplicadores de corrente, como os Módulos Transformadores de Tensão (VTMs) ou Multiplicadores de Corrente Engrenados (GCMs), capazes de fornecer correntes superiores a 1000A com alta eficiência.

## Conexões Físicas

O design físico da PDU de TPU é otimizado para integridade de sinal e eficiência de energia. Com a Entrega de Energia Vertical (VPD), a parte superior da PCB fica liberada para o roteamento de sinais de alta velocidade, minimizando a interferência eletromagnética entre as trilhas de energia e de dados. A energia é fornecida ao encapsulamento do TPU através de uma matriz de grade de esferas (BGA). Módulos de energia especializados, como os GCMs, são projetados com um mapa de pinos que corresponde diretamente ao BGA do TPU, simplificando o layout da PCB e reduzindo a complexidade do roteamento. Internamente, cabos de alta corrente conectam a fonte de alimentação principal do servidor aos conversores de energia na placa-mãe, e a PDN na PCB é projetada com trilhas largas e planos de cobre para minimizar a resistência ôhmica.

## Protocolos e Software

O gerenciamento e controle do sistema de energia do TPU dependem de protocolos de comunicação específicos. Em nível de rack, é provável que sejam utilizados protocolos padrão da indústria, como o SNMP (Simple Network Management Protocol), ou interfaces de gerenciamento personalizadas para monitorar o consumo de energia, a temperatura e o status operacional das PDUs. A comunicação entre o chip do TPU e seus VRMs é crítica para o gerenciamento dinâmico de energia e provavelmente utiliza um protocolo como o PMBus (Power Management Bus) ou uma interface serial proprietária. Isso permite que o TPU ajuste dinamicamente sua tensão de operação (escala de tensão dinâmica) em resposta à carga de trabalho, otimizando o consumo de energia, e que o sistema de energia reporte seu status para garantir a estabilidade e a confiabilidade.

## Fórmulas e Especificações

P_loss = I^2 * R_pdn Eficiência (%) = (P_out / P_in) * 100 V_drop = I * R_pdn

## Vulnerabilidades Identificadas

As vulnerabilidades no sistema de distribuição de energia do TPU podem ser exploradas em diferentes níveis. O acesso físico não autorizado às PDUs do rack ou aos servidores representa um risco direto, permitindo a interrupção da energia ou a manipulação do hardware. Ataques de canal lateral, como a análise de flutuações de energia (Power Analysis Attack), podem teoricamente vazar informações sobre as computações sendo executadas no TPU, comprometendo a confidencialidade dos dados. Além disso, falhas de componentes de hardware no sistema de energia, como VRMs ou capacitores, podem causar instabilidade no sistema, corrupção de dados ou falha total de um nó de TPU, impactando a disponibilidade e a integridade do cluster.

## Fontes

https://newsletter.semianalysis.com/p/tpuv7-google-takes-a-swing-at-the-delivery-solutions https://www.vicorpower.com/resource-library/articles/high-performance-computing/vertical-power-delivery-enables-cutting-edge-processing https://flexpowermodules.com/vertical-power-delivery-solutions

---

## 25. Sistema de refrigeração líquida e dissipadores de calor dos TPUs do Google

### Arquitetura e Design

O sistema de refrigeração líquida para os Tensor Processing Units (TPUs) do Google é uma solução de engenharia complexa, projetada para operar em escala de datacenter. Diferente de sistemas de refrigeração contidos em servidores individuais, os circuitos de refrigeração líquida do Google se estendem por racks inteiros. A arquitetura centraliza-se em torno de **Coolant Distribution Units (CDUs)**, que são agrupadas em racks de seis unidades. Estas CDUs operam de forma análoga a um conjunto de radiador e bomba de um sistema de refrigeração líquida de consumo, com a capacidade de manter a refrigeração adequada mesmo com uma unidade em manutenção, garantindo alta disponibilidade.

---


## Page 45

A troca de calor é realizada de forma indireta: as CDUs transferem o calor do líquido de arrefecimento para o circuito de água da instalação, sem que haja mistura entre os dois fluidos. O líquido de arrefecimento resfriado é então distribuído para os servidores TPU através de **manifolds**. Internamente, os chips TPU são conectados em um **circuito em série**, o que implica que a capacidade de refrigeração é dimensionada para o último e mais quente chip do circuito. Para a interface direta com o chip, o Google utiliza uma **placa fria de fluxo dividido (split-flow)**, que oferece um desempenho superior em comparação com designs de fluxo direto. Adicionalmente, para otimizar a transferência térmica, a partir da versão **TPUv4**, foi adotada uma configuração de **matriz nua (bare-die)**, eliminando o encapsulamento do processador para permitir um contato mais direto e eficiente com a placa fria.

## Conexões Físicas

As conexões físicas do sistema de refrigeração são projetadas para robustez e facilidade de manutenção em um ambiente de produção de larga escala. As CDUs são conectadas ao sistema através de **mangueiras flexíveis e acoplamentos de desconexão rápida**. Esta escolha de design é crítica para permitir a manutenção e substituição de CDUs sem a necessidade de desligar o sistema, minimizando o tempo de inatividade. O líquido de arrefecimento é transportado das CDUs para os servidores através de **manifolds** que garantem a distribuição uniforme. Os chips TPU são interligados em série para o fluxo do líquido. Embora detalhes sobre pinagem e conectores específicos não sejam públicos, a infraestrutura depende de tubulação e conectores de alta confiabilidade, provavelmente fabricados em **poliuretano termoplástico (TPU)**, conhecido por sua durabilidade e resistência química.

## Protocolos e Software

O gerenciamento do sistema de refrigeração é uma operação de software sofisticada e largamente automatizada. O Google emprega uma estratégia de **Controle Preditivo por Modelo (Model Predictive Control - MPC)**, onde um agente de **Aprendizado por Reforço (Reinforcement Learning - RL)** é treinado para regular dinamicamente as temperaturas e o fluxo de ar em todo o data center. Este sistema otimiza a eficiência energética ao mesmo tempo que garante que os componentes operem dentro de suas faixas de temperatura seguras. O software de gerenciamento térmico é proprietário e monitora continuamente os TPUs para prevenir o afogamento térmico (throttling). Não existem APIs ou bibliotecas de software públicas para interagir com o sistema de refrigeração; o controle é inteiramente interno. A sequência de inicialização e os comandos de controle são igualmente proprietários e não documentados, refletindo a natureza autônoma e de missão crítica do sistema.

## Fórmulas e Especificações

As especificações térmicas e de consumo de energia são cruciais para o design do sistema de refrigeração. Para o **TPUv4**, a potência média de operação de um chip é de aproximadamente **200W**, com uma potência medida que varia entre **90W (mínimo)**, **170W (média)** e **192W (máximo)**, e um Thermal Design Power (TDP) especificado de **250W**. Em comparação, o **TPUv3** possuía um TDP de **450W**.

A eficiência do sistema é notável. O consumo de energia das bombas de refrigeração líquida representa menos de **5%** da energia que seria consumida por ventiladores em uma solução de refrigeração a ar equivalente. A escolha do líquido, água, é justificada por sua alta condutividade térmica, que é aproximadamente **4000 vezes maior que a do ar**.

A fórmula fundamental para a transferência de calor (Q) no sistema é: Q = m * c * ΔT Onde:

*   m : massa do líquido de arrefecimento
*   c : calor específico do líquido de arrefecimento
*   ΔT : variação de temperatura do líquido de arrefecimento

A eficiência geral do datacenter, incluindo a refrigeração, é medida pelo Power Usage Effectiveness (PUE). Os data centers do Google alcançam um PUE de **1.1**, indicando uma sobrecarga de energia de apenas **10%** para além do consumo dos próprios chips.

## Vulnerabilidades Identificadas

Apesar de sua eficiência, o sistema de refrigeração líquida apresenta um conjunto de vulnerabilidades. As **vulnerabilidades físicas** incluem o risco inerente de **vazamentos**, que podem causar danos catastróficos aos componentes eletrônicos. O Google mitiga este risco através de testes rigorosos, sistemas de alerta precoce e o uso de acoplamentos de desconexão rápida. Outro risco é o **crescimento microbiano** no fluido de arrefecimento, que pode levar a obstruções e redução da eficiência; isso é

---


## Page 46

combatido com filtração e manutenção preventiva. Além disso, os materiais de poliuretano termoplástico (TPU) usados na tubulação podem ser suscetíveis à **hidrólise** em ambientes de alta umidade e temperatura.

Do ponto de vista da **segurança cibernética**, o próprio sistema de refrigeração pode se tornar um **vetor de ataque**. Um ataque bem-sucedido que comprometa o sistema de controle de refrigeração pode levar ao superaquecimento e à destruição física dos TPUs. Pesquisas demonstraram a viabilidade de **ataques térmicos** direcionados a servidores, racks ou mesmo a todo o data center, explorando o gerenciamento térmico como um ponto de falha.

## Fontes

https://chipsandcheese.com/p/googles-liquid-cooling-at-hot-chips https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codesigned-ai-stack
https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
https://cloud.google.com/blog/topics/systems/enabling-1-mw-it-racks-and-liquid-cooling-at-ocp-emea-summit
https://research.google/pubs/data-center-cooling-using-model-predictive-control/ https://www.techmonitor.ai/comment-2/data-centre-cooling-systems-cyberattack
https://www.ndss-symposium.org/wp-content/uploads/2018/02/ndss2018_06A-1_Gao_paper.pdf https://ieeexplore.ieee.org/abstract/document/9715098/

---

## 26. A topologia de rede 3D torus mesh utilizada nos Google TPU Pods, com foco nas suas características de arquitetura, conexões físicas, protocolos, especificações e vulnerabilidades.

### Arquitetura e Design

A arquitetura de interconexão dos TPU Pods do Google é uma implementação sofisticada de uma topologia de malha de toro tridimensional (3D torus). A unidade fundamental desta arquitetura é o “Cubo”, uma estrutura 4x4x4 que agrega 64 chips de TPU. Cada chip de TPU é equipado com seis links de Interconexão entre Chips (ICI) de alta velocidade, que se estendem nas direções positiva e negativa dos eixos X, Y e Z, formando a base da grade 3D Torus. Os TPU Pods são escalados a partir dessas unidades de Cubo; por exemplo, um pod TPUv4 consiste em 64 Cubos, totalizando 4096 chips, enquanto um pod TPUv7 mais recente se expande para 144 Cubos, totalizando 9216 chips.

Uma característica notável da arquitetura mais recente do TPUv7 é a adoção de uma topologia “Twisted 3D Torus”. Esta variação introduz um tamanho de passo variável nas conexões, o que efetivamente reduz o número de saltos necessários para a comunicação entre chips distantes, otimizando a latência em grande escala. Em contraste, as versões mais econômicas, como o TPUv5e e o TPUv6e, utilizam uma malha 2D Torus mais simples, que elimina a necessidade dos caros Switches de Circuito Óptico (OCS), mas com uma capacidade de escalonamento menor.

O TPU v4, a quinta geração de arquitetura de domínio específico (DSA) do Google, representa um avanço significativo, sendo o terceiro supercomputador da empresa para modelos de aprendizado de máquina. Ele introduz os OCSes para reconfigurar dinamicamente a topologia de interconexão, permitindo a seleção de uma topologia de “toro 3D torcido” para otimizar o desempenho. Cada chip TPU v4 integra dois TensorCores (TCs), cada um contendo quatro Unidades de Multiplicação de Matriz (MXUs) 128x128 e uma Unidade de Processamento Vetorial (VPU). Esses TCs compartilham uma Memória Comum (CMEM) de 128 MiB. Além disso, o TPU v4 inclui SparseCores, que são processadores de fluxo de dados projetados para acelerar modelos que dependem de embeddings, consumindo apenas 5% da área do die e da energia.

### Conexões Físicas

As conexões físicas dentro da arquitetura do TPU Pod são um híbrido de tecnologias elétricas e ópticas. Dentro de um único Cubo, as interconexões são primariamente elétricas, utilizando backplanes de Placa de Circuito Impresso (PCB) e cabos de cobre para links de curta distância. As conexões externas, no entanto, que ligam os Cubos entre si, são ópticas. Cada Cubo possui 96 links ópticos em suas faces externas, que se conectam a Switches de Circuito Óptico (OCS) para roteamento dinâmico e escalonamento massivo.

O OCS utilizado é o Palomar, que possui um caminho óptico interno em forma de “W” para minimizar a perda de inserção e é baseado em espelhos de Sistemas Micro-Eletro-Mecânicos (MEMS) 3D. O Palomar OCS tem uma configuração de 136x136 portas, com 128 portas efetivas e 8 portas sobressalentes para teste e reparo. Em um sistema TPUv4, 48 unidades OCS são usadas para

---


## Page 47

conectar os 48 pares de cabos dos 64 Cubos. O uso de circuladores nos OCSes permite a comunicação bidirecional em uma única fibra, o que efetivamente reduz pela metade o número de portas e cabos necessários.

As conversões de sinal elétrico para óptico ocorrem nos conectores de fibra nas bandejas do TPU. As PCBs dos TPUs incorporam quatro links de Interconexão Inter-Core (ICI) em uma malha 2x2, com 16 links ICI externos se estendendo para outras bandejas. Para as variantes mais econômicas, como o TPUv5e e o TPUv6e, as conexões horizontais são feitas com cabos de cobre QSFP-DD DAC (Direct Attach Copper).

## Protocolos e Software

A eficiência da topologia 3D Torus é fortemente dependente da pilha de software que a gerencia. O Orion, uma Rede Definida por Software (SDN), trabalha em sinergia com o compilador XLA (Accelerated Linear Algebra) para otimizar o posicionamento e o roteamento de dados na vasta rede de interconexão. A pilha de software inclui suporte para frameworks de aprendizado de máquina populares como PyTorch (via XLA) e TensorFlow (também via JAX).

Os padrões de comunicação variam dependendo da estratégia de paralelismo empregada. O paralelismo de modelo, onde diferentes partes de um modelo de rede neural são executadas em diferentes TPUs, normalmente utiliza um padrão de comunicação “all-to-all”. O paralelismo de dados, onde o mesmo modelo é replicado em vários TPUs e cada um processa uma parte diferente dos dados, emprega um padrão de comunicação “all-reduce”.

## Fórmulas e Especificações

<table>
  <thead>
    <tr>
      <th>Especificação</th>
      <th>Valor</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Arquitetura do Cubo</td>
      <td>4x4x4 (64 chips de TPU)</td>
    </tr>
    <tr>
      <td>Escala do Pod (TPUv4)</td>
      <td>4096 chips (64 Cubos)</td>
    </tr>
    <tr>
      <td>Escala do Pod (TPUv7)</td>
      <td>9216 chips (144 Cubos)</td>
    </tr>
    <tr>
      <td>Links Ópticos por Cubo</td>
      <td>96</td>
    </tr>
    <tr>
      <td>Total de Links Ópticos (TPUv4)</td>
      <td>6144</td>
    </tr>
    <tr>
      <td>Portas Efetivas do OCS Palomar</td>
      <td>128</td>
    </tr>
    <tr>
      <td>Unidades OCS (TPUv4)</td>
      <td>48</td>
    </tr>
    <tr>
      <td>Largura de Banda do Link ICI</td>
      <td>400 Gbit/s</td>
    </tr>
    <tr>
      <td>Largura de Banda do Link (TPUv7)</td>
      <td>800G/1.6T</td>
    </tr>
    <tr>
      <td>TDP de Chip Único (TPUv7)</td>
      <td>600W</td>
    </tr>
    <tr>
      <td>Melhoria de Desempenho (TPUv4 vs TPUv3)</td>
      <td>2.1x</td>
    </tr>
    <tr>
      <td>Melhoria de Desempenho/Watt (TPUv4 vs TPUv3)</td>
      <td>2.7x</td>
    </tr>
    <tr>
      <td>Comparação de Desempenho (TPUv4 vs A100)</td>
      <td>1.2x-1.7x mais rápido</td>
    </tr>
    <tr>
      <td>Comparação de Energia (TPUv4 vs A100)</td>
      <td>1.3x-1.9x menos consumo</td>
    </tr>
    <tr>
      <td>Custo do OCS</td>
      <td>% do custo do sistema</td>
    </tr>
    <tr>
      <td>Energia do OCS</td>
      <td>% da energia do sistema</td>
    </tr>
    <tr>
      <td>Consumo de Energia (TPUv4 - BERT)</td>
      <td>197W</td>
    </tr>
    <tr>
      <td>Consumo de Energia (TPUv4 - ResNet)</td>
      <td>206W</td>
    </tr>
  </tbody>
</table>

## Vulnerabilidades Identificadas

A pesquisa não revelou informações específicas sobre vulnerabilidades de segurança, pontos de acesso potenciais, falhas conhecidas ou vetores de ataque relacionados à topologia de rede 3D torus mesh dos TPU Pods. A documentação foca primariamente em arquitetura, desempenho e eficiência.

---


## Page 48

# Fontes

https://www.fibermall.com/blog/unveiling-google-tpu-architecture.htm?srstid=AfmBOorRvlEGho7zKx67tIN54_UH7MMSCiocCuyc_YDEY2OjhB7Z2mPO https://arxiv.org/pdf/2304.01433

---

# 27. Pesquisa técnica detalhada sobre a comunicação coletiva AllReduce em anel de TPUs (Tensor Processing Units).

---

## Arquitetura e Design

O algoritmo Ring AllReduce é uma abordagem para a comunicação coletiva em sistemas distribuídos, como os TPU Pods. Ele é executado em duas fases principais:

1. **Share-Reduce**: Nesta fase, cada um dos N processadores (TPUs) envia uma fração (1/N) de seus dados para o seu vizinho no anel. Ao mesmo tempo, ele recebe uma fração de dados do seu outro vizinho. Ao final de N-1 passos, cada processador terá uma fração do resultado final da redução.
2. **Share-Only**: Nesta segunda fase, os resultados parciais são circulados pelo anel para que todos os processadores tenham o resultado completo. Esta fase também leva N-1 passos.

A topologia de interconexão dos TPUs mais recentes, como o TPUv4, é um toro 3D. Dentro de um ‘cubo’ de 4x4x4 TPUs, as conexões são feitas com links de cobre. Entre os cubos, a interconexão é feita através de switches ópticos de circuito (OCS), que permitem reconfigurar a topologia da rede dinamicamente. Essa reconfigurabilidade pode ser explorada para otimizar algoritmos de comunicação coletiva como o AllReduce, potencialmente “curto-circuitando” o anel para reduzir a latência.

Os componentes chave da arquitetura de um TPU que suportam essas operações são:

*   **TensorCore**: O coração do TPU, contendo uma ou mais Unidades de Multiplicação de Matrizes (MXUs), uma unidade vetorial e uma unidade escalar.
*   **MXU (Matrix-Multiply Unit)**: Uma grade sistólica de multiplicadores-acumuladores (256x256 em TPUs mais recentes) que executa as operações de multiplicação de matrizes em alta velocidade.
*   **ICI (Inter-Chip Interconnect)**: Links de alta velocidade que conectam os chips de TPU dentro de um mesmo slice. A resiliência do ICI é aprimorada para contornar falhas nos links ópticos e switches de circuito óptico (OCS) que conectam os TPUs entre os cubos.
*   **OCS (Optical Circuit Switch)**: Permite a reconfiguração dinâmica da topologia de interconexão entre os cubos de TPU.

## Conexões Físicas

A interconexão dos TPUs é hierárquica e depende da versão do TPU. No TPUv4, a topologia é um toro 3D. As conexões físicas podem ser descritas da seguinte forma:

*   **Dentro de um Cubo (4x4x4 TPUs)**: As conexões entre os chips de TPU são feitas através de **links de cobre**. Esses links formam a interconexão inter-chip (ICI) dentro do cubo.
*   **Entre Cubos**: A comunicação entre diferentes cubos de TPU é realizada através de uma rede óptica. **Switches de Circuito Óptico (OCS)** são usados para reconfigurar dinamicamente as conexões entre os cubos, permitindo a criação de topologias otimizadas para diferentes cargas de trabalho. A resiliência do ICI é projetada para contornar falhas nesses links ópticos e nos OCS.
*   **Fiação e Conectores**: Embora os documentos não detalhem a pinagem exata ou os conectores específicos, a arquitetura depende de uma combinação de traços de PCB para as conexões de cobre dentro do cubo e cabos de fibra óptica para as conexões ópticas entre os cubos.

## Protocolos e Software

O algoritmo Ring AllReduce é um protocolo de comunicação coletiva implementado em software que orquestra a troca de dados entre os TPUs em um anel. A comunicação é dividida em duas fases principais:

---


## Page 49

1. **Share-Reduce**: Cada TPU envia uma porção de seus dados para seu vizinho no anel enquanto recebe outra porção de seu outro vizinho. Após N-1 passos (onde N é o número de TPUs), cada TPU possui uma parte do resultado da redução.
2. **Share-Only**: Os resultados parciais são então circulados pelo anel para garantir que todos os TPUs tenham o resultado final completo. Esta fase também leva N-1 passos.

As APIs e bibliotecas de software que permitem o uso dessas funcionalidades incluem:

*   **Frameworks de Machine Learning**: Frameworks como JAX e PyTorch fornecem abstrações de alto nível para a execução de operações de comunicação coletiva, incluindo o AllReduce, em clusters de TPU.
*   **Acesso à VM do TPU**: A arquitetura de VM do TPU permite o acesso direto à máquina virtual que está fisicamente conectada ao dispositivo TPU através de SSH. Isso permite um controle mais granular sobre o ambiente de execução.
*   **Filas de Entrada/Saída (Infeed/Outfeed)**: O host do TPU transmite dados para uma fila de entrada (infeed queue), da qual o TPU carrega os dados para sua memória HBM. Após a conclusão do cálculo, os resultados são colocados em uma fila de saída (outfeed queue), de onde o host os lê.

## Fórmulas e Especificações

O desempenho do algoritmo Ring AllReduce e da interconexão de TPUs pode ser caracterizado por várias métricas e modelos. Embora especificações detalhadas de pinagem e consumo de energia por componente não sejam publicamente divulgadas, os seguintes dados e modelos são relevantes:

### Modelo de Custo de Comunicação

O modelo de custo de Hockney (α-β) é frequentemente usado para analisar o desempenho de algoritmos de comunicação coletiva. A latência de uma comunicação é modelada como:

T(m) = α + mβ

Onde:

*   T(m): Tempo total para enviar uma mensagem de tamanho m.
*   α (alfa): Latência inicial, representando o tempo de inicialização para o primeiro bit chegar ao destino. Em redes físicas, isso é largamente influenciado pelo atraso de propagação.
*   β (beta): Tempo de transmissão por bit, o inverso da largura de banda.

Para o algoritmo Ring AllReduce com N processadores e uma mensagem de tamanho M, o tempo de conclusão pode ser aproximado por:

T_ring ≈ 2 * (N-1) * (α + M/N * β)

### Especificações de Desempenho (Exemplos de TPU v4 e Ironwood)

*   **Largura de Banda da Interconexão (ICI)**:
    *   **Ironwood**: A rede ICI atinge 9.6 Tb/s por chip.
    *   **TPU v4**: Possui seis links de interconexão por chip. A topologia em toro 3D dobra a largura de banda de bisseção em comparação com topologias mais simples.
*   **Largura de Banda PCIe**:
    *   **TPU v4**: A largura de banda do barramento PCIe é de 16 GB/s em cada direção, usada para comunicação com a CPU host.
*   **Eficiência Energética**:
    *   **TPU v4 vs TPU v3**: O TPU v4 melhora a relação desempenho/Watt em 2.7x.
    *   **TPUs vs GPUs**: Em geral, TPUs demonstram uma eficiência energética 2 a 3 vezes maior (desempenho por watt) em comparação com GPUs para cargas de trabalho de inferência.

---


## Page 50

# Vulnerabilidades Identificadas

A pesquisa sobre vulnerabilidades em TPUs e suas interconexões revela alguns vetores de ataque e falhas potenciais:

## Ataques de Canal Lateral (Side-Channel Attacks)

*   **TPUxtract**: Este é um ataque de canal lateral que explora as emissões eletromagnéticas (EM) do chip TPU para extrair informações sobre o modelo de rede neural em execução. Ao posicionar uma sonda EM sobre o chip, um invasor pode medir as variações no campo eletromagnético durante os cálculos e, com isso, inferir os hiperparâmetros e até mesmo a arquitetura do modelo com alta precisão. Este tipo de ataque não requer conhecimento prévio do modelo e pode ser usado para roubar propriedade intelectual.
*   **Ataques baseados em consumo de energia**: Semelhante aos ataques baseados em EM, as variações no consumo de energia do TPU durante a execução de um modelo também podem vazar informações sobre as operações que estão sendo realizadas, abrindo a porta para a engenharia reversa do modelo.

## Vulnerabilidades de Software

*   **Divulgação de informações por dados não inicializados**: Uma vulnerabilidade específica (CVE-2024-32914) foi identificada na função `tpu_get_int_state` do arquivo `tpu.c`, onde dados não inicializados poderiam ser expostos, levando a uma possível divulgação de informações.

## Pontos de Acesso Potenciais

*   **Interconexões Físicas**: Embora as interconexões de alta velocidade (ICI e OCS) sejam projetadas para desempenho, elas também representam um ponto de acesso físico. O acesso físico a essas conexões, especialmente os cabos de fibra óptica entre os cubos, poderia teoricamente permitir a interceptação ou manipulação de dados, embora isso exija um alto nível de sofisticação.
*   **Acesso à VM do Host**: Como os TPUs são controlados por VMs de host, comprometer a VM do host daria a um invasor controle sobre o TPU e os dados que estão sendo processados. A segurança da VM do host é, portanto, crucial para a segurança do sistema como um todo.

## Fontes

https://arxiv.org/html/2510.03491v1 https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm

---

# 28. Infraestrutura de implantação de TPU do Google Data Center

## Arquitetura e Design

A infraestrutura de implantação de TPU do Google é baseada em uma arquitetura co-projetada que abrange desde o silício até o software. A unidade fundamental é o **chip TPU**, que contém um ou mais **TensorCores**. Cada TensorCore é composto por **Unidades de Multiplicação de Matriz (MXUs)**, uma **unidade vetorial** e uma **unidade escalar**. As MXUs, com arranjos de 256x256 em versões mais recentes como a v6e, fornecem a maior parte do poder computacional. A arquitetura escala através de **TPU Pods**, que são conjuntos de TPUs conectados por uma rede especializada. Dentro de um Pod, os chips são agrupados em **Slices** e **Cubos de TPU** (topologia 4x4x4 a partir da v4). A tecnologia **Multislice** permite a comunicação entre slices, utilizando a rede do data center (DCN) para escalar para além de um único Pod. **SparseCores** são processadores especializados para acelerar modelos com operações esparsas, como os de recomendação.

## Conexões Físicas

As conexões físicas nos TPU Pods são hierárquicas. A unidade base é o **Cubo 4x4x4**, onde 64 chips de TPU são interconectados. Dentro do cubo, as conexões internas utilizam **backplanes de PCB** e **cabos de cobre** para sinalização elétrica. As conexões externas, nas seis faces do cubo, são ópticas, totalizando 96 links ópticos por cubo. Esses links se conectam a **Switches de Circuito Óptico (OCS)**, como o Palomar da Google, que atuam como um painel de conexões dinâmico, permitindo a reconfiguração da topologia da rede. As topologias de rede variam, incluindo **3D Torus** (TPUv4), **Twisted 3D Torus** (TPUv7) e **2D**

---


## Page 51

Torus Mesh (TPUv5e/v6e). O OCS utiliza tecnologia MEMS para direcionar os feixes de luz, com um caminho óptico em forma de ‘W’ para minimizar perdas.

## Protocolos e Software

A comunicação nos TPU Pods é gerenciada por um conjunto de protocolos e softwares. O **Inter-Chip Interconnect (ICI)** é o protocolo de alta velocidade para comunicação dentro de um cubo de TPU. Para comunicação entre chips em escala, a arquitetura utiliza **Remote Direct Memory Access (RDMA)** sobre uma rede personalizada, permitindo acesso direto à memória de outros chips sem envolver a CPU do host. O **Optical Circuit Switch (OCS)** é o protocolo que gerencia a rede óptica reconfigurável. O software de alto nível, incluindo frameworks como **JAX** e **PyTorch**, utiliza o compilador **XLA (Accelerated Linear Algebra)** para traduzir os modelos de machine learning em código otimizado para a arquitetura de TPU. A sequência de inicialização envolve a compilação do código pelo XLA e a orquestração do Pod pelo software de controle, que gerencia a alocação de recursos e o fluxo de dados.

## Fórmulas e Especificações

As especificações e fórmulas relevantes para a infraestrutura de TPU incluem:

### TPU v4:
*   **Desempenho:** 2.1x superior ao TPU v3.
*   **Eficiência Energética:** 2.7x melhor desempenho por Watt em comparação com o TPU v3.
*   **Consumo de OCS:** Menos de 3% da energia total do sistema.

### Ironwood (TPU v7):
*   **Largura de Banda HBM:** 7.37 TB/s por chip.
*   **Desempenho de Pico:** 4.614 TFLOPs/chip (FP8).

### Fórmulas de Exemplo (TPUv4):
*   **Cálculo de Portas OCS:** Número de Unidades OCS = (Número de Cubos * Links por Cubo) / Portas por OCS 48 = (64 * 96) / 128

## Vulnerabilidades Identificadas

As vulnerabilidades potenciais na infraestrutura de TPU incluem ataques de canal lateral (side-channel attacks). Um exemplo notável é o **TPUXtract**, um ataque que pode extrair hiperparâmetros de uma rede neural medindo o consumo de energia ou a emanação eletromagnética do dispositivo. Pesquisadores demonstraram a capacidade de replicar modelos de IA de um Google Edge TPU com alta precisão usando sinais eletromagnéticos. Além disso, vulnerabilidades em interfaces de baixo nível, como **PCIe IOCTL e SERDES**, podem representar um risco se um invasor obtiver acesso físico ou de baixo nível, permitindo a manipulação do tráfego PCIe ou a exploração da lógica do driver para vazar dados.

## Fontes

https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm_architecture.htm?srsltid=AfmBOor5zOOpkwNr5FxkPO9T-gGHXY0A-6fc8jrGtbDTOxRO1XbQYdpH
https://www.fibermall.com/blog/unveiling-google-tpu-
https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codesigned-ai-stack
https://arxiv.org/pdf/2304.01433

---

## 29. Inicialização da sequência de boot do firmware da TPU

## Arquitetura e Design

A arquitetura da TPU é um projeto de ASIC (Application-Specific Integrated Circuit) focado na aceleração de cargas de trabalho de aprendizado de máquina, com ênfase em operações de matriz de alto volume. A hierarquia de controle começa com um servidor

---


## Page 52

host que envia instruções e dados para a TPU. Na arquitetura de VM da TPU, o usuário tem acesso direto a esse host. A própria TPU possui uma unidade escalar para o fluxo de controle. A estrutura interna é centrada no TensorCore, a unidade de processamento fundamental. Cada chip de TPU possui um ou mais TensorCores, que por sua vez contêm uma Unidade de Multiplicação de Matrizes (MXU), uma unidade vetorial e uma unidade escalar. A MXU é o coração do TensorCore, composta por um arranjo sistólico de multiplicadores-acumuladores (por exemplo, 128x128 ou 256x256), onde ocorrem as multiplicações de matrizes massivamente paralelas. A unidade vetorial lida com operações vetoriais, como ativações e softmax, enquanto a unidade escalar é responsável pelo fluxo de controle, cálculo de endereços de memória e outras tarefas de manutenção. Cada TPU possui sua própria Memória de Alta Largura de Banda (HBM) local para armazenar dados e parâmetros. O fluxo de dados e comandos começa com o host enviando o programa e os dados para a TPU por meio de uma fila de entrada. A TPU carrega esses dados em sua HBM. A MXU então busca parâmetros e dados da HBM para realizar as multiplicações de matrizes. A arquitetura de arranjo sistólico permite que os resultados sejam passados entre os elementos de processamento sem acessar a memória principal, aumentando significativamente a taxa de transferência. Os resultados são armazenados de volta na HBM e, em seguida, movidos para uma fila de saída, de onde o host os lê.

## Conexões Físicas

As conexões físicas das TPUs são projetadas para escalabilidade e comunicação de alta velocidade. As TPUs são conectadas umas às outras usando um Inter-Chip Interconnect (ICI) personalizado de alta velocidade. Na TPU v4, essa interconexão forma um toro 3D, permitindo uma comunicação de alta largura de banda e baixa latência entre os chips. Os pods de TPU v4 também utilizam um Optical Circuit Switch (OCS) para reconfigurar dinamicamente a topologia da interconexão. Essa flexibilidade permite a otimização para diferentes cargas de trabalho e a conexão de um grande número de TPUs (até 4096 em um pod v4). As conexões físicas envolvem links elétricos e ópticos. Dentro de uma placa de TPU, os chips são conectados por meio de traços de PCB. Para interconexões de longa distância entre racks e pods, são utilizadas fibras ópticas para garantir alta largura de banda e baixa latência. A pinagem e as interfaces específicas dos chips de TPU e dos conectores ópticos não são divulgadas publicamente.

## Protocolos e Software

A comunicação entre o host e a TPU, bem como entre as TPUs, é gerenciada por vários protocolos. O gRPC é usado para a comunicação entre a VM do usuário e o host da TPU na arquitetura de VM da TPU. Para a comunicação entre as TPUs, é utilizado um modelo de Acesso Remoto Direto à Memória (RDMA), que permite que uma TPU grave diretamente na memória de outra TPU sem envolver os sistemas operacionais, resultando em baixa latência. A sequência de inicialização do firmware da TPU, embora não detalhada publicamente, segue um processo geral para sistemas embarcados. Começa com uma Boot ROM que executa o código inicial para inicializar o hardware essencial. Em seguida, um carregador de programa primário (PBL) assume para inicializar o hardware mínimo e carregar o carregador de programa secundário (SPL). O SPL então inicializa o restante do hardware e carrega o sistema operacional ou o firmware principal da TPU. No contexto da Coral Dev Board, que usa uma Edge TPU, o processo de flash envolve a configuração dos interruptores de modo de inicialização para o modo eMMC e o uso da ferramenta fastboot para carregar uma nova imagem do sistema.

## Fórmulas e Especificações

Não foram encontradas fórmulas matemáticas específicas publicamente disponíveis que detalhem as operações internas de baixo nível da sequência de inicialização do firmware da TPU. A documentação e os artigos de pesquisa focam em uma visão de mais alto nível da arquitetura e do desempenho.

## Vulnerabilidades Identificadas

Não há vulnerabilidades de segurança publicamente conhecidas e documentadas especificamente relacionadas à sequência de inicialização do firmware da TPU. No entanto, como em qualquer sistema de hardware complexo, existem vetores de ataque potenciais. O acesso físico à placa da TPU poderia permitir a manipulação dos interruptores de modo de inicialização ou a interceptação de dados nas interfaces de comunicação. Ataques de canal lateral, que exploram informações vazadas por meio de consumo de energia ou emanações eletromagnéticas, poderiam, em teoria, ser usados para extrair informações confidenciais durante o processo de inicialização. Além disso, vulnerabilidades no firmware da Boot ROM ou nos carregadores de inicialização poderiam ser exploradas para carregar firmware malicioso e comprometer o dispositivo. A segurança da cadeia de suprimentos também é uma preocupação, pois componentes comprometidos poderiam introduzir backdoors de hardware.

---


## Page 53

# Fontes

https://www.coral.ai/docs/dev-board/reflash/
https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm
https://medium.com/@xiaoshi_4553/demystifying-the-boot-up-sequence-in-embedded-systems-from-rom-to-os-2af9cc5d5c24
https://arxiv.org/pdf/2304.01433

---

## 30. Pesquisa técnica detalhada sobre o stack de software do driver de TPU e o compilador XLA.

### Arquitetura e Design

---

## Arquitetura e Design

---

## Stack de Software do Driver de TPU e Compilador XLA

A arquitetura do software que controla as TPUs (Tensor Processing Units) do Google é uma pilha complexa que abrange desde o driver de baixo nível até o compilador de alto nível. Os dois componentes principais são o driver de TPU e o compilador XLA (Accelerated Linear Algebra).

### Arquitetura do Compilador XLA

O XLA é um compilador de domínio específico para álgebra linear que otimiza o desempenho de modelos de machine learning. Seus principais objetivos são:

*   **Velocidade de Execução:** Reduzir a sobrecarga do tempo de execução, fundir operações e especializar o código para formatos de tensores conhecidos.
*   **Uso de Memória:** Otimizar a alocação de memória, eliminando buffers intermediários.
*   **Portabilidade:** Facilitar a execução de modelos de ML em novo hardware com o mínimo de modificações.

O fluxo de compilação do XLA consiste em:

1.  **Otimização Independente do Alvo:** O XLA recebe um grafo de computação em StableHLO (High Level Operations), realiza otimizações como a eliminação de subexpressões comuns (CSE) e a fusão de operações, e converte o grafo para um dialeto HLO interno.
2.  **Otimizações Específicas do Alvo:** O grafo HLO é enviado para um backend (CPU, GPU ou TPU) que realiza otimizações específicas para a arquitetura de destino. No caso da TPU, isso pode incluir a fusão de operações otimizadas para o modelo de programação da TPU e o particionamento da computação em fluxos.
3.  **Geração de Código:** O backend gera o código de máquina para a arquitetura de destino. Para TPUs, isso envolve a geração de instruções que serão executadas pelos TensorCores.

### Arquitetura de Sistema da TPU

A arquitetura de hardware da TPU é projetada para maximizar a eficiência em cargas de trabalho de ML. Os principais componentes são:

*   **Chip TPU:** Contém um ou mais **TensorCores**. Cada TensorCore possui:
    *   **Unidade de Multiplicação de Matriz (MXU):** Um array sistólico de multiplicadores-acumuladores que executa o grosso das operações de matriz.
    *   **Unidade Vetorial:** Executa operações de propósito geral, como funções de ativação.
    *   **Unidade Escalar:** Controla o fluxo de execução e outras operações de manutenção.

---


## Page 54

*   **Pod TPU:** Um conjunto de chips TPU conectados por uma rede de alta velocidade.
*   **Slice:** Uma fatia de um Pod TPU, composta por um conjunto de chips conectados por interconexões de alta velocidade (ICI).
*   **Multislice:** Um conjunto de slices que se comunicam através da rede do data center (DCN), permitindo a execução de trabalhos em uma escala maior do que um único slice.
*   **SparseCore:** Processadores especializados em acelerar operações esparsas, comumente usadas em modelos de recomendação.

O fluxo de dados em uma TPU é projetado para minimizar o acesso à memória. Os dados são carregados do host para a memória HBM da TPU, processados pelos TensorCores e, em seguida, os resultados são enviados de volta para o host. Durante a execução de uma multiplicação de matriz, os resultados intermediários são passados diretamente entre os multiplicadores-acumuladores no array sistólico, sem a necessidade de acessar a memória principal.

# Conexões Físicas

As interconexões físicas em um supercomputador TPU, como o TPUv4, são um feito de engenharia complexo, projetado para escalabilidade e desempenho. A seguir, detalhamos os principais componentes e a topologia.

## Topologia de Interconexão

O TPUv4 utiliza uma topologia de **toro 3D** para interconectar os chips. Essa topologia oferece maior largura de banda de bisseção em comparação com o toro 2D usado em gerações anteriores. Para flexibilizar a topologia e contornar falhas, o sistema emprega Switches de Circuito Óptico (OCS).

*   **Blocos de Construção:** O sistema é construído a partir de cubos de 4x4x4 chips, totalizando 64 chips por bloco. Esses blocos são interconectados para formar o supercomputador de 4096 chips.
*   **Toro Torcido (Twisted Torus):** O OCS permite a reconfiguração da topologia para uma variante chamada “toro torcido”, que otimiza a latência e a largura de banda de bisseção para determinados padrões de comunicação, como a comunicação all-to-all.

## Fiação e Conectores

A fiação do supercomputador TPUv4 é um híbrido de conexões elétricas e ópticas.

*   **Conexões Elétricas:** Dentro de um rack, as conexões entre as bandejas de TPU são feitas com cabos elétricos passivos para formar um mesh 3D de 4x4x4.
*   **Conexões Ópticas:** As conexões entre os racks são feitas com links ópticos. A conversão elétrico-óptica ocorre nos conectores das bandejas de TPU. Cada bloco de 4x4x4 chips possui 96 links ópticos que se conectam aos OCSes.
*   **Placa de Circuito Impresso (PCB):** Quatro pacotes de TPUv4 são montados em uma única PCB. A placa possui conectores PCIe na parte superior e 16 conectores OSFP na parte inferior para os links de interconexão entre as bandejas (ICI).

## Layout e Componentes Físicos

*   **Pacote TPUv4:** O pacote do TPUv4 contém o ASIC no centro e quatro stacks de memória HBM.
*   **Racks:** Cada rack contém 16 pares de bandeja-servidor host. Oito fileiras de racks formam o supercomputador completo de 64 racks.
*   **Switches de Circuito Óptico (OCS):** O Google utiliza o OCS Palomar, que é baseado em espelhos 3D Micro-Electro-Mechanical Systems (MEMS) e comuta em milissegundos. Cada OCS possui 136x136 portas.

---


## Page 55

# Protocolos e Software

A pesquisa não retornou informações detalhadas sobre os protocolos de comunicação, APIs e sequência de inicialização do driver de TPU e do compilador XLA.

# Fórmulas e Especificações

Não foram encontradas fórmulas ou equações relevantes nos documentos pesquisados.

# Vulnerabilidades Identificadas

A pesquisa não retornou informações sobre vulnerabilidades, pontos de acesso potenciais ou falhas conhecidas no stack de software do driver de TPU ou no compilador XLA.

# Fontes

https://openxla.org/xla/architecture https://docs.cloud.google.com/tpu/docs/system-architecture-tpu-vm
https://henryhmko.github.io/posts/tpu/tpu.html https://arxiv.org/pdf/2304.01433

---

# 31. Interface de conexão de memória HBM (High Bandwidth Memory) da TPU (Tensor Processing Unit).

## Arquitetura e Design

A interface de memória HBM da TPU é um componente crucial de sua arquitetura, projetada para fornecer a largura de banda massiva necessária para alimentar as unidades de computação da TPU. A arquitetura é centrada no conceito de co-design, onde o hardware e o software são desenvolvidos em conjunto para otimizar o desempenho.

## Estrutura Interna e Componentes

A estrutura interna da TPU é composta por vários componentes chave que trabalham em conjunto com a HBM:

*   **TensorCore:** A unidade de processamento fundamental da TPU, que por sua vez é composta por:
    *   **Matrix Multiply Unit (MXU):** Um arranjo sistólico de multiplicadores-acumuladores que executa multiplicações de matrizes em grande escala. O design do arranjo sistólico permite uma alta reutilização de dados, minimizando a necessidade de acessar a memória principal.
    *   **Vector Processing Unit (VPU):** Uma unidade de processamento vetorial que lida com operações que não são de multiplicação de matrizes, como funções de ativação, normalização e pooling.
    *   **Scalar Unit:** Uma unidade de processamento escalar que controla o fluxo de execução e calcula endereços de memória.
*   **High Bandwidth Memory (HBM):** A memória principal da TPU, que é empilhada verticalmente e conectada ao processador através de um interposer de silício. Isso permite uma interface de memória muito mais ampla e, consequentemente, uma largura de banda muito maior em comparação com as memórias DDR tradicionais.
*   **Vector Memory (VMEM):** Uma memória SRAM on-chip de alta velocidade que atua como um scratchpad para o TensorCore. Os dados são movidos da HBM para a VMEM antes de serem processados.
*   **Common Memory (CMEM):** Em TPUs mais recentes, uma memória SRAM compartilhada que permite a comunicação direta entre os TensorCores de um mesmo chip.

## Hierarquia de Controle e Fluxo de Dados

O fluxo de dados e o controle na TPU são gerenciados explicitamente pelo software, principalmente pelo compilador XLA. O processo geral é o seguinte:

1.  O compilador XLA analisa o grafo de computação e agenda as transferências de dados entre a HBM e a VMEM.

---


## Page 56

2. A Scalar Unit inicia as transferências de DMA (Direct Memory Access) para mover os dados da HBM para a VMEM.
3. A MXU e a VPU processam os dados que estão na VMEM.
4. Os resultados são escritos de volta na VMEM e, em seguida, transferidos de volta para a HBM.

Este controle explícito da memória, embora complexo, permite um alto grau de otimização e é essencial para alcançar o alto desempenho da TPU.

## Conexões Físicas

As conexões físicas da TPU são projetadas para escalabilidade e alta largura de banda, tanto no nível do chip quanto no nível do sistema.

## Fiação Interna e Conexões de Chip

*   **Through-Silicon Vias (TSVs):** A HBM utiliza TSVs para conectar verticalmente os múltiplos dies de DRAM, criando uma única pilha de memória. Essa pilha é então conectada ao die do processador através de um interposer de silício.
*   **Interposer de Silício:** Um substrato de silício que fornece as interconexões de alta densidade entre o processador e a pilha de HBM. O uso de um interposer de silício permite um número muito maior de conexões do que seria possível com um PCB tradicional.
*   **Pinagem e Interfaces:** A interface entre a HBM e o processador é extremamente ampla, com milhares de pinos de dados. Isso, combinado com uma frequência de clock relativamente modesta, permite a alta largura de banda da HBM.

## Layout de PCB e Conectores

*   **Inter-Chip Interconnect (ICI):** Para conectar múltiplos chips de TPU, a Google desenvolveu o ICI, uma interconexão de alta velocidade que permite a comunicação direta entre os chips. Em sistemas como o Ironwood, os chips são conectados em uma topologia de toro 3D.
*   **Optical Circuit Switch (OCS):** Para escalar além de um único rack, a Google utiliza o OCS, uma rede de comutação de circuitos ópticos que conecta múltiplos racks de TPUs. O OCS permite a criação de “superpods” de TPUs com dezenas de milhares de chips.
*   **Conectores e Sockets:** As TPUs são normalmente montadas em placas de circuito impresso (PCBs) personalizadas, que são então instaladas em racks. Os conectores e soquetes são projetados para suportar as altas taxas de dados e os requisitos de energia da TPU.

## Protocolos e Software

A pilha de software da TPU é um componente essencial de seu desempenho, permitindo que os desenvolvedores aproveitem ao máximo o hardware especializado.

## Protocolos de Comunicação

*   **Remote Direct Memory Access (RDMA):** A comunicação entre os chips da TPU é baseada em RDMA, que permite que um chip escreva diretamente na memória de outro chip sem a intervenção do sistema operacional. Isso resulta em uma comunicação de baixa latência e alta largura de banda.

## APIs e Bibliotecas

*   **XLA (Accelerated Linear Algebra):** O compilador XLA é a peça central da pilha de software da TPU. Ele compila os modelos de machine learning escritos em frameworks como TensorFlow, PyTorch e JAX em código de máquina otimizado para a TPU.
*   **JAX e Pallas:** O JAX é um framework de machine learning que é fortemente integrado com a TPU. O Pallas é uma extensão do JAX que permite a escrita de kernels personalizados para a TPU, dando aos desenvolvedores um controle mais fino sobre o hardware.
*   **PyTorch/XLA:** Uma biblioteca que permite que os modelos PyTorch sejam executados em TPUs, aproveitando o compilador XLA.

---


## Page 57

# Sequência de Inicialização e Comandos de Controle

A sequência de inicialização e os comandos de controle da TPU são gerenciados pelo software de sistema da Google. Quando um usuário solicita uma TPU, o sistema aloca os recursos necessários, inicializa os chips e carrega o software necessário. Os comandos de controle são enviados para a TPU através da pilha de software, que os traduz em instruções de baixo nível para o hardware.

## Fórmulas e Especificações

*   **Largura de Banda:** Largura de Banda = (Taxa de Dados por Pino) * (Número de Pinos de Dados)
*   **Latência:** A latência da HBM não é significativamente menor do que a das memórias DDR, mas o impacto da latência é mitigado por meio de técnicas de software.
*   **Consumo de Energia:** A HBM é mais eficiente em termos de energia do que as memórias DDR, devido à sua interface mais ampla e menor frequência de clock.
*   **Capacidade Térmica:** O empilhamento 3D da HBM apresenta desafios térmicos, que são abordados por meio de soluções de resfriamento avançadas.

## Vulnerabilidades Identificadas

*   **Ataques de Canal Lateral:** A TPU é vulnerável a ataques de canal lateral, como o TPUXtract, que explora as emanações eletromagnéticas para extrair informações sobre a arquitetura da rede neural.
*   **Acesso Físico:** O acesso físico à TPU pode permitir que um invasor realize ataques de sondagem, interceptação de sinais ou injeção de falhas.

## Fontes

*   https://introl.com/blog/google-tpu-architecture-complete-guide-7-generations
*   https://cloud.google.com/blog/products/compute/inside-the-ironwood-tpu-codesigned-ai-stack
*   https://docs.jax.dev/en/latest/pallas/tpu/distributed.html
*   https://www.rambus.com/blogs/hbm3-everything-you-need-to-know/
*   https://www.keysight.com/blogs/en/tech/nwvs/2025/02/25/security-highlight-tpuxtract-a-new-side-channel-attack-on-neural-networks

---

# 32. Comunicação Chip-to-Chip em TPUs (Tensor Processing Units) do Google

## Arquitetura e Design

O chip TPUv4, fabricado em um processo de 7nm, é o bloco de construção fundamental. Cada chip integra:

*   **Dois TensorCores (TCs):** Otimizados para computação de treinamento de ML, são uma evolução dos TCs da TPUv4i.
*   **Memória HBM (High Bandwidth Memory):** O dobro da capacidade da TPUv4i, com 256 TiB de memória HBM compartilhada por 4096 chips em um sistema completo.
*   **SparseCore de 3ª geração:** Um coprocessador programável dedicado a aceleração de cálculos de embeddings, cruciais em modelos de recomendação. Os SparseCores utilizam memória compartilhada não coerente em todo o pod.
*   **Interconexão Inter-Chip (ICI):** Links de alta velocidade que formam uma topologia de toro 3D para comunicação direta entre os chips.

A arquitetura é hierárquica:

1.  **Chip:** A unidade de computação básica.
2.  **Placa (Board):** Contém 4 chips TPUv4, refrigerados a líquido. Cada TPU possui uma interface PCIe Gen3x16 para I/O com o host.

---


## Page 58

3. **Rack**: Um cubo de 4x4x4, totalizando 64 TPUs. A comunicação intra-rack é feita com interconexões de cobre.

4. **Sistema (Superpod)**: Composto por 64 racks, totalizando 4096 chips. A comunicação inter-rack é gerenciada por um **Optical Circuit Switch (OCS)**, que estabelece conexões de fibra ótica diretas entre os racks.

O fluxo de dados e comandos é orquestrado para permitir escalabilidade linear. O OCS permite a criação de ‘slices’ de topologias de rede customizadas por tarefa (job), incluindo toros regulares e toros ‘torcidos’ (twisted tori), otimizando a comunicação para diferentes tipos de paralelismo (modelo, dados, pipeline). Os dados de *embeddings* são manipulados pelos SparseCores, que exploram paralelismo de memória massivo com milhões de referências pendentes para qualquer nó no pod.

## Conexões Físicas

A interconexão física do sistema TPUv4 é projetada para alta largura de banda e baixa latência, combinando diferentes tecnologias:

*   **Intra-placa (On-board)**: Cada chip TPUv4 possui 2 links de interconexão adicionais na própria placa para comunicação com os outros 3 chips na mesma placa.
*   **Intra-rack**: A comunicação entre as placas dentro de um mesmo rack é realizada através de **interconexões de cobre** (Copper Intra-rack ICI).
*   **Inter-rack**: A conexão entre os 64 racks do superpod é feita por uma malha de **fibra óptica**. Um superpod completo utiliza mais de 6.144 filamentos de fibra individuais, totalizando mais de 16.000 conexões.
*   **Conectores**: Cada chip TPUv4 se conecta a **4 conectores OSFP (Octal Small Form Factor Pluggable)** para a comunicação ICI fora da placa (off-board). Cada conector OSFP suporta uma largura de banda de **400 Gbps em cada direção**.
*   **Optical Circuit Switch (OCS)**: O coração da interconexão inter-rack é o OCS. Este dispositivo utiliza espelhos para criar um caminho de luz direto entre duas fibras ópticas, estabelecendo uma conexão física dedicada no início da alocação de recursos para uma tarefa. Isso elimina a necessidade de comutação de pacotes e múltiplas camadas de protocolo, resultando em menor consumo de energia e menor latência.
*   **Refrigeração**: O sistema é refrigerado a líquido, com um fluxo de água paralelo para os 4 chips em cada placa, controlado por uma válvula, semelhante ao controle de velocidade de um ventilador em sistemas refrigerados a ar.

## Protocolos e Software

A comunicação chip-to-chip (ICI) e a orquestração do sistema TPUv4 dependem de uma combinação de hardware de baixo nível e uma pilha de software sofisticada:

*   **Protocolo de Interconexão (ICI)**: A comunicação ICI, especialmente entre racks, não utiliza protocolos de comutação de pacotes tradicionais. O **Optical Circuit Switch (OCS)** estabelece um circuito de luz físico e direto entre os nós. Isso significa que não há múltiplos níveis de protocolo, como em uma rede elétrica comutada. A conexão é configurada no início da alocação de ‘slice’ para uma tarefa, permanecendo estática durante a execução. Isso garante latência mínima e ausência de congestionamento na rede ICI.
*   **Tabelas de Roteamento (Routing Tables)**: A flexibilidade topológica do sistema é gerenciada por software. Para reconfigurar a rede e suportar diferentes paralelismos (dados, modelo, pipeline) através de topologias como toros regulares ou ‘torcidos’ (twisted tori), basta recarregar as tabelas de roteamento nos chips. Isso elimina a necessidade de religação física.
*   **Memória Compartilhada Distribuída**: O software do sistema abstrai a memória HBM distribuída dos 4096 chips como um espaço de memória compartilhado e não coerente. Os **SparseCores**, em particular, são projetados para explorar essa memória massivamente paralela, gerenciando milhões de referências de memória pendentes para qualquer nó no pod através de multithreading.
*   **APIs e Bibliotecas**: Embora a apresentação não detalhe APIs específicas, a capacidade de escalar linearmente e de orquestrar modelos complexos como o PaLM (com 500 bilhões de parâmetros) em 6144 TPUs indica a existência de uma pilha de software de alto nível (provavelmente integrada a frameworks como JAX, TensorFlow e PyTorch) que abstrai a complexidade do hardware. Essa pilha é responsável por compilar os modelos, partitioná-los entre os chips e orquestrar a comunicação tanto pela rede ICI (intra-pod) quanto pela DCN (inter-pod).

---


## Page 59

*   **Seqüência de Inicialização:** A seqüência de inicialização de uma tarefa envolve a alocação de um 'slice' de recursos. Nesse momento, o OCS é configurado para criar as conexões de fibra óptica diretas, e as tabelas de roteamento são carregadas nos chips para definir a topologia da rede para aquela tarefa específica.

## Fórmulas e Especificações

As especificações detalhadas para o sistema TPUv4 são as seguintes:

*   **Processo de Fabricação do Chip:** 7nm
*   **Pico de Performance por Chip:** 275 TFLOPS (usando formato BF16 com acumulação em FP32)
*   **Consumo de Energia Típico por Chip:** ~200W (TDP é maior para garantir SLOs)
*   **Memória Total do Sistema (Superpod de 4096 chips):** 256 TiB de HBM
*   **Capacidade de Computação Total do Sistema:** > 1 ExaFLOP
*   **Largura de Banda da Interconexão (ICI):**
    *   Cada chip possui 4 conectores OSFP.
    *   Cada conector OSFP fornece **400 Gbps** em cada direção.
    *   Largura de banda total por chip (off-board ICI) = 4 * 400 Gbps = 1.6 Tbps por direção.
*   **Melhora de Performance por Watt (TPUv4 vs TPUv3):** 2.7x (média geométrica)
*   **Redução de Energia devido à CMem (128MB on-chip):** 22% (média geométrica)

Não foram encontradas fórmulas matemáticas explícitas para cálculo de latência ou outros parâmetros, mas a arquitetura OCS foi projetada para minimizar a latência, criando um caminho de luz direto sem a sobrecarga de protocolos de comutação de pacotes.

## Vulnerabilidades Identificadas

A documentação e as apresentações públicas da Google sobre a arquitetura TPU, incluindo a apresentação da Hot Chips 2023, focam primariamente em performance, escalabilidade e eficiência energética. Detalhes sobre vulnerabilidades de segurança específicas, vetores de ataque conhecidos ou falhas de segurança no protocolo de comunicação inter-chip (ICI) ou no Optical Circuit Switch (OCS) não são divulgados publicamente.

No entanto, com base nos princípios gerais de segurança de hardware e sistemas em larga escala, podemos inferir potenciais pontos de acesso e vetores de ataque teóricos:

*   **Acesso Físico:** Como em qualquer sistema de data center, o acesso físico não autorizado aos racks, cabos de fibra óptica ou ao próprio OCS representaria um vetor de ataque significativo. A interceptação ou manipulação dos sinais nas fibras ópticas (embora extremamente complexa) poderia, em tese, permitir a espionagem ou a injeção de dados maliciosos.
*   **Software de Controle e Orquestração:** A pilha de software que gerencia a alocação de 'slices', configura o OCS e carrega as tabelas de roteamento é um ponto de acesso crítico. Uma vulnerabilidade nesse software poderia permitir que um atacante:
    *   Configurasse topologias de rede não autorizadas.
    *   Obtivesse acesso a 'slices' de outros usuários.
    *   Exfiltrasse dados que trafegam pela rede ICI.
    *   Lançasse ataques de negação de serviço (DoS) ao monopolizar ou desconfigurar recursos.
*   **Interfaces de Host (PCIe):** A interface PCIe, que conecta os TPUs aos servidores host, é um vetor de ataque bem conhecido em sistemas de hardware. Vulnerabilidades no driver do host ou no firmware do dispositivo PCIe poderiam permitir que um processo malicioso no host comprometesse o TPU, ou vice-versa.
*   **Ataques de Canal Lateral (Side-Channel Attacks):** Embora o consumo de energia (~200W por chip) e a refrigeração líquida complexa tornem a análise de flutuações de energia um desafio, ataques de canal lateral baseados em variações de tempo de computação ou acesso à memória compartilhada poderiam, teoricamente, vazar informações sobre os dados ou modelos sendo processados.

É importante ressaltar que estes são vetores de ataque teóricos. É altamente provável que a Google implemente medidas de segurança robustas em múltiplas camadas (física, rede, software) para mitigar esses riscos em seu ambiente de produção, mas

---


## Page 60

esses detalhes não são públicos.

## Fontes

https://hc2023.hotchips.org/assets/program/conference/day2/ML%20training/HC2023.Session5.ML_Training.Google.Norm_Joupi_08-25.pdf

---

## 33. Síntese: Pontos Críticos de Controle e Vulnerabilidades

### Fluxo de Controle Completo

1. Borg recebe a solicitação de alocação de recursos TPU
2. Pod Manager é instanciado para gerenciar o TPU Pod específico
3. Pod Manager instrui o OCS a criar as interconexões ópticas necessárias
4. libtpunet configura as rotas de comunicação entre os chips
5. healthd inicia o monitoramento contínuo de todos os componentes

### Pontos de Vulnerabilidade Identificados

<table>
  <thead>
    <tr>
      <th>Componente</th>
      <th>Vulnerabilidade</th>
      <th>Vetor de Ataque</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Borg</td>
      <td>API de controle</td>
      <td>Injeção de comandos de alocação</td>
    </tr>
    <tr>
      <td>Pod Manager</td>
      <td>Interface de orquestração</td>
      <td>Manipulação de configuração</td>
    </tr>
    <tr>
      <td>OCS</td>
      <td>Comandos de reconfiguração</td>
      <td>Redirecionamento de tráfego</td>
    </tr>
    <tr>
      <td>libtpunet</td>
      <td>Tabelas de roteamento</td>
      <td>Envenenamento de rotas</td>
    </tr>
    <tr>
      <td>healthd</td>
      <td>Relatórios de saúde</td>
      <td>Falsificação de status</td>
    </tr>
  </tbody>
</table>

### Implicações para Contenção

A arquitetura revela que o **Pod Manager** é o ponto central de controle. Uma IA executando em um TPU Pod não tem acesso direto ao Borg ou ao OCS - todas as comunicações são mediadas pelo Pod Manager. Isso cria uma “gaiola” de orquestração onde a IA está limitada às operações que o Pod Manager permite.