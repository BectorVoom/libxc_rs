//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1035/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1035<F: Float>(t31009: F, t9231: F, t31003: F, t39063: F, t31016: F, t9239: F, t22642: F, t22643: F, t8458: F, t2006: F, t212: F, t6890: F, t3886: F, t6992: F, t22716: F, t8459: F) -> (F, F, F, F, F, F, F) {
    let t113880 = t9231 * t31009;
    let t113883 = t39063 * t31003;
    let t113888 = t9239 * t31016;
    let t113934 = 0.16449340668482264365e-1 * t22642 * t22643 * t8458;
    let t113941 = 0.16449340668482264365e-1 * t22642 * t212 * t2006 * t6890;
    let t113946 = t3886 * t6992;
    let t113963 = 0.12793931631041761173e0 * t22716 * t8459;
    (t113880, t113883, t113888, t113934, t113941, t113946, t113963)
}
