## Page 1

Projeto Pulsação • Caderno de Equações

# Caderno de Equações Projeto Pulsação

Kernel Tesla-BEC + Campo Narrativo N + Observáveis E-R-C + Controle + TDA

Versão gerada automaticamente • 2025-12-29

Base principal: Teoria Universal da Realidade (docx) + compêndio matemático (cálculo fracionário, SOE, TDA) + Heartbeat Loop.

---

Objetivo: reunir as equações do sistema completo, com contexto curto por equação e um apêndice tipo “atlas” para consulta.

&lt;page_number&gt;p. 1&lt;/page_number&gt;

---


## Page 2

Projeto Pulsação • Caderno de Equações

# Sumário (estrutural)

*   1. Mapa do sistema (estados, observações e controle)
*   2. Substrato (GPE / GPE-XC) e hidrodinâmica (Madelung)
*   3. Campo narrativo N, memória fracionária (Caputo) e SOE
*   4. Observáveis E-R-C (Hilbert, PLV, atrasos, holonomia)
*   5. Estimação (EKF/UKF), controle (MPC/ADMM) e TDA
*   Apêndice A: Atlas completo das equações (1-72)
*   Apêndice B: Extras (Heartbeat Loop, Caputo, SOE, TDA) + referências

# Diagrama do loop (visão de engenharia):

