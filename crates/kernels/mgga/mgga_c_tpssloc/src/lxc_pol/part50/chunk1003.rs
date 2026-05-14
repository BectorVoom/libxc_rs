//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1003/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1003<F: Float>(t25: F, t265: F, t394: F, t1070: F, t1637: F, t193: F, t30924: F, t30930: F, t33013: F, t33043: F, t336: F, t4700: F, t6822: F, t7627: F, t1409: F, t32907: F, t40: F, t8425: F, t28: F, t7540: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t33044 = piecewise3(t395, t1070 * t193 * t33013 * t336 - t1637 * t30924 * t4700 + 2.0 * t1637 * t30930 * t4700 - 2.0 * t4700 * t6822 * t7627, t33043);
    let t33049 = piecewise3(t115, t32907, t8425 * t1409 / 2.0 + t33044 * t40 / 2.0);
    let t33065 = t28 * t7540;
    (t33044, t33049, t33065)
}
