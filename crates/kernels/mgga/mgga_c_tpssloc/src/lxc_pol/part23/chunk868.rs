//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 868/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk868<F: Float>(t11243: F, t5992: F, t11265: F, t1128: F, t6031: F, t1147: F, t6063: F, t3400: F, t6084: F, t300: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F, t1227: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18746 = t11243 * t5992;
    let t18754 = t11265 * t5992;
    let t18840 = t6031 * t1128;
    let t18899 = t6063 * t1147;
    let t18910 = t3400 * t6084;
    let t18915 = t300 * t6063;
    let t18972 = t5002 * t4997;
    let t18975 = t248 * t11784 * t5971;
    let t18976 = t1227 * t18975;
    (t18746, t18754, t18840, t18899, t18910, t18915, t18972, t18975, t18976)
}
