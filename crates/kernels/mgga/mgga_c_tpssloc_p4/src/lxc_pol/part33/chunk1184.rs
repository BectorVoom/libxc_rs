//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1184/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1184<F: Float>(t1603: F, t7593: F, t5677: F, t6690: F, t23593: F, t23394: F, t5919: F, t6704: F, t5681: F, t6689: F, t1945: F, t5848: F) -> (F, F, F, F, F, F, F, F) {
    let t28488 = t1603 * t7593;
    let t28491 = t6690 * t5677;
    let t28492 = t23593 * t28491;
    let t28495 = t23394 * t5919;
    let t28496 = t6704 * t28495;
    let t28499 = t6690 * t5681;
    let t28500 = t6689 * t28499;
    let t28505 = t5848 * t1945;
    (t28488, t28491, t28492, t28495, t28496, t28499, t28500, t28505)
}
