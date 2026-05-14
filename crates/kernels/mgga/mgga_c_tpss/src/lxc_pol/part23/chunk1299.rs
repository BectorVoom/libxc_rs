//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1299/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1299<F: Float>(t19632: F, t5706: F, t13965: F, t18547: F, t22964: F, t1760: F, t18295: F, t6274: F, t4466: F, t60738: F, t12865: F, t18454: F, t12819: F, t12831: F, t19476: F, t13000: F) -> (F, F, F, F, F, F, F, F) {
    let t65540 = 6.0 * t5706 * t19632;
    let t65543 = 6.0 * t18547 * t22964 * t13965;
    let t65548 = 2.0 * t1760 * t6274 * t18295;
    let t65551 = t60738 * t4466;
    let t65552 = 7.0 / 1152.0 * t65551;
    let t65553 = t18454 * t12865;
    let t65555 = t18454 * t12819;
    let t65557 = t19476 * t12831;
    let t65559 = t19476 * t13000;
    (t65540, t65543, t65548, t65552, t65553, t65555, t65557, t65559)
}
