//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2407/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2407<F: Float>(t48155: F, t59657: F, t60163: F, t60168: F, t60173: F, t68536: F, t68541: F, t68545: F, t68549: F, t68552: F, t68556: F, t68563: F) -> F {
    let t68825 = F::cast_from(0.16431333333333333333e0_f64) * t68536 - F::cast_from(0.27385555555555555556e-1_f64) * t68541 + F::new(0.197176e1) * t68545 - F::new(0.147882e1) * t68549 - F::cast_from(0.98587999999999999998e0_f64) * t68552 + F::cast_from(0.49293999999999999999e0_f64) * t68556 + F::cast_from(0.16431333333333333333e0_f64) * t60163 + F::cast_from(0.5477111111111111111e0_f64) * t60168 - F::cast_from(0.27385555555555555555e0_f64) * t60173 - F::cast_from(0.26574814814814814815e0_f64) * t59657 - F::cast_from(0.10954222222222222222e0_f64) * t68563 + F::cast_from(0.54771111111111111112e0_f64) * t48155;
    t68825
}
