//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 949/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk949<F: Float>(t70442: F, t70444: F, t70479: F, t71832: F, t76137: F, t76492: F, t76495: F, t76497: F, t76499: F, t78502: F, t78503: F, t78504: F, t78514: F, t78518: F, t78522: F, t78526: F, t78528: F) -> (F,) {
    let t80527 = -t78502 - t78503 + t78504 + 0.58171619854173713844e-5 * t76137 - t71832 + t70442 - t70444 + 0.76860658247009135562e-5 * t76492 - t78514 - t78518 - t78522 - t78526 - t76495 - t76497 + t70479 - 0.35038612185802734374e-6 * t76499 - t78528;
    (t80527,)
}
