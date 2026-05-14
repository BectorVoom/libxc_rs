//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 960/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk960<F: Float>(t10295: F, t13642: F, t17286: F, t17288: F, t17290: F, t21120: F, t21132: F, t21136: F, t21140: F, t21161: F, t21168: F, t340: F, t343: F, t974: F, t1597: F, t5836: F) -> (F, F, F, F) {
    let t21444 = t10295 + 5.0 / 9.0 * t13642 - t17286 / 9.0 + 2.0 / 3.0 * t17288 - t17290 / 3.0 + 2.0 / 27.0 * t21132 - t21120 / 3.0 + t21168 / 6.0 + t21140 - t21161 + t21136 / 6.0;
    let t21446 = t340 * t21444 * t343;
    let t21447 = t974 * t21446;
    let t21452 = t5836 * t1597 * t343;
    (t21444, t21446, t21447, t21452)
}
