//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1261/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1261<F: Float>(t22644: F, t81152: F, t1988: F, t81071: F, t225: F, t22643: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F) -> (F, F, F, F, F, F) {
    let t81281 = t81152 * t22644;
    let t81282 = F::new(0.98696044010893586188e-1) * t81281;
    let t81317 = t81071 * t1988;
    let t81318 = F::new(0.27720185200590482541e0) * t81317;
    let t81326 = t22643 * t225;
    let t81398 = t81144 * t9537 * t1987;
    let t81399 = F::new(0.13707783890401886971e-2) * t81398;
    let t81437 = t835 * t107;
    let t81438 = F::new(154.0) / F::new(27.0) * t81437;
    let t81439 = t240 * t656;
    (t81282, t81318, t81326, t81399, t81438, t81439)
}
