//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1382/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1382<F: Float>(t1795: F, t2105: F, t10456: F, t1165: F, t13146: F, t1338: F, t18898: F, t20289: F, t20294: F, t20319: F, t2056: F, t3537: F, t4347: F, t62230: F, t6323: F, t645: F, t67250: F, t67316: F, t67519: F, t67538: F, t67541: F) -> (F, F) {
    let t67552 = t1795 * t2105;
    let t67557 = F::new(4.0) * t10456 * t6323 + F::new(2.0) * t1165 * t67538 + F::new(2.0) * t13146 * t6323 + F::new(2.0) * t1338 * t62230 + F::new(4.0) * t1338 * t67250 + F::new(2.0) * t1338 * t67552 + F::new(4.0) * t18898 * t3537 + F::new(2.0) * t20289 * t2105 + F::new(4.0) * t20294 * t3537 + F::new(4.0) * t20319 * t2056 + F::new(4.0) * t20319 * t4347 + F::new(4.0) * t645 * t67541 + F::new(2.0) * t67316 + t67519;
    (t67552, t67557)
}
