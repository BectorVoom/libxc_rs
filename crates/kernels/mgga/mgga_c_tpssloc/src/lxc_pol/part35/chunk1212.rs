//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1212/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1212<F: Float>(t29884: F, t576: F, t1390: F, t20416: F, t1983: F, t6878: F, t20085: F, t7753: F, t28821: F, t7756: F, t28823: F, t7685: F, t28835: F, t7687: F, t97817: F, t7688: F) -> (F, F, F, F, F, F, F, F) {
    let t105150 = t576 * t29884;
    let t105159 = t1390 * t20416;
    let t105162 = 3.0 * t1983 * t6878 * t105159;
    let t105165 = 6.0 * t1983 * t7753 * t20085;
    let t105167 = 3.0 * t28821 * t7756;
    let t105169 = 6.0 * t7685 * t28823;
    let t105171 = 9.0 * t7685 * t28835;
    let t105175 = 9.0 * t1983 * t97817 * t7687;
    let t105177 = 9.0 * t28821 * t7688;
    (t105150, t105162, t105165, t105167, t105169, t105171, t105175, t105177)
}
