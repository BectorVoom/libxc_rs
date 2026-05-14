//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1147/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1147<F: Float>(t22892: F, t22893: F, t31194: F, t22642: F, t22690: F, t31193: F, t552: F, t6955: F, t31206: F, t6897: F, t794: F, t22716: F, t8480: F, t31203: F, t6914: F, t31207: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t114060 = t22892 * t22893 * t31194;
    let t114064 = 0.16449340668482264365e-1 * t22642 * t22690 * t31193;
    let t114069 = t552 * t6955;
    let t114097 = t6897 * t794 * t31206;
    let t114104 = 0.12793931631041761173e0 * t22716 * t8480;
    let t114105 = t6914 * t31203;
    let t114116 = t6883 * t31207;
    (t114060, t114064, t114069, t114097, t114104, t114105, t114116)
}
