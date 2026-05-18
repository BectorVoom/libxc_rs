//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 589/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk589<F: Float>(t645: F, t79: F, t72: F, t605: F, t608: F, t641: F, t71: F, t107: F, t625: F, t63: F, t656: F, t666: F) -> (F, F, F, F, F, F) {
    let t6491 = t79 * t645;
    let t6492 = t72 * t6491;
    let t6495 = t605 * t608;
    let t6509 = t71 * t641;
    let t6528 = t625 * t107;
    let t6530 = t63 * t656;
    let t6531 = t6530 * t666;
    (t6492, t6495, t6509, t6528, t6530, t6531)
}
