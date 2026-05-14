//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1287/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1287<F: Float>(t81423: F, t83688: F, t83894: F, t83971: F, t1401: F, t81455: F, t111: F, t7002: F, t12521: F, t6534: F, t22479: F, t3938: F, t1873: F, t3941: F, t9416: F, t16535: F) -> (F, F, F, F, F, F, F) {
    let t83973 = t81423 + t83688 + t83894 + t83971;
    let t83979 = 0.135e2 * t1401 * t81455;
    let t83980 = t7002 * t111;
    let t83984 = 0.405e2 * t12521 * t6534;
    let t83988 = 0.405e2 * t3938 * t22479;
    let t83991 = 27.0 * t3941 * t1873 * t9416;
    let t83993 = 81.0 * t16535 * t6534;
    (t83973, t83979, t83980, t83984, t83988, t83991, t83993)
}
