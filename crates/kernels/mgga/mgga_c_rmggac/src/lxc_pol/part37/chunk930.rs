//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 930/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk930<F: Float>(t80351: F, t80355: F, t80366: F, t80370: F, t235: F, t515: F, t70063: F, t70101: F, t71670: F, t71671: F, t71672: F, t78352: F, t78355: F, t78359: F, t78362: F, t78364: F, t78368: F, t78371: F, t78372: F, t78375: F, t80347: F, t80349: F) -> (F, F) {
    let t80372 = t80351 + t80355 + t80366 + t80370;
    let t80376 = -t78352 - t78355 + t80347 - 0.91976356987732177729e-5 * t70063 - t80349 - t71670 - t71671 - t71672 + t78359 - t70101 + t78362 + t78364 - t78368 - 0.19957069503106347607e-1 * t235 * t515 * t80372 + t78371 - t78372 - t78375;
    (t80372, t80376)
}
