//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1282/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1282<F: Float>(t25: F, t265: F, t394: F, t7642: F, t1409: F, t2116: F, t40: F, t7552: F, t1419: F, t337: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t7992 = piecewise3::<F>(t395, F::new(0.0), t7642);
    let t7997 = piecewise3::<F>(t115, t7552, t2116 * t1409 / F::new(2.0) + t7992 * t40 / F::new(2.0));
    let t7998 = t1419 * t337;
    let t7999 = t7998 * t1887;
    (t7992, t7997, t7998, t7999)
}
