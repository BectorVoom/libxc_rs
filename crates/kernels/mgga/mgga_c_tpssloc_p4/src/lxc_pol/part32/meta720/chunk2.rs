//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2289/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2289<F: Float>(t25: F, t265: F, t394: F, t100624: F, t1409: F, t16558: F, t2116: F, t27373: F, t29507: F, t3966: F, t40: F, t5398: F, t607: F, t7274: F, t7992: F, t99069: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t103113 = piecewise3::<F>(t395, F::new(0.0), t100624);
    let t103125 = piecewise3::<F>(t115, t99069, t103113 * t40 / F::new(2.0) + t29507 * t607 / F::new(2.0) + t27373 * t1409 + t7992 * t3966 + t7274 * t5398 / F::new(2.0) + t2116 * t16558 / F::new(2.0));
    t103125
}
