//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2596/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2596<F: Float>(t1009: F, t22113: F, t1011: F, t1212: F, t18375: F, t5002: F, t1218: F, t1737: F, t18943: F, t19080: F, t5014: F, t65617: F, t65619: F, t65628: F, t65632: F, t65637: F, t65647: F, t65649: F, t65651: F, t66159: F) -> (F, F) {
    let t72361 = t22113 * t1009;
    let t72363 = t72361 * t1011 * t1212;
    let t72366 = t5002 * t18375;
    let t72380 = -t65617 / F::new(2304.0) - t65619 / F::new(2304.0) + t72363 * t1218 / F::new(3072.0) + t72366 / F::new(1536.0) - t65628 / F::new(648.0) + t65632 / F::new(4608.0) + t65637 / F::new(27.0) + t5002 * t18943 / F::new(1024.0) + t65647 / F::new(6912.0) - t66159 * t1737 / F::new(96.0) - t19080 * t5014 / F::new(96.0) - t65649 / F::new(2304.0) - t65651 / F::new(144.0);
    (t72361, t72380)
}
