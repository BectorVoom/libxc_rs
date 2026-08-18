# Adaptive Precision Architecture for High-Accuracy Quantum Chemistry on Commodity GPUs

## 1. Executive Summary

Quantum chemistry software has traditionally relied heavily on IEEE 754 binary64 (`FP64`, or `f64`) arithmetic because electronic structure calculations contain long reductions, strong cancellation, large dynamic ranges, iterative nonlinear procedures, and numerically sensitive linear algebra. This creates a major performance problem on commodity GPUs: modern gaming and workstation GPUs are designed primarily for graphics and AI workloads and therefore provide much higher throughput for FP32, FP16/BF16, FP8, or integer matrix operations than for general-purpose FP64 arithmetic.

The proposed solution is **not** to replace every FP64 operation by FP32, nor to emulate FP64 uniformly throughout the program. Instead, the central design principle is:

> **Use expensive FP64 arithmetic only where the numerical error can materially affect the final observable, and use the fastest safe arithmetic everywhere else.**

The proposed architecture combines four techniques:

1. **Error-aware FP32/FP64 dynamic precision for direct Gaussian integral evaluation.**
2. **A two-component FP32 representation, called double-single or single-single arithmetic, as an intermediate precision tier for recurrence and contraction operations.**
3. **Ozaki-style high-precision GEMM emulation using low-precision matrix engines for large DF/RI tensor contractions.**
4. **SCF- and residual-aware precision escalation, with strict FP64 verification near convergence and at the final result.**

This gives the following conceptual hierarchy:

```text
                 Required numerical reliability
                           increases
                              ↑
                              │
                     Native FP64
                              │
                    Double-single
                              │
                         FP32
                              │
                           Skip
                              │
                              └────────────→ increasing computational cost
```

For matrix-dominated operations, a separate path should be used:

```text
Large dense tensor contraction / GEMM
              │
              ├── Native FP64
              │
              └── Ozaki / INT8 / FP8 / FP16
                  high-precision GEMM emulation
```

The key idea is therefore **precision specialization by numerical role**, rather than precision specialization merely by data type.

There is substantial prior evidence supporting the individual components of this strategy. Dynamic FP32/FP64 ERI evaluation has been demonstrated in Hartree–Fock and DFT calculations; mixed-precision eigensolvers have been demonstrated for electronic-structure calculations; error-free matrix multiplication techniques can reconstruct high-precision matrix products from lower-precision GEMMs; and a 2026 adaptive-precision density-fitting implementation reported overall DFT speedups up to 3.04× on an RTX 4090 while keeping converged energies within (10^{-7}) of its FP64 reference.

---

# 2. Problem Definition

The objective is:

> **Accelerate quantum chemistry calculations that conventionally require FP64 on GPUs with weak native FP64 throughput, without sacrificing the numerical accuracy required by the target calculation.**

The target workload includes, in particular:

* one-electron Gaussian integrals;
* two-electron repulsion integrals;
* three-center and two-center density-fitting integrals;
* recurrence relations used by Rys quadrature, McMurchie–Davidson, Obara–Saika, or related algorithms;
* integral contraction;
* Coulomb and exchange matrix construction;
* density fitting / resolution-of-the-identity tensor contractions;
* SCF iterations;
* generalized eigenvalue problems;
* density-matrix construction;
* reductions and energy evaluation.

A fundamental distinction must be made between two accuracy requirements.

### Scientific accuracy

For many production calculations, the relevant target may be approximately

[
|E-E_{\mathrm{reference}}| < 10^{-6}\ {\rm Hartree}
]

or another physically justified observable-level tolerance.

### Implementation parity

A library implementation may instead require element-wise agreement such as

[
|x_{\mathrm{GPU}}-x_{\mathrm{reference}}| < 10^{-12}.
]

The second requirement is substantially harder. A design that produces chemically accurate total energies is not automatically suitable for strict integral-by-integral parity testing.

The precision controller must therefore take the **requested numerical contract** as an explicit input.

---

# 3. Why Uniform FP32 Is Insufficient

IEEE FP32 provides 24 bits of significand precision, whereas FP64 provides 53 bits.

Approximately,

[
u_{32}=2^{-24},
]

while

[
u_{64}=2^{-53},
]

where (u) denotes unit roundoff.

For a simple local operation, FP32 may be completely adequate. The difficulty arises when errors are amplified by:

* cancellation;
* long recurrence chains;
* accumulation of thousands or millions of contributions;
* diffuse Gaussian functions;
* high angular momentum;
* near-linear dependence;
* poorly conditioned overlap matrices;
* eigensolver residual propagation;
* nonlinear SCF feedback.

Consequently,

```text
all FP64 → FP32
```

is not an acceptable general solution.

At the opposite extreme,

```text
all FP64 → software-emulated FP64
```

is also inefficient because many operations never require 53 significand bits.

The correct problem is therefore an **error-allocation problem**.

---

# 4. Proposed Architecture

The complete architecture contains five numerical layers.

