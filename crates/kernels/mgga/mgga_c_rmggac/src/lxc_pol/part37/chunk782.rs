//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 782/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk782<F: Float>(t5148: F, t74815: F, t556: F, t69144: F, t2842: F, t69436: F, t75086: F, t4669: F, t74801: F, t305: F, t75141: F, t76049: F, t7788: F, t76053: F, t74802: F, t7782: F) -> (F, F, F, F, F, F, F, F, F) {
    let t76362 = 0.5987120850931904282e-1 * t5148 * t74815;
    let t76363 = t69144 * t556;
    let t76365 = t69436 * t2842;
    let t76367 = t5148 * t75086;
    let t76368 = 0.15965655602485078085e0 * t76367;
    let t76370 = 0.8980681276397856423e-1 * t4669 * t74801;
    let t76372 = 0.2993560425465952141e-1 * t305 * t75141;
    let t76373 = t7788 * t76049;
    let t76375 = t7788 * t76053;
    let t76377 = t7782 * t74802;
    (t76362, t76363, t76365, t76368, t76370, t76372, t76373, t76375, t76377)
}
