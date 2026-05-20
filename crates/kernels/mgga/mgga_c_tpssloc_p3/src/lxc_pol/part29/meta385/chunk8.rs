//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1569/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1569<F: Float>(t25: F, t265: F, t394: F, t13493: F, t14666: F, t14673: F, t1074: F, t12606: F, t13503: F, t13504: F, t13506: F, t1408: F, t1409: F, t1534: F, t1642: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t606: F, t607: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t14675 = piecewise3::<F>(t395, t14666 + t14673, t13493);
    let t14687 = piecewise3::<F>(t115, t13493 * t25 / F::new(2.0) + t4324 * t606 + t1534 * t2249 / F::new(2.0) + t2756 * t1408 / F::new(2.0) + t13503 + t13504 - t13506, t14675 * t40 / F::new(2.0) + t4705 * t607 + t1642 * t2250 / F::new(2.0) + t3220 * t1409 / F::new(2.0) + t1074 * t3966 + t396 * t12606 / F::new(2.0));
    t14687
}
