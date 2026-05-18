//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 508/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk508<F: Float>(t1343: F, t3856: F, t820: F, t248: F, t2691: F, t557: F, t555: F, t1361: F, t835: F, t1336: F, t1369: F, t1995: F, t241: F) -> (F, F, F, F, F) {
    let t3858 = t1343 * t820 * t3856;
    let t3862 = t2691 * t557 * t248;
    let t3864 = F::new(119.0) / F::new(13824.0) * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    (t3858, t3862, t3864, t3867, t3869)
}
