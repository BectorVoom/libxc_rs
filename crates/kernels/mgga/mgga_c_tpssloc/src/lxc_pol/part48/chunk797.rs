//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 797/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk797<F: Float>(t794: F, t852: F, t213: F, t225: F, t23202: F, t23211: F, t1081: F, t2752: F, t22573: F, t6875: F, t111: F, t7222: F, t112: F, t24447: F, t24007: F, t24141: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t82133 = t794 * t852;
    let t82159 = t213 * t852 * t225;
    let t82197 = t23202 * t225;
    let t82287 = t23211 * t225;
    let t83555 = t2752 * t1081;
    let t83886 = t6875 * t22573;
    let t84033 = t7222 * t111;
    let t84078 = t24447 * t112;
    let t84097 = t24007 * t111;
    let t84433 = t24141 * t225;
    (t82133, t82159, t82197, t82287, t83555, t83886, t84033, t84078, t84097, t84433)
}
