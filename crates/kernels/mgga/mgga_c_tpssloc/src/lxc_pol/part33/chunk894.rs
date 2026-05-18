//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 894/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk894<F: Float>(t1171: F, t6109: F, t6011: F, t699: F, t6014: F, t6017: F, t135: F, t6146: F, t1174: F, t6140: F, t4889: F, t4916: F) -> (F, F, F, F, F, F, F) {
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    let t18505 = t699 * t6014;
    let t18512 = t699 * t6017;
    let t18529 = t135 * t6146;
    let t18530 = t1174 * t18529;
    let t18532 = t135 * t6140;
    let t18533 = t1174 * t18532;
    let t18536 = t4889 * t4916;
    (t18489, t18494, t18505, t18512, t18530, t18533, t18536)
}
