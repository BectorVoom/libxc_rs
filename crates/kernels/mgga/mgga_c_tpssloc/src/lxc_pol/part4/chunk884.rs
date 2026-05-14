//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 884/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk884<F: Float>(t1227: F, t15734: F, t15437: F, t3505: F, t3576: F, t5064: F, t13969: F, t4988: F, t1725: F, t698: F, t1174: F, t225: F, t4941: F, t5053: F, t5168: F, t592: F) -> (F, F, F, F, F, F, F, F) {
    let t15735 = t1227 * t15734;
    let t15737 = t15437 * t3505;
    let t15740 = t5064 * t3576;
    let t15743 = t13969 * t4988;
    let t15745 = 5.0 / 10368.0 * t1227 * t15743;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15797 = t4941 * t225;
    let t15820 = t5053 * t225;
    let t15877 = t592 * t5168;
    (t15735, t15737, t15740, t15745, t15754, t15797, t15820, t15877)
}
