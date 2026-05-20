//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1106/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1106<F: Float>(t22748: F, t80727: F, t22723: F, t268: F, t534: F, t22706: F, t22695: F, t22704: F, t22705: F, t3719: F, t562: F, t1307: F, t26331: F, t26446: F) -> (F, F, F, F, F) {
    let t81043 = t80727 * t22748;
    let t81046 = t22723 * t534 * t268;
    let t81047 = t81046 * t22706;
    let t81050 = t22704 * t22705 * t22695;
    let t81052 = t562 * t3719;
    let t81055 = t26331 * t26446 * t81052 * t1307;
    (t81043, t81047, t81050, t81052, t81055)
}
