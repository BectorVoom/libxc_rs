//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 765/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk765<F: Float>(t28067: F, t6936: F, t22839: F, t6371: F, t1998: F, t236: F, t6330: F, t22845: F, t6347: F, t6926: F, t6375: F, t6916: F) -> (F, F, F, F, F) {
    let t28068 = t6936 * t28067;
    let t28070 = t22839 * t6371;
    let t28073 = t1998 * t236 * t6330;
    let t28074 = t22845 * t28073;
    let t28077 = t1998 * t236 * t6347;
    let t28078 = t6926 * t28077;
    let t28080 = t6916 * t6375;
    (t28068, t28070, t28074, t28078, t28080)
}
