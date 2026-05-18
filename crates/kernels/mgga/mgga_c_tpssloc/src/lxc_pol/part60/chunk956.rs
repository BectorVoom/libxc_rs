//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 956/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk956<F: Float>(t1799: F, t2085: F, t22704: F, t22705: F, t33280: F, t33281: F, t6914: F, t1338: F, t33266: F, t33285: F, t6883: F, t33284: F, t6897: F, t794: F) -> (F, F, F, F, F, F) {
    let t122448 = t2085 * t1799;
    let t122460 = t22704 * t22705 * t33280;
    let t122462 = t6914 * t33281;
    let t122475 = t1338 * t33266;
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    (t122448, t122460, t122462, t122475, t122503, t122507)
}
