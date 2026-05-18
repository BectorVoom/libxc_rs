//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1313/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1313<F: Float>(t119942: F, t641: F, t8513: F, t33118: F, t6504: F, t26043: F, t8307: F, t32781: F, t532: F, t1983: F, t6879: F, t26149: F, t8450: F) -> (F, F, F, F, F) {
    let t119944 = t8513 * t119942 * t641;
    let t119952 = t8513 * t33118 * t6504;
    let t119965 = t8513 * t8307 * t26043;
    let t119999 = t532 * t32781;
    let t120002 = F::new(3.0) * t1983 * t119999 * t6879;
    let t120003 = t8450 * t26149;
    (t119944, t119952, t119965, t120002, t120003)
}
