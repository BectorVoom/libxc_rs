//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1288/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1288<F: Float>(t22644: F, t81152: F, t22643: F, t6891: F, t81195: F, t12434: F, t1985: F, t214: F, t225: F, t567: F, t1377: F, t1385: F, t22635: F, t26331: F, t3734: F) -> (F, F, F, F) {
    let t81281 = t81152 * t22644;
    let t81282 = F::new(0.98696044010893586188e-1) * t81281;
    let t81284 = t81195 * t22643 * t6891;
    let t81291 = t1985 * t214 * t12434 * t225 * t567;
    let t81300 = t26331 * t22635 * t1377 * t3734 * t1385;
    (t81282, t81284, t81291, t81300)
}
