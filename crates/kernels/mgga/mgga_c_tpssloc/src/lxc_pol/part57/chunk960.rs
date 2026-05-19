//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 960/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk960<F: Float>(t126413: F, t22986: F, t23270: F, t30633: F, t118885: F, t118893: F, t1880: F, t28294: F, t30663: F, t118903: F, t28431: F, t6553: F, t6571: F) -> (F, F, F, F, F, F) {
    let t126417 = F::cast_from(0.13159472534785811492e0_f64) * t22986 * t23270 * t30633 * t126413;
    let t126418 = F::cast_from(0.16449340668482264365e-1_f64) * t118885;
    let t126419 = F::cast_from(0.76763589786250567036e-1_f64) * t118893;
    let t126422 = F::cast_from(0.3289868133696452873e-1_f64) * t1880 * t30663 * t28294;
    let t126423 = F::cast_from(0.16449340668482264365e-1_f64) * t118903;
    let t126427 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t6553 * t6571 * t28431;
    (t126417, t126418, t126419, t126422, t126423, t126427)
}
