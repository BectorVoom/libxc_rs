//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1113/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1113<F: Float>(t12225: F, t22641: F, t268: F, t547: F, t6559: F, t12248: F, t2006: F, t22644: F, t81152: F, t1988: F, t81071: F, t225: F, t22643: F, t1987: F, t81144: F, t9537: F) -> (F, F, F, F, F, F, F) {
    let t81195 = t22641 * t12225;
    let t81228 = t6559 * t547 * t268;
    let t81243 = t12248 * t2006;
    let t81281 = t81152 * t22644;
    let t81282 = 0.98696044010893586188e-1 * t81281;
    let t81317 = t81071 * t1988;
    let t81318 = 0.27720185200590482541e0 * t81317;
    let t81326 = t22643 * t225;
    let t81398 = t81144 * t9537 * t1987;
    (t81195, t81228, t81243, t81282, t81318, t81326, t81398)
}
