//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1180/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1180<F: Float>(t25: F, t2749: F, t606: F, t868: F, t2745: F, t1877: F, t1915: F, t2249: F, t22951: F, t22959: F, t22961: F, t22964: F, t22968: F, t23286: F, t23290: F, t23295: F, t2522: F, t4314: F, t6542: F, t6666: F, t6670: F, t6671: F) -> (F, F, F, F) {
    let t23296 = t25 * t2749;
    let t23299 = t606 * t868;
    let t23302 = t25 * t2745;
    let t23309 = F::new(3.0) * t4314 * t1915 * t22951 + F::new(3.0) * t2522 * t6666 * t6542 - F::new(3.0) * t22959 * t22961 + F::new(3.0) * t2522 * t1915 * t22964 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t22968 + t1877 * t23286 * t25 / F::new(2.0) - t1877 * t23290 * t6671 + t1877 * t6666 * t606 + t1877 * t23295 * t23296 - t1877 * t6670 * t23299 - t1877 * t6670 * t23302 / F::new(2.0) + t1877 * t1915 * t2249 / F::new(2.0);
    (t23296, t23299, t23302, t23309)
}
