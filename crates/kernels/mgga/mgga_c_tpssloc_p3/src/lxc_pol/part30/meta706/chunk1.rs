//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2321/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2321<F: Float>(t25: F, t265: F, t394: F, t100578: F, t100623: F, t100528: F, t1409: F, t16558: F, t1965: F, t25883: F, t28756: F, t3966: F, t40: F, t5398: F, t607: F, t6835: F, t7643: F, t99069: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t100624 = t100578 + t100623;
    let t100625 = piecewise3::<F>(t395, t100528, t100624);
    let t100637 = piecewise3::<F>(t115, t99069, t100625 * t40 / F::new(2.0) + t28756 * t607 / F::new(2.0) + t25883 * t1409 + t7643 * t3966 + t6835 * t5398 / F::new(2.0) + t1965 * t16558 / F::new(2.0));
    (t100624, t100637)
}
