//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1010/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1010<F: Float>(t1873: F, t91854: F, t23938: F, t6534: F, t91857: F, t26977: F, t22479: F, t7042: F, t31518: F, t650: F, t2312: F, t8595: F) -> (F, F, F, F, F, F, F) {
    let t115813 = F::cast_from(4.0_f64) * t91854 * t1873;
    let t115815 = F::cast_from(4.0_f64) * t23938 * t6534;
    let t115817 = F::cast_from(2.0_f64) * t91857 * t1873;
    let t115819 = F::cast_from(4.0_f64) * t26977 * t6534;
    let t115821 = F::cast_from(2.0_f64) * t7042 * t22479;
    let t115919 = F::cast_from(2.0_f64) * t650 * t31518;
    let t115920 = t2312 * t8595;
    (t115813, t115815, t115817, t115819, t115821, t115919, t115920)
}
