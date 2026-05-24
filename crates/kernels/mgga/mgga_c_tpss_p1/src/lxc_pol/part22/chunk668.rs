//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 668/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk668<F: Float>(t33: F, t259: F, t479: F, t2445: F, t3157: F, t1006: F, t1157: F, t1992: F, t2829: F, t481: F, t57: F, t581: F, t826: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t3158 = piecewise3::<F>(t480, t3157, t2445);
    let t3165 = piecewise3::<F>(t386, t2445 * t33 / F::new(2.0) + t826 * t1006 + t259 * t2829 / F::new(2.0), t3158 * t57 / F::new(2.0) - t1157 * t581 - t481 * t1992 / F::new(2.0));
    (t3158, t3165)
}
