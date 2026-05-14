//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1130/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1130<F: Float>(t19790: F, t19803: F, t225: F, t1814: F, t5343: F, t3901: F, t6420: F, t6378: F, t68: F, t1307: F, t210: F, t6370: F, t1810: F, t5187: F, t6374: F, t1358: F, t6379: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19804 = t19790 + t19803;
    let t19805 = t19804 * t225;
    let t19810 = t1814 * t5343;
    let t19813 = t3901 * t6420;
    let t19815 = t6378 * t68;
    let t19823 = t210 * t6370 * t1307;
    let t19827 = t210 * t1810 * t5187;
    let t19831 = t210 * t6374 * t1307;
    let t19834 = t6379 * t1358;
    (t19804, t19805, t19810, t19813, t19815, t19823, t19827, t19831, t19834)
}
