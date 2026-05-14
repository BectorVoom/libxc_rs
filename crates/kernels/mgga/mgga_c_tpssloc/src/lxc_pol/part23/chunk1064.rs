//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1064/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1064<F: Float>(t39336: F, t761: F, t39488: F, t2374: F, t39519: F, t39503: F, t39391: F, t39537: F, t39344: F, t39362: F, t2751: F, t39494: F, t153: F, t157: F, t39842: F, t39354: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t40721 = 0.21053605041484726346e2 * t761 * t39336;
    let t40732 = 0.6233709278045326953e3 * t761 * t39488;
    let t40741 = 0.43374325201206959368e-1 * t2374 * t39519;
    let t40743 = 0.12842595503380418954e1 * t2374 * t39503;
    let t40748 = 0.35089341735807877242e1 * t761 * t39391;
    let t40760 = 0.12304822629859687989e5 * t761 * t39537;
    let t40764 = 0.46785788981077169656e1 * t761 * t39344;
    let t40766 = 0.62337092780453269531e3 * t761 * t39362;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0 / t40771;
    let t40779 = 0.51947577317044391277e2 * t761 * t39494;
    let t40784 = t153 * t157 * t39842;
    let t40790 = 0.21687162600603479684e-1 * t2374 * t39354;
    (t40721, t40732, t40741, t40743, t40748, t40760, t40764, t40766, t40772, t40779, t40784, t40790)
}
