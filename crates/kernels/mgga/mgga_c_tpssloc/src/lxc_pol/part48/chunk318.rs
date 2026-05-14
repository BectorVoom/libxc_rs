//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 318/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk318<F: Float>(t25: F, t28: F, t265: F, t394: F, t504: F, t2057: F, t202: F, t2056: F, t193: F, t870: F, t1877: F, t40: F, t52: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t2058 = t2057 * t25;
    let t2061 = t202 * t2056;
    let t2063 = t193 * t2061 * t870;
    let t2064 = piecewise3(t395, 0.0, t2063);
    let t2067 = piecewise3(t115, t1877 * t2058 / 2.0, t2064 * t40 / 2.0);
    let t2068 = t2057 * t28;
    let t2071 = piecewise3(t505, 0.0, t2063);
    let t2074 = piecewise3(t401, t1877 * t2068 / 2.0, t2071 * t52 / 2.0);
    let t2075 = t2067 + t2074;
    (t2061, t2064, t2071, t2075)
}
