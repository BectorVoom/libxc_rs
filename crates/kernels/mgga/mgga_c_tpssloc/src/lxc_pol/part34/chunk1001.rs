//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1001/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1001<F: Float>(t22642: F, t22690: F, t26395: F, t22863: F, t7737: F, t26426: F, t81046: F, t7732: F, t81195: F, t22832: F, t5234: F, t1831: F, t80866: F, t22782: F, t7712: F, t80939: F) -> (F, F, F, F, F, F, F, F) {
    let t90993 = t22642 * t22690 * t26395;
    let t91000 = t22863 * t7737;
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    let t91100 = t5234 * t22832;
    let t91149 = t80866 * t1831;
    let t91160 = t5234 * t22782;
    let t91167 = t80939 * t7712;
    (t90993, t91000, t91078, t91081, t91100, t91149, t91160, t91167)
}
