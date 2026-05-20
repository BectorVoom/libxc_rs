//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2276/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2276<F: Float>(t1458: F, t2311: F, t1873: F, t22479: F, t7676: F, t7467: F, t9348: F, t45632: F, t111: F, t26097: F, t12734: F, t2314: F, t26135: F) -> (F, F, F, F, F, F, F, F) {
    let t90381 = t2311 * t1458;
    let t90383 = F::new(2.0) * t90381 * t1873;
    let t90385 = F::new(2.0) * t7676 * t22479;
    let t90387 = F::new(2.0) * t9348 * t7467;
    let t90399 = F::new(2.0) * t45632 * t1873;
    let t90400 = t26097 * t111;
    let t90404 = F::new(4.0) * t12734 * t7467;
    let t90406 = F::new(4.0) * t2314 * t26135;
    (t90381, t90383, t90385, t90387, t90399, t90400, t90404, t90406)
}
