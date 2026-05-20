//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 525/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk525<F: Float>(t1365: F, t68: F, t248: F, t2691: F, t557: F, t555: F, t1361: F, t835: F, t1336: F, t1995: F, t241: F, t67: F) -> (F, F, F, F, F, F) {
    let t3843 = t68 * t1365;
    let t3862 = t2691 * t557 * t248;
    let t3864 = F::new(119.0) / F::new(13824.0) * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    (t3843, t3862, t3864, t3865, t3866, t3870)
}
