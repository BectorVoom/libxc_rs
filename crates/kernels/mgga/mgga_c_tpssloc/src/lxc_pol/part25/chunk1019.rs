//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1019/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1019<F: Float>(t81766: F, t849: F, t23132: F, t2617: F, t23133: F, t2707: F, t6621: F, t9997: F, t23127: F, t2703: F, t9609: F, t131: F, t23121: F, t9537: F, t236: F, t81613: F) -> (F, F, F, F, F, F, F, F) {
    let t81767 = t81766 * t849;
    let t81769 = t2617 * t23132;
    let t81770 = t81769 * t849;
    let t81772 = t23133 * t2707;
    let t81774 = t6621 * t9997;
    let t81776 = t23127 * t2703;
    let t81779 = t6621 * t9609;
    let t81782 = t23121 * t131 * t9537;
    let t81783 = t81613 * t236;
    (t81767, t81770, t81772, t81774, t81776, t81779, t81782, t81783)
}
