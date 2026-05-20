//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2057/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2057<F: Float>(t1354: F, t39947: F, t12365: F, t3853: F, t12267: F, t3798: F, t12297: F, t12385: F, t12300: F, t3858: F, t12283: F, t12404: F) -> (F, F, F, F, F, F, F) {
    let t39948 = t39947 * t1354;
    let t39950 = t12365 * t3853;
    let t39955 = t12267 * t3798;
    let t39956 = t39955 * t1354;
    let t39958 = t12385 * t12297;
    let t39960 = t12300 * t3858;
    let t39971 = t12283 * t12404;
    (t39948, t39950, t39955, t39956, t39958, t39960, t39971)
}
