//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1834/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1834<F: Float>(t13550: F, t13563: F, t10296: F, t10298: F, t10302: F, t13566: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F) -> (F, F, F) {
    let t14287 = F::cast_from(0.27785333333333333334e0_f64) * t13550;
    let t14291 = F::cast_from(0.22954444444444444444e0_f64) * t13563;
    let t14304 = -F::cast_from(0.68863333333333333333e0_f64) * t13566 - F::cast_from(0.57386111111111111112e0_f64) * t13569 + F::new(0.20659e1) * t13572 - F::cast_from(0.68863333333333333334e0_f64) * t13575 - F::cast_from(0.34431666666666666667e0_f64) * t13578 - F::new(0.309885e1) * t13581 + F::new(0.20659e1) * t13584 + F::new(0.103295e1) * t13587 - F::cast_from(0.23154444444444444444e0_f64) * t10296 + F::cast_from(0.69463333333333333333e-1_f64) * t10302 + F::cast_from(0.23154444444444444444e-1_f64) * t10298;
    (t14287, t14291, t14304)
}
