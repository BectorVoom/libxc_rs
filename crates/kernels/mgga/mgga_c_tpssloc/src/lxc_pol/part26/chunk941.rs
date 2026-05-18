//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 941/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk941<F: Float>(t25: F, t265: F, t394: F, t10150: F, t11098: F, t11103: F, t1074: F, t2249: F, t2250: F, t2756: F, t3220: F, t396: F, t40: F, t606: F, t607: F, t873: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> F {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t11105 = piecewise3::<f64>(t395, t11098 + t11103, t10150);
    let t11115 = piecewise3::<f64>(t115, t10150 * t25 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2756 * t606 + F::new(3.0) / F::new(2.0) * t873 * t2249 + t265 * t9257 / F::new(2.0), t11105 * t40 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t3220 * t607 + F::new(3.0) / F::new(2.0) * t1074 * t2250 + t396 * t9258 / F::new(2.0));
    t11115
}
