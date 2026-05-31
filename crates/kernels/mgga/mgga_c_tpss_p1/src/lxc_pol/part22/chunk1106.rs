//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1106/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1106<F: Float>(t1073: F, t12083: F, t12086: F, t12136: F, t12155: F, t12159: F, t12161: F, t12164: F, t12167: F, t12170: F, t1543: F, t2932: F, t2969: F, t2976: F, t4125: F, t4147: F, t4181: F, t421: F, t9365: F, t9419: F, t9471: F) -> F {
    let t12175 = F::cast_from(0.5848223622634646207e0_f64) * t9365 * t1543 + F::cast_from(0.11696447245269292414e1_f64) * t2969 * t4181 - F::cast_from(2.0_f64) * t12083 * t2932 - F::cast_from(0.11696447245269292414e1_f64) * t12086 * t2976 + F::cast_from(0.5848223622634646207e0_f64) * t1073 * t12136 - F::cast_from(0.310907e-1_f64) * t12155 * t421 + t12159 - t12161 + t12164 + t12167 + t12170 - F::cast_from(4.0_f64) * t9471 * t4125 + F::cast_from(0.64327917994770140268e2_f64) * t9419 * t4147;
    t12175
}
