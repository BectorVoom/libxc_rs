//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 929/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk929<F: Float>(t25: F, t265: F, t394: F, t21076: F, t21381: F, t21701: F, t1408: F, t1409: F, t1534: F, t1642: F, t20216: F, t20217: F, t396: F, t40: F, t5397: F, t5398: F, t5669: F, t5955: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t21703 = piecewise3::<F>(t395, t21381 + t21701, t21076);
    let t21713 = piecewise3::<F>(t115, t21076 * t25 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t5669 * t1408 + F::new(3.0) / F::new(2.0) * t1534 * t5397 + t265 * t20216 / F::new(2.0), t21703 * t40 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t5955 * t1409 + F::new(3.0) / F::new(2.0) * t1642 * t5398 + t396 * t20217 / F::new(2.0));
    t21713
}
