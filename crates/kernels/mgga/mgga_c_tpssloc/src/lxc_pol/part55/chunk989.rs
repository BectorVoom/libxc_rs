//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 989/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk989<F: Float>(t32726: F, t553: F, t1336: F, t1814: F, t31192: F, t31200: F, t32743: F, t32747: F, t32751: F, t32753: F, t544: F, t8483: F, t1378: F, t225: F, t567: F, t7722: F) -> (F, F, F, F) {
    let t32755 = t553 * t32726;
    let t32757 = -t1336 * t32753 + t1814 * t8483 + t32755 * t544 - t31192 - t31200 - t32743 - t32747 + t32751;
    let t32758 = t1378 * t32757;
    let t32761 = t7722 * t225 * t567;
    (t32755, t32757, t32758, t32761)
}
