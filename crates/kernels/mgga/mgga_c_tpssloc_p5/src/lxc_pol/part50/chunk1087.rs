//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1087/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1087<F: Float>(t214: F, t32748: F, t1985: F, t1825: F, t31211: F, t32726: F, t553: F, t1336: F, t1814: F, t31192: F, t31200: F, t32743: F, t32747: F, t544: F, t8483: F) -> (F, F, F, F) {
    let t32749 = t214 * t32748;
    let t32751 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t32749;
    let t32753 = t31211 * t1825;
    let t32755 = t553 * t32726;
    let t32757 = -t1336 * t32753 + t1814 * t8483 + t32755 * t544 - t31192 - t31200 - t32743 - t32747 + t32751;
    (t32749, t32753, t32755, t32757)
}
