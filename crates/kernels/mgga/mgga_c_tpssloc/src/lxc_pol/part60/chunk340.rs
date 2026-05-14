//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 340/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk340<F: Float>(t25: F, t265: F, t394: F, t1964: F, t1918: F, t40: F, t337: F, t50: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t2116 = piecewise3(t395, 0.0, t1964);
    let t2119 = piecewise3(t115, t1918, t2116 * t40 / 2.0);
    let t2120 = t50 * t337;
    let t2121 = t2120 * t1887;
    (t2116, t2119, t2120, t2121)
}