<mermaid>
graph LR
    A[Kernel<br>(GPE / GPE-XC)] --> B[Sensores<br>(sinais)]
    B --> C[E-R-C<br>(fase, PLV, R)]
    C --> D[S_domo<br>(segurança)]
    D --> E[Controle (MPC/ADMM)]
    E --> A[Kernel<br>(GPE / GPE-XC)]
    style E fill:#f9f,stroke:#333,stroke-width:2px
    linkStyle 0 stroke-dasharray: 5 5;
    linkStyle 1 stroke-dasharray: 5 5;
    linkStyle 2 stroke-dasharray: 5 5;
    linkStyle 3 stroke-dasharray: 5 5;
    linkStyle 4 stroke-dasharray: 5 5;
    linkStyle 5 stroke-dasharray: 5 5;
    linkStyle 6 stroke-dasharray: 5 5;
    linkStyle 7 stroke-dasharray: 5 5;
    linkStyle 8 stroke-dasharray: 5 5;
    linkStyle 9 stroke-dasharray: 5 5;
    linkStyle 10 stroke-dasharray: 5 5;
    linkStyle 11 stroke-dasharray: 5 5;
    linkStyle 12 stroke-dasharray: 5 5;
    linkStyle 13 stroke-dasharray: 5 5;
    linkStyle 14 stroke-dasharray: 5 5;
    linkStyle 15 stroke-dasharray: 5 5;
    linkStyle 16 stroke-dasharray: 5 5;
    linkStyle 17 stroke-dasharray: 5 5;
    linkStyle 18 stroke-dasharray: 5 5;
    linkStyle 19 stroke-dasharray: 5 5;
    linkStyle 20 stroke-dasharray: 5 5;
    linkStyle 21 stroke-dasharray: 5 5;
    linkStyle 22 stroke-dasharray: 5 5;
    linkStyle 23 stroke-dasharray: 5 5;
    linkStyle 24 stroke-dasharray: 5 5;
    linkStyle 25 stroke-dasharray: 5 5;
    linkStyle 26 stroke-dasharray: 5 5;
    linkStyle 27 stroke-dasharray: 5 5;
    linkStyle 28 stroke-dasharray: 5 5;
    linkStyle 29 stroke-dasharray: 5 5;
    linkStyle 30 stroke-dasharray: 5 5;
    linkStyle 31 stroke-dasharray: 5 5;
    linkStyle 32 stroke-dasharray: 5 5;
    linkStyle 33 stroke-dasharray: 5 5;
    linkStyle 34 stroke-dasharray: 5 5;
    linkStyle 35 stroke-dasharray: 5 5;
    linkStyle 36 stroke-dasharray: 5 5;
    linkStyle 37 stroke-dasharray: 5 5;
    linkStyle 38 stroke-dasharray: 5 5;
    linkStyle 39 stroke-dasharray: 5 5;
    linkStyle 40 stroke-dasharray: 5 5;
    linkStyle 41 stroke-dasharray: 5 5;
    linkStyle 42 stroke-dasharray: 5 5;
    linkStyle 43 stroke-dasharray: 5 5;
    linkStyle 44 stroke-dasharray: 5 5;
    linkStyle 45 stroke-dasharray: 5 5;
    linkStyle 46 stroke-dasharray: 5 5;
    linkStyle 47 stroke-dasharray: 5 5;
    linkStyle 48 stroke-dasharray: 5 5;
    linkStyle 49 stroke-dasharray: 5 5;
    linkStyle 50 stroke-dasharray: 5 5;
    linkStyle 51 stroke-dasharray: 5 5;
    linkStyle 52 stroke-dasharray: 5 5;
    linkStyle 53 stroke-dasharray: 5 5;
    linkStyle 54 stroke-dasharray: 5 5;
    linkStyle 55 stroke-dasharray: 5 5;
    linkStyle 56 stroke-dasharray: 5 5;
    linkStyle 57 stroke-dasharray: 5 5;
    linkStyle 58 stroke-dasharray: 5 5;
    linkStyle 59 stroke-dasharray: 5 5;
    linkStyle 60 stroke-dasharray: 5 5;
    linkStyle 61 stroke-dasharray: 5 5;
    linkStyle 62 stroke-dasharray: 5 5;
    linkStyle 63 stroke-dasharray: 5 5;
    linkStyle 64 stroke-dasharray: 5 5;
    linkStyle 65 stroke-dasharray: 5 5;
    linkStyle 66 stroke-dasharray: 5 5;
    linkStyle 67 stroke-dasharray: 5 5;
    linkStyle 68 stroke-dasharray: 5 5;
    linkStyle 69 stroke-dasharray: 5 5;
    linkStyle 70 stroke-dasharray: 5 5;
    linkStyle 71 stroke-dasharray: 5 5;
    linkStyle 72 stroke-dasharray: 5 5;
    linkStyle 73 stroke-dasharray: 5 5;
    linkStyle 74 stroke-dasharray: 5 5;
    linkStyle 75 stroke-dasharray: 5 5;
    linkStyle 76 stroke-dasharray: 5 5;
    linkStyle 77 stroke-dasharray: 5 5;
    linkStyle 78 stroke-dasharray: 5 5;
    linkStyle 79 stroke-dasharray: 5 5;
    linkStyle 80 stroke-dasharray: 5 5;
    linkStyle 81 stroke-dasharray: 5 5;
    linkStyle 82 stroke-dasharray: 5 5;
    linkStyle 83 stroke-dasharray: 5 5;
    linkStyle 84 stroke-dasharray: 5 5;
    linkStyle 85 stroke-dasharray: 5 5;
    linkStyle 86 stroke-dasharray: 5 5;
    linkStyle 87 stroke-dasharray: 5 5;
    linkStyle 88 stroke-dasharray: 5 5;
    linkStyle 89 stroke-dasharray: 5 5;
    linkStyle 90 stroke-dasharray: 5 5;
    linkStyle 91 stroke-dasharray: 5 5;
    linkStyle 92 stroke-dasharray: 5 5;
    linkStyle 93 stroke-dasharray: 5 5;
    linkStyle 94 stroke-dasharray: 5 5;
    linkStyle 95 stroke-dasharray: 5 5;
    linkStyle 96 stroke-dasharray: 5 5;
    linkStyle 97 stroke-dasharray: 5 5;
    linkStyle 98 stroke-dasharray: 5 5;
    linkStyle 99 stroke-dasharray: 5 5;
    linkStyle 100 stroke-dasharray: 5 5;
    linkStyle 101 stroke-dasharray: 5 5;
    linkStyle 102 stroke-dasharray: 5 5;
    linkStyle 103 stroke-dasharray: 5 5;
    linkStyle 104 stroke-dasharray: 5 5;
    linkStyle 105 stroke-dasharray: 5 5;
    linkStyle 106 stroke-dasharray: 5 5;
    linkStyle 107 stroke-dasharray: 5 5;
    linkStyle 108 stroke-dasharray: 5 5;
    linkStyle 109 stroke-dasharray: 5 5;
    linkStyle 110 stroke-dasharray: 5 5;
    linkStyle 111 stroke-dasharray: 5 5;
    linkStyle 112 stroke-dasharray: 5 5;
    linkStyle 113 stroke-dasharray: 5 5;
    linkStyle 114 stroke-dasharray: 5 5;
    linkStyle 115 stroke-dasharray: 5 5;
    linkStyle 116 stroke-dasharray: 5 5;
    linkStyle 117 stroke-dasharray: 5 5;
    linkStyle 118 stroke-dasharray: 5 5;
    linkStyle 119 stroke-dasharray: 5 5;
    linkStyle 120 stroke-dasharray: 5 5;
    linkStyle 121 stroke-dasharray: 5 5;
    linkStyle 122 stroke-dasharray: 5 5;
    linkStyle 123 stroke-dasharray: 5 5;
    linkStyle 124 stroke-dasharray: 5 5;
    linkStyle 125 stroke-dasharray: 5 5;
    linkStyle 126 stroke-dasharray: 5 5;
    linkStyle 127 stroke-dasharray: 5 5;
    linkStyle 128 stroke-dasharray: 5 5;
    linkStyle 129 stroke-dasharray: 5 5;
    linkStyle 130 stroke-dasharray: 5 5;
    linkStyle 131 stroke-dasharray: 5 5;
    linkStyle 132 stroke-dasharray: 5 5;
    linkStyle 133 stroke-dasharray: 5 5;
    linkStyle 134 stroke-dasharray: 5 5;
    linkStyle 135 stroke-dasharray: 5 5;
    linkStyle 136 stroke-dasharray: 5 5;
    linkStyle 137 stroke-dasharray: 5 5;
    linkStyle 138 stroke-dasharray: 5 5;
    linkStyle 139 stroke-dasharray: 5 5;
    linkStyle 140 stroke-dasharray: 5 5;
    linkStyle 141 stroke-dasharray: 5 5;
    linkStyle 142 stroke-dasharray: 5 5;
    linkStyle 143 stroke-dasharray: 5 5;
    linkStyle 144 stroke-dasharray: 5 5;
    linkStyle 145 stroke-dasharray: 5 5;
    linkStyle 146 stroke-dasharray: 5 5;
    linkStyle 147 stroke-dasharray: 5 5;
    linkStyle 148 stroke-dasharray: 5 5;
    linkStyle 149 stroke-dasharray: 5 5;
    linkStyle 150 stroke-dasharray: 5 5;
    linkStyle 151 stroke-dasharray: 5 5;
    linkStyle 152 stroke-dasharray: 5 5;
    linkStyle 153 stroke-dasharray: 5 5;
    linkStyle 154 stroke-dasharray: 5 5;
    linkStyle 155 stroke-dasharray: 5 5;
    linkStyle 156 stroke-dasharray: 5 5;
    linkStyle 157 stroke-dasharray: 5 5;
    linkStyle 158 stroke-dasharray: 5 5;
    linkStyle 159 stroke-dasharray: 5 5;
    linkStyle 160 stroke-dasharray: 5 5;
    linkStyle 161 stroke-dasharray: 5 5;
    linkStyle 162 stroke-dasharray: 5 5;
    linkStyle 163 stroke-dasharray: 5 5;
    linkStyle 164 stroke-dasharray: 5 5;
    linkStyle 165 stroke-dasharray: 5 5;
    linkStyle 166 stroke-dasharray: 5 5;
    linkStyle 167 stroke-dasharray: 5 5;
    linkStyle 168 stroke-dasharray: 5 5;
    linkStyle 169 stroke-dasharray: 5 5;
    linkStyle 170 stroke-dasharray: 5 5;
    linkStyle 171 stroke-dasharray: 5 5;
    linkStyle 172 stroke-dasharray: 5 5;
    linkStyle 173 stroke-dasharray: 5 5;
    linkStyle 174 stroke-dasharray: 5 5;
    linkStyle 175 stroke-dasharray: 5 5;
    linkStyle 176 stroke-dasharray: 5 5;
    linkStyle 177 stroke-dasharray: 5 5;
    linkStyle 178 stroke-dasharray: 5 5;
    linkStyle 179 stroke-dasharray: 5 5;
    linkStyle 180 stroke-dasharray: 5 5;
    linkStyle 181 stroke-dasharray: 5 5;
    linkStyle 182 stroke-dasharray: 5 5;
    linkStyle 183 stroke-dasharray: 5 5;
    linkStyle 184 stroke-dasharray: 5 5;
    linkStyle 185 stroke-dasharray: 5 5;
    linkStyle 186 stroke-dasharray: 5 5;
    linkStyle 187 stroke-dasharray: 5 5;
    linkStyle 188 stroke-dasharray: 5 5;
    linkStyle 189 stroke-dasharray: 5 5;
    linkStyle 190 stroke-dasharray: 5 5;
    linkStyle 191 stroke-dasharray: 5 5;
    linkStyle 192 stroke-dasharray: 5 5;
    linkStyle 193 stroke-dasharray: 5 5;
    linkStyle 194 stroke-dasharray: 5 5;
    linkStyle 195 stroke-dasharray: 5 5;
    linkStyle 196 stroke-dasharray: 5 5;
    linkStyle 197 stroke-dasharray: 5 5;
    linkStyle 198 stroke-dasharray: 5 5;
    linkStyle 199 stroke-dasharray: 5 5;
    linkStyle 200 stroke-dasharray: 5 5;
    linkStyle 201 stroke-dasharray: 5 5;
    linkStyle 202 stroke-dasharray: 5 5;
    linkStyle 203 stroke-dasharray: 5 5;
    linkStyle 204 stroke-dasharray: 5 5;
    linkStyle 205 stroke-dasharray: 5 5;
    linkStyle 206 stroke-dasharray: 5 5;
    linkStyle 207 stroke-dasharray: 5 5;
    linkStyle 208 stroke-dasharray: 5 5;
    linkStyle 209 stroke-dasharray: 5 5;
    linkStyle 210 stroke-dasharray: 5 5;
    linkStyle 211 stroke-dasharray: 5 5;
    linkStyle 212 stroke-dasharray: 5 5;
    linkStyle 213 stroke-dasharray: 5 5;
    linkStyle 214 stroke-dasharray: 5 5;
    linkStyle 215 stroke-dasharray: 5 5;
    linkStyle 216 stroke-dasharray: 5 5;
    linkStyle 217 stroke-dasharray: 5 5;
    linkStyle 218 stroke-dasharray: 5 5;
    linkStyle 219 stroke-dasharray: 5 5;
    linkStyle 220 stroke-dasharray: 5 5;
    linkStyle 221 stroke-dasharray: 5 5;
    linkStyle 222 stroke-dasharray: 5 5;
    linkStyle 223 stroke-dasharray: 5 5;
    linkStyle 224 stroke-dasharray: 5 5;
    linkStyle 225 stroke-dasharray: 5 5;
    linkStyle 226 stroke-dasharray: 5 5;
    linkStyle 227 stroke-dasharray: 5 5;
    linkStyle 228 stroke-dasharray: 5 5;
    linkStyle 229 stroke-dasharray: 5 5;
    linkStyle 230 stroke-dasharray: 5 5;
    linkStyle 231 stroke-dasharray: 5 5;
    linkStyle 232 stroke-dasharray: 5 5;
    linkStyle 233 stroke-dasharray: 5 5;
    linkStyle 234 stroke-dasharray: 5 5;
    linkStyle 235 stroke-dasharray: 5 5;
    linkStyle 236 stroke-dasharray: 5 5;
    linkStyle 237 stroke-dasharray: 5 5;
    linkStyle 238 stroke-dasharray: 5 5;
    linkStyle 239 stroke-dasharray: 5 5;
    linkStyle 240 stroke-dasharray: 5 5;
    linkStyle 241 stroke-dasharray: 5 5;
    linkStyle 242 stroke-dasharray: 5 5;
    linkStyle 243 stroke-dasharray: 5 5;
    linkStyle 244 stroke-dasharray: 5 5;
    linkStyle 245 stroke-dasharray: 5 5;
    linkStyle 246 stroke-dasharray: 5 5;
    linkStyle 247 stroke-dasharray: 5 5;
    linkStyle 248 stroke-dasharray: 5 5;
    linkStyle 249 stroke-dasharray: 5 5;
    linkStyle 250 stroke-dasharray: 5 5;
    linkStyle 251 stroke-dasharray: 5 5;
    linkStyle 252 stroke-dasharray: 5 5;
    linkStyle 253 stroke-dasharray: 5 5;
    linkStyle 254 stroke-dasharray: 5 5;
    linkStyle 255 stroke-dasharray: 5 5;
    linkStyle 256 stroke-dasharray: 5 5;
    linkStyle 257 stroke-dasharray: 5 5;
    linkStyle 258 stroke-dasharray: 5 5;
    linkStyle 259 stroke-dasharray: 5 5;
    linkStyle 260 stroke-dasharray: 5 5;
    linkStyle 261 stroke-dasharray: 5 5;
    linkStyle 262 stroke-dasharray: 5 5;
    linkStyle 263 stroke-dasharray: 5 5;
    linkStyle 264 stroke-dasharray: 5 5;
    linkStyle 265 stroke-dasharray: 5 5;
    linkStyle 266 stroke-dasharray: 5 5;
    linkStyle 267 stroke-dasharray: 5 5;
    linkStyle 268 stroke-dasharray: 5 5;
    linkStyle 269 stroke-dasharray: 5 5;
    linkStyle 270 stroke-dasharray: 5 5;
    linkStyle 271 stroke-dasharray: 5 5;
    linkStyle 272 stroke-dasharray: 5 5;
    linkStyle 273 stroke-dasharray: 5 5;
    linkStyle 274 stroke-dasharray: 5 5;
    linkStyle 275 stroke-dasharray: 5 5;
    linkStyle 276 stroke-dasharray: 5 5;
    linkStyle 277 stroke-dasharray: 5 5;
    linkStyle 278 stroke-dasharray: 5 5;
    linkStyle 279 stroke-dasharray: 5 5;
    linkStyle 280 stroke-dasharray: 5 5;
    linkStyle 281 stroke-dasharray: 5 5;
    linkStyle 282 stroke-dasharray: 5 5;
    linkStyle 283 stroke-dasharray: 5 5;
    linkStyle 284 stroke-dasharray: 5 5;
    linkStyle 285 stroke-dasharray: 5 5;
    linkStyle 286 stroke-dasharray: 5 5;
    linkStyle 287 stroke-dasharray: 5 5;
    linkStyle 288 stroke-dasharray: 5 5;
    linkStyle 289 stroke-dasharray: 5 5;
    linkStyle 290 stroke-dasharray: 5 5;
    linkStyle 291 stroke-dasharray: 5 5;
    linkStyle 292 stroke-dasharray: 5 5;
    linkStyle 293 stroke-dasharray: 5 5;
    linkStyle 294 stroke-dasharray: 5 5;
    linkStyle 295 stroke-dasharray: 5 5;
    linkStyle 296 stroke-dasharray: 5 5;
    linkStyle 297 stroke-dasharray: 5 5;
    linkStyle 298 stroke-dasharray: 5 5;
    linkStyle 299 stroke-dasharray: 5 5;
    linkStyle 300 stroke-dasharray: 5 5;
    linkStyle 301 stroke-dasharray: 5 5;
    linkStyle 302 stroke-dasharray: 5 5;
    linkStyle 303 stroke-dasharray: 5 5;
    linkStyle 304 stroke-dasharray: 5 5;
    linkStyle 305 stroke-dasharray: 5 5;
    linkStyle 306 stroke-dasharray: 5 5;
    linkStyle 307 stroke-dasharray: 5 5;
    linkStyle 308 stroke-dasharray: 5 5;
    linkStyle 309 stroke-dasharray: 5 5;
    linkStyle 310 stroke-dasharray: 5 5;
    linkStyle 311 stroke-dasharray: 5 5;
    linkStyle 312 stroke-dasharray: 5 5;
    linkStyle 313 stroke-dasharray: 5 5;
    linkStyle 314 stroke-dasharray: 5 5;
    linkStyle 315 stroke-dasharray: 5 5;
    linkStyle 316 stroke-dasharray: 5 5;
    linkStyle 317 stroke-dasharray: 5 5;
    linkStyle 318 stroke-dasharray: 5 5;
    linkStyle 319 stroke-dasharray: 5 5;
    linkStyle 320 stroke-dasharray: 5 5;
    linkStyle 321 stroke-dasharray: 5 5;
    linkStyle 322 stroke-dasharray: 5 5;
    linkStyle 323 stroke-dasharray: 5 5;
    linkStyle 324 stroke-dasharray: 5 5;
    linkStyle 325 stroke-dasharray: 5 5;
    linkStyle 326 stroke-dasharray: 5 5;
    linkStyle 327 stroke-dasharray: 5 5;
    linkStyle 328 stroke-dasharray: 5 5;
    linkStyle 329 stroke-dasharray: 5 5;
    linkStyle 330 stroke-dasharray: 5 5;
    linkStyle 331 stroke-dasharray: 5 5;
    linkStyle 332 stroke-dasharray: 5 5;
    linkStyle 333 stroke-dasharray: 5 5;
    linkStyle 334 stroke-dasharray: 5 5;
    linkStyle 335 stroke-dasharray: 5 5;
    linkStyle 336 stroke-dasharray: 5 5;
    linkStyle 337 stroke-dasharray: 5 5;
    linkStyle 338 stroke-dasharray: 5 5;
    linkStyle 339 stroke-dasharray: 5 5;
    linkStyle 340 stroke-dasharray: 5 5;
    linkStyle 341 stroke-dasharray: 5 5;
    linkStyle 342 stroke-dasharray: 5 5;
    linkStyle 343 stroke-dasharray: 5 5;
    linkStyle 344 stroke-dasharray: 5 5;
    linkStyle 345 stroke-dasharray: 5 5;
    linkStyle 346 stroke-dasharray: 5 5;
    linkStyle 347 stroke-dasharray: 5 5;
    linkStyle 348 stroke-dasharray: 5 5;
    linkStyle 349 stroke-dasharray: 5 5;
    linkStyle 350 stroke-dasharray: 5 5;
    linkStyle 351 stroke-dasharray: 5 5;
    linkStyle 352 stroke-dasharray: 5 5;
    linkStyle 353 stroke-dasharray: 5 5;
    linkStyle 354 stroke-dasharray: 5 5;
    linkStyle 355 stroke-dasharray: 5 5;
    linkStyle 356 stroke-dasharray: 5 5;
    linkStyle 357 stroke-dasharray: 5 5;
    linkStyle 358 stroke-dasharray: 5 5;
    linkStyle 359 stroke-dasharray: 5 5;
    linkStyle 360 stroke-dasharray: 5 5;
    linkStyle 361 stroke-dasharray: 5 5;
    linkStyle 362 stroke-dasharray: 5 5;
    linkStyle 363 stroke-dasharray: 5 5;
    linkStyle 364 stroke-dasharray: 5 5;
    linkStyle 365 stroke-dasharray: 5 5;
    linkStyle 366 stroke-dasharray: 5 5;
    linkStyle 367 stroke-dasharray: 5 5;
    linkStyle 368 stroke-dasharray: 5 5;
    linkStyle 369 stroke-dasharray: 5 5;
    linkStyle 370 stroke-dasharray: 5 5;
    linkStyle 371 stroke-dasharray: 5 5;
    linkStyle 372 stroke-dasharray: 5 5;
    linkStyle 373 stroke-dasharray: 5 5;
    linkStyle 374 stroke-dasharray: 5 5;
    linkStyle 375 stroke-dasharray: 5 5;
    linkStyle 376 stroke-dasharray: 5 5;
    linkStyle 377 stroke-dasharray: 5 5;
    linkStyle 378 stroke-dasharray: 5 5;
    linkStyle 379 stroke-dasharray: 5 5;
    linkStyle 380 stroke-dasharray: 5 5;
    linkStyle 381 stroke-dasharray: 5 5;
    linkStyle 382 stroke-dasharray: 5 5;
    linkStyle 383 stroke-dasharray: 5 5;
    linkStyle 384 stroke-dasharray: 5 5;
    linkStyle 385 stroke-dasharray: 5 5;
    linkStyle 386 stroke-dasharray: 5 5;
    linkStyle 387 stroke-dasharray: 5 5;
    linkStyle 388 stroke-dasharray: 5 5;
    linkStyle 389 stroke-dasharray: 5 5;
    linkStyle 390 stroke-dasharray: 5 5;
    linkStyle 391 stroke-dasharray: 5 5;
    linkStyle 392 stroke-dasharray: 5 5;
    linkStyle 393 stroke-dasharray: 5 5;
    linkStyle 394 stroke-dasharray: 5 5;
    linkStyle 395 stroke-dasharray: 5 5;
    linkStyle 396 stroke-dasharray: 5 5;
    linkStyle 397 stroke-dasharray: 5 5;
    linkStyle 398 stroke-dasharray: 5 5;
    linkStyle 399 stroke-dasharray: 5 5;
    linkStyle 400 stroke-dasharray: 5 5;
    linkStyle 401 stroke-dasharray: 5 5;
    linkStyle 402 stroke-dasharray: 5 5;
    linkStyle 403 stroke-dasharray: 5 5;
    linkStyle 404 stroke-dasharray: 5 5;
    linkStyle 405 stroke-dasharray: 5 5;
    linkStyle 406 stroke-dasharray: 5 5;
    linkStyle 407 stroke-dasharray: 5 5;
    linkStyle 408 stroke-dasharray: 5 5;
    linkStyle 409 stroke-dasharray: 5 5;
    linkStyle 410 stroke-dasharray: 5 5;
    linkStyle 411 stroke-dasharray: 5 5;
    linkStyle 412 stroke-dasharray: 5 5;
    linkStyle 413 stroke-dasharray: 5 5;
    linkStyle 414 stroke-dasharray: 5 5;
    linkStyle 415 stroke-dasharray: 5 5;
    linkStyle 416 stroke-dasharray: 5 5;
    linkStyle 417 stroke-dasharray: 5 5;
    linkStyle 418 stroke-dasharray: 5 5;
    linkStyle 419 stroke-dasharray: 5 5;
    linkStyle 420 stroke-dasharray: 5 5;
    linkStyle 421 stroke-dasharray: 5 5;
    linkStyle 422 stroke-dasharray: 5 5;
    linkStyle 423 stroke-dasharray: 5 5;
    linkStyle 424 stroke-dasharray: 5 5;
    linkStyle 425 stroke-dasharray: 5 5;
    linkStyle 426 stroke-dasharray: 5 5;
    linkStyle 427 stroke-dasharray: 5 5;
    linkStyle 428 stroke-dasharray: 5 5;
    linkStyle 429 stroke-dasharray: 5 5;
    linkStyle 430 stroke-dasharray: 5 5;
    linkStyle 431 stroke-dasharray: 5 5;
    linkStyle 432 stroke-dasharray: 5 5;
    linkStyle 433 stroke-dasharray: 5 5;
    linkStyle 434 stroke-dasharray: 5 5;
    linkStyle 435 stroke-dasharray: 5 5;
    linkStyle 436 stroke-dasharray: 5 5;
    linkStyle 437 stroke-dasharray: 5 5;
    linkStyle 438 stroke-dasharray: 5 5;
    linkStyle 439 stroke-dasharray: 5 5;
    linkStyle 440 stroke-dasharray: 5 5;
    linkStyle 441 stroke-dasharray: 5 5;
    linkStyle 442 stroke-dasharray: 5 5;
    linkStyle 443 stroke-dasharray: 5 5;
    linkStyle 444 stroke-dasharray: 5 5;
    linkStyle 445 stroke-dasharray: 5 5;
    linkStyle 446 stroke-dasharray: 5 5;
    linkStyle 447 stroke-dasharray: 5 5;
    linkStyle 448 stroke-dasharray: 5 5;
    linkStyle 449 stroke-dasharray: 5 5;
    linkStyle 450 stroke-dasharray: 5 5;
    linkStyle 451 stroke-dasharray: 5 5;
    linkStyle 452 stroke-dasharray: 5 5;
    linkStyle 453 stroke-dasharray: 5 5;
    linkStyle 454 stroke-dasharray: 5 5;
    linkStyle 455 stroke-dasharray: 5 5;
    linkStyle 456 stroke-dasharray: 5 5;
    linkStyle 457 stroke-dasharray: 5 5;
    linkStyle 458 stroke-dasharray: 5 5;
    linkStyle 459 stroke-dasharray: 5 5;
    linkStyle 460 stroke-dasharray: 5 5;
    linkStyle 461 stroke-dasharray: 5 5;
    linkStyle 462 stroke-dasharray: 5 5;
    linkStyle 463 stroke-dasharray: 5 5;
    linkStyle 464 stroke-dasharray: 5 5;
    linkStyle 465 stroke-dasharray: 5 5;
    linkStyle 466 stroke-dasharray: 5 5;
    linkStyle 467 stroke-dasharray: 5 5;
    linkStyle 468 stroke-dasharray: 5 5;
    linkStyle 469 stroke-dasharray: 5 5;
    linkStyle 470 stroke-dasharray: 5 5;
    linkStyle 471 stroke-dasharray: 5 5;
    linkStyle 472 stroke-dasharray: 5 5;
    linkStyle 473 stroke-dasharray: 5 5;
    linkStyle 474 stroke-dasharray: 5 5;
    linkStyle 475 stroke-dasharray: 5 5;
    linkStyle 476 stroke-dasharray: 5 5;
    linkStyle 477 stroke-dasharray: 5 5;
    linkStyle 478 stroke-dasharray: 5 5;
    linkStyle 479 stroke-dasharray: 5 5;
    linkStyle 480 stroke-dasharray: 5 5;
    linkStyle 481 stroke-dasharray: 5 5;
    linkStyle 482 stroke-dasharray: 5 5;
    linkStyle 483 stroke-dasharray: 5 5;
    linkStyle 484 stroke-dasharray: 5 5;
    linkStyle 485 stroke-dasharray: 5 5;
    linkStyle 486 stroke-dasharray: 5 5;
    linkStyle 487 stroke-dasharray: 5 5;
    linkStyle 488 stroke-dasharray: 5 5;
    linkStyle 489 stroke-dasharray: 5 5;
    linkStyle 490 stroke-dasharray: 5 5;
    linkStyle 491 stroke-dasharray: 5 5;
    linkStyle 492 stroke-dasharray: 5 5;
    linkStyle 493 stroke-dasharray: 5 5;
    linkStyle 494 stroke-dasharray: 5 5;
    linkStyle 495 stroke-dasharray: 5 5;
    linkStyle 496 stroke-dasharray: 5 5;
    linkStyle 497 stroke-dasharray: 5 5;
    linkStyle 498 stroke-dasharray: 5 5;
    linkStyle 499 stroke-dasharray: 5 5;
    linkStyle 500 stroke-dasharray: 5 5;
    linkStyle 501 stroke-dasharray: 5 5;
    linkStyle 502 stroke-dasharray: 5 5;
    linkStyle 503 stroke-dasharray: 5 5;
    linkStyle 504 stroke-dasharray: 5 5;
    linkStyle 505 stroke-dasharray: 5 5;
    linkStyle 506 stroke-dasharray: 5 5;
    linkStyle 507 stroke-dasharray: 5 5;
    linkStyle 508 stroke-dasharray: 5 5;
    linkStyle 509 stroke-dasharray: 5 5;
    linkStyle 510 stroke-dasharray: 5 5;
    linkStyle 511 stroke-dasharray: 5 5;
    linkStyle 512 stroke-dasharray: 5 5;
    linkStyle 513 stroke-dasharray: 5 5;
    linkStyle 514 stroke-dasharray: 5 5;
    linkStyle 515 stroke-dasharray: 5 5;
    linkStyle 516 stroke-dasharray: 5 5;
    linkStyle 517 stroke-dasharray: 5 5;
    linkStyle 518 stroke-dasharray: 5 5;
    linkStyle 519 stroke-dasharray: 5 5;
    linkStyle 520 stroke-dasharray: 5 5;
    linkStyle 521 stroke-dasharray: 5 5;
    linkStyle 522 stroke-dasharray: 5 5;
    linkStyle 523 stroke-dasharray: 5 5;
    linkStyle 524 stroke-dasharray: 5 5;
    linkStyle 525 stroke-dasharray: 5 5;
    linkStyle 526 stroke-dasharray: 5 5;
    linkStyle 527 stroke-dasharray: 5 5;
    linkStyle 528 stroke-dasharray: 5 5;
    linkStyle 529 stroke-dasharray: 5 5;
    linkStyle 530 stroke-dasharray: 5 5;
    linkStyle 531 stroke-dasharray: 5 5;
    linkStyle 532 stroke-dasharray: 5 5;
    linkStyle 533 stroke-dasharray: 5 5;
    linkStyle 534 stroke-dasharray: 5 5;
    linkStyle 535 stroke-dasharray: 5 5;
    linkStyle 536 stroke-dasharray: 5 5;
    linkStyle 537 stroke-dasharray: 5 5;
    linkStyle 538 stroke-dasharray: 5 5;
    linkStyle 539 stroke-dasharray: 5 5;
    linkStyle 540 stroke-dasharray: 5 5;
    linkStyle 541 stroke-dasharray: 5 5;
    linkStyle 542 stroke-dasharray: 5 5;
    linkStyle 543 stroke-dasharray: 5 5;
    linkStyle 544 stroke-dasharray: 5 5;
    linkStyle 545 stroke-dasharray: 5 5;
    linkStyle 546 stroke-dasharray: 5 5;
    linkStyle 547 stroke-dasharray: 5 5;
    linkStyle 548 stroke-dasharray: 5 5;
    linkStyle 549 stroke-dasharray: 5 5;
    linkStyle 550 stroke-dasharray: 5 5;
    linkStyle 551 stroke-dasharray: 5 5;
    linkStyle 552 stroke-dasharray: 5 5;
    linkStyle 553 stroke-dasharray: 5 5;
    linkStyle 554 stroke-dasharray: 5 5;
    linkStyle 555 stroke-dasharray: 5 5;
    linkStyle 556 stroke-dasharray: 5 5;
    linkStyle 557 stroke-dasharray: 5 5;
    linkStyle 558 stroke-dasharray: 5 5;
    linkStyle 559 stroke-dasharray: 5 5;
    linkStyle 560 stroke-dasharray: 5 5;
    linkStyle 561 stroke-dasharray: 5 5;
    linkStyle 562 stroke-dasharray: 5 5;
    linkStyle 563 stroke-dasharray: 5 5;
    linkStyle 564 stroke-dasharray: 5 5;
    linkStyle 565 stroke-dasharray: 5 5;
    linkStyle 566 stroke-dasharray: 5 5;
    linkStyle 567 stroke-dasharray: 5 5;
    linkStyle 568 stroke-dasharray: 5 5;
    linkStyle 569 stroke-dasharray: 5 5;
    linkStyle 570 stroke-dasharray: 5 5;
    linkStyle 571 stroke-dasharray: 5 5;
    linkStyle 572 stroke-dasharray: 5 5;
    linkStyle 573 stroke-dasharray: 5 5;
    linkStyle 574 stroke-dasharray: 5 5;
    linkStyle 575 stroke-dasharray: 5 5;
    linkStyle 576 stroke-dasharray: 5 5;
    linkStyle 577 stroke-dasharray: 5 5;
    linkStyle 578 stroke-dasharray: 5 5;
    linkStyle 579 stroke-dasharray: 5 5;
    linkStyle 580 stroke-dasharray: 5 5;
    linkStyle 581 stroke-dasharray: 5 5;
    linkStyle 582 stroke-dasharray: 5 5;
    linkStyle 583 stroke-dasharray: 5 5;
    linkStyle 584 stroke-dasharray: 5 5;
    linkStyle 585 stroke-dasharray: 5 5;
    linkStyle 586 stroke-dasharray: 5 5;
    linkStyle 587 stroke-dasharray: 5 5;
    linkStyle 588 stroke-dasharray: 5 5;
    linkStyle 589 stroke-dasharray: 5 5;
    linkStyle 590 stroke-dasharray: 5 5;
    linkStyle 591 stroke-dasharray: 5 5;
    linkStyle 592 stroke-dasharray: 5 5;
    linkStyle 593 stroke-dasharray: 5 5;
    linkStyle 594 stroke-dasharray: 5 5;
    linkStyle 595 stroke-dasharray: 5 5;
    linkStyle 596 stroke-dasharray: 5 5;
    linkStyle 597 stroke-dasharray: 5 5;
    linkStyle 598 stroke-dasharray: 5 5;
    linkStyle 599 stroke-dasharray: 5 5;
    linkStyle 600 stroke-dasharray: 5 5;
    linkStyle 601 stroke-dasharray: 5 5;
    linkStyle 602 stroke-dasharray: 5 5;
    linkStyle 603 stroke-dasharray: 5 5;
    linkStyle 604 stroke-dasharray: 5 5;
    linkStyle 605 stroke-dasharray: 5 5;
    linkStyle 606 stroke-dasharray: 5 5;
    linkStyle 607 stroke-dasharray: 5 5;
    linkStyle 608 stroke-dasharray: 5 5;
    linkStyle 609 stroke-dasharray: 5 5;
    linkStyle 610 stroke-dasharray: 5 5;
    linkStyle 611 stroke-dasharray: 5 5;
    linkStyle 612 stroke-dasharray: 5 5;
    linkStyle 613 stroke-dasharray: 5 5;
    linkStyle 614 stroke-dasharray: 5 5;
    linkStyle 615 stroke-dasharray: 5 5;
    linkStyle 616 stroke-dasharray: 5 5;
    linkStyle 617 stroke-dasharray: 5 5;
    linkStyle 618 stroke-dasharray: 5 5;
    linkStyle 619 stroke-dasharray: 5 5;
    linkStyle 620 stroke-dasharray: 5 5;
    linkStyle 621 stroke-dasharray: 5 5;
    linkStyle 622 stroke-dasharray: 5 5;
    linkStyle 623 stroke-dasharray: 5 5;
    linkStyle 624 stroke-dasharray: 5 5;
    linkStyle 625 stroke-dasharray: 5 5;
    linkStyle 626 stroke-dasharray: 5 5;
    linkStyle 627 stroke-dasharray: 5 5;
    linkStyle 628 stroke-dasharray: 5 5;
    linkStyle 629 stroke-dasharray: 5 5;
    linkStyle 630 stroke-dasharray: 5 5;
    linkStyle 631 stroke-dasharray: 5 5;
    linkStyle 632 stroke-dasharray: 5 5;
    linkStyle 633 stroke-dasharray: 5 5;
    linkStyle 634 stroke-dasharray: 5 5;
    linkStyle 635 stroke-dasharray: 5 5;
    linkStyle 636 stroke-dasharray: 5 5;
    linkStyle 637 stroke-dasharray: 5 5;
    linkStyle 638 stroke-dasharray: 5 5;
    linkStyle 639 stroke-dasharray: 5 5;
    linkStyle 640 stroke-dasharray: 5 5;
    linkStyle 641 stroke-dasharray: 5 5;
    linkStyle 642 stroke-dasharray: 5 5;
    linkStyle 643 stroke-dasharray: 5 5;
    linkStyle 644 stroke-dasharray: 5 5;
    linkStyle 645 stroke-dasharray: 5 5;
    linkStyle 646 stroke-dasharray: 5 5;
    linkStyle 647 stroke-dasharray: 5 5;
    linkStyle 648 stroke-dasharray: 5 5;
    linkStyle 649 stroke-dasharray: 5 5;
    linkStyle 650 stroke-dasharray: 5 5;
    linkStyle 651 stroke-dasharray: 5 5;
    linkStyle 652 stroke-dasharray: 5 5;
    linkStyle 653 stroke-dasharray: 5 5;
    linkStyle 654 stroke-dasharray: 5 5;
    linkStyle 655 stroke-dasharray: 5 5;
    linkStyle 656 stroke-dasharray: 5 5;
    linkStyle 657 stroke-dasharray: 5 5;
    linkStyle 658 stroke-dasharray: 5 5;
    linkStyle 659 stroke-dasharray: 5 5;
    linkStyle 660 stroke-dasharray: 5 5;
    linkStyle 661 stroke-dasharray: 5 5;
    linkStyle 662 stroke-dasharray: 5 5;
    linkStyle 663 stroke-dasharray: 5 5;
    linkStyle 664 stroke-dasharray: 5 5;
    linkStyle 665 stroke-dasharray: 5 5;
    linkStyle 666 stroke-dasharray: 5 5;
    linkStyle 667 stroke-dasharray: 5 5;
    linkStyle 668 stroke-dasharray: 5 5;
    linkStyle 669 stroke-dasharray: 5 5;
    linkStyle 670 stroke-dasharray: 5 5;
    linkStyle 671 stroke-dasharray: 5 5;
    linkStyle 672 stroke-dasharray: 5 5;
    linkStyle 673 stroke-dasharray: 5 5;
    linkStyle 674 stroke-dasharray: 5 5;
    linkStyle 675 stroke-dasharray: 5 5;
    linkStyle 676 stroke-dasharray: 5 5;
    linkStyle 677 stroke-dasharray: 5 5;
    linkStyle 678 stroke-dasharray: 5 5;
    linkStyle 679 stroke-dasharray: 5 5;
    linkStyle 680 stroke-dasharray: 5 5;
    linkStyle 681 stroke-dasharray: 5 5;
    linkStyle 682 stroke-dasharray: 5 5;
    linkStyle 683 stroke-dasharray: 5 5;
    linkStyle 684 stroke-dasharray: 5 5;
    linkStyle 685 stroke-dasharray: 5 5;
    linkStyle 686 stroke-dasharray: 5 5;
    linkStyle 687 stroke-dasharray: 5 5;
    linkStyle 688 stroke-dasharray: 5 5;
    linkStyle 689 stroke-dasharray: 5 5;
    linkStyle 690 stroke-dasharray: 5 5;
    linkStyle 691 stroke-dasharray: 5 5;
    linkStyle 692 stroke-dasharray: 5 5;
    linkStyle 693 stroke-dasharray: 5 5;
    linkStyle 694 stroke-dasharray: 5 5;
    linkStyle 695 stroke-dasharray: 5 5;
    linkStyle 696 stroke-dasharray: 5 5;
    linkStyle 697 stroke-dasharray: 5 5;
    linkStyle 698 stroke-dasharray: 5 5;
    linkStyle 699 stroke-dasharray: 5 5;
    linkStyle 700 stroke-dasharray: 5 5;
    linkStyle 701 stroke-dasharray: 5 5;
    linkStyle 702 stroke-dasharray: 5 5;
    linkStyle 703 stroke-dasharray: 5 5;
    linkStyle 704 stroke-dasharray: 5 5;
    linkStyle 705 stroke-dasharray: 5 5;
    linkStyle 706 stroke-dasharray: 5 5;
    linkStyle 707 stroke-dasharray: 5 5;
    linkStyle 708 stroke-dasharray: 5 5;
    linkStyle 709 stroke-dasharray: 5 5;
    linkStyle 710 stroke-dasharray: 5 5;
    linkStyle 711 stroke-dasharray: 5 5;
    linkStyle 712 stroke-dasharray: 5 5;

