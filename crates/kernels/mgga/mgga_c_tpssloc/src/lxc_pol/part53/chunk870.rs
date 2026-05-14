//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 870/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk870<F: Float>(t1458: F, t7039: F, t2035: F, t4072: F, t191: F, t192: F, t27215: F, t33409: F, t6547: F, t1888: F, t31333: F, t86873: F, t1880: F, t8547: F, t87782: F, t23204: F, t33408: F, t6562: F) -> (F, F, F, F, F, F, F) {
    let t121004 = t7039 * t1458;
    let t121007 = t2035 * t4072;
    let t121210 = t27215 * t191 * t192;
    let t121296 = t6547 * t33409;
    let t121299 = t1888 * t86873 * t31333;
    let t121302 = t1880 * t87782 * t8547;
    let t121305 = t6562 * t23204 * t33408;
    (t121004, t121007, t121210, t121296, t121299, t121302, t121305)
}
