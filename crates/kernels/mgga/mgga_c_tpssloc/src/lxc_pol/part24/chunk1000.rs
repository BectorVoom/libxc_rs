//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1000/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1000<F: Float>(t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11259: F, t11261: F, t11266: F) -> F {
    let t11343 = -F::new(0.104195e0) * t11230 + F::new(0.62517e0) * t11233 + F::new(0.68863333333333333332e0) * t11137 + F::new(0.34431666666666666666e0) * t11139 - F::new(0.103295e1) * t11141 - F::new(0.51647499999999999999e0) * t11143 + F::new(0.57386111111111111112e0) * t11150 - F::new(0.20659e1) * t11156 + F::new(0.309885e1) * t11165 + F::new(0.516475e0) * t11174 - F::new(0.157790625e0) * t11245 + F::new(0.3529725e1) * t11259 + F::new(0.6311625e0) * t11261 + F::new(0.264729375e1) * t11266;
    t11343
}
