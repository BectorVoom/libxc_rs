//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 825/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk825<F: Float>(t1409: F, t22510: F, t24498: F, t27356: F, t5392: F, t5398: F, t5415: F, t56: F, t7251: F, t67: F, t1864: F, t7445: F, t7974: F) -> (F, F, F) {
    let t29473 = F::new(88.0) / F::new(9.0) * t5415 * t56 + F::new(40.0) / F::new(9.0) * t27356 * t1409 + F::new(5.0) / F::new(18.0) * t24498 * t5392 - F::new(5.0) / F::new(6.0) * t7251 * t5398 - t22510;
    let t29474 = t29473 * t67;
    let t29475 = t29474 * t1864;
    let t29478 = t7974 * t7445;
    (t29473, t29475, t29478)
}
