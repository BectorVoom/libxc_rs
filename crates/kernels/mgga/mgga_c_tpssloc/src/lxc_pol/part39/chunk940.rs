//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 940/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk940<F: Float>(t1409: F, t2244: F, t9300: F, t2274: F, t3966: F, t607: F, t2250: F, t3990: F, t12606: F, t55: F, t12677: F, t12681: F, t12684: F, t12687: F, t1414: F, t1420: F, t2262: F, t2275: F, t2278: F, t39: F, t3982: F, t3985: F, t51: F, t615: F, t9311: F) -> (F,) {
    let t12695 = t9300 * t1409 * t2244;
    let t12698 = t2274 * t3966;
    let t12699 = t12698 * t607;
    let t12702 = t3990 * t2250;
    let t12705 = t55 * t12606;
    let t12708 = 220.0 / 27.0 * t2262 * t1414 - 40.0 / 27.0 * t615 * t3982 - 40.0 / 9.0 * t615 * t3985 - 5.0 / 108.0 * t39 * t12677 + 5.0 / 9.0 * t39 * t12681 + 5.0 / 18.0 * t39 * t12684 + 5.0 / 6.0 * t39 * t12687 - 20.0 / 27.0 * t1420 * t2275 + 20.0 / 9.0 * t1420 * t2278 + 5.0 / 108.0 * t51 * t12695 + 5.0 / 9.0 * t51 * t12699 + 5.0 / 18.0 * t51 * t12702 - 5.0 / 6.0 * t51 * t12705 + t9311;
    (t12708,)
}