```text
┌─────────────────────────────────────────────┐
│ Layer 5: Final validation / observables     │
│ FP64                                        │
├─────────────────────────────────────────────┤
│ Layer 4: Sensitive residuals and reductions │
│ FP64 / compensated arithmetic              │
├─────────────────────────────────────────────┤
│ Layer 3: Difficult local calculations       │
│ double-single or FP64                       │
├─────────────────────────────────────────────┤
│ Layer 2: Bulk local calculations            │
│ FP32                                        │
├─────────────────────────────────────────────┤
│ Layer 1: Provably insignificant terms       │
│ screened out                                │
└─────────────────────────────────────────────┘
```

Large matrix operations use an orthogonal decision:

```text
                         GEMM
                           │
              ┌────────────┴────────────┐
              │                         │
         Native FP64           Emulated precision
                                      │
                         FP32 / FP16 / FP8 / INT8
                                      │
                           Ozaki reconstruction
```

A runtime component called the **Precision Controller** decides which path is appropriate.

---

# 5. The Precision Controller

## 5.1 Inputs

For each operation, the controller should consider some or all of:

[
B
=

\text{mathematical contribution bound},
]

[
R
=

\text{current SCF residual},
]

[
\Delta E
========

E_k-E_{k-1},
]

[
\Delta D
========

|D_k-D_{k-1}|,
]

[
L
=

\text{recurrence or accumulation length},
]

[
X
=

\text{dynamic/exponent range},
]

and the requested output tolerance

[
\varepsilon_{\mathrm{target}}.
]

It should also know hardware-specific performance characteristics:

```text
FP32 throughput
FP64 throughput
FP16/BF16/FP8/INT8 matrix throughput
memory bandwidth
subgroup/warp width
available FMA semantics
```

The final dispatch becomes conceptually

[
P=f(B,R,\Delta E,\Delta D,L,X,\varepsilon_{\mathrm{target}},H),
]

where (H) describes the hardware.

---

# 6. Direct Gaussian Integral Evaluation

Direct Gaussian integral kernels are usually composed of many relatively small recurrence calculations rather than a few large GEMMs. This matters because small matrix-vector operations and irregular ERI tensor blocks generally do not make effective use of large Tensor Core matrix operations. The recent adaptive density-fitting study makes this distinction explicitly: direct ERI contractions are generally too small and irregular, while density-fitting contractions produce much larger GEMMs suitable for matrix accelerators.

Therefore the direct integral path should use:

```text
FP32
   ↓
double-single
   ↓
FP64
```

rather than forcing the calculation through Tensor Core GEMM.

---

# 7. Tier 1: Screening

The cheapest operation is one that is never performed.

For a shell quartet,

[
(\mu\nu|\lambda\sigma),
]

a Schwarz-type upper bound gives

[
|(\mu\nu|\lambda\sigma)|
\le
\sqrt{(\mu\nu|\mu\nu)}
\sqrt{(\lambda\sigma|\lambda\sigma)}.
]

However, precision selection should preferably use an estimate of the **effect on the destination quantity**, not merely the magnitude of the integral.

For a Coulomb-like contribution,

[
\Delta J_{\mu\nu}
=================

(\mu\nu|\lambda\sigma)D_{\lambda\sigma},
]

one may construct a conservative estimate

[
C_J
===

B_{\mu\nu\lambda\sigma}
\max_{\lambda\sigma}|D_{\lambda\sigma}|.
]

Analogous exchange-oriented estimates can be constructed for the (K) build.

The controller can then use:

```text
if contribution_bound < epsilon_skip:
    skip
```

This creates an important link between **integral screening and precision selection**: a term slightly above the screening threshold does not necessarily deserve FP64.

---

# 8. Tier 2: FP32 Integral Evaluation

If

[
C < T_{\mathrm{FP32}},
]

the entire primitive or contracted contribution can be evaluated in FP32.

A practical FP32 path can include:

* coordinate differences;
* many recurrence intermediates;
* polynomial terms;
* primitive contraction terms;
* local temporary arrays.

However, sensitive scalar quantities need not follow the same precision.

For example:

```text
Gaussian exponent processing     FP64 or scaled FP32
special-function initialization  FP64
bulk recurrence                  FP32
local accumulation               compensated FP32 or higher
```

This heterogeneous treatment is preferable to requiring a single scalar type for the entire kernel.

Dynamic selection between single and double precision ERI evaluation was already demonstrated by Luehr, Ufimtsev, and Martínez. Their method showed that precision error could be controlled by computing only the largest integrals in double precision.

That result motivates the first level of the proposed controller.

---

# 9. Tier 3: Double-Single Arithmetic

There is a large numerical gap between FP32 and FP64.

Using native FP64 for every operation that fails an FP32 test wastes computational capacity.

An intermediate representation can be defined as

```text
struct DS {
    hi: f32,
    lo: f32,
}
```

with

[
x \approx x_{\mathrm{hi}}+x_{\mathrm{lo}}.
]

This is sometimes called **double-single**, **single-single**, or a two-word floating-point representation.

