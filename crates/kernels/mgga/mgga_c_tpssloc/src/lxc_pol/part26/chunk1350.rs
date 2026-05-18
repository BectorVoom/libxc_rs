//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1350/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1350<F: Float>(t225: F, t24873: F, t1235: F, t7319: F, t24705: F, t491: F, t24574: F, t24639: F, t24568: F, t24634: F, t1090: F, t11918: F, t1238: F, t1252: F, t2154: F, t2155: F, t24589: F, t24601: F, t24868: F, t24880: F, t3487: F, t3598: F, t3600: F, t45375: F, t7283: F, t7287: F, t7300: F, t7301: F, t85687: F) -> F {
    let t85717 = t24873 * t225;
    let t85724 = t7319 * t1235;
    let t85728 = t24705 * t491;
    let t85733 = t24574 * t24639;
    let t85739 = t24574 * t24568;
    let t85741 = t24574 * t24634;
    let t85749 = F::new(6.0) * t24880 * t3600 - F::new(6.0) * t85717 * t1252 - F::new(0.16449340668482264365e-1) * t24589 * t24601 * t85687 * t1090 + F::new(0.16449340668482264365e-1) * t24589 * t85724 * t7287 + F::new(0.82246703342411321826e-2) * t24589 * t85728 * t7287 - t45375 * t2155 + F::new(0.16449340668482264365e-1) * t85733 - F::new(0.82246703342411321825e-2) * t7283 * t7300 * t7301 * t11918 - F::new(0.16449340668482264365e-1) * t85739 - F::new(0.54831135561607547883e-2) * t85741 + F::new(2.0) * t1238 * t3598 * t2154 * t11918 - F::new(3.0) * t3487 * t24868;
    t85749
}
