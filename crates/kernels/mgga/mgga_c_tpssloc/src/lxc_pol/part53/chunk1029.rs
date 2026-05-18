//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1029/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1029<F: Float>(t28: F, t265: F, t504: F, t123836: F, t123888: F, t123938: F, t1409: F, t32102: F, t34061: F, t3966: F, t52: F, t607: F, t8770: F, t33853: F, t532: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t123940 = piecewise3::<f64>(t505, F::new(0.0), t123836);
    let t123947 = piecewise3::<f64>(t401, t123888 + t123938, t123940 * t52 / F::new(2.0) - t32102 * t1409 / F::new(2.0) - t34061 * t607 / F::new(2.0) - t8770 * t3966 / F::new(2.0));
    let t123975 = t532 * t33853;
    (t123947, t123975)
}
