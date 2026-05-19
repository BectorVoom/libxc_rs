//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1372/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1372<F: Float>(t25: F, t265: F, t394: F, t119677: F, t118965: F, t1409: F, t31823: F, t33750: F, t3966: F, t40: F, t607: F, t8678: F, t24932: F, t7467: F, t27888: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t123037 = piecewise3::<F>(t395, F::new(0.0), t119677);
    let t123044 = piecewise3::<F>(t115, t118965, t123037 * t40 / F::new(2.0) + t31823 * t1409 / F::new(2.0) + t33750 * t607 / F::new(2.0) + t8678 * t3966 / F::new(2.0));
    let t123050 = t24932 * t7467;
    let t123052 = t27888 * t7467;
    (t123044, t123050, t123052)
}
