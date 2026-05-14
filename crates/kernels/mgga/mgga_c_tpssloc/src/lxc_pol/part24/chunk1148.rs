//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1148/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1148<F: Float>(t1338: F, t225: F, t236: F, t22828: F, t80853: F, t22783: F, t3872: F, t12353: F, t6952: F, t22788: F, t1336: F, t2690: F, t6950: F, t1369: F, t22782: F, t3777: F) -> (F, F, F, F, F, F, F) {
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    let t80857 = t80853 * t80855 * t22828;
    let t80859 = t22783 * t3872;
    let t80861 = t6952 * t12353;
    let t80863 = t22788 * t3872;
    let t80866 = t1336 * t6950 * t2690;
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    (t80854, t80857, t80859, t80861, t80863, t80867, t80869)
}
