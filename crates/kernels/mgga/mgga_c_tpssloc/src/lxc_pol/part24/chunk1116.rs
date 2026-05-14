//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1116/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1116<F: Float>(t25: F, t265: F, t394: F, t1068: F, t1070: F, t193: F, t23734: F, t23738: F, t23742: F, t23772: F, t3209: F, t3213: F, t336: F, t4700: F, t6822: F, t1965: F, t2250: F, t23309: F, t40: F, t607: F, t6835: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t23773 = piecewise3(t395, t1070 * t193 * t23734 * t336 - 2.0 * t1068 * t23738 * t4700 + 2.0 * t23742 * t3213 * t4700 - t3209 * t4700 * t6822, t23772);
    let t23780 = piecewise3(t115, t23309, t23773 * t40 / 2.0 + t6835 * t607 + t1965 * t2250 / 2.0);
    (t23773, t23780)
}
