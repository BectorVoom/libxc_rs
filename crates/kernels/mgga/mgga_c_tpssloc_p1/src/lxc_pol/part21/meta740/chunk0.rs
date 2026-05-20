//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2604/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2604<F: Float>(t11745: F, t15737: F, t1227: F, t13969: F, t15649: F, t43763: F, t44827: F, t11539: F, t1174: F, t14740: F, t14731: F, t135: F, t15666: F) -> (F, F, F, F, F, F) {
    let t52908 = t15737 * t11745;
    let t52917 = t1227 * t13969 * t15649;
    let t52919 = t44827 * t43763;
    let t52926 = t1174 * t11539 * t14740;
    let t52932 = t1174 * t11539 * t14731;
    let t52935 = t1174 * t135 * t15666;
    (t52908, t52917, t52919, t52926, t52932, t52935)
}
