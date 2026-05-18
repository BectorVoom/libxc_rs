//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1259/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1259<F: Float>(t22690: F, t7732: F, t81195: F, t22832: F, t5234: F, t1831: F, t80866: F, t22782: F, t7712: F, t80939: F, t26271: F, t80779: F) -> (F, F, F, F, F, F) {
    let t91081 = t81195 * t22690 * t7732;
    let t91100 = t5234 * t22832;
    let t91149 = t80866 * t1831;
    let t91160 = t5234 * t22782;
    let t91167 = t80939 * t7712;
    let t91206 = t80779 * t26271;
    (t91081, t91100, t91149, t91160, t91167, t91206)
}
