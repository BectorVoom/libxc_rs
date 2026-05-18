//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 154/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk154<F: Float>(t407: F, t410: F, t413: F, t417: F) -> (F, F, F) {
    let t432 = F::new(0.705945e1) * t410 + F::new(0.1549425e1) * t407 + F::new(0.420775e0) * t413 + F::new(0.1562925e0) * t417;
    let t435 = F::new(1.0) + F::new(0.32163958997385070134e2) / t432;
    let t436 = f64::ln(t435);
    (t432, t435, t436)
}
