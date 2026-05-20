//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1365/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1365<F: Float>(t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t2341: F, t92: F, t100: F, t2349: F, t4098: F, t751: F, t172: F, t4095: F) -> (F, F, F, F, F, F, F) {
    let t12747 = t2281 * t1454;
    let t12750 = F::new(4.0) / F::new(3.0) * t626 * t4044;
    let t12752 = F::new(2.0) / F::new(3.0) * t626 * t4068;
    let t12774 = t92 * t2341;
    let t12795 = t100 * t2349;
    let t12850 = F::new(2.0) * t4098 * t751;
    let t12858 = t4095 * t172;
    (t12747, t12750, t12752, t12774, t12795, t12850, t12858)
}
