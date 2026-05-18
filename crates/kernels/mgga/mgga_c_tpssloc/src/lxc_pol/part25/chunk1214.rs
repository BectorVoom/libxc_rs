//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1214/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1214<F: Float>(t225: F, t24200: F, t10049: F, t24297: F, t24314: F, t24325: F, t2597: F, t2713: F, t2718: F, t2720: F, t2742: F, t7092: F, t7106: F, t82129: F, t82131: F, t82135: F, t82138: F, t855: F, t866: F) -> F {
    let t85079 = t24200 * t225;
    let t85093 = -F::new(18.0) * t2713 * t24314 + F::new(6.0) * t10049 * t7092 + F::new(0.9869604401089358619e-1) * t82129 + F::new(12.0) * t2713 * t24325 - F::new(3.0) * t85079 * t866 + F::new(6.0) * t24297 * t2720 + F::new(6.0) * t855 * t2718 * t7106 * t2742 - F::new(0.11514538467937585055e0) * t82131 + F::new(0.49348022005446793095e-1) * t82135 - F::new(0.9869604401089358619e-1) * t82138 - F::new(18.0) * t2597 * t24314;
    t85093
}
