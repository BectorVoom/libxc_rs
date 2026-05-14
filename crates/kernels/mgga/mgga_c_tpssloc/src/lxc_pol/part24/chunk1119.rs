//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1119/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1119<F: Float>(t2314: F, t6534: F, t12739: F, t1873: F, t5113: F, t1268: F, t22479: F, t22461: F, t22559: F, t22600: F, t2363: F, t23844: F, t23846: F, t6517: F, t671: F, t12461: F, t3698: F) -> (F, F) {
    let t23848 = 4.0 * t2314 * t6534;
    let t23850 = 2.0 * t12739 * t1873;
    let t23852 = 4.0 * t5113 * t6534;
    let t23854 = 2.0 * t1268 * t22479;
    let t23855 = 4.0 * t22461 * t671 + 2.0 * t2363 * t6517 + t22559 + 2.0 * t22600 + t23844 + t23846 + t23848 + t23850 + t23852 + t23854;
    let t23857 = t12461 * t3698;
    (t23855, t23857)
}