A carefully implemented two-component FP32 representation can provide roughly twice the significand information of FP32, although it retains an FP32-like exponent range. Extended-precision GPU arithmetic based on paired floating-point values has been studied previously.

## 9.1 Addition

An error-free transformation such as `TwoSum` decomposes an addition into a rounded result and its rounding residual:

[
s=\operatorname{fl}(a+b),
]

[
a+b=s+e.
]

The pair ((s,e)) can then be renormalized.

## 9.2 Multiplication

With an IEEE-compatible fused multiply-add,

[
p=\operatorname{fl}(ab),
]

and

[
e=\operatorname{fma}(a,b,-p)
]

captures the product residual under the appropriate floating-point assumptions.

Error-free transformations for summation and dot products were developed extensively by Ogita, Rump, and Oishi and provide the mathematical foundation for compensated and multiword arithmetic.

## 9.3 Intended role

Double-single should **not** be treated as a universal replacement for FP64.

It is most useful for:

* recurrence intermediates;
* contracted intermediate quantities;
* locally sensitive polynomial expressions;
* moderately ill-conditioned shell quartets;
* accumulation where FP32 is insufficient but full FP64 is unnecessarily expensive.

It should not automatically replace FP64 for:

* final energies;
* final residual computation;
* badly conditioned linear algebra;
* critical transcendental evaluations;
* extreme exponent ranges;
* strict validation fallback.

---

# 10. Dynamic-Range Protection

Two-component FP32 arithmetic improves significand precision but does not reproduce the full exponent range of FP64.

This is particularly relevant to Gaussian basis calculations because quantities may contain factors such as

[
e^{-\alpha r^2},
]

where (\alpha) and (r) can generate a very wide dynamic range.

Therefore the implementation should optionally represent sensitive quantities as

[
x=m,2^e
]

with a normalized mantissa (m) and a separately tracked exponent (e).

Conceptually:

```text
ScaledDS {
    mantissa: DS,
    exponent: i32
}
```

The controller should detect:

* exponent spread;
* subnormal risk;
* overflow risk;
* strong cancellation.

If any exceeds the calibrated DS operating region,

```text
DS → FP64 fallback
```

should occur automatically.

---

# 11. Sensitive Functions: Boys and Rys Initialization

Special functions should be treated more conservatively than ordinary recurrence arithmetic.

For example, the Boys function

[
F_n(T)
======

\int_0^1 t^{2n}e^{-Tt^2},dt
]

can appear in sensitive regions involving:

* small (T);
* large (T);
* high order (n);
* recurrence transitions.

Likewise, Rys roots and weights form the numerical foundation for all subsequent recurrence calculations.

The recommended design is therefore:

```text
roots / weights / critical seeds
            ↓
       FP64 by default
            ↓
bulk recurrence
    FP32 / DS adaptive
```

This avoids contaminating the entire recurrence tree with inaccurate seeds while retaining the benefit of low-precision bulk arithmetic.

An alternative is to provide separately validated FP32 and DS approximations for restricted numerical regions, but they should be introduced only after exhaustive error testing.

---

# 12. Precision-Aware Recurrence

For an integral recurrence of the schematic form

[
I_{n+1}
=======

aI_n+bI_{n-1},
]

the local numerical difficulty depends on more than the final integral magnitude.

The controller should additionally estimate:

```text
angular momentum
recurrence depth
coefficient magnitudes
cancellation ratio
primitive exponent ratio
contraction length
```

A cancellation indicator can be approximated by

[
\kappa_{\mathrm{local}}
=======================

\frac{\sum_i|x_i|}
{\left|\sum_i x_i\right|+\delta}.
]

Large (\kappa_{\mathrm{local}}) indicates loss of significant digits.

An adaptive recurrence kernel could therefore behave as:

```text
FP32
   │
   ├── stable → continue
   │
   └── cancellation detected
            ↓
           DS
            │
            └── still unsafe
                    ↓
                   FP64
```

This can be implemented either through precomputed shell-class rules or through runtime diagnostics.

For GPU efficiency, preclassification is preferable whenever possible because branch divergence inside a warp or subgroup can be expensive.

---

# 13. Contraction and Reduction

Even when individual integral values are accurately computed in FP32, accumulation can destroy that accuracy.

For

[
S=\sum_{i=1}^{N}x_i,
]

naive serial FP32 accumulation should not be assumed safe when (N) is large.

The proposed hierarchy is:

```text
thread-local accumulation
          ↓
pairwise or compensated reduction
          ↓
subgroup reduction
          ↓
workgroup partial sum
          ↓
global partial buffer
          ↓
high-precision final reduction
```

Error-free and compensated summation techniques provide substantially better numerical behavior than naive summation and have well-established numerical foundations.

A recommended policy is:

```text
local:
    FP32 or DS

subgroup:
    pairwise tree

workgroup:
    DS or FP64

final global reduction:
    FP64
```

If reproducibility is required, unordered global atomics should not determine the final answer. A fixed reduction tree should be used.

---

