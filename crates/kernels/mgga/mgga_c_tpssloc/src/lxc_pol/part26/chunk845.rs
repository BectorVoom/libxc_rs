//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 845/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk845<F: Float>(t10595: F, t10596: F, t273: F, t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10553: F) -> (F, F, F) {
    let t10597 = t10595 * t10596;
    let t10599 = 1.0/pow_3_2(t273);
    let t10600 = t10599 * t10596;
    let t10602 = 0.16557e0 * t10311 - 0.49671e0 * t10318 - 0.40256666666666666668e0 * t10556 + 0.20128333333333333333e0 * t10558 - 0.60385000000000000001e0 * t10560 + 0.30192500000000000001e0 * t10562 - 0.33547222222222222222e0 * t10566 + 0.12077e1 * t10569 - 0.181155e1 * t10572 - 0.301925e0 * t10575 + 0.258925e1 * t10589 + 0.16504875e0 * t10591 + 0.19419375e1 * t10597 - 0.412621875e-1 * t10600;
    let t10603 = t10553 + t10602;
    (t10597, t10600, t10603)
}
