//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 575/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk575<F: Float>(t265: F, t394: F, t1052: F, t1604: F, t1626: F, t1635: F, t388: F, t1070: F, t1534: F, t1545: F, t1559: F, t1585: F, t1587: F, t1591: F, t193: F, t336: F) -> (F, F) {
    let t395 = t265 < t394;
    let t1637 = -t1052 * t1635 + t1604 * t388 + t1626 * t388;
    let t1642 = piecewise3::<F>(t395, t1070 * t1637 * t193 * t336 - t1545 + t1559 + t1585 + t1587 - t1591, t1534);
    (t1637, t1642)
}
