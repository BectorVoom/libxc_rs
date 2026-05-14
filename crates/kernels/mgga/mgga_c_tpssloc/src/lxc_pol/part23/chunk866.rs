//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 866/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk866<F: Float>(t3431: F, t6126: F, t1174: F, t6130: F, t11539: F, t6119: F, t4889: F, t4896: F, t11570: F, t5392: F, t1171: F, t6109: F, t6011: F, t699: F, t6014: F, t6017: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18451 = t3431 * t6126;
    let t18452 = t1174 * t18451;
    let t18454 = t3431 * t6130;
    let t18455 = t1174 * t18454;
    let t18457 = t11539 * t6119;
    let t18458 = t1174 * t18457;
    let t18460 = t4889 * t4896;
    let t18469 = t11570 * t5392;
    let t18489 = t6109 * t1171;
    let t18494 = t699 * t6011;
    let t18505 = t699 * t6014;
    let t18512 = t699 * t6017;
    (t18451, t18452, t18454, t18455, t18457, t18458, t18460, t18469, t18489, t18494, t18505, t18512)
}
