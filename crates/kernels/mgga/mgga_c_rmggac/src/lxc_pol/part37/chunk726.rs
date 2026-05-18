//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 726/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk726<F: Float>(t70500: F, t7553: F, t7555: F, t290: F, t70499: F, t2012: F, t7349: F, t2019: F, t640: F, t68788: F, t7764: F, t2010: F, t7755: F) -> (F, F, F, F, F, F) {
    let t70502 = t7553 * t7555 * t70500;
    let t70504 = t290 * t70499;
    let t70506 = t7349 * t2012 * t70504;
    let t70510 = t2019 * t7764 * t640 * t68788;
    let t70512 = t290 * t68788;
    let t70514 = t2010 * t7755 * t70512;
    (t70502, t70504, t70506, t70510, t70512, t70514)
}