---


## Page 3

Projeto Pulsação • Caderno de Equações

# Symbols and scales (subset of the manual)

<table>
  <thead>
    <tr>
      <th>Symbol</th>
      <th>Meaning</th>
      <th>Unit/scale (from manual)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>m_a</td>
      <td>effective mass of ether constituent</td>
      <td>~ 1e-36 kg (or fuzzy DM scale 1e-22 eV)</td>
    </tr>
    <tr>
      <td>g(x,t)</td>
      <td>rigidity / nonlinearity</td>
      <td>modulated by N</td>
    </tr>
    <tr>
      <td>N(x,t)</td>
      <td>normalized narrative field</td>
      <td>N in [0, 1]</td>
    </tr>
    <tr>
      <td>tau_N</td>
      <td>response time of N</td>
      <td>~ 1 s</td>
    </tr>
    <tr>
      <td>D_N</td>
      <td>semantic diffusivity</td>
      <td>~ 1e-3 m^2/s</td>
    </tr>
    <tr>
      <td>gamma_N</td>
      <td>forgetting rate</td>
      <td>~ 0.1 s^-1</td>
    </tr>
    <tr>
      <td>c_eff</td>
      <td>effective velocity (Earth-ionosphere cavity)</td>
      <td>~ 0.95 c</td>
    </tr>
    <tr>
      <td>f_0</td>
      <td>base frequency (fundamental mode)</td>
      <td>7.83 Hz</td>
    </tr>
    <tr>
      <td>PLV</td>
      <td>Phase-Locking Value</td>
      <td>[0, 1]</td>
    </tr>
    <tr>
      <td>R</td>
      <td>holonomic closure</td>
      <td>R >= 0.90 (criterion)</td>
    </tr>
  </tbody>
