//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 185/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk185<F: Float>(t608: F, t65: F, t34: F, t36: F, t43: F, t607: F, t55: F, t583: F, t61: F, t59: F, t39: F, t44: F, t51: F, t33: F, t40: F, t73: F, rho0: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t609 = t608 * t65;
    let t612 = t34 * rho0;
    let t614 = 1.0 / t36 / t612;
    let t615 = sigma0 * t614;
    let t618 = t43 * t607;
    let t621 = t55 * t607;
    let t625 = 1.0 / t61 / t583;
    let t626 = t59 * t625;
    let t627 = 8.0 / 3.0 * t626;
    let t628 = -8.0 / 3.0 * t615 * t44 + 5.0 / 6.0 * t39 * t618 - 5.0 / 6.0 * t51 * t621 + t627;
    let t629 = t33 * t628;
    let t632 = t40 * t40;
    let t634 = 1.0 / t73 / t632;
    (t609, t615, t621, t625, t626, t627, t628, t629, t632, t634)
}
