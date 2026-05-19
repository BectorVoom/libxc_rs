//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1104/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1104<F: Float>(t13563: F, t10296: F, t10298: F, t10302: F, t13566: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F) -> (F, F) {
    let t13679 = F::cast_from(0.13418888888888888889e0_f64) * t13563;
    let t13692 = -F::cast_from(0.40256666666666666667e0_f64) * t13566 - F::cast_from(0.33547222222222222222e0_f64) * t13569 + F::new(0.12077e1) * t13572 - F::cast_from(0.40256666666666666666e0_f64) * t13575 - F::cast_from(0.20128333333333333333e0_f64) * t13578 - F::new(0.181155e1) * t13581 + F::new(0.12077e1) * t13584 + F::new(0.60385e0) * t13587 - F::cast_from(0.18396666666666666667e0_f64) * t10296 + F::new(0.5519e-1) * t10302 + F::cast_from(0.18396666666666666667e-1_f64) * t10298;
    (t13679, t13692)
}
