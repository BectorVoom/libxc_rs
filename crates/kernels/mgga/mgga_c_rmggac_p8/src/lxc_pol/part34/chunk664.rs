//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 664/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk664<F: Float>(t504: F, t8619: F, t874: F, t9486: F, t2447: F, t4616: F, t2227: F, t570: F, t1652: F, t698: F, t551: F, t1614: F) -> (F, F, F, F, F, F, F) {
    let t41886 = t504 * t8619;
    let t43970 = t874 * t9486;
    let t43974 = t4616 * t2447;
    let t44157 = t2227 * t570;
    let t44183 = t698 * t1652;
    let t44187 = t2227 * t551;
    let t44194 = t698 * t1614;
    (t41886, t43970, t43974, t44157, t44183, t44187, t44194)
}
