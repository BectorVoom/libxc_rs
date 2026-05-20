//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2192/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2192<F: Float>(t90541: F, t1834: F, t794: F, t22892: F, t6891: F, t22704: F, t26355: F, t81326: F, t26197: F, t80670: F, t1307: F, t22635: F, t26331: F, t5187: F, t567: F) -> (F, F, F, F, F, F) {
    let t90542 = F::cast_from(0.38381794893125283518e-1_f64) * t90541;
    let t90544 = t794 * t1834;
    let t90546 = t22892 * t90544 * t6891;
    let t90547 = F::cast_from(0.16449340668482264365e-1_f64) * t90546;
    let t90549 = t22704 * t81326 * t26355;
    let t90550 = F::cast_from(0.16449340668482264365e-1_f64) * t90549;
    let t90551 = t80670 * t26197;
    let t90556 = t26331 * t22635 * t567 * t5187 * t1307;
    (t90542, t90544, t90547, t90550, t90551, t90556)
}