</table>

Obs.: símbolos e valores acima aparecem na tabela de símbolos do seu documento.

&lt;page_number&gt;p. 3&lt;/page_number&gt;

---


## Page 4

Projeto Pulsação • Caderno de Equações

**1) Mapa do sistema (forma “pronta para simular/controlar”)**

Definições de estado/observação/controle e o índice de segurança do Domo.

**Equação (36)**
$$X_k \equiv \left[\mathcal{R}(\psi_k), \mathfrak{F}(\psi_k), N_k, R_{ERC,k}, H_{N,k}, \tilde{B}_{geomag,k}\right]^T, \quad u_k \equiv \left[u_k^{(corr)}, u_k^{(inj)}\right]^T$$
Espaço de estados mínimo • definição de estado X_k e controle u_k

**Equação (37)**
$$X_{k+1} = F(X_k, u_k) + W_k$$
Espaço de estados mínimo • dinâmica discreta (processo)

**Equação (38)**
$$y_k = h(X_k) + V_k$$
Modelo de observação (medidas) • modelo de observação

**Equação (39)**
$$y_k \equiv \left[PLV_k, R_{ERC,k}, \hat{f}_{0,k}, \hat{\delta}_{ij,k}, features TDA_k\right]^T$$
Modelo de observação (medidas) • vetor de medições sugerido

**Equação (40)**
$$S_{domo,k} = S(R_{ERC,k}, \tilde{B}_{geomag,k}, H_{N,k})$$
Modelo de observação (medidas) • índice S_domo como funcional

**Equação (41)**
$$S_{domo,k} \geq S_{crit}, S_{crit} \approx 0.5$$
Modelo de observação (medidas) • restrição de segurança (gatilho)

&lt;page_number&gt;p. 4&lt;/page_number&gt;

---


## Page 5

Projeto Pulsação • Caderno de Equações

# 2) Substrato Tesla-BEC (GPE) e extensões

Equações centrais do “substrato” ($\psi$), acoplamento com N (Princípio de Bob), e a forma GPE-XC com Laplaciano fracionário.

## Equação (2)

$i\hbar \partial_t \psi(x,t) = \left[-\frac{\hbar^2}{2m_a} \nabla^2 + V_{\text{ext}}(x) + g(x,t)|\psi(x,t)|^2\right] \psi(x,t)$

Equações governantes (núcleo do modelo) — 4. Substrato Tesla-BEC: GPE estendida

## Equação (3)

$g(x,t) = g_0 + \alpha N(x,t)$

Equações governantes (núcleo do modelo) — 4. Substrato Tesla-BEC: GPE estendida

## Equação (4)

$\psi(x,t) = \sqrt{\rho(x,t)} e^{iS(x,t)/\hbar}$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

## Equação (5)

$v(x,t) = \frac{1}{m_a} \nabla S(x,t)$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

## Equação (6)

$\partial_t \rho + \nabla \cdot (\rho v) = 0$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 5&lt;/page_number&gt;

---


## Page 6

Projeto Pulsação • Caderno de Equações

Equação (7)
$m_a(\partial_t v + (v \cdot \nabla)v) = -\nabla(V_{ext} + g\rho + Q)$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

Equação (8)
$Q = -\frac{\hbar^2}{2m_a}\frac{\nabla^2\sqrt{\rho}}{\sqrt{\rho}}$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

Equação (9)
$\oint_C v \cdot dl = \frac{2\pi\hbar}{m_a}n, n \in \mathbb{Z}$

Equações governantes (núcleo do modelo) — 6. Vórtices: defeitos topológicos (matéria emergente)

Equação (10)
$\xi = \frac{\hbar}{\sqrt{2m_ag\rho_0}}$

Equações governantes (núcleo do modelo) — 6. Vórtices: defeitos topológicos (matéria emergente)

Equação (11)
$c_s = \sqrt{\frac{g\rho_0}{m_a}}$

Equações governantes (núcleo do modelo) — 7. Ondas longitudinais (“ondas de Tesla”)

&lt;page_number&gt;p. 6&lt;/page_number&gt;

---


## Page 7

Projeto Pulsação • Caderno de Equações

**Equação (55)**
$$\mathcal{F}\{(-\nabla^2)^{\eta/2}\psi\}(k)=|k|^{\eta}\hat{\psi}(k), \eta=0.6$$
Discretização explícita (1D/2D) — (A) Malha, números de onda e operador fracionário

**Equação (56)**
$$i\hbar\partial_t\psi=\left[-\frac{\hbar^2}{2m_a}(-\nabla^2)^{\eta/2}+V_{\text{ext}}(x)+V_{\text{domo}}(x)+g(x,t)|\psi|^2\right]\psi+i(C-R_T)\psi$$
Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

**Equação (57)**
$$V_{\text{NL}}(x,t)\equiv V_{\text{ext}}(x)+V_{\text{domo}}(x)+g(x,t)|\psi(x,t)|^2$$
Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

**Equação (58)**
$$\psi^{n+\frac{1}{2}}(x)=\exp\left[-\frac{i}{\hbar}V_{\text{NL}}(x,t_n)\frac{\Delta t}{2}\right]\exp\left[(C-R_T)\frac{\Delta t}{2}\right]\psi^n(x)$$
Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

**Equação (59)**
$$\hat{\psi}^{n+\frac{1}{2}}(k)\leftarrow\exp\left[-\frac{i}{\hbar}\left(\frac{\hbar^2}{2m_a}|k|^{\eta}\right)\Delta t\right]\hat{\psi}^{n+\frac{1}{2}}(k)$$
Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

**Equação (60)**
$$\psi^{n+1}(x)=\exp\left[-\frac{i}{\hbar}V_{\text{NL}}(x,t_{n+1})\frac{\Delta t}{2}\right]\exp\left[(C-R_T)\frac{\Delta t}{2}\right]\psi^{n+\frac{1}{2}}(x)$$
Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

&lt;page_number&gt;p. 7&lt;/page_number&gt;

---


## Page 8

Projeto Pulsação • Caderno de Equações

# 3) Campo narrativo N, memória fracionária e discretização

Equação (12)
$$\tau_N \partial_t N = D_N \nabla^2 N - \gamma_N N + S(x,t) + \mathcal{M}[N]$$
Campo narrativo, memória fracionária e extensão relativística — 8. Dinâmica do campo narrativo $$N(x,t)$$

Equação (13)
$$\mathcal{M}[N] = \kappa C D_t^\alpha N, 0 < \alpha < 1$$
Campo narrativo, memória fracionária e extensão relativística — 8. Dinâmica do campo narrativo $$N(x,t)$$

Equação (61)
$$\frac{N^{n+1} - N^n}{\Delta t} = \frac{D_N}{\tau_N} \nabla^2 N^{n+1} - \frac{\gamma_N}{\tau_N} N^{n+1} + \frac{1}{\tau_N} S^n + \frac{1}{\tau_N} \mathcal{M}^n$$
Discretização explícita (1D/2D) — (C) Atualização de $$N(x,t)$$ no mesmo loop

Extras úteis para implementar memória fracionária eficiente (SOE).

