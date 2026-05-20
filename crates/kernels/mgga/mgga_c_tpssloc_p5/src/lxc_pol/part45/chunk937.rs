//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 937/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk937<F: Float>(t23168: F, t30678: F, t23035: F, t2379: F, t30676: F, t6637: F, t30686: F, t6579: F, t1902: F, t2631: F, t1888: F, t22996: F, t2632: F) -> (F, F, F, F, F) {
    let t112968 = t23168 * t30678;
    let t112969 = F::cast_from(0.15352717957250113407e0_f64) * t112968;
    let t112973 = F::cast_from(0.9869604401089358619e-1_f64) * t23035 * t6637 * t30676 * t2379;
    let t112974 = t6579 * t30686;
    let t112975 = F::cast_from(0.76763589786250567036e-1_f64) * t112974;
    let t112976 = t1902 * t2631;
    let t112980 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t22996 * t112976 * t2632;
    (t112969, t112973, t112975, t112976, t112980)
}
