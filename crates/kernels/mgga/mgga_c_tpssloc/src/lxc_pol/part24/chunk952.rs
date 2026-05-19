//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 952/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk952<F: Float>(t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F) -> F {
    let t10804 = F::new(0.20839e0) * t10311 - F::new(0.62517e0) * t10318 - F::cast_from(0.68863333333333333332e0_f64) * t10556 + F::cast_from(0.34431666666666666666e0_f64) * t10558 - F::new(0.103295e1) * t10560 + F::cast_from(0.51647499999999999999e0_f64) * t10562 - F::cast_from(0.57386111111111111112e0_f64) * t10566 + F::new(0.20659e1) * t10569 - F::new(0.309885e1) * t10572 - F::new(0.516475e0) * t10575 + F::new(0.3529725e1) * t10589 + F::new(0.6311625e0) * t10591 + F::cast_from(0.264729375e1_f64) * t10597 - F::cast_from(0.157790625e0_f64) * t10600;
    t10804
}
