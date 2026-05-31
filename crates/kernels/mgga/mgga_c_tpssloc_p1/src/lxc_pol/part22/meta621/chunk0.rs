//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2153/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2153<F: Float>(t53033: F, t1213: F, t1735: F, t248: F, t45017: F, t10477: F, t1742: F, t11713: F, t3503: F, t1210: F, t11529: F, t1174: F, t4729: F) -> (F, F, F, F, F, F) {
    let t53034 = t53033 / F::cast_from(3456.0_f64);
    let t53079 = t1213 * t248 * t45017 * t1735;
    let t53081 = t1742 * t10477;
    let t53083 = t11713 * t3503 * t53081;
    let t53087 = t11713 * t1210 * t53081;
    let t53096 = t1174 * t11529 * t4729;
    (t53034, t53079, t53081, t53083, t53087, t53096)
}
