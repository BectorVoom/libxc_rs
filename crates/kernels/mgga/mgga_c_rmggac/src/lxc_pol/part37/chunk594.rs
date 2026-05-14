//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 594/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk594<F: Float>(t2123: F, t558: F, t35959: F, t3839: F, t3851: F, t22: F, t235: F, t34812: F, t504: F, t8619: F, t874: F, t9486: F, t2447: F, t4616: F, t2227: F, t570: F) -> (F, F, F, F, F, F, F, F) {
    let t41122 = t2123 * t558;
    let t41400 = t3839 * t35959;
    let t41407 = t3851 * t35959;
    let t41738 = t235 * t34812 * t22;
    let t41886 = t504 * t8619;
    let t43970 = t874 * t9486;
    let t43974 = t4616 * t2447;
    let t44157 = t2227 * t570;
    (t41122, t41400, t41407, t41738, t41886, t43970, t43974, t44157)
}
