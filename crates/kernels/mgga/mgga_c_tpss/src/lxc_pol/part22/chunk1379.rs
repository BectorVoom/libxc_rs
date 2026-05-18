//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1379/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1379<F: Float>(t19396: F, t5791: F, t18646: F, t6073: F, t6077: F, t62306: F, t6080: F, t1792: F, t18356: F, t18363: F, t18649: F, t19408: F, t20246: F, t5785: F, t5794: F, t6304: F, t65214: F, t65289: F, t65400: F, t65403: F) -> F {
    let t67491 = F::new(32.0) / F::new(9.0) * t19396 * t5791;
    let t67496 = t6073 * t18646;
    let t67510 = t62306 * t6077;
    let t67512 = t6080 * t18646;
    let t67514 = t67491 - F::new(2.0) / F::new(3.0) * t65403 * t1792 - F::new(2.0) / F::new(3.0) * t18363 * t6304 + F::new(88.0) / F::new(27.0) * t67496 - F::new(2.0) / F::new(3.0) * t65214 * t1792 - F::new(4.0) / F::new(3.0) * t65400 * t1792 - F::new(10.0) / F::new(3.0) * t20246 * t18356 - F::new(4.0) / F::new(3.0) * t19396 * t5794 - F::new(10.0) / F::new(3.0) * t18649 * t19408 - F::new(10.0) / F::new(3.0) * t5785 * t65289 - F::new(440.0) / F::new(27.0) * t67510 - F::new(176.0) / F::new(27.0) * t67512;
    t67514
}
