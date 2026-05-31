//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 929/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk929<F: Float>(t10544: F, t2798: F, t2807: F, t896: F, t2815: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10542: F) -> (F, F, F) {
    let t10545 = F::cast_from(0.93932222222222222223e0_f64) * t10544;
    let t10547 = t2798 * t896 * t2807;
    let t10550 = t2815 * t896 * t2807;
    let t10553 = -F::cast_from(0.60384999999999999999e0_f64) * t10530 - F::cast_from(0.27595e0_f64) * t10296 + F::cast_from(0.16557e0_f64) * t10302 + F::cast_from(0.5519e-1_f64) * t10298 - F::cast_from(0.36793333333333333333e-1_f64) * t10307 - F::cast_from(0.82785e-1_f64) * t10323 + F::cast_from(0.181155e1_f64) * t10538 - F::cast_from(0.82785e-1_f64) * t10314 + F::cast_from(0.49671e0_f64) * t10320 - t10542 - t10545 - F::cast_from(0.3883875e1_f64) * t10547 + F::cast_from(0.247573125e0_f64) * t10550 - F::cast_from(0.33114e0_f64) * t10300;
    (t10547, t10550, t10553)
}
