//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 786/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk786<F: Float>(t1873: F, t2314: F, t5113: F, t1268: F, t6534: F, t6515: F, t6517: F, t671: F, t1271: F, t191: F, t192: F) -> (F, F, F) {
    let t6867 = F::new(2.0) * t2314 * t1873;
    let t6869 = F::new(2.0) * t5113 * t1873;
    let t6871 = F::new(2.0) * t1268 * t6534;
    let t6872 = F::new(2.0) * t6517 * t671 + t6515 + t6867 + t6869 + t6871;
    let t6875 = t1271 * t191;
    let t6876 = t6875 * t192;
    (t6872, t6875, t6876)
}
