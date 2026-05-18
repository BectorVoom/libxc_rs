//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1377/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1377<F: Float>(t18660: F, t6080: F, t18670: F, t19388: F, t42178: F, t5784: F, t20275: F, t5483: F, t1675: F, t19380: F, t5790: F, t1791: F, t1792: F, t18305: F, t18663: F, t18666: F, t19352: F, t20282: F, t5489: F, t5794: F, t6073: F, t6304: F, t65217: F, t65396: F, t65410: F) -> F {
    let t67436 = F::new(32.0) / F::new(9.0) * t6080 * t18660;
    let t67440 = F::new(80.0) / F::new(9.0) * t18670 * t19388;
    let t67441 = t42178 * t5784;
    let t67451 = F::new(16.0) / F::new(9.0) * t5483 * t20275;
    let t67454 = F::new(16.0) / F::new(9.0) * t1675 * t5790 * t19380;
    let t67462 = t67436 + F::new(10.0) * t18666 * t65410 + t67440 - F::new(10.0) / F::new(3.0) * t67441 * t5489 + t65217 * t1792 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t19352 * t5794 + t6073 * t18663 / F::new(3.0) - t67451 - t67454 + t18305 * t6304 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t5483 * t20282 + t1675 * t1791 * t65396 / F::new(3.0);
    t67462
}
