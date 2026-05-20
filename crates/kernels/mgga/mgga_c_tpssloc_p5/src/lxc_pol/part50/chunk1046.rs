//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1046/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1046<F: Float>(t1920: F, t30874: F, t1945: F, t362: F, t884: F, t6784: F, t8400: F, t986: F, t6800: F, t6810: F, t6799: F, t1948: F, t6768: F) -> (F, F, F, F, F, F, F, F) {
    let t30876 = F::cast_from(0.54831135561607547883e-2_f64) * t1920 * t30874;
    let t30877 = t362 * t1945;
    let t30878 = t30877 * t884;
    let t30879 = t6784 * t30878;
    let t30882 = t986 * t8400;
    let t30885 = t6810 * t6800;
    let t30886 = t6799 * t30885;
    let t30889 = t1948 * t6768;
    (t30876, t30877, t30878, t30879, t30882, t30885, t30886, t30889)
}
