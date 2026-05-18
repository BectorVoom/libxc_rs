//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1374/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1374<F: Float>(t19349: F, t62342: F, t1791: F, t65208: F, t1675: F, t18645: F, t6090: F, t18350: F, t18673: F, t19342: F, t24290: F, t62281: F, t62285: F, t62294: F, t62339: F, t62351: F, t65169: F, t65172: F, t65175: F, t65178: F, t7690: F) -> F {
    let t67369 = F::new(160.0) / F::new(9.0) * t19349 * t62342;
    let t67378 = t1791 * t65208;
    let t67385 = t1675 * t18645 * t6090;
    let t67387 = F::new(80.0) / F::new(9.0) * t62281 + F::new(80.0) / F::new(9.0) * t62285 - F::new(20.0) * t65178 * t62339 - t67369 + F::new(20.0) / F::new(3.0) * t65169 * t18673 + F::new(20.0) / F::new(3.0) * t65172 * t18673 + F::new(20.0) / F::new(3.0) * t65175 * t18673 + F::new(20.0) / F::new(3.0) * t19349 * t62351 + F::new(20.0) / F::new(3.0) * t18350 * t67378 - t62294 - F::new(40.0) * t7690 * t24290 * t19342 + F::new(88.0) / F::new(27.0) * t67385;
    t67387
}
