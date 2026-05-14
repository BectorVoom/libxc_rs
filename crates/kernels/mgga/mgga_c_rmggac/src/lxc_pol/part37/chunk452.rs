//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 452/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk452<F: Float>(t13984: F, t638: F, t639: F, t3080: F, t321: F, t262: F, t7204: F, t333: F, t7192: F, t2060: F, t2123: F, t739: F, t352: F, t8620: F, t13862: F, t335: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13986 = t638 * t639 * t13984;
    let t13988 = t3080 * t321;
    let t13989 = t262 * t13988;
    let t13990 = t7204 * t13989;
    let t13992 = t3080 * t333;
    let t13993 = t262 * t13992;
    let t13994 = t7192 * t13993;
    let t13996 = t2060 * t2123;
    let t13997 = t739 * t13996;
    let t13998 = 0.2993560425465952141e-1 * t13997;
    let t14003 = t3080 * t352;
    let t14004 = t262 * t14003;
    let t14005 = t8620 * t14004;
    let t14007 = t13862 * t335;
    (t13986, t13988, t13989, t13990, t13992, t13993, t13994, t13996, t13998, t14003, t14004, t14005, t14007)
}
