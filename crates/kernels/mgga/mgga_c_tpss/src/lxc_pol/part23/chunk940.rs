//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 940/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk940<F: Float>(t1098: F, t9660: F, t140: F, t3039: F, t1127: F, t650: F, t1015: F, t242: F, t1125: F, t2850: F, t3090: F, t2846: F, t2845: F, t400: F, t2192: F, t359: F, t461: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9661 = t1098 * t9660;
    let t9663 = t140 * t3039;
    let t9664 = t1098 * t9663;
    let t9666 = t650 * t1127;
    let t9668 = t242 * t9666 * t1015;
    let t9669 = t1125 * t9668;
    let t9672 = t242 * t3090 * t2850;
    let t9673 = t1125 * t9672;
    let t9676 = t242 * t3090 * t2846;
    let t9677 = t1125 * t9676;
    let t9684 = 1.0 / t400 / t2845;
    let t9699 = t359 * t2192 * t461;
    (t9661, t9663, t9664, t9666, t9668, t9669, t9672, t9673, t9676, t9677, t9684, t9699)
}
