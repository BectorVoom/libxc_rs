//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1372/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1372<F: Float>(t18347: F, t18649: F, t19404: F, t5785: F, t6077: F, t62247: F, t62250: F, t62311: F, t62330: F, t65285: F, t65293: F, t67326: F, t67331: F, t67333: F, t67335: F, t67337: F) -> F {
    let t67342 = -F::new(5.0) / F::new(3.0) * t62311 * t6077 - F::new(10.0) / F::new(3.0) * t18649 * t19404 + F::new(10.0) / F::new(3.0) * t62330 * t6077 - F::new(5.0) / F::new(3.0) * t5785 * t65285 + F::new(10.0) * t67326 * t18347 + t67331 + t67333 + t67335 + t67337 - F::new(5.0) / F::new(3.0) * t5785 * t65293 + F::new(16.0) / F::new(9.0) * t62247 - F::new(8.0) / F::new(9.0) * t62250;
    t67342
}
