//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1426/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1426<F: Float>(t111: F, t4025: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t2331: F, t4067: F, t2341: F, t92: F, t100: F, t2349: F) -> (F, F, F, F, F, F, F) {
    let t12725 = t4025 * t111;
    let t12747 = t2281 * t1454;
    let t12750 = F::new(4.0) / F::new(3.0) * t626 * t4044;
    let t12752 = F::new(2.0) / F::new(3.0) * t626 * t4068;
    let t12757 = t2331 * t4067;
    let t12774 = t92 * t2341;
    let t12795 = t100 * t2349;
    (t12725, t12747, t12750, t12752, t12757, t12774, t12795)
}
