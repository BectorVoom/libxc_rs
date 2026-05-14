//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 814/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk814<F: Float>(t2553: F, t30676: F, t6552: F, t6637: F, t22893: F, t23164: F, t30677: F, t1902: F, t22986: F, t6646: F, t776: F, t829: F, t23168: F, t30678: F, t23035: F, t2379: F) -> (F, F, F, F, F) {
    let t112959 = 0.3289868133696452873e-1 * t6552 * t6637 * t30676 * t2553;
    let t112961 = t23164 * t22893 * t30677;
    let t112962 = 0.3289868133696452873e-1 * t112961;
    let t112967 = 0.6579736267392905746e-1 * t22986 * t6646 * t1902 * t776 * t829;
    let t112968 = t23168 * t30678;
    let t112969 = 0.15352717957250113407e0 * t112968;
    let t112973 = 0.9869604401089358619e-1 * t23035 * t6637 * t30676 * t2379;
    (t112959, t112962, t112967, t112969, t112973)
}
