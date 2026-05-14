//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 917/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk917<F: Float>(t20861: F, t819: F, t820: F, t20853: F, t232: F, t5527: F, t4181: F, t9646: F, t16839: F, t2645: F, t5591: F, t1484: F, t2632: F, t5611: F, t4180: F, t119: F, t20800: F) -> (F, F, F, F, F, F, F, F) {
    let t20963 = t819 * t820 * t20861;
    let t20969 = t819 * t820 * t20853;
    let t20972 = t232 * t5527;
    let t20974 = t9646 * t4181 * t20972;
    let t20978 = t2645 * t16839 * t5591;
    let t20981 = t2632 * t1484;
    let t20983 = t2645 * t16839 * t20981;
    let t20986 = t2632 * t5611;
    let t20988 = t4180 * t4181 * t20986;
    let t20993 = t119 * t20800;
    (t20963, t20969, t20974, t20978, t20983, t20986, t20988, t20993)
}
