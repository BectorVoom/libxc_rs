//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 877/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk877<F: Float>(t10311: F, t10318: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10589: F, t10591: F, t10597: F, t10600: F, t10680: F, t913: F) -> (F, F) {
    let t10695 = 0.16431333333333333333e0 * t10311 - 0.49293999999999999999e0 * t10318 - 0.39862222222222222223e0 * t10556 + 0.19931111111111111111e0 * t10558 - 0.59793333333333333333e0 * t10560 + 0.29896666666666666667e0 * t10562 - 0.33218518518518518518e0 * t10566 + 0.11958666666666666667e1 * t10569 - 0.17938e1 * t10572 - 0.29896666666666666667e0 * t10575 + 0.1898925e1 * t10589 + 0.3071625e0 * t10591 + 0.142419375e1 * t10597 - 0.76790625e-1 * t10600;
    let t10696 = t10680 + t10695;
    let t10697 = t10696 * t913;
    (t10696, t10697)
}