# 14. Density Fitting and RI Require a Different Strategy

Density fitting transforms part of the electronic-structure calculation into large tensor contractions.

A representative structure is

[
W_{is}^{q}
==========

\sum_r B_{ir}^{q} C_{rs},
]

followed by

[
K_{ij}
======

\sum_{qs}W_{is}^{q}W_{js}^{q}.
]

These operations can be organized as large GEMMs.

This is precisely where AI-oriented matrix engines become attractive.

The 2026 adaptive-density-fitting study used INT8-emulated FP64 GEMM for the expensive exchange build while retaining FP64 where appropriate. It reported that DF is much more suitable for Tensor Cores than direct ERI contractions because DF contains several large contractions rather than many small irregular ones.

Therefore:

```text
Direct ERI:
    FP32 / DS / FP64

DF / RI tensor contraction:
    native FP64 OR
    Ozaki-style emulated high-precision GEMM
```

should be treated as two separate optimization problems.

---

# 15. Ozaki Matrix Multiplication

The Ozaki approach decomposes a high-precision matrix into lower-precision slices.

For example,

[
A
=

A_0+A_1+\cdots+A_{p-1},
]

and

[
B
=

B_0+B_1+\cdots+B_{q-1}.
]

Then

[
AB
==

\sum_{i=0}^{p-1}
\sum_{j=0}^{q-1}
A_iB_j.
]

The expensive products (A_iB_j) can be evaluated using very high-throughput low-precision GEMM hardware.

The original error-free matrix multiplication framework was developed by Ozaki, Ogita, Oishi, and Rump.

More recent work has demonstrated implementations using integer Tensor Cores and other low-precision matrix units. Ootomo, Ozaki, and Yokota showed that high-precision GEMM can exploit INT8 matrix engines on consumer GPUs, while also emphasizing that performance and accuracy depend on input exponent distributions and the chosen number of splits.

Ozaki Scheme II further develops GEMM-oriented emulation using integer modular techniques and has demonstrated high FP64-emulation throughput on hardware including RTX 4090.

---

# 16. Adaptive Ozaki Precision

Using the maximum number of Ozaki slices at every SCF iteration would again waste computation.

Instead, define the emulation precision as

[
m_{\mathrm{emu}},
]

the approximate number of mantissa bits retained.

Early in SCF:

```text
small number of slices
≈ FP32-level effective precision
```

Near convergence:

```text
larger number of slices
≈ intermediate precision
```

Finally:

```text
native FP64 or full FP64 emulation
```

The 2026 adaptive DF work used the relative energy change

[
\Delta E_i^{\mathrm{rel}}
=========================

\left|
\frac{E_i-E_{i-1}}{E_i}
\right|
]

to select the emulation precision. Its published example moves through effective mantissa levels of approximately 23, 31, 39, and 47 bits before returning to FP64.

That is strong evidence for a general principle:

> **The precision used during an iterative electronic-structure calculation should increase as convergence progresses.**

The proposed architecture generalizes this principle beyond DF.

---

# 17. Limitations of Ozaki Arithmetic

Ozaki emulation is not automatically superior to native FP64.

Important costs include:

1. matrix decomposition;
2. storage of multiple slices;
3. multiple low-precision GEMMs;
4. final accumulation;
5. sensitivity to the exponent distribution of the matrix.

The integer-Tensor-Core Ozaki study specifically identifies wide exponent ranges as a disadvantage because additional slices are required to cover the mantissa space, increasing both memory consumption and computation.

Therefore, before selecting Ozaki GEMM, the controller should estimate

[
X_A
===

\max_i e(A_i)-\min_i e(A_i)
]

and similarly (X_B).

The runtime decision should be approximately:

```text
if matrix is small:
    normal GPU kernel
else if exponent spread is too large:
    native FP64 GEMM
else if emulated_GEMM_cost < native_FP64_cost:
    Ozaki / low-precision GEMM
else:
    native FP64 GEMM
```

This must be hardware-calibrated.

---

# 18. SCF as a Precision-Control Loop

An SCF calculation is itself an iterative correction process.

That property should be exploited explicitly.

Let

[
R_k
]

be a suitable SCF residual, for example a commutator-based residual, and let

[
\Delta D_k
==========

|D_k-D_{k-1}|.
]

A precision policy can be written as:

[
p_{k+1}
=======

f(
|\Delta E_k|,
\Delta D_k,
|R_k|
).
]

## Phase A: Far from convergence

Use aggressive low precision:

```text
ERI:
    primarily FP32

DF K:
    low-level emulation

reductions:
    compensated FP32 / DS

eigensolver:
    mixed precision
```

Errors at this stage behave partly like perturbations to the current iterate.

## Phase B: Intermediate convergence

Increase precision:

```text
important ERIs:
    DS

critical ERIs:
    FP64

DF K:
    more emulation slices

residual:
    FP64
```

## Phase C: Near convergence

Use strict arithmetic:

```text
Fock-sensitive terms:
    DS / FP64

residual:
    FP64

energy:
    FP64

density:
    FP64 verification
```