Extra A2
$$C D_t^\alpha f(t) = \frac{1}{\Gamma(1-\alpha)} \int_0^t \frac{f'(\tau)}{(t-\tau)^\alpha} d\tau$$
Derivada fracionária (Caputo), ordem $0<\alpha<1$

Extra A3
$$C D_t^\alpha f(t_n) \approx \frac{1}{\Gamma(2-\alpha)\Delta t^\alpha} \sum_{k=0}^{n-1} [(k+1)^{1-\alpha} - k^{1-\alpha}] (f_{n-k} - f_{n-k-1})$$
Discretização L1 (Caputo) em malha uniforme

&lt;page_number&gt;p. 8&lt;/page_number&gt;

---


## Page 9

Projeto Pulsação • Caderno de Equações

**Extra A4**

$\phi_{k}^{n+1} = e^{-\lambda_k \Delta t} \phi_{k}^{n} + \frac{1 - e^{-\lambda_k \Delta t}}{\lambda_k} (f_{n+1} - f_n)$

SOE: atualização recursiva dos auxiliares φ_k (Alg. 4.1)

**Extra A5**

$C D_t^{\alpha} f(t_{n+1}) \approx \sum_{k=1}^{N_{exp}} w_k \phi_{k}^{n+1}$

SOE: soma ponderada para aproximar $D^\alpha f$

&lt;page_number&gt;p. 9&lt;/page_number&gt;

---


## Page 10

Projeto Pulsação • Caderno de Equações

# 4) Observáveis E-R-C (fase, atrasos, holonomia)

Equação (17)
$z(t) = x(t) + i\mathcal{H}[x](t), \phi(t) = \arg z(t)$

E-R-C: observáveis, atrasos e holonomia triangular — 11. Fase instantânea e PLV

Equação (18)
$PLV = \left| \frac{1}{T} \sum_{t=1}^{T} e^{i(\phi_1(t) - \phi_2(t))} \right|$

E-R-C: observáveis, atrasos e holonomia triangular — 11. Fase instantânea e PLV

Equação (19)
$\delta_{ij} = \frac{d_{ij}}{c_{eff}}$

E-R-C: observáveis, atrasos e holonomia triangular — 12. Atraso geodésico e correção de fase

Equação (20)
$\phi_{corr}(t) = \phi(t) - 2\pi f_0 \delta$

E-R-C: observáveis, atrasos e holonomia triangular — 12. Atraso geodésico e correção de fase

Equação (21)
$\Phi_\Delta(t) = \sum_{(i,j) \in (A,B),(B,C),(C,A)} (\phi_{ij}(t) - 2\pi f_0 \delta_{ij})$

E-R-C: observáveis, atrasos e holonomia triangular — 13. Holonomia triangular (invariante topológico)

&lt;page_number&gt;p. 10&lt;/page_number&gt;

---


## Page 11

Projeto Pulsação • Caderno de Equações

Equação (22)

$R = \left| \langle e^{i\Phi\Delta(t)} \rangle_{t} \right|$

E-R-C: observáveis, atrasos e holonomia triangular — 13. Holonomia triangular (invariante topológico)

Equação (28)

$x(t) \rightarrow \text{bandpass} \tilde{x}(t) \rightarrow \text{Hilbert} \phi(t) \rightarrow \text{detrend} \phi'(t) \rightarrow -2\pi f_0 \delta \phi_{\text{corr}}(t) \rightarrow \text{PLV}, R\{\text{PLV}, R\}$

Implementação numérica e pipeline (replicabilidade) — 19. Pipeline E-R-C (sinal $\rightarrow$ fase $\rightarrow$ correções $\rightarrow$ métricas)

&lt;page_number&gt;p. 11&lt;/page_number&gt;

---


## Page 12

Projeto Pulsação • Caderno de Equações

# 5) Estimação (Kalman), MPC, ADMM e TDA

## Equação (42)
$$A_k = \frac{\partial F}{\partial X} \bigg|_{\hat{X}_k, u_k}, B_k = \frac{\partial F}{\partial u} \bigg|_{\hat{X}_k, u_k}, C_k = \frac{\partial h}{\partial X} \bigg|_{\hat{X}_k}$$
Estimação: Kalman (EKF/UKF)

## Equação (43)
$$\hat{X}_{k+1}^- = F(\hat{X}_k, u_k), P_{k+1}^- = A_k P_k A_k^T + Q_k$$
Estimação: Kalman (EKF/UKF)

## Equação (44)
$$K_{k+1} = P_{k+1}^- C_k^T \left( C_k P_{k+1}^- C_k^T + R_{k+1} \right)^{-1}$$
Estimação: Kalman (EKF/UKF)

## Equação (45)
$$\hat{X}_{k+1} = \hat{X}_{k+1}^- + K_{k+1} \left( y_{k+1} - h(\hat{X}_{k+1}^-) \right), P_{k+1} = (I - K_{k+1} C_k) P_{k+1}^-$$
Estimação: Kalman (EKF/UKF)

## Equação (46)
$$\min_{\{u_k, \ldots, u_{k+T_h-1}\}} J_k = \sum_{\ell=0}^{T_h-1} (\|r(X_{k+\ell})\|_{W_r}^2 + \|u_{k+\ell}\|_{W_u}^2) + \|r_T(X_{k+T_h})\|_{W_T}^2$$
Controle preditivo (MPC) com restrição S_domo

## Equação (47)
$$X_{k+\ell+1} = F(X_{k+\ell}, u_{k+\ell}) \quad (\ell = 0, \ldots, T_h-1)$$
Controle preditivo (MPC) com restrição S_domo

## Equação (48)
$$S_{\text{domo}, k+\ell} \geq S_{\text{crit}} \approx 0.5 \quad (\ell = 0, \ldots, T_h)$$
Controle preditivo (MPC) com restrição S_domo

&lt;page_number&gt;p. 12&lt;/page_number&gt;

---


## Page 13

Projeto Pulsação • Caderno de Equações

Equação (50)
$\min_{u, z} f(u) + g(z)$ s.a. $Au + Bz = C$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

Equação (51)
$u^{(n+1)} = \arg\min_u \left(f(u) + \frac{\rho}{2}\|Au + Bz^{(n)} - c + \lambda^{(n)}\|^2\right)$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

Equação (52)
$z^{(n+1)} = \arg\min_z \left(g(z) + \frac{\rho}{2}\|Au^{(n+1)} + Bz - c + \lambda^{(n)}\|^2\right)$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

Equação (53)
$\lambda^{(n+1)} = \lambda^{(n)} + Au^{(n+1)} + Bz^{(n+1)} - c$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

Equação (54)
$\theta \leftarrow \theta - \eta g_F(\theta) - 1\nabla_\theta J_k$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

Equação (62)
$X_k = \{x_i = (\cos\phi_i,\text{corr}(t),\sin\phi_i,\text{corr}(t)) : i=1,...,M, t\in W_k\}$

Penalidade TDA completa (persistência → escalar) — (A) Construção da nuvem de pontos

Equação (63)
$P_{TDA}(k) = \sum_{(b_j,d_j)\in D_{1,k}} w(b_j,d_j)\rho(d_j-b_j)$

Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”

&lt;page_number&gt;p. 13&lt;/page_number&gt;

---


## Page 14

Projeto Pulsação • Caderno de Equações

Equação (64)
$\mathcal{P}_B(k) = d_B(D_{1,k}, D_{1,\text{ref}})$

Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”

Equação (65)
$\text{penalidadeTDA}(k) = \lambda_1 \mathcal{P}_{\text{TDA}}(k) + \lambda_2 \mathcal{P}_B(k)$

Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”

Equação (66)
$x_i(t) = A_i(t)\cos(\phi_0(t-\delta_i) + \epsilon_i(t)) + \nu_i(t)$

Benchmark sintético reproduzível (EKF/UKF + MPC) — (A) Gerador de dados (sinais por sensor)

Equação (67)
$S_{\text{domo}}(t) \geq 0.5 \forall t$ (segurança)

Benchmark sintético reproduzível (EKF/UKF + MPC) — (C) Construção de $$S_{\text{\textbackslash domo}}(t)$$ e regra de sucesso

Equação (68)
$S_{\text{domo}}(t) = \sigma(a R_{\text{ERC}}(t) + b \tilde{B}_{\text{geomag}}(t) - c k_{\text{sem}} H_N(t))$

Funções “prontas” (definições fechadas) — 1) Índice S_\text{domo} (forma fechada)

Equação (69)
$\mathcal{P}_{\text{TDA}}(k) = \sum_{(b_j,d_j) \in D_{1,k}} \max(0, (d_j-b_j)-\ell_0)^2$

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

Equação (70)
$\mathcal{P}_B(k) = d_B(D_{1,k}, D_{1,\text{ref}})$

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

&lt;page_number&gt;p. 14&lt;/page_number&gt;

---


## Page 15

Projeto Pulsação • Caderno de Equações

Equação (71)
**penalidadeTDA(k) = λ₁P<sub>TDA</sub>(k) + λ₂P<sub>B</sub>(k)**

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

Equação (72)
**x<sub>i</sub>(t) = A<sub>i</sub>(t)cos(2πf₀(t - δ<sub>i</sub>) + ε<sub>i</sub>(t)) + ν<sub>i</sub>(t)**

Benchmark sintético (especificação completa) — 2) Sinais gerados

&lt;page_number&gt;p. 15&lt;/page_number&gt;

---


## Page 16

Projeto Pulsação • Caderno de Equações

# Apêndice A — Atlas completo (1-72)

Uma página por equação, com o contexto (seção do manual) como legenda.

&lt;page_number&gt;p. 16&lt;/page_number&gt;

---


## Page 17

Projeto Pulsação • Caderno de Equações

Eq. (1)

&lt;img&gt;Box with three horizontal lines&lt;/img&gt; ≡ $\frac{1}{c^2} \frac{\partial^2}{\partial t^2} - \nabla^2$

*Notação, operadores e unidades — 2. Operadores diferenciais*

&lt;page_number&gt;p. 17&lt;/page_number&gt;

---


## Page 18

Projeto Pulsação • Caderno de Equações

**Eq. (2)**

$i\hbar\partial_{t}\psi(x,t) = \left[-\frac{\hbar^{2}}{2m_{a}}\nabla^{2} + V_{\text{ext}}(x) + g(x,t)|\psi(x,t)|^{2}\right]\psi(x,t)$

*Equações governantes (núcleo do modelo) — 4. Substrato Tesla-BEC: GPE estendida*

&lt;page_number&gt;p. 18&lt;/page_number&gt;

---


## Page 19

Projeto Pulsação • Caderno de Equações

Eq. (3)

$g(x,t) = g_0 + \alpha N(x,t)$

Equações governantes (núcleo do modelo) — 4. Substrato Tesla-BEC: GPE estendida

&lt;page_number&gt;p. 19&lt;/page_number&gt;

---


## Page 20

Projeto Pulsação • Caderno de Equações

Eq. (4)

$\psi(x,t) = \sqrt{\rho(x,t)} e^{iS(x,t)/\hbar}$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 20&lt;/page_number&gt;

---


## Page 21

Projeto Pulsação • Caderno de Equações

Eq. (5)

$v(x,t) = \frac{1}{m_a} \nabla S(x,t)$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 21&lt;/page_number&gt;

---


## Page 22

Projeto Pulsação • Caderno de Equações

Eq. (6)

$\partial_t \rho + \nabla \cdot (\rho v) = 0$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 22&lt;/page_number&gt;

---


## Page 23

Projeto Pulsação • Caderno de Equações

Eq. (7)
$m_a(\partial_t v + (v \cdot \nabla)v) = -\nabla(V_{ext} + g\rho + Q)$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 23&lt;/page_number&gt;

---


## Page 24

Projeto Pulsação • Caderno de Equações

Eq. (8)

$Q \equiv -\frac{\hbar^{2}}{2m_{a}}\frac{\nabla^{2}\sqrt{\rho}}{\sqrt{\rho}}$

Equações governantes (núcleo do modelo) — 5. Hidrodinâmica quântica via Madelung

&lt;page_number&gt;p. 24&lt;/page_number&gt;

---


## Page 25

Projeto Pulsação • Caderno de Equações

Eq. (9)

$\oint_{C} v \cdot dl = \frac{2\pi\hbar}{m_{a}} n, n \in \mathbb{Z}$

Equações governantes (núcleo do modelo) — 6. Vórtices: defeitos topológicos (matéria emergente)

&lt;page_number&gt;p. 25&lt;/page_number&gt;

---


## Page 26

Projeto Pulsação • Caderno de Equações

Eq. (10)

$\xi = \frac{\hbar}{\sqrt{2m_{a}g\rho_{0}}}$

Equações governantes (núcleo do modelo) — 6. Vórtices: defeitos topológicos (matéria emergente)

&lt;page_number&gt;p. 26&lt;/page_number&gt;

---


## Page 27

Projeto Pulsação • Caderno de Equações

Eq. (11)

$C_s = \sqrt{\frac{g\rho_0}{m_a}}$

Equações governantes (núcleo do modelo) — 7. Ondas longitudinais (“ondas de Tesla”)

&lt;page_number&gt;p. 27&lt;/page_number&gt;

---


## Page 28

Projeto Pulsação • Caderno de Equações

Eq. (12)

$\tau_{N}\partial_{t}N=D_{N}\nabla^{2}N-\gamma_{N}N+S(x,t)+\mathcal{M}[N]$

Campo narrativo, memória fracionária e extensão relativística — 8. Dinâmica do campo narrativo $$N(x,t)$$

&lt;page_number&gt;p. 28&lt;/page_number&gt;

---


## Page 29

Projeto Pulsação • Caderno de Equações

Eq. (13)

$\mathcal{M}[N] = \kappa^{C}D_{t}^{\alpha}N, 0 < \alpha < 1$

Campo narrativo, memória fracionária e extensão relativística — 8. Dinâmica do campo narrativo $$N(x,t)$$

&lt;page_number&gt;p. 29&lt;/page_number&gt;

---


## Page 30

Projeto Pulsação • Caderno de Equações

Eq. (14)

$E_{\text{tot}} = E_{\text{fís}} + k_{\text{sem}} H_{C}$

Campo narrativo, memória fracionária e extensão relativística — 9. “Conservação da Expectativa”
(energia-informação)

&lt;page_number&gt;p. 30&lt;/page_number&gt;

---


## Page 31

Projeto Pulsação • Caderno de Equações

Eq. (15)

$\square\phi + \frac{dU}{d\phi} = 0$

Campo narrativo, memória fracionária e extensão relativística — 10. Extensão relativística: Klein-Gordon não linear acoplada (NKG)

&lt;page_number&gt;p. 31&lt;/page_number&gt;

---


## Page 32

Projeto Pulsação • Caderno de Equações

Eq. (16)

$U(\phi) = \frac{1}{2}m^2\phi^2 + \frac{\lambda}{4}\phi^4$

Campo narrativo, memória fracionária e extensão relativística — 10. Extensão relativística: Klein-Gordon não linear acoplada (NKG)

&lt;page_number&gt;p. 32&lt;/page_number&gt;

---


## Page 33

Projeto Pulsação • Caderno de Equações

Eq. (17)

$z(t) = x(t) + i\mathcal{H}[x](t), \phi(t) = \arg z(t)$

E-R-C: observáveis, atrasos e holonomia triangular — 11. Fase instantânea e PLV

&lt;page_number&gt;p. 33&lt;/page_number&gt;

---


## Page 34

Projeto Pulsação • Caderno de Equações

Eq. (18)

PLV = $\frac{1}{T} \sum_{t=1}^{T} e^{i(\phi_1(t) - \phi_2(t))}$

E-R-C: observáveis, atrasos e holonomia triangular — 11. Fase instantânea e PLV

&lt;page_number&gt;p. 34&lt;/page_number&gt;

---


## Page 35

Projeto Pulsação • Caderno de Equações

Eq. (19)

