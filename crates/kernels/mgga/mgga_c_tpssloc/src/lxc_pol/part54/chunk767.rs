//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 767/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk767<F: Float>(t25: F, t265: F, t394: F, t1484: F, t2057: F, t202: F, t7844: F, t1530: F, t1877: F, t193: F, t2522: F, t7114: F, t870: F, t1408: F, t1409: F, t2064: F, t40: F, t7545: F, t7809: F, t7845: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7856 = t2057 * t1484;
    let t7859 = t202 * t7844;
    let t7864 = -t1530 * t1877 * t7114 + t193 * t7859 * t870 + 3.0 * t2522 * t7856;
    let t7865 = piecewise3(t395, 0.0, t7864);
    let t7870 = piecewise3(t115, 3.0 / 2.0 * t2522 * t7809 + t1877 * t7845 * t25 / 2.0 - t1877 * t7114 * t7545 / 2.0 + t1877 * t2057 * t1408 / 2.0, t2064 * t1409 / 2.0 + t7865 * t40 / 2.0);
    (t7859, t7864, t7865, t7870)
}
