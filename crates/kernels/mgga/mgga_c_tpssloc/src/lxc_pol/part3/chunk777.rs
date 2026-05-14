//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 777/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk777<F: Float>(t491: F, t5011: F, t1246: F, t1215: F, t1932: F, t475: F, t1755: F, t1751: F, t493: F, t5052: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1756: F, t1758: F, t3604: F, t3610: F, t3624: F, t470: F, t494: F, t4964: F, t5064: F, t5069: F, t5073: F) -> (F, F, F, F, F, F, F) {
    let t5075 = t491 * t5011;
    let t5076 = t5075 * t1246;
    let t5079 = t1932 * t1215 * t475;
    let t5080 = t1755 * t5079;
    let t5083 = t1751 * t1215;
    let t5084 = t5083 * t1246;
    let t5086 = t493 * t5052;
    let t5088 = t1201 * t1758 + t1244 * t5073 + t1244 * t5076 + t1244 * t5084 + t1247 * t5064 + t1249 * t1729 + t1756 * t3604 + 2.0 * t3610 * t5069 - t3624 * t5080 + t470 * t5086 + t494 * t4964;
    (t5075, t5076, t5079, t5080, t5084, t5086, t5088)
}
