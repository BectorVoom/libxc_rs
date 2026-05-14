//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 963/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk963<F: Float>(t25: F, t265: F, t394: F, t1873: F, t6534: F, t3941: F, t3938: F, t8326: F, t671: F, t649: F, t89: F, t88: F, t30952: F, t30776: F, t40: F, t607: F, t8678: F, t191: F, t192: F, t7412: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t31280 = t1873 * t6534;
    let t31282 = 54.0 * t3941 * t31280;
    let t31283 = t3938 * t8326;
    let t31284 = 0.135e2 * t31283;
    let t31285 = t8326 * t671;
    let t31286 = t3941 * t31285;
    let t31287 = 27.0 * t31286;
    let t31537 = t649 * t1873;
    let t31540 = t89 * t6534;
    let t31717 = t88 * t6534;
    let t31823 = piecewise3(t395, 0.0, t30952);
    let t31828 = piecewise3(t115, t30776, t31823 * t40 / 2.0 + t8678 * t607 / 2.0);
    let t31832 = t7412 * t191 * t192;
    (t31280, t31282, t31284, t31285, t31287, t31537, t31540, t31717, t31823, t31828, t31832)
}
