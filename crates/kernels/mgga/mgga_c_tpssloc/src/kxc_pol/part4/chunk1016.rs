//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1016/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1016<F: Float>(t15395: F, t18206: F, t15338: F, t4904: F, t3447: F, t3431: F, t6126: F, t1174: F, t6130: F, t11539: F, t6119: F, t4889: F, t4896: F, t18215: F, t4900: F, t11570: F, t5392: F) -> (F, F, F, F, F, F, F, F) {
    let t18443 = t15395 * t18206;
    let t18446 = t15338 * t4904;
    let t18447 = t3447 * t18446;
    let t18451 = t3431 * t6126;
    let t18452 = t1174 * t18451;
    let t18454 = t3431 * t6130;
    let t18455 = t1174 * t18454;
    let t18457 = t11539 * t6119;
    let t18458 = t1174 * t18457;
    let t18460 = t4889 * t4896;
    let t18466 = t4900 * t18215;
    let t18469 = t11570 * t5392;
    (t18443, t18447, t18452, t18455, t18458, t18460, t18466, t18469)
}
