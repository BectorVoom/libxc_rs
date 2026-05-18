//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 919/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk919<F: Float>(t218: F, t33947: F, t10110: F, t1527: F, t8733: F, t259: F, t31971: F, t32014: F, t33372: F, t33410: F, t33420: F, t33423: F, t33430: F, t33935: F, t33940: F, t7087: F, t7830: F, t7842: F, t855: F) -> (F, F, F) {
    let t33948 = t218 * t33947;
    let t33951 = t10110 * t8733 * t1527;
    let t33960 = -F::new(0.3289868133696452873e-1) * t33372 - t31971 + F::new(4.0) * t855 * t33935 + F::new(4.0) * t7087 * t7830 + t33940 * t259 + t33948 * t259 - F::new(6.0) * t855 * t33951 - F::new(0.3289868133696452873e-1) * t33410 - t32014 - F::new(0.6579736267392905746e-1) * t33420 - F::new(0.3289868133696452873e-1) * t33423 + F::new(0.3289868133696452873e-1) * t33430 - F::new(2.0) * t7087 * t7842;
    (t33948, t33951, t33960)
}
