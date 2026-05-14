//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1225/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1225<F: Float>(t59688: F, t59694: F, t68444: F, t68446: F, t68448: F, t68494: F, t68498: F, t76610: F, t76614: F, t76618: F, t76622: F, t76626: F, t324: F, t76602: F, t300: F, t1589: F, t69012: F) -> (F, F, F) {
    let t76630 = 0.12361111111111111111e-1 * t68444 + 0.13734567901234567901e-1 * t68446 - 0.49444444444444444444e-1 * t68448 + 0.24722222222222222222e-1 * t68494 - 0.74166666666666666668e-1 * t68498 - 0.24722222222222222222e-1 * t76610 + 0.2225e0 * t76614 - 0.33375e0 * t76618 + 0.55625000000000000001e-1 * t76622 + 0.74166666666666666668e-1 * t76626 + 0.49444444444444444445e-1 * t59688 - 0.24722222222222222222e-1 * t59694;
    let t76632 = (t76602 + t76630) * t324;
    let t76634 = 0.19751673498613801407e-1 * t300 * t76632;
    let t76636 = 0.23392894490538584828e1 * t69012 * t1589;
    (t76632, t76634, t76636)
}
