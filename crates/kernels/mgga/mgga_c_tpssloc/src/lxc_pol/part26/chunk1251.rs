//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1251/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1251<F: Float>(t1988: F, t81071: F, t225: F, t22942: F, t22643: F, t22637: F, t81228: F, t1307: F, t567: F, t22635: F, t26331: F, t3719: F) -> (F, F, F, F) {
    let t81317 = t81071 * t1988;
    let t81318 = F::cast_from(0.27720185200590482541e0_f64) * t81317;
    let t81319 = t22942 * t225;
    let t81326 = t22643 * t225;
    let t81328 = t81228 * t81326 * t22637;
    let t81330 = t567 * t1307;
    let t81333 = t26331 * t22635 * t81330 * t3719;
    (t81318, t81319, t81328, t81333)
}
