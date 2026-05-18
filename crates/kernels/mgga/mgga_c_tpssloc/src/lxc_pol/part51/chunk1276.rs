//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1276/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1276<F: Float>(t22751: F, t31195: F, t22892: F, t22893: F, t31194: F, t22642: F, t22690: F, t31193: F, t552: F, t6955: F, t31206: F, t6897: F, t794: F) -> (F, F, F, F, F) {
    let t114057 = t22751 * t31195;
    let t114060 = t22892 * t22893 * t31194;
    let t114064 = F::new(0.16449340668482264365e-1) * t22642 * t22690 * t31193;
    let t114069 = t552 * t6955;
    let t114097 = t6897 * t794 * t31206;
    (t114057, t114060, t114064, t114069, t114097)
}
