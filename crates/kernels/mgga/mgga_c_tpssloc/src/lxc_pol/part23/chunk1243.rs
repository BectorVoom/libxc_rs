//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1243/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1243<F: Float>(t2798: F, t77041: F, t41942: F, t77075: F, t42087: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77102: F) -> (F, F, F) {
    let t77105 = t2798 * t77041;
    let t77107 = t41942 * t77075;
    let t77114 = t42087 - 0.21908444444444444444e0 * t76899 + 0.65725333333333333332e0 * t76903 - 0.10954222222222222222e0 * t76906 - 0.295764e1 * t76912 + 0.1898925e1 * t77102 + 0.12401580246913580247e1 * t47787 - 0.28483875e1 * t77105 + 0.1151859375e0 * t77107 - 0.19931111111111111111e1 * t76587 + 0.71752000000000000001e1 * t76595 - 0.79724444444444444444e0 * t76610 - 0.107628e2 * t76618 + 0.23917333333333333333e1 * t76626;
    (t77105, t77107, t77114)
}
