//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1240/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1240<F: Float>(t33476: F, t776: F, t114992: F, t115009: F, t118387: F, t1877: F, t22960: F, t24339: F, t25024: F, t25028: F, t2522: F, t25366: F, t25377: F, t25385: F, t26563: F, t26744: F, t31430: F, t31434: F, t31451: F, t33486: F, t7114: F, t7475: F, t7545: F, t8566: F) -> (F, F) {
    let t121818 = t33476 * t776;
    let t121833 = -t1877 * t7114 * t118387 / 2.0 + 3.0 / 2.0 * t2522 * t8566 * t25028 + 3.0 / 2.0 * t2522 * t8566 * t25385 - t1877 * t24339 * t33486 / 2.0 - t1877 * t26744 * t31451 / 2.0 + 3.0 / 2.0 * t2522 * t8566 * t25024 - 3.0 * t26563 * t22960 * t121818 + 3.0 / 2.0 * t2522 * t31430 * t7475 - t1877 * t114992 * t7545 / 2.0 - 3.0 / 2.0 * t115009 * t25366 - t1877 * t31434 * t25377 / 2.0;
    (t121818, t121833)
}
