//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 549/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk549<F: Float>(t234: F, t252: F, t1905: F, t794: F, t6562: F, t6604: F, t814: F) -> (F, F, F, F, F) {
    let t6638 = t234 * t252;
    let t6643 = t794 * t1905;
    let t6644 = t6562 * t6643;
    let t6645 = F::new(0.41123351671205660912e-2) * t6644;
    let t6646 = t6604 * t814;
    (t6638, t6643, t6644, t6645, t6646)
}