## Phase D: Final verification

Perform a strict evaluation using the requested validation policy.

For example:

```text
rebuild critical Fock terms
recompute energy
recompute SCF residual
check density change
verify precision contract
```

If validation fails:

```text
promote offending numerical classes
→ recompute
```

This gives a **fail-closed** design rather than silently accepting an inaccurate mixed-precision result.

---

# 19. Mixed-Precision Eigensolver

SCF normally requires solving a standard or generalized eigenproblem.

For example,

[
FC=SC\varepsilon.
]

There is no requirement that every internal operation be FP64 if the final eigenpairs satisfy strict FP64 residual tests.

A practical mixed-precision eigensolver can use:

```text
FP32:
    bulk matrix products
    subspace operations
    approximate preconditioning

FP64:
    residual construction
    orthogonality checks
    Rayleigh quotients where required
    final eigenpair validation
```

Dynamic precision has already been investigated for large-scale electronic-structure eigensolvers. A 2023 study dynamically switched among single, mixed, and double precision during iterative diagonalization, and subsequent work has continued this direction for GPU-oriented DFT eigensolvers.

This provides another independent justification for making precision dependent on convergence state.

---

# 20. Proposed Unified Decision Logic

The central runtime policy can be expressed as:

```text
for each SCF iteration:

    convergence = measure(
        energy_change,
        density_change,
        residual_norm
    )

    global_precision_state =
        precision_from_convergence(convergence)

    for each integral batch:

        contribution_bound =
            estimate_physical_contribution(...)

        numerical_risk =
            estimate(
                angular_momentum,
                recurrence_depth,
                exponent_range,
                cancellation,
                contraction_length
            )

        precision =
            choose_precision(
                contribution_bound,
                numerical_risk,
                global_precision_state,
                target_tolerance
            )

        dispatch:
            SKIP
            FP32
            DOUBLE_SINGLE
            FP64

    for each large DF/RI contraction:

        exponent_range = analyze_matrix_range()

        required_bits =
            bits_from_convergence(
                global_precision_state
            )

        dispatch:
            native_FP64_GEMM
            Ozaki_FP32
            Ozaki_FP16
            Ozaki_FP8
            Ozaki_INT8

    evaluate residual in FP64

    if convergence criterion is reached:
        perform strict final verification
```

---

# 21. Precision Should Be Monotonic During SCF

An important practical rule is:

> Once precision has been promoted during an SCF calculation, do not automatically demote it again.

For example,

```text
FP32
  ↓
DS
  ↓
FP64
```

is allowed, but

```text
FP64
  ↓
FP32
```

within the same convergence trajectory should normally be prohibited.

The recent adaptive DF implementation followed the same basic rule because SCF energy differences are not guaranteed to decrease monotonically, and allowing precision to oscillate can destabilize the calculation.

---

# 22. Hardware Calibration

The optimal policy is GPU-dependent.

At initialization, a small calibration suite should measure:

```text
FP32 FMA throughput
FP64 FMA throughput
DS add/mul throughput
FP32 GEMM
native FP64 GEMM
FP16 GEMM
BF16 GEMM
FP8 GEMM, if available
INT8 GEMM
memory bandwidth
reduction performance
```

Then construct empirical cost models such as

[
C_{\mathrm{FP64}}(m,n,k),
]

and

[
C_{\mathrm{Ozaki}}(m,n,k,s,X),
]

where (s) is the number of slices and (X) is exponent spread.

This prevents hardcoding assumptions such as

```text
Tensor Core is always faster
```

which is demonstrably false for some problem sizes and some high-FP64-throughput GPUs. The adaptive DF experiments, for example, found substantially different relative benefits on RTX-class GPUs and H100.

---

# 23. Proposed Software Abstraction

A useful arithmetic abstraction is:

```rust
trait QuantumReal {
    fn add(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn fma(self, b: Self, c: Self) -> Self;
}
```

with implementations conceptually corresponding to:

```text
F32
DoubleSingle
F64
```

However, kernels should not become excessively generic if generic abstraction prevents compiler optimization.

A better production design may generate specialized kernels:

```text
eri_fp32
eri_ds
eri_fp64
```

from one symbolic or intermediate representation.

The runtime then dispatches entire batches rather than branching on precision for individual arithmetic instructions.

This avoids severe warp/subgroup divergence.

---

# 24. Batch-Level Precision Classification

Precision selection should usually occur at:

```text
shell batch
shell quartet batch
integral tile
DF matrix tile
SCF stage
```

rather than per scalar.

For example:

```text
batch 0 → FP32 queue
batch 1 → FP32 queue
batch 2 → DS queue
batch 3 → FP64 queue
...
```

Then execute:

```text
launch eri_fp32(FP32_queue)
launch eri_ds(DS_queue)
launch eri_fp64(FP64_queue)
```

This preserves SIMT efficiency while retaining numerical adaptivity.

---

# 25. Strict Final Verification

Adaptive precision should be viewed as a speculative fast path guarded by strict validation.