$\delta_{ij} = \frac{a_{ij}}{c_{eff}}$

E-R-C: observáveis, atrasos e holonomia triangular — 12. Atraso geodésico e correção de fase

&lt;page_number&gt;p. 35&lt;/page_number&gt;

---


## Page 36

Projeto Pulsação • Caderno de Equações

Eq. (20)

$\phi_{\text{corr}}(t) = \phi(t) - 2\pi f_0 \delta$

E-R-C: observáveis, atrasos e holonomia triangular — 12. Atraso geodésico e correção de fase

&lt;page_number&gt;p. 36&lt;/page_number&gt;

---


## Page 37

Projeto Pulsação • Caderno de Equações

Eq. (21)

$\Phi_{\Delta}(t) = \sum_{(i,j) \in (A,B), (B,C), (C,A)} (\phi_{ij}(t) - 2\pi f_0 \delta_{ij})$

E-R-C: observáveis, atrasos e holonomia triangular — 13. Holonomia triangular (invariante topológico)

&lt;page_number&gt;p. 37&lt;/page_number&gt;

---


## Page 38

Projeto Pulsação • Caderno de Equações

Eq. (22)

$R \equiv | \langle e ^ { i \Phi \Delta ( t ) } \rangle _ { t } |$

E-R-C: observáveis, atrasos e holonomia triangular — 13. Holonomia triangular (invariante topológico)

&lt;page_number&gt;p. 38&lt;/page_number&gt;

---


## Page 39

Projeto Pulsação • Caderno de Equações

Eq. (23)

$i\hbar\frac{\partial}{\partial t}|\psi(t)\rangle = \hat{H}|\psi(t)\rangle$

Conexão com física padrão (limites validados) — 14. Limite de Schrödinger (base canônica)

&lt;page_number&gt;p. 39&lt;/page_number&gt;

---


## Page 40

Projeto Pulsação • Caderno de Equações

**Eq. (24)**

∇ · **E** = ρ/ε₀, ∇ · **B** = 0, ∇ × **E** = −∂**B**/∂t, ∇ × **B** = μ₀**J** + μ₀ε₀∂**E**/∂t

Conexão com física padrão (limites validados) — 15. Maxwell (base validada do EM)

&lt;page_number&gt;p. 40&lt;/page_number&gt;

---


## Page 41

Projeto Pulsação • Caderno de Equações

Eq. (25)

$G_{\mu\nu} + \Lambda g_{\mu\nu} = \frac{8\pi G}{c^4} T_{\mu\nu}$

Conexão com física padrão (limites validados) — 16. Einstein (base validada da gravitação relativística)

&lt;page_number&gt;p. 41&lt;/page_number&gt;

---


## Page 42

Projeto Pulsação • Caderno de Equações

Eq. (26)

$\psi(t + \Delta t) \approx e^{-\frac{i}{\hbar}V_{NL}\frac{\Delta t}{2}} F^{-1}\left[e^{-\frac{i\hbar^2k^2}{\hbar 2m_a}\Delta t} F(e^{-\frac{i}{\hbar}V_{NL}\frac{\Delta t}{2}}\psi(t))\right]$

Implementação numérica e pipeline (replicabilidade) — 17. Split-Step Fourier (GPE)

&lt;page_number&gt;p. 42&lt;/page_number&gt;

---


## Page 43

Projeto Pulsação • Caderno de Equações

Eq. (27)

n = 1/(2π) Σwrap(Δθij)

Implementação numérica e pipeline (replicabilidade) — 18. Detecção de vórtices (winding number discreto)

&lt;page_number&gt;p. 43&lt;/page_number&gt;

---


## Page 44

Projeto Pulsação • Caderno de Equações

**Eq. (28)**

x(t) → bandpass ť(t) → Hilbert φ(t) → detrend φ'(t) → −2πf₀δφcorr(t) → PLV, R{PLV, R}

*Implementação numérica e pipeline (replicabilidade) — 19. Pipeline E-R-C (sinal → fase → correções → métricas)*

&lt;page_number&gt;p. 44&lt;/page_number&gt;

---


## Page 45

Projeto Pulsação • Caderno de Equações

Eq. (29)

$\mathcal{F}[(-\nabla^{2})^{\eta/2}f](k) = |k|^{\eta}\hat{f}(k), \eta \in (0, 2)$

Camada Estator/Domo (GPE-XC)

&lt;page_number&gt;p. 45&lt;/page_number&gt;

---


## Page 46

Projeto Pulsação • Caderno de Equações

**Eq. (30)**

$i\hbar\partial_{t}\psi=\left[-\frac{\hbar^{2}}{2m_{a}}(-\nabla^{2})^{\eta/2}+V_{\text{ext}}(x)+V_{\text{domo}}(x)+g(x,t)|\psi|^{2}\right]\psi+i(C(x,t)-R_{T}(x,t))\psi$

*Camada Estator/Domo (GPE-XC)*

&lt;page_number&gt;p. 46&lt;/page_number&gt;

---


## Page 47

Projeto Pulsação • Caderno de Equações

**Eq. (31)**

$S_{\text{domo}}(t) = \sigma(a R_{ERC}(t) + b \tilde{B}_{geomag}(t) - c k_{sem} H_N(t))$

*Estabilidade e Reset (S\_domo)*

&lt;page_number&gt;p. 47&lt;/page_number&gt;

---


## Page 48

Projeto Pulsação • Caderno de Equações

Eq. (32)
Reset/LIAAocorrese $S_{domo}(t) < S_{crit}, \quad S_{crit} \approx 0.5$

*Estabilidade e Reset ($S_{domo}$)

&lt;page_number&gt;p. 48&lt;/page_number&gt;

---


## Page 49

Projeto Pulsação • Caderno de Equações

Eq. (33)
$VR_{\epsilon}(X) = \{\sigma \subseteq X : \max_{x_i, x_j \in \sigma} d(x_i, x_j) \leq \epsilon\}$

Topologia persistente (TODA)

&lt;page_number&gt;p. 49&lt;/page_number&gt;

---


## Page 50

Projeto Pulsação • Caderno de Equações

**Eq. (34)**

$[g_{F}(\theta)]_{ij} = E_{x \sim p(\cdot|\theta)}[\partial_{\theta_{i}}\log p(x|\theta) \partial_{\theta_{j}}\log p(x|\theta)]$

*Geometria da informação (Fisher)*

&lt;page_number&gt;p. 50&lt;/page_number&gt;

---


## Page 51

Projeto Pulsação • Caderno de Equações

Eq. (35)

$\tilde{\nabla}_{\theta}\mathcal{L} = G_{F}(\theta)^{-1}\nabla_{\theta}\mathcal{L}$

Geometria da informação (Fisher)

&lt;page_number&gt;p. 51&lt;/page_number&gt;

---


## Page 52

Projeto Pulsação • Caderno de Equações

**Eq. (36)**

$X_k \equiv \left[ \Re(\psi_k), \Im(\psi_k), N_k, R_{ERC,k}, H_{N,k}, \tilde{B}_{geomag,k} \right]^T, \quad u_k \equiv \left[ u_k^{(corr)}, u_k^{(inj)} \right]^T$

*Espaço de estados mínimo*

&lt;page_number&gt;p. 52&lt;/page_number&gt;

---


## Page 53

Projeto Pulsação • Caderno de Equações

Eq. (37)
$X_{k+1} = F(X_k, u_k) + w_k$

Espaço de estados mínimo

&lt;page_number&gt;p. 53&lt;/page_number&gt;

---


## Page 54

Projeto Pulsação • Caderno de Equações

Eq. (38)

$y_k = h(X_k) + v_k$

Modelo de observação (medidas)

&lt;page_number&gt;p. 54&lt;/page_number&gt;

---


## Page 55

Projeto Pulsação • Caderno de Equações

**Eq. (39)**

$y_k \equiv \left[ PLV_k, R_{ERC,k}, \hat{f}_{0,k}, \hat{\delta}_{ij,k}, features\ TDA_k \right]^T$

*Modelo de observação (medidas)*

&lt;page_number&gt;p. 55&lt;/page_number&gt;

---


## Page 56

Projeto Pulsação • Caderno de Equações

Eq. (40)

$S_{\text{domo},k} = S(R_{ERC,k}, \tilde{B}_{geomag,k}, H_{N,k})$

Modelo de observação (medidas)

&lt;page_number&gt;p. 56&lt;/page_number&gt;

---


## Page 57

Projeto Pulsação • Caderno de Equações

Eq. (41)

$S_{domo,k} \geq S_{crit}, S_{crit} \approx 0.5$

Modelo de observação (medidas)

&lt;page_number&gt;p. 57&lt;/page_number&gt;

---


## Page 58

Projeto Pulsação • Caderno de Equações

Eq. (42)

$A_k = \frac{\partial F}{\partial X} |_{\hat{X}_k, u_k}, B_k = \frac{\partial F}{\partial u} |_{\hat{X}_k, u_k}, C_k = \frac{\partial h}{\partial X} |_{\hat{X}_k}$

Estimação: Kalman (EKF/UKF)

&lt;page_number&gt;p. 58&lt;/page_number&gt;

---


## Page 59

Projeto Pulsação • Caderno de Equações

Eq. (43)

$\hat{X}_{k+1}^{-} = F(\hat{X}_k, u_k), \quad P_{k+1}^{-} = A_kP_kA_k^T + Q_k$

Estimação: Kalman (EKF/UKF)

&lt;page_number&gt;p. 59&lt;/page_number&gt;

---


## Page 60

Projeto Pulsação • Caderno de Equações

**Eq. (44)**

$\boldsymbol{K}_{k+1} = \boldsymbol{P}_{k+1}^{-} \boldsymbol{C}_{k+1}^{\text{T}} \left( \boldsymbol{C}_{k+1} \boldsymbol{P}_{k+1}^{-} \boldsymbol{C}_{k+1}^{\text{T}} + \boldsymbol{R}_{k+1} \right)^{-1}$

*Estimação: Kalman (EKF/UKF)*

&lt;page_number&gt;p. 60&lt;/page_number&gt;

---


## Page 61

Projeto Pulsação • Caderno de Equações

**Eq. (45)**

$\hat{X}_{k+1} = \hat{X}_{k+1}^{-} + K_{k+1}\left(y_{k+1} - h(\hat{X}_{k+1}^{-})\right), P_{k+1} = (I - K_{k+1}C_{k+1})P_{k+1}^{-}$

*Estimação: Kalman (EKF/UKF)*

&lt;page_number&gt;p. 61&lt;/page_number&gt;

---


## Page 62

Projeto Pulsação • Caderno de Equações

**Eq. (46)**

$\min_{\{u_k, \ldots, u_{k+T_h-1}\}} J_k = \sum_{\ell=0}^{T_h - 1} (\|r(X_{k+\ell})\|_{W_r}^2 + \|u_{k+\ell}\|_{W_u}^2) + \|r_T(X_{k+T_h})\|_{W_T}^2$

*Controle preditivo (MPC) com restrição S_domo*

&lt;page_number&gt;p. 62&lt;/page_number&gt;

---


## Page 63

Projeto Pulsação • Caderno de Equações

**Eq. (47)**
$$X_{k+\ell+1} = F(X_{k+\ell}, u_{k+\ell}) \quad (\ell = 0, ..., T_h - 1)$$

*Controle preditivo (MPC) com restrição S\_domo*

&lt;page_number&gt;p. 63&lt;/page_number&gt;

---


## Page 64

Projeto Pulsação • Caderno de Equações

**Eq. (48)**
$S_{domo,k+\ell} \geq S_{crit} \approx 0.5 \quad (\ell = 0, ..., T_h)$

*Controle preditivo (MPC) com restrição S\_domo*

&lt;page_number&gt;p. 64&lt;/page_number&gt;

---


## Page 65

Projeto Pulsação • Caderno de Equações

**Eq. (49)**

$r(X) \equiv [ 1 - R_{ERC}, \max(0, S_{crit} - S_{domo}), \text{penalidade TDA}(D_k) ]^T$

*Controle preditivo (MPC) com restrição S\domo*

&lt;page_number&gt;p. 65&lt;/page_number&gt;

---


## Page 66

Projeto Pulsação • Caderno de Equações

Eq. (50)

$\min_{u, z} f(u) + g(z)$ s. a. $Au + Bz = C$

*Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural*

&lt;page_number&gt;p. 66&lt;/page_number&gt;

---


## Page 67

Projeto Pulsação • Caderno de Equações

**Eq. (51)**

$\boldsymbol{u}^{(n+1)} = \underset{\boldsymbol{u}}{\arg\operatorname*{min}} \left(f(\boldsymbol{u}) + \frac{\rho}{2}\|\boldsymbol{A}\boldsymbol{u} + \boldsymbol{B}\boldsymbol{z}^{(n)} - \boldsymbol{c} + \boldsymbol{\lambda}^{(n)}\|^2\right)$

*Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural*

&lt;page_number&gt;p. 67&lt;/page_number&gt;

---


## Page 68

Projeto Pulsação • Caderno de Equações

**Eq. (52)**

$z^{(n+1)} = \arg\min_z \left(g(z) + \frac{\rho}{2} \|Au^{(n+1)} + Bz - c + \lambda^{(n)}\|^2\right)$

*Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural*

&lt;page_number&gt;p. 68&lt;/page_number&gt;

---


## Page 69

Projeto Pulsação • Caderno de Equações

Eq. (53)
$\lambda^{(n+1)} = \lambda^{(n)} + A\mathcal{U}^{(n+1)} + B\mathcal{Z}^{(n+1)} - C$

*Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural*

&lt;page_number&gt;p. 69&lt;/page_number&gt;

---


## Page 70

Projeto Pulsação • Caderno de Equações

