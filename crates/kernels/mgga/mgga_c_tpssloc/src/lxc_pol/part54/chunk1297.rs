//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1297/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1297<F: Float>(t22724: F, t31569: F, t31589: F, t6897: F, t794: F, t31668: F, t532: F, t22573: F, t8606: F, t31867: F, t9239: F, t31863: F) -> (F, F, F, F, F, F) {
    let t115629 = t22724 * t31569;
    let t115630 = F::new(0.26044789391763585244e-1) * t115629;
    let t115658 = t6897 * t794 * t31589;
    let t115774 = t532 * t31668;
    let t115925 = t8606 * t22573;
    let t116082 = t9239 * t31867;
    let t116106 = t9239 * t31863;
    (t115630, t115658, t115774, t115925, t116082, t116106)
}
