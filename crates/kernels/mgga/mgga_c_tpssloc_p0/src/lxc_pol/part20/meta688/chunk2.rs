//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2607/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2607<F: Float>(t11539: F, t1174: F, t14731: F, t135: F, t15666: F, t11665: F, t15572: F, t3515: F, t4983: F, t49850: F, t11818: F, t1213: F, t248: F, t5012: F) -> (F, F, F, F, F) {
    let t52932 = t1174 * t11539 * t14731;
    let t52935 = t1174 * t135 * t15666;
    let t52942 = t11665 * t15572;
    let t52952 = t3515 * t49850 * t4983;
    let t52953 = t52952 / F::cast_from(4608.0_f64);
    let t52973 = t1213 * t248 * t11818 * t5012;
    (t52932, t52935, t52942, t52953, t52973)
}