A final verification procedure should check at least:

[
|E_k-E_{k-1}| < \varepsilon_E,
]

[
|R_k| < \varepsilon_R,
]

[
|D_k-D_{k-1}| < \varepsilon_D.
]

For library parity tests, additional comparisons can include:

[
\max_i |I_i-I_i^{\mathrm{FP64}}|,
]

[
\max_{ij}|F_{ij}-F_{ij}^{\mathrm{FP64}}|,
]

and

[
|E-E^{\mathrm{FP64}}|.
]

The important rule is:

```text
failure of validation
        ↓
precision promotion
        ↓
recomputation
```

not

```text
failure of validation
        ↓
accept approximate result
```

---

# 26. Accuracy Modes

A practical library should expose several contracts.

## Mode A — Fast Scientific

Example target:

[
|\Delta E| < 10^{-6}\ {\rm Ha}.
]

Allow aggressive:

```text
FP32
DS
low-slice Ozaki
```

with FP64 final verification.

## Mode B — High Accuracy

Example:

[
|\Delta E| < 10^{-9}\ {\rm Ha}.
]

Use:

```text
FP32 only for strongly bounded contributions
DS for substantial bulk work
FP64 for critical terms
high-precision reductions
```

## Mode C — Reference / Parity

Example:

[
\max |\Delta I| < 10^{-12}.
]

Use adaptive acceleration only where an error estimator certifies safety.

Everything else falls back to FP64.

This distinction is essential because scientific energy accuracy and element-wise library parity are fundamentally different numerical objectives.

---

# 27. Validation Program

The implementation should not be accepted based only on total-energy agreement.

Validation should occur in several layers.

## 27.1 Arithmetic microtests

Test:

```text
TwoSum
TwoProd
DS add
DS mul
DS FMA
renormalization
scaled DS
```

against MPFR or another higher-precision reference.

Cover:

```text
normal numbers
subnormals
very small values
very large values
cancellation
mixed signs
large exponent differences
```

---

## 27.2 Integral-level tests

Test individually:

```text
1e overlap
1e kinetic
1e nuclear attraction

2e ERI

2c2e

3c2e
```

for:

```text
s, p, d, f, g, ...
contracted functions
diffuse functions
tight functions
large inter-center distance
near-coincident centers
```

Record errors separately for:

```text
FP32
DS
FP64
adaptive
```

---

## 27.3 Recurrence stress tests

Construct deliberately difficult cases involving:

* high angular momentum;
* large recurrence depth;
* extreme Gaussian exponents;
* diffuse/tight exponent combinations;
* severe cancellation;
* long contractions.

These tests are more important than random average-case tests.

---

## 27.4 Matrix-operation validation

For each relevant matrix shape:

[
C=AB,
]

compare:

```text
native FP64
FP32
Ozaki FP32
Ozaki FP16
Ozaki FP8
Ozaki INT8
```

over matrices with controlled exponent ranges.

This is necessary because Ozaki performance and accuracy depend strongly on the exponent distribution.

---

## 27.5 SCF validation

Test:

```text
RHF
UHF
RKS
UKS
hybrid DFT
```

over systems with:

```text
easy convergence
poor convergence
small HOMO-LUMO gaps
diffuse basis sets
charged systems
open-shell systems
large basis sets
```

Measure:

[
\Delta E,
]

[
\Delta D,
]

[
\Delta F,
]

number of SCF iterations,

and convergence failures.

---

# 28. Benchmarking Methodology

Performance should be decomposed into:

```text
integral generation
screening
precision classification
Fock construction
DF contractions
eigensolver
XC
reductions
precision conversion
final validation
```

Report:

[
S_{\mathrm{kernel}}
===================

\frac{T_{\mathrm{FP64,kernel}}}
{T_{\mathrm{adaptive,kernel}}},
]

and, more importantly,

[
S_{\mathrm{total}}
==================

\frac{T_{\mathrm{FP64,total}}}
{T_{\mathrm{adaptive,total}}}.
]

Kernel-only speedups can be misleading.

The 2026 DF study explicitly included all computational steps when reporting whole-calculation DFT speedups and found up to 3.04× on RTX 4090 and 4.64× on RTX 6000 Ada in the tested systems.

---

# 29. Experimental Ablations

To scientifically establish which mechanisms actually provide value, benchmark at least:

```text
A0: pure FP64 baseline

A1: FP32 + FP64 dynamic ERI

A2: A1 + double-single tier

A3: A2 + compensated reductions

A4: A3 + SCF-driven precision scheduling

A5: A4 + Ozaki DF/RI GEMM

A6: A5 + mixed-precision eigensolver

A7: full architecture + final verification
```

This isolates the contribution of each technique.

Measure both:

```text
speed
accuracy
```

because a method that is faster only by silently changing the numerical problem is not a valid optimization.

---

# 30. Failure Modes

The implementation must explicitly detect or test the following failure modes.

### F1. FP32 cancellation

Symptoms:

```text
large sum(abs(x))
small abs(sum(x))
```

Action:

