//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1188/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1188<F: Float>(t24165: F, t532: F, t12030: F, t2092: F, t24088: F, t24092: F, t3758: F, t39913: F, t7214: F, t80678: F, t80683: F, t80687: F, t80689: F, t80709: F, t80711: F, t80714: F) -> (F, F) {
    let t84347 = t532 * t24165;
    let t84389 = F::cast_from(0.29608813203268075857e0_f64) * t80678 - F::cast_from(0.14804406601634037928e0_f64) * t80683 - F::cast_from(0.49348022005446793095e-1_f64) * t80687 + F::cast_from(0.11514538467937585055e0_f64) * t80689 - F::new(3.0) * t12030 * t7214 - F::new(18.0) * t3758 * t24092 - F::cast_from(0.49348022005446793095e-1_f64) * t80709 - F::cast_from(0.15626873635058151147e0_f64) * t80711 - F::cast_from(0.9869604401089358619e-1_f64) * t80714 + F::new(6.0) * t3758 * t24088 - F::new(3.0) * t39913 * t2092;
    (t84347, t84389)
}