Eq. (54)

$\theta \leftarrow \theta - \eta g_{F}(\theta)^{-1}\nabla_{\theta}J_{k}$

Solução numérica do MPC (ADMM/Tikhonov) e gradiente natural

&lt;page_number&gt;p. 70&lt;/page_number&gt;

---


## Page 71

Projeto Pulsação • Caderno de Equações

Eq. (55)

$\mathcal{F}\{(-\nabla^{2})^{\eta/2}\psi\}(k)=|k|^{\eta}\hat{\psi}(k), \eta=0.6$

Discretização explícita (1D/2D) — (A) Malha, números de onda e operador fracionário

&lt;page_number&gt;p. 71&lt;/page_number&gt;

---


## Page 72

Projeto Pulsação • Caderno de Equações

**Eq. (56)**

$i\hbar\partial_{t}\psi=\left[-\frac{\hbar^{2}}{2m_{a}}(-\nabla^{2})^{\eta/2}+V_{\text{ext}}(x)+V_{\text{domo}}(x)+g(x,t)|\psi|^{2}\right]\psi+i(C-R_{T})\psi$

Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

&lt;page_number&gt;p. 72&lt;/page_number&gt;

---


## Page 73

Projeto Pulsação • Caderno de Equações

**Eq. (57)**

$V_{\text{NL}}(x,t) \equiv V_{\text{ext}}(x) + V_{\text{domo}}(x) + g(x,t)|\psi(x,t)|^2$

*Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step*

&lt;page_number&gt;p. 73&lt;/page_number&gt;

---


## Page 74

Projeto Pulsação • Caderno de Equações

**Eq. (58)**

$\psi^{n+\frac{1}{2}}(x) = \exp\left[-\frac{i}{\hbar}V_{NL}(x,t_n)\frac{\Delta t}{2}\right]\exp\left[(C-R_T)\frac{\Delta t}{2}\right]\psi^n(x)$

Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

&lt;page_number&gt;p. 74&lt;/page_number&gt;

---


## Page 75

Projeto Pulsação • Caderno de Equações

Eq. (59)

$\hat{\psi}^{n+\frac{1}{2}}(k) \leftarrow \exp[-\frac{i}{\hbar}\left(\frac{\hbar^2}{2m_a}|k|^{\eta}\right)\Delta t]\hat{\psi}^{n+\frac{1}{2}}(k)$

Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

&lt;page_number&gt;p. 75&lt;/page_number&gt;

---


## Page 76

Projeto Pulsação • Caderno de Equações

**Eq. (60)**

$\psi^{n+1}(x) = \exp\left[-\frac{i}{\hbar}V_{NL}(x,t_{n+1})\frac{\Delta t}{2}\right]\exp\left[(C-R_T)\frac{\Delta t}{2}\right]\psi^{n+\frac{1}{2}}(x)$

Discretização explícita (1D/2D) — (B) GPE-XC (forma operacional) e passo split-step

&lt;page_number&gt;p. 76&lt;/page_number&gt;

---


## Page 77

Projeto Pulsação • Caderno de Equações

**Eq. (61)**

$\frac{N^{n+1} - N^n}{\Delta t} = \frac{D_N}{\tau_N}\nabla^2N^{n+1} - \frac{\gamma_N}{\tau_N}N^{n+1} + \frac{1}{\tau_N}S^n + \frac{1}{\tau_N}\mathcal{M}^n$

*Discretização explícita (1D/2D) — (C) Atualização de $$N(x,t)$$ no mesmo loop*

&lt;page_number&gt;p. 77&lt;/page_number&gt;

---


## Page 78

Projeto Pulsação • Caderno de Equações

**Eq. (62)**

$X_k = \{x_i = (\cos\phi_i,\text{corr}(t),\sin\phi_i,\text{corr}(t)) : i=1,...,M, t\in W_k\}$

*Penalidade TDA completa (persistência → escalar) — (A) Construção da nuvem de pontos*

&lt;page_number&gt;p. 78&lt;/page_number&gt;

---


## Page 79

Projeto Pulsação • Caderno de Equações

Eq. (63)

$P_{\text{TDA}}(k) = \sum_{(b_j, d_j) \in D_{1,k}} w(b_j, d_j) \rho(d_j - b_j)$

Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”

&lt;page_number&gt;p. 79&lt;/page_number&gt;

---


## Page 80

Projeto Pulsação • Caderno de Equações

Eq. (64)

$P_B(k) = d_B(D_{1,k}, D_{1,ref})$

Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”

&lt;page_number&gt;p. 80&lt;/page_number&gt;

---


## Page 81

Projeto Pulsação • Caderno de Equações

Eq. (65)

**penalidadeTDA(k) = λ₁ P<sub>TDA</sub>(k) + λ₂ P<sub>B</sub>(k)**

*Penalidade TDA completa (persistência → escalar) — (B) Diagrama e escalar de “integridade”*

&lt;page_number&gt;p. 81&lt;/page_number&gt;

---


## Page 82

Projeto Pulsação • Caderno de Equações

Eq. (66)

$x_i(t) = A_i(t)\cos(\phi_0(t - \delta_i) + \epsilon_i(t)) + \nu_i(t)$

Benchmark sintético reproduzível (EKF/UKF + MPC) — (A) Gerador de dados (sinais por sensor)

&lt;page_number&gt;p. 82&lt;/page_number&gt;

---


## Page 83

Projeto Pulsação • Caderno de Equações

Eq. (67)

$S_{\text{domo}}(t) \geq 0.5 \forall t$ (segurança)

Benchmark sintético reproduzível (EKF/UKF + MPC) — (C) Construção de $$S_{\text{\textit{domo}}}$$ e regra de sucesso

&lt;page_number&gt;p. 83&lt;/page_number&gt;

---


## Page 84

Projeto Pulsação • Caderno de Equações

Eq. (68)

$S_{\text{domo}}(t) = \sigma(a R_{ERC}(t) + b \tilde{B}_{geomag}(t) - c k_{sem} \tilde{H}_N(t))$

Funções “prontas” (definições fechadas) — 1) Índice S\_domo (forma fechada)

&lt;page_number&gt;p. 84&lt;/page_number&gt;

---


## Page 85

Projeto Pulsação • Caderno de Equações

Eq. (69)

$P_{\text{TDA}}(k) = \sum_{(b_j,d_j) \in D_{1,k}} \max(0, (d_j - b_j) - \ell_0)^2$

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

&lt;page_number&gt;p. 85&lt;/page_number&gt;

---


## Page 86

Projeto Pulsação • Caderno de Equações

Eq. (70)

$P_B(k) = d_B(D_{1,k}, D_{1,ref})$

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

&lt;page_number&gt;p. 86&lt;/page_number&gt;

---


## Page 87

Projeto Pulsação • Caderno de Equações

Eq. (71)

**penalidadeTDA(k) = λ₁P<sub>TDA</sub>(k) + λ₂P<sub>B</sub>(k)**

Funções “prontas” (definições fechadas) — 2) Penalidade TDA (default)

&lt;page_number&gt;p. 87&lt;/page_number&gt;

---


## Page 88

Projeto Pulsação • Caderno de Equações

Eq. (72)

$x_i(t) = A_i(t)\cos(2\pi f_0(t - \delta_i) + \epsilon_i(t)) + \nu_i(t)$

Benchmark sintético (especificação completa) — 2) Sinais gerados

&lt;page_number&gt;p. 88&lt;/page_number&gt;

---


## Page 89

Projeto Pulsação • Caderno de Equações

# Apêndice B — Extras + referências

Extras (Caputo, SOE, TDA, Heartbeat).

## Extra A1

$\ddot{x} - \mu(1-x^2)\dot{x} + x = 0$

Heartbeat Loop (oscilador de ciclo-limite)

## Extra A2

