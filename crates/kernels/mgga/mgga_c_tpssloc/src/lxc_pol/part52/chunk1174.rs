//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1174/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1174<F: Float>(t32: F, t607: F, t33114: F, t645: F, t8513: F, t7440: F, t79: F, t641: F, t33118: F, t6504: F, t26043: F, t8307: F, t32781: F, t532: F, t1983: F, t6879: F) -> (F, F, F, F, F, F) {
    let t119931 = t32 * t607;
    let t119938 = t8513 * t33114 * t645;
    let t119942 = t79 * t7440;
    let t119944 = t8513 * t119942 * t641;
    let t119952 = t8513 * t33118 * t6504;
    let t119965 = t8513 * t8307 * t26043;
    let t119999 = t532 * t32781;
    let t120002 = 3.0 * t1983 * t119999 * t6879;
    (t119931, t119938, t119944, t119952, t119965, t120002)
}
