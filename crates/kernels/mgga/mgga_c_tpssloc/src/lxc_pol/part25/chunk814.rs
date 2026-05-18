//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 814/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk814<F: Float>(t10544: F, t2798: F, t2807: F, t896: F, t2815: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10542: F) -> (F, F, F) {
    let t10545 = F::new(0.93932222222222222223e0) * t10544;
    let t10547 = t2798 * t896 * t2807;
    let t10550 = t2815 * t896 * t2807;
    let t10553 = -F::new(0.60384999999999999999e0) * t10530 - F::new(0.27595e0) * t10296 + F::new(0.16557e0) * t10302 + F::new(0.5519e-1) * t10298 - F::new(0.36793333333333333333e-1) * t10307 - F::new(0.82785e-1) * t10323 + F::new(0.181155e1) * t10538 - F::new(0.82785e-1) * t10314 + F::new(0.49671e0) * t10320 - t10542 - t10545 - F::new(0.3883875e1) * t10547 + F::new(0.247573125e0) * t10550 - F::new(0.33114e0) * t10300;
    (t10547, t10550, t10553)
}
