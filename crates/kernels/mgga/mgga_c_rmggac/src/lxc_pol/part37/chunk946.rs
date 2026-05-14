//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 946/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk946<F: Float>(t70321: F, t72: F, t73484: F, t739: F, t76036: F, t78439: F, t78440: F, t78444: F, t78446: F, t78451: F, t78454: F, t78457: F, t78462: F, t78464: F, t78465: F, t78469: F, t80402: F, t80407: F, t80413: F, t80421: F, t80426: F, t80433: F, t80442: F, t80449: F, t80462: F, t80466: F, t80472: F, t80477: F, t80478: F, t80482: F, t80485: F, t80489: F, t80493: F, t80497: F, t82: F, t884: F) -> (F,) {
    let t80509 = -t78439 + t78440 + t78444 + t72 * t82 * (t80407 + t80413 + t80421 + t80426 + t80433 + t80442 + t80449 + t80462 + t80466 + t80472 + t80477 + t80482 + t80485 + t80489 + t80493 + t80497) + t78446 - t78451 - 0.17519306092901367186e-5 * t76036 + t78454 - t78457 - t73484 + t78462 - t70321 - t78464 - t78465 + t78469 - 0.59871208509319042821e-1 * t739 * t80402 + 0.59871208509319042821e-1 * t884 * t80478;
    (t80509,)
}