```text
FP32 → DS
```

### F2. DS exponent failure

Symptoms:

```text
underflow
overflow
wide exponent spread
```

Action:

```text
DS → scaled DS or FP64
```

### F3. Long accumulation

Action:

```text
pairwise / compensated reduction
```

### F4. SCF stagnation

Action:

```text
promote precision globally
```

### F5. Eigensolver residual stagnation

Action:

```text
recompute residual in FP64
promote solver precision
```

### F6. Ozaki slice explosion

Symptoms:

```text
wide exponent range
too many required slices
```

Action:

```text
Ozaki → native FP64
```

### F7. Final validation failure

Action:

```text
recompute affected component in FP64
```

---

# 31. Recommended Development Order

## Phase 1 — Establish reference behavior

Implement or retain the complete FP64 path.

No mixed precision should be introduced until this baseline is stable.

---

## Phase 2 — Instrument numerical sensitivity

Collect:

```text
integral magnitude
Schwarz bounds
density-weighted contribution bounds
recurrence depth
cancellation estimates
exponent ranges
reduction lengths
SCF residuals
```

This data is required to design a rational controller.

---

## Phase 3 — Add FP32 direct integrals

Implement separate FP32 kernels.

Compare every integral class with FP64.

Determine empirically which numerical regions are safe.

---

## Phase 4 — Add dynamic FP32/FP64 dispatch

This is the lowest-risk mixed-precision optimization and is already strongly supported by previous ERI work.

---

## Phase 5 — Add double-single

Introduce:

```text
TwoSum
FastTwoSum
TwoProd/FMA
renormalization
DS addition
DS multiplication
```

Use DS only for numerical classes that fail FP32 but do not require FP64.

---

## Phase 6 — Improve reductions

Introduce deterministic hierarchical reduction and compensation.

Measure the accuracy/performance tradeoff independently of integral generation.

---

## Phase 7 — SCF-driven precision control

Use:

[
\Delta E,
\quad
\Delta D,
\quad
R
]

to promote precision as convergence proceeds.

---

## Phase 8 — DF/RI emulated GEMM

Only after large GEMM hotspots are clearly identified should Ozaki-style emulation be introduced.

Do not apply it indiscriminately to small direct-integral operations.

---

## Phase 9 — Mixed-precision eigensolver

Move bulk eigensolver work to lower precision while retaining strict FP64 residual tests.

---

## Phase 10 — Fail-closed final verification

Make strict validation part of the algorithm rather than merely part of the test suite.

---

# 32. Recommended Final Architecture

The resulting system is:

```text
                         SCF Controller
                              │
                 ┌────────────┼────────────┐
                 │            │            │
              ΔEnergy      ΔDensity     Residual
                 │            │            │
                 └────────────┼────────────┘
                              │
                    Precision Controller
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
 Direct Integrals        DF / RI GEMM          Eigensolver
        │                     │                     │
 ┌──────┼───────┐       ┌─────┴──────┐       ┌──────┴──────┐
 │      │       │       │            │       │             │
FP32    DS     FP64    Native FP64   Ozaki   low precision  FP64
 │      │       │                      │            │
 └──────┴───────┘                 low-bit GEMM     │
        │                                           │
        └──────────────────┬────────────────────────┘
                           │
                    Accurate Reduction
                           │
                           ▼
                    FP64 Verification
                           │
                    ┌──────┴──────┐
                    │             │
                  PASS          FAIL
                    │             │
                 return      promote precision
                                  │
                               recompute
```

---

# 33. Central Research Hypothesis

The architecture is based on the following hypothesis:

> **The numerical sensitivity of quantum chemistry is sufficiently nonuniform in space, integral magnitude, recurrence structure, matrix operation, and SCF iteration that most arithmetic does not require native FP64, provided that numerical error is explicitly estimated and sensitive quantities are recomputed at higher precision.**

Existing studies already support several parts of this hypothesis independently:

* dynamic FP32/FP64 ERI evaluation;
* dynamic/mixed-precision electronic-structure eigensolvers;
* reduced-precision quantum-chemistry matrix operations;
* high-precision GEMM reconstruction from low-precision matrix engines;
* adaptive INT8-based density fitting in production-style DFT calculations.

The proposed contribution is to combine these ideas into a **single error-driven precision architecture spanning direct integrals, recurrence, reductions, DF/RI contractions, eigensolvers, and SCF convergence**.

A formal claim of scientific novelty would require a dedicated literature review, but this unified cross-layer controller is a substantially more ambitious design than simply replacing selected FP64 GEMMs or ERIs with lower precision.

---

# 34. Final Recommendation

The highest-priority implementation is:

```text
1. FP64 reference implementation

2. numerical instrumentation

3. FP32 integral kernels

4. adaptive FP32/FP64 direct-integral dispatch

5. double-single intermediate tier

6. compensated hierarchical reductions

7. SCF-dependent precision escalation

8. Ozaki low-bit GEMM for DF/RI

9. mixed-precision eigensolver

10. strict FP64 final verification
```

The critical design rule is:

