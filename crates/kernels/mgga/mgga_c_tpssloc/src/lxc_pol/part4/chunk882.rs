//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 882/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk882<F: Float>(t15640: F, t3506: F, t13969: F, t4973: F, t1227: F, t11153: F, t3584: F, t1734: F, t3508: F, t3548: F, t4889: F, t135: F, t5045: F, t1174: F, t1222: F, t4966: F) -> (F, F, F, F, F, F, F) {
    let t15642 = t3506 * t15640 / 1152.0;
    let t15643 = t13969 * t4973;
    let t15645 = t1227 * t15643 / 1728.0;
    let t15654 = t3584 * t11153;
    let t15659 = t1734 * t3508;
    let t15671 = t4889 * t3548 / 162.0;
    let t15689 = t135 * t5045;
    let t15691 = t1174 * t15689 / 432.0;
    let t15699 = t4966 * t1222 / 2304.0;
    (t15642, t15645, t15654, t15659, t15671, t15691, t15699)
}
