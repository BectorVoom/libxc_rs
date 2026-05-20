//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1827/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1827<F: Float>(t26426: F, t81046: F, t22690: F, t7732: F, t81195: F, t22832: F, t5234: F, t1336: F, t22759: F, t5252: F, t836: F, t5293: F, t80820: F) -> (F, F, F, F, F) {
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    let t91100 = t5234 * t22832;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91120 = t80820 * t5293;
    (t91078, t91081, t91100, t91113, t91120)
}
