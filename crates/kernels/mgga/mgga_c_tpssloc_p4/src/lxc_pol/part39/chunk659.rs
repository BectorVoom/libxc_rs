//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 659/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk659<F: Float>(t25: F, t265: F, t394: F, t2756: F, t3219: F, t1074: F, t2249: F, t2250: F, t396: F, t40: F, t606: F, t607: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t3220 = piecewise3::<F>(t395, t3219, t2756);
    let t3227 = piecewise3::<F>(t115, t2756 * t25 / F::new(2.0) + t873 * t606 + t265 * t2249 / F::new(2.0), t3220 * t40 / F::new(2.0) + t1074 * t607 + t396 * t2250 / F::new(2.0));
    (t3220, t3227)
}
