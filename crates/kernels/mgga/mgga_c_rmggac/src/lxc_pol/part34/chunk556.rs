//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 556/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk556<F: Float>(t13935: F, t13938: F, t13943: F, t13903: F, t13906: F, t13929: F, t13932: F, t13941: F, t14476: F, t14477: F, t14478: F, t14481: F, t14482: F, t14483: F, t14484: F, t14485: F, t14486: F, t14487: F) -> (F, F, F) {
    let t14490 = F::new(0.48384206071776340879e-3) * t13935;
    let t14491 = F::new(0.14464861606874801909e-3) * t13938;
    let t14493 = F::new(0.12857654761666490586e-3) * t13943;
    let t14494 = t14476 - t14477 - t14478 - F::new(0.68186654135613354322e-2) * t13903 + F::new(0.13637330827122670864e-1) * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - F::new(0.45360193192290319574e-3) * t13929 + F::new(0.63504270469206447404e-3) * t13932 + t14490 + t14491 - F::new(0.19286482142499735878e-3) * t13941 - t14493;
    (t14490, t14493, t14494)
}
