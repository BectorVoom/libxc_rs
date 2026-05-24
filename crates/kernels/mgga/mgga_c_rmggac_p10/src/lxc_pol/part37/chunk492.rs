//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 492/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk492<F: Float>(t13980: F, t638: F, t639: F, t2127: F, t640: F, t3080: F, t321: F, t262: F, t7204: F, t333: F, t7192: F, t2060: F, t2123: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13982 = t638 * t639 * t13980;
    let t13984 = t640 * t2127;
    let t13986 = t638 * t639 * t13984;
    let t13988 = t3080 * t321;
    let t13989 = t262 * t13988;
    let t13990 = t7204 * t13989;
    let t13992 = t3080 * t333;
    let t13993 = t262 * t13992;
    let t13994 = t7192 * t13993;
    let t13996 = t2060 * t2123;
    (t13982, t13984, t13986, t13988, t13989, t13990, t13992, t13993, t13994, t13996)
}
