//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1348/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1348<F: Float>(t136: F, t17292: F, t13598: F, t13712: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F) -> (F, F) {
    let t17293 = t136 * t17292;
    let t17295 = -F::cast_from(0.26837777777777777779e0_f64) * t13598 + t13712 + F::new(0.16557e0) * t17280 + F::cast_from(0.67094444444444444443e-1_f64) * t17149 - F::cast_from(0.20128333333333333333e0_f64) * t17165 + F::cast_from(0.10064166666666666667e0_f64) * t17175 - F::new(0.301925e0) * t17189 + F::cast_from(0.18396666666666666667e-1_f64) * t17286 - F::new(0.11038e0) * t17288 + F::new(0.5519e-1) * t17290 - F::new(0.82785e-1) * t17293;
    (t17293, t17295)
}
