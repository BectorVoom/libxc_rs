//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1290/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1290<F: Float>(t1395: F, t7020: F, t12513: F, t12537: F, t1396: F, t1398: F, t1404: F, t2023: F, t2029: F, t23863: F, t23901: F, t3: F, t3932: F, t3946: F, t580: F, t7003: F, t80593: F, t80597: F, t80599: F, t80601: F, t80605: F, t83973: F, t84019: F) -> (F,) {
    let t84024 = t1395 * t7020;
    let tv4rho3sigma0 = t3 * t580 * t83973 + t12513 * t2029 + t12537 * t2023 + 3.0 * t1396 * t23901 + t1398 * t84019 + 3.0 * t1404 * t23863 + 3.0 * t3932 * t7020 + 3.0 * t3946 * t7003 + 3.0 * t80593 + 3.0 * t80597 + 6.0 * t80599 + 3.0 * t80601 + 3.0 * t80605 + 6.0 * t84024;
    (tv4rho3sigma0,)
}
