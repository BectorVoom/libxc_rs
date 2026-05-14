//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1160/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1160<F: Float>(t22706: F, t81046: F, t22695: F, t22704: F, t22705: F, t3719: F, t562: F, t1307: F, t26331: F, t26446: F, t1992: F, t550: F, t6976: F, t81028: F, t22863: F, t6979: F) -> (F, F, F, F, F, F) {
    let t81047 = t81046 * t22706;
    let t81050 = t22704 * t22705 * t22695;
    let t81052 = t562 * t3719;
    let t81055 = t26331 * t26446 * t81052 * t1307;
    let t81059 = t1992 * t6976 * t81028 * t550;
    let t81061 = t22863 * t6979;
    (t81047, t81050, t81052, t81055, t81059, t81061)
}