$C\mathcal{D}_t^\alpha f(t) = \frac{1}{\Gamma(1-\alpha)} \int_0^t \frac{f'(\tau)}{(t-\tau)^\alpha} d\tau$

Derivada fracionária (Caputo), ordem $0<\alpha<1$

## Extra A3

$C\mathcal{D}_t^\alpha f(t_n) \approx \frac{1}{\Gamma(2-\alpha)\Delta t^\alpha} \sum_{k=0}^{n-1} [(k+1)^{1-\alpha} - k^{1-\alpha}] (f_{n-k} - f_{n-k-1})$

Discretização L1 (Caputo) em malha uniforme

## Extra A4

$\phi_k^{n+1} = e^{-\lambda_k\Delta t}\phi_k^n + \frac{1-e^{-\lambda_k\Delta t}}{\lambda_k}(f_{n+1}-f_n)$

SOE: atualização recursiva dos auxiliares $\phi_k$ (Alg. 4.1)

## Extra A5

$C\mathcal{D}_t^\alpha f(t_{n+1}) \approx \sum_{k=1}^{N_{\text{exp}}} w_k \phi_k^{n+1}$

SOE: soma ponderada para aproximar $D^\alpha f$

&lt;page_number&gt;p. 89&lt;/page_number&gt;

---


## Page 90

Projeto Pulsação • Caderno de Equações

**Extra A6**
$$d_B(D_1,D_2) = \inf_{\gamma} \sup_{p \in D_1} ||p - \gamma(p)||_\infty$$
Distância de gargalo (bottleneck) em diagramas de persistência

**Extra A7**
$$g_F(\theta) = E_{x \sim p(\cdot|\theta)}[\nabla_\theta log p(x|\theta) \nabla_\theta log p(x|\theta)^T]$$
Métrica de Fisher (forma matricial)

**Referências internas (fontes do projeto)**

*   TURR: “Teoria Universal da Realidade” (docx) — núcleo do kernel Tesla-BEC, E-R-C, TDA e controle.
*   TMC: “Technical Mathematics Compendium” (pdf) — cálculo fracionário e SOE (Algoritmo 4.1).
*   HB: “O Heartbeat Loop: A Assinatura Universal da Vida” (pdf) — oscilador de Van der Pol e ciclo-limite.

&lt;page_number&gt;p. 90&lt;/page_number&gt;

---


## Page 91

Projeto Pulsação • Caderno de Equações

# Apêndice C — Extras avançados

Mais fórmulas (derivações + atalhos) para o sistema completo.
Organizado por blocos: Kernel/Hidro, Fracionário/SOE, E-R-C, Controle, TDA, Heartbeat, Relativístico e Lógica modal.

## Blocos incluídos

*   C1–C10 Kernel Tesla-BEC (energia, vórtices, dispersão)
*   C11–C18 Memória fracionária (RL/GL/Laplace) + esquemas
*   C19–C27 Observáveis E-R-C (Hilbert, espectro, holonomia)
*   C28–C39 Estimação/Controle (EKF, UKF, MPC, ADMM, Fisher)
*   C40–C46 TDA (VR, fronteira, Betti, distâncias, entropia)
*   C47–C53 Heartbeat Loop + relativístico (NKG, ação, tensor Tμν)
*   C54–C60 Lógica modal (Gödel/Scott) + partição

&lt;page_number&gt;p. 91&lt;/page_number&gt;

---


## Page 92

Projeto Pulsação • Caderno de Equações

**Extra C1**

$E[\psi] = \int_{\Omega} \left( \frac{\hbar^2}{2m_a} |\nabla \psi|^2 + V|\psi|^2 + \frac{g}{2}|\psi|^4 \right) d^d x$

*Funcional de energia da GPE (forma padrão).*

**Extra C2**

$\mu \psi = \left( -\frac{\hbar^2}{2m_a} \nabla^2 + V + g|\psi|^2 \right) \psi$

*Equação estacionária (definição operacional do potencial químico).*

**Extra C3**

$\nabla \times v = \frac{2\pi\hbar}{m_a} \sum_{\ell} n_\ell \delta^{(2)}(x-x_\ell) \hat{z}$

*Vorticidade concentrada em núcleos (vórtices quantizados).*

**Extra C4**

$\nabla Q = -\frac{\hbar^2}{4m_a} \nabla \left( \frac{\nabla^2\rho}{\rho} - \frac{1}{2}\frac{|\nabla\rho|^2}{\rho^2} \right)$

*Gradiente do potencial quântico em termos de rho (identidade útil).*

&lt;page_number&gt;p. 92&lt;/page_number&gt;

---


## Page 93

Projeto Pulsação • Caderno de Equações

**Extra C5**

$\omega^2(k) = c_s^2k^2 + \frac{\hbar^2k^4}{4m_a^2}$

Dispersão de Bogoliubov (caso Laplaciano clássico).

**Extra C6**

$\omega^2(k) = c_s^2|k|^{\eta} + \frac{\hbar^2|k|^{2\eta}}{4m_a^2}$

Dispersão tipo Bogoliubov (heurística para Laplaciano fracionário).

**Extra C7**

$\xi = \frac{\hbar}{\sqrt{2m_ag\rho_0}}, \quad t_{\xi} = \frac{m_a\xi^2}{\hbar}$

Escalas naturais: comprimento de cura ($\xi$) e tempo associado.

**Extra C8**

$\psi(r,\theta) = \sqrt{\rho(r)}e^{in\theta}, \quad n \in \mathbb{Z}$

Ansatz polar para vórtice (2D).

&lt;page_number&gt;p. 93&lt;/page_number&gt;

---


## Page 94

Projeto Pulsação • Caderno de Equações

**Extra C9**

$v_\theta(r) = \frac{n\hbar}{m_a r}, \quad \Gamma = \oint v \cdot dl = \frac{2\pi\hbar}{m_a} n$

Velocidade azimutal e circulação quantizada.

**Extra C10**

$E_v \simeq \pi\rho_0\frac{\hbar^2}{m_a}n^2\ln(\frac{R}{\xi})$

Energia logarítmica de vórtice (estimativa assintótica).

**Extra C11**

$_0D_t^\alpha f(t) = \frac{1}{\Gamma(1-\alpha)}\frac{d}{dt}\int_0^t \frac{f(\tau)}{(t-\tau)^\alpha} d\tau, \quad 0<\alpha<1$

Derivada fracionária de Riemann–Liouville ($0<\alpha<1$).

**Extra C12**

$C D_t^\alpha f(t) = _0D_t^\alpha(f(t)-f(0))$

Relação Caputo $\leftrightarrow$ Riemann–Liouville (para $0<\alpha<1$).

**Extra C13**

$D_t^\alpha f(t_n) \approx \Delta t^{-\alpha}\sum_{k=0}^n (-1)^k \binom{\alpha}{k} f_{n-k}, \quad \binom{\alpha}{k} = \frac{\Gamma(\alpha+1)}{\Gamma(k+1)\Gamma(\alpha-k+1)}$

Aproximação de Grünwald–Letnikov (diferenças fracionárias).

&lt;page_number&gt;p. 94&lt;/page_number&gt;

---


## Page 95

Projeto Pulsação • Caderno de Equações

**Extra C14**

$\mathcal{L}\{^C D_t^\alpha f\}(s) = s^\alpha F(s) - s^{\alpha-1}f(0)$

*Transformada de Laplace do operador de Caputo (0<α<1).*

**Extra C15**

$t^{-\alpha} \approx \sum_{k=1}^{N_{exp}} w_k e^{-\lambda_k t}, \quad w_k > 0, \lambda_k > 0$

*SOE: aproximação soma-de-exponenciais do kernel de memória.*

**Extra C16**

$\phi_k^{n+1} = e^{-\lambda_k \Delta t} \phi_k^n + \frac{1-e^{-\lambda_k \Delta t}}{\lambda_k} (f_{n+1}-f_n)$

*SOE: atualização recursiva do auxiliar phi_k (forma padrão).*

**Extra C17**

$^C D_t^\alpha f(t_n) \approx \Delta t^{-\alpha} \sum_{j=0}^{n} \omega_j^{(\alpha)} f_{n-j}$

*Convolution quadrature (forma genérica; pesos via função geradora).*

**Extra C18**

$^C D_t^\alpha f(t_{n+\sigma}) \approx \frac{1}{\Gamma(2-\alpha)\Delta t^\alpha} \sum_{k=0}^{n} a_{n-k}^{(\alpha,\sigma)} (f_{k+1}-f_k), \quad \sigma = 1-\frac{\alpha}{2}$

*Esquema L2-1σ (compacto) para Caputo: melhor ordem e estabilidade.*

&lt;page_number&gt;p. 95&lt;/page_number&gt;

---


## Page 96

Projeto Pulsação • Caderno de Equações

**Extra C19**

$\mathcal{H}[x](t) = \frac{1}{\pi} \text{p.v.} \int_{-\infty}^{\infty} \frac{x(\tau)}{t - \tau} d\tau$

*Transformada de Hilbert (definição integral, valor principal).*

**Extra C20**

$f_i(t) = \frac{1}{2\pi} \frac{d\phi_i(t)}{dt}, \quad \phi_i(t) = \arg \left( x_i(t) + i\mathcal{H}[x_i](t) \right)$

*Frequência instantânea a partir da fase analítica.*

**Extra C21**

$C_{ij}(f) = \frac{|S_{ij}(f)|^2}{S_{ii}(f) S_{jj}(f)} \in [0, 1]$

*Coerência espectral (magnitude squared coherence).*

**Extra C22**

$S_{ij}(f) = X_i(f) X_j^*(f), \quad X_i(f) = \mathcal{F}\{x_i(t)\}$

*Espectro cruzado (definição operacional).*

**Extra C23**

$\Phi_{ABC}(f) = \arg \left( S_{AB}(f) S_{BC}(f) S_{CA}(f) \right)$

*Closure phase (análogo interferométrico / holonomia triangular).*

&lt;page_number&gt;p. 96&lt;/page_number&gt;

---


## Page 97

Projeto Pulsação • Caderno de Equações

**Extra C24**

$\Phi_{\mathcal{C}}(t) = \sum_{(i,j)\in\mathcal{C}} (\phi_{ij}(t) - 2\pi f_0 \delta_{ij})$

Holonomia em um ciclo $C$ (generalização para grafos).

**Extra C25**

$R_{ERC}(\mathcal{C}) = \left|\left\langle e^{i\Phi_{\mathcal{C}}(t)} \right\rangle_t\right|$

Fechamento holonômico (concentração circular) para qualquer ciclo.

**Extra C26**

$d = 2R_E \arcsin\left(\sqrt{\sin^2\frac{\Delta\varphi}{2} + \cos\varphi_1 \cos\varphi_2 \sin^2\frac{\Delta\lambda}{2}}\right)$

Distância geodésica (Haversine) em esfera de raio $R_E$.

**Extra C27**

$\delta_{ij} = \frac{d_{ij}}{C_{eff}}, \quad \phi_{\text{corr}} = \phi - 2\pi f_0 \delta_{ij}$

Atraso geodésico e correção de fase (passo E-R-C).

**Extra C28**

$x_{k+1} = f(x_k, u_k) + w_k, \quad w_k \sim \mathcal{N}(0, Q_k)$

Modelo de processo (estado) com ruído gaussiano.

&lt;page_number&gt;p. 97&lt;/page_number&gt;

---


## Page 98

Projeto Pulsação • Caderno de Equações

**Extra C29**

$y_k = h(x_k) + v_k, \quad v_k \sim \mathcal{N}(0, R_k)$

*Modelo de medição com ruído gaussiano.*

**Extra C30**

$\hat{x}_{k|k-1} = f(\hat{x}_{k-1|k-1}, u_{k-1})$

*EKF: predição do estado (time update).*

**Extra C31**

$P_{k|k-1} = F_k P_{k-1|k-1} F_k^T + Q_k, \quad F_k = \frac{\partial f}{\partial x}\bigg|_{\hat{x}_{k-1|k-1}}$

*EKF: predição de covariância (linearização via Jacobiano F_k).*

**Extra C32**

$K_k = P_{k|k-1} H_k^T \left(H_k P_{k|k-1} H_k^T + R_k\right)^{-1}, \quad H_k = \frac{\partial h}{\partial x}\bigg|_{\hat{x}_{k|k-1}}$

*EKF: ganho de Kalman (measurement update).*

**Extra C33**

$\hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k \left(y_k - h(\hat{x}_{k|k-1})\right)$

*EKF: correção por inovação.*

**Extra C34**

$\chi_0 = \hat{x}, \quad \chi_i = \hat{x} + \left(\sqrt{(n+\lambda)P}\right)_i, \quad \chi_{i+n} = \hat{x} - \left(\sqrt{(n+\lambda)P}\right)_i$

$i = 1, \ldots, n$

*UKF: pontos sigma (construção simétrica).*

&lt;page_number&gt;p. 98&lt;/page_number&gt;

---


## Page 99

Projeto Pulsação • Caderno de Equações

**Extra C35**

$W_0^{(m)} = \frac{\lambda}{n + \lambda}, \quad W_0^{(c)} = \frac{\lambda}{n + \lambda} + (1 - \alpha^2 + \beta)$

$W_i^{(m)} = W_i^{(c)} = \frac{1}{2(n + \lambda)}, \quad i = 1, \ldots, 2n$

UKF: pesos (média/covariância).

**Extra C36**

$\min_{\{u_t\}} J = \sum_{t=0}^{T-1} \left(\|y_t - y^\star\|_Q^2 + \|u_t\|_R^2\right) + \|x_T - x^\star\|_{Q_f}^2$

S.a. $x_{t+1} = f(x_t, u_t), \quad x_t \in \mathcal{X}, \quad u_t \in \mathcal{U}, \quad S_{\text{domo}}(t) \geq S_{\text{min}}$

MPC: custo + restrições + segurança do domo.

**Extra C37**

$\min_{x, z} f(x) + g(z) \quad \text{s.a.} \quad Ax + Bz = c$

$x^{k+1} = \arg\min_x \left(f(x) + \frac{\rho}{2}\|Ax + Bz^k - c + u^k\|_2^2\right)$

$z^{k+1} = \arg\min_z \left(g(z) + \frac{\rho}{2}\|Ax^{k+1} + Bz - c + u^k\|_2^2\right)$

$u^{k+1} = u^k + Ax^{k+1} + Bz^{k+1} - c$

ADMM: split canônico (cola otimização não suave no MPC).

**Extra C38**

$\nabla_x \mathcal{L}(x, \lambda) = 0, \quad g(x) \leq 0, \quad \lambda \geq 0, \quad \lambda \odot g(x) = 0$

Condições KKT (Lagrangiano, viabilidade, complementaridade).

&lt;page_number&gt;p. 99&lt;/page_number&gt;

---


## Page 100

Projeto Pulsação • Caderno de Equações

**Extra C39**
θ ← θ − η g_F(θ)^{-1} ∇_θ J(θ), g_F(θ) = E[∇ log p ∇ log p^T]
Natural gradient (métrica de Fisher) para calibração/controle.

**Extra C40**
VR_ε(X) = {σ ⊆ X : d(x_i, x_j) ≤ ε ∀x_i, x_j ∈ σ}
Complexo de Vietoris–Rips (definição).

**Extra C41**
∂_k[v_0, ..., v_k] = ∑_{i=0}^{k} (-1)^i [v_0, ..., v̂_i, ..., v_k]
Operador de fronteira (simplicial).

**Extra C42**
β_k = rank Z_k - rank B_k, Z_k = ker ∂_k, B_k = im ∂_{k+1}
Número de Betti (topologia algébrica).

**Extra C43**
TP_p(D) = ∑_{(b,d)∈D} (d-b)^p
Total persistence (resumo escalar do diagrama).

&lt;page_number&gt;p. 100&lt;/page_number&gt;

---


## Page 101

Projeto Pulsação • Caderno de Equações

**Extra C44**
$$d_B(D_1,D_2) = \operatorname*{inf}_{\gamma} \sup_{p \in D_1} \| p - \gamma(p) \|_\infty$$
Distância bottleneck entre diagramas (gargalo).

**Extra C45**
$$W_p(D_1,D_2) = \left( \operatorname*{inf}_{\gamma} \sum_{p \in D_1} \| p - \gamma(p) \|_\infty^p \right)^{1/p}$$
Distância de Wasserstein (p) entre diagramas.

**Extra C46**
$$H(D) = -\sum_i p_i \log p_i, \quad p_i = \frac{d_i - b_i}{\sum_j (d_j - b_j)}$$
Entropia persistente (normalização por tempos de vida).

**Extra C47**
$$\ddot{x} - \mu(1-x^2)\dot{x} + \omega_0^2x = \epsilon \sin(\omega t)$$
Van der Pol forçado (entrainment).

**Extra C48**
$$\dot{\theta} = \omega + Z(\theta)I(t)$$
Redução de fase (PRC) para osciladores (forma compacta).

&lt;page_number&gt;p. 101&lt;/page_number&gt;

---


## Page 102

Projeto Pulsação • Caderno de Equações

**Extra C49**

$$\dot{\theta}_i = \omega_i + \frac{K}{N} \sum_{j=1}^{N} \sin(\theta_j - \theta_i)$$

*Acoplamento de Kuramoto (sincronização).*

**Extra C50**

$$G_{\mu\nu} + \Lambda g_{\mu\nu} = 8\pi G T_{\mu\nu}$$

*Equações de Einstein (referência estrutural).*

**Extra C51**

$$\Box \phi + m^2 \phi + \lambda \phi^3 = \beta N(x,t)$$

*NKG com termo fonte (acoplamento fenomenológico ao campo N).*

**Extra C52**

$$S = \int \mathcal{L}(\phi, \partial\phi, N, \partial N) d^4x$$

*Ação (formato geral) para fechar por princípio variacional.*

&lt;page_number&gt;p. 102&lt;/page_number&gt;

---


## Page 103

Projeto Pulsação • Caderno de Equações

**Extra C53**
$$T_{\mu\nu} = \partial_\mu \phi \partial_\nu \phi - g_{\mu\nu} L$$
Tensor energia-momento (campo escalar).

**Extra C54**
$$G(x) \equiv \forall \varphi \left( P(\varphi) \rightarrow \varphi(x) \right)$$
Gödel: definição de 'God-like' (forma padrão).

**Extra C55**
$$(P(\varphi) \wedge \Box \forall x (\varphi(x) \rightarrow \psi(x))) \rightarrow P(\psi)$$
Gödel: axioma de herança de positividade (esboço).

**Extra C56**
$$P(\varphi) \rightarrow \Diamond \exists x \varphi(x)$$
Gödel: possibilidade de instância para propriedade positiva.

**Extra C57**
$$\varphi \text{ Ess. } x \equiv \varphi(x) \wedge \forall \psi (\psi(x) \rightarrow \Box \forall y (\varphi(y) \rightarrow \psi(y)))$$
Gödel/Scott: essência (definição típica).

**Extra C58**
$$E(x) \equiv \forall \varphi (\varphi \text{ Ess. } x \rightarrow \Box \exists y \varphi(y))$$
Gödel: existência necessária (definição).

&lt;page_number&gt;p. 103&lt;/page_number&gt;

---


## Page 104

Projeto Pulsação • Caderno de Equações

Extra C59

$\exists x G(x)$

Gödel/Scott: conclusão modal (símbolo-alvo).

Extra C60

$Z = \int D\phi \exp(-\frac{1}{\hbar}S[\phi])$

Integral funcional (partição): linguagem de campo (bônus).

&lt;page_number&gt;p. 104&lt;/page_number&gt;