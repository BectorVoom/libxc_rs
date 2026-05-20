//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2939/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939<F: Float>(t10189: F, t5842: F, t2986: F, t2990: F, t13847: F, t13861: F, t17841: F, t2987: F, t10186: F, t13812: F, t17788: F, t17805: F, t17811: F, t17814: F, t17818: F, t17854: F, t17867: F, t4531: F, t47966: F, t48184: F) -> F {
    let t61189 = t10189 * t5842;
    let t61191 = t2986 * t61189 * t2990;
    let t61200 = t2986 * t13847 * t13861;
    let t61210 = t2987 * t17841;
    let t61214 = F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t4531 * t47966 - F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t48184 * t13812 - F::cast_from(0.18518518518518518518e-3_f64) * t61191 + F::cast_from(0.14814814814814814814e-2_f64) * t10186 * t17805 + F::cast_from(0.29629629629629629628e-2_f64) * t10186 * t17867 + F::cast_from(0.29629629629629629628e-2_f64) * t10186 * t17788 - F::cast_from(0.37037037037037037036e-3_f64) * t61200 - F::cast_from(0.88888888888888888885e-2_f64) * t10186 * t17811 + F::cast_from(0.59259259259259259257e-2_f64) * t10186 * t17814 + F::cast_from(0.11851851851851851851e-1_f64) * t10186 * t17854 - F::cast_from(0.29629629629629629628e-2_f64) * t10186 * t17818 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t61210 * t2990;
    t61214
}