> **Precision must be controlled by estimated error and convergence state, not by a fixed global scalar type.**

For direct Gaussian integral calculations, the preferred hierarchy is:

[
\boxed{
\text{screen}
\rightarrow
\mathrm{FP32}
\rightarrow
\mathrm{double\text{-}single}
\rightarrow
\mathrm{FP64}
}
]

For large DF/RI tensor contractions:

[
\boxed{
\mathrm{native\ FP64}
\quad\text{or}\quad
\mathrm{Ozaki\ low\text{-}precision\ GEMM}
}
]

For SCF:

[
\boxed{
\text{low precision early}
\rightarrow
\text{progressive promotion}
\rightarrow
\text{strict FP64 verification}
}
]

This architecture offers a credible path toward exploiting commodity AI-oriented GPUs for high-accuracy quantum chemistry while retaining an explicit and testable numerical accuracy contract.

---

# References

1. **N. Luehr, I. S. Ufimtsev, and T. J. Martínez**, “Dynamic Precision for Electron Repulsion Integral Evaluation on Graphical Processing Units (GPUs),” *Journal of Chemical Theory and Computation*, 7, 949–954 (2011).
   URL: https://doi.org/10.1021/ct100701w

2. **J. Woo, S. Kim, and W. Y. Kim**, “Dynamic Precision Approach for Accelerating Large-Scale Eigenvalue Solvers in Electronic Structure Calculations on Graphics Processing Units,” *Journal of Chemical Theory and Computation*, 19, 1457–1465 (2023).
   URL: https://doi.org/10.1021/acs.jctc.2c00983

3. **W. Dawson, J. Domke, T. Nakajima, and K. Ozaki**, “Reducing Numerical Precision Requirements in Quantum Chemistry Calculations,” *Journal of Chemical Theory and Computation*, 20, 10826–10837 (2024).
   DOI URL: https://doi.org/10.1021/acs.jctc.4c00938
   Preprint: https://arxiv.org/abs/2407.13299

4. **J. Woo and S. Choi**, “A Mixed-Precision Approach to a Preconditioned Eigensolver for Efficient Density Functional Calculations on AI-Focused GPUs,” *Journal of Chemical Theory and Computation* (2026).
   URL: https://doi.org/10.1021/acs.jctc.5c01800

5. **“Accelerating Density Fitting with Adaptive-precision and 8-bit Integer on AI Accelerators,”** 2026. The implementation evaluates adaptive INT8-emulated FP64 GEMM for density fitting and reports whole-calculation DFT acceleration on RTX 4090, RTX 6000 Ada, and H100 GPUs.
   DOI URL: https://doi.org/10.1021/acs.jpca.6c00225
   Preprint: https://arxiv.org/abs/2601.08077

6. **K. Ozaki, T. Ogita, S. Oishi, and S. M. Rump**, “Error-Free Transformations of Matrix Multiplication by Using Fast Routines of Matrix Multiplication and Its Applications,” *Numerical Algorithms*, 59, 95–118 (2012).
   URL: https://doi.org/10.1007/s11075-011-9478-1

7. **H. Ootomo, K. Ozaki, and R. Yokota**, “DGEMM on Integer Matrix Multiplication Unit.” The work investigates high-precision matrix multiplication using INT8 matrix engines and evaluates consumer GPUs as well as HPC GPUs.
   URL: https://arxiv.org/abs/2306.11975

8. **K. Ozaki, Y. Uchino, and T. Imamura**, “Ozaki Scheme II: A GEMM-oriented Emulation of Floating-Point Matrix Multiplication Using an Integer Modular Technique” (2025).
   URL: https://arxiv.org/abs/2504.08009

9. **T. Ogita, S. M. Rump, and S. Oishi**, “Accurate Sum and Dot Product,” *SIAM Journal on Scientific Computing*, 26(6), 1955–1988 (2005). This work develops error-free transformations and compensated algorithms relevant to accurate reduction and multiword arithmetic.
   DOI URL: https://doi.org/10.1137/030601818
   Author PDF: https://www.tuhh.de/ti3/paper/rump/OgRuOi05.pdf

10. **A. Thall**, “Extended-Precision Floating-Point Numbers for GPU Computation.” This work discusses GPU implementations using multiple FP32 components to obtain extended significand precision while retaining single-precision exponent characteristics.
    URL: https://andrewthall.org/papers/df64_qf128.pdf

11. **Y. Hida, X. S. Li, and D. H. Bailey**, “Algorithms for Quad-Double Precision Floating Point Arithmetic,” *15th IEEE Symposium on Computer Arithmetic* (2001). The paper provides foundational algorithms for multi-component floating-point arithmetic.
    URL: https://doi.org/10.1109/ARITH.2001.930115

12. **NVIDIA**, “Floating Point and IEEE 754 Compliance for NVIDIA GPUs.” This technical documentation discusses GPU floating-point behavior, fused multiply-add, and IEEE 754 considerations.
    URL: https://docs.nvidia.com/cuda/floating-point/index.html
