//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1121/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1121<F: Float>(t1988: F, t81071: F, t225: F, t22643: F, t22637: F, t81228: F, t1307: F, t567: F, t22635: F, t26331: F, t3719: F, t6888: F, t6891: F, t80707: F) -> (F, F, F, F) {
    let t81317 = t81071 * t1988;
    let t81326 = t22643 * t225;
    let t81328 = t81228 * t81326 * t22637;
    let t81330 = t567 * t1307;
    let t81333 = t26331 * t22635 * t81330 * t3719;
    let t81339 = t6888 * t80707 * t6891;
    (t81317, t81328, t81333, t81339)
}
