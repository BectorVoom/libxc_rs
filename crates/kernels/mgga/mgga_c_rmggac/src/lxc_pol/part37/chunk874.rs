//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 874/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk874<F: Float>(t73746: F, t73749: F, t73764: F, t73787: F, t76628: F, t76631: F, t76632: F, t76633: F, t76634: F, t76635: F, t76637: F, t76638: F, t76639: F, t76640: F, t76641: F, t76642: F, t76643: F) -> (F,) {
    let t79976 = -t76628 - 0.43798265232253417968e-6 * t73746 - 0.35038612185802734374e-6 * t73749 - t76631 + t76632 - t76633 - t76634 - t76635 - 0.52557918278704101561e-6 * t73764 + t76637 - t76638 - t76639 + t76640 - t76641 - t76642 + t76643 - 0.87596530464506835932e-6 * t73787;
    (t79976,)
}
