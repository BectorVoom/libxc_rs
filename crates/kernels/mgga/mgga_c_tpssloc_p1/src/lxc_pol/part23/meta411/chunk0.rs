//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1227/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1227<F: Float>(t1174: F, t6177: F, t698: F, t3545: F, t6109: F, t15753: F, t4889: F, t1244: F, t3068: F, t478: F, t6163: F, t6183: F) -> (F, F, F, F, F) {
    let t66057 = t1174 * t698 * t6177;
    let t66500 = t6109 * t3545;
    let t66545 = t4889 * t15753;
    let t66622 = t1244 * t478 * t6163 * t3068;
    let t66668 = t1174 * t698 * t6183;
    (t66057, t66500, t66545, t66622, t66668)
}
