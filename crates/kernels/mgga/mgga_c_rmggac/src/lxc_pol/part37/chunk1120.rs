//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1120/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1120<F: Float>(t76517: F, t1540: F, t3292: F, t70556: F, t70574: F, t70578: F, t76539: F, t76542: F, t78567: F, t78571: F, t78574: F, t78575: F, t78576: F, t78577: F, t78578: F, t78582: F, t78585: F, t78588: F) -> F {
    let t80530 = F::cast_from(0.16566831523319392754e-1_f64) * t76517;
    let t80534 = t78567 + t78571 - t80530 + t78574 - t78575 - t78576 + t78577 - t78578 + F::cast_from(0.40878380883436523435e-5_f64) * t70556 + t70574 + t70578 - F::cast_from(0.19957069503106347607e-1_f64) * t1540 * t3292 + t78582 + t76539 - t76542 - t78585 - t78588;
    t80534
}
