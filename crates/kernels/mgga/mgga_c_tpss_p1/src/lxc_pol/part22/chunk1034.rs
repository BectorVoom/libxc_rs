//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1034/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1034<F: Float>(t11002: F, t11024: F, t11028: F, t11033: F, t11037: F, t11080: F, t11083: F, t11086: F, t11089: F, t11091: F, t11094: F, t11096: F) -> (F, F) {
    let t11188 = F::cast_from(0.13418888888888888889e0_f64) * t11002;
    let t11205 = -F::cast_from(0.20128333333333333333e0_f64) * t11024 - F::new(0.181155e1) * t11028 + F::new(0.12077e1) * t11033 + F::new(0.60385e0) * t11037 + F::new(0.16504875e0) * t11080 + F::new(0.19419375e1) * t11083 - F::cast_from(0.412621875e-1_f64) * t11086 - F::new(0.258925e1) * t11089 - F::new(0.1294625e1) * t11091 + F::new(0.16504875e0) * t11094 + F::new(0.82524375e-1) * t11096;
    (t11188, t11205)
}
