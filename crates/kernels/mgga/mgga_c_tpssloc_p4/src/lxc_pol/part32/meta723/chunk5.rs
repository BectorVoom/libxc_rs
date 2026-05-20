//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2311/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2311<F: Float>(t8074: F, t94909: F, t24826: F, t29745: F, t24574: F, t29705: F, t477: F, t6238: F, t1090: F, t17635: F, t19145: F, t24812: F, t24820: F, t24821: F, t27549: F, t27550: F, t27551: F, t29753: F, t7283: F, t7362: F, t85863: F, t85986: F, t86000: F, t95125: F, t95134: F, t95136: F) -> F {
    let t103867 = t94909 * t8074;
    let t103877 = t24826 * t29745;
    let t103879 = t24574 * t29705;
    let t103881 = t477 * t6238;
    let t103889 = F::cast_from(0.18277045187202515961e-2_f64) * t85986 - F::cast_from(0.14621636149762012769e-1_f64) * t103867 + F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t27550 * t27551 * t17635 - F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t24820 * t19145 * t24821 + F::cast_from(0.27415567780803773942e-2_f64) * t103877 - t95125 - F::cast_from(0.27415567780803773942e-2_f64) * t103879 + t86000 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t103881 * t1090 + t95134 - t95136 + F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t85863 * t29753;
    t103889
}
