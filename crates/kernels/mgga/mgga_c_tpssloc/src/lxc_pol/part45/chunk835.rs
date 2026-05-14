//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 835/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk835<F: Float>(t23139: F, t8339: F, t23171: F, t23228: F, t8335: F, t30623: F, t81651: F, t82074: F, t22986: F, t23270: F, t2553: F, t30622: F, t2717: F, t6662: F, t1888: F, t865: F) -> (F, F, F, F, F) {
    let t112855 = t23139 * t8339;
    let t112863 = 0.16449340668482264365e-1 * t23171 * t23228 * t8335;
    let t112867 = t81651 * t82074 * t30623;
    let t112868 = 0.3289868133696452873e-1 * t112867;
    let t112872 = 0.3289868133696452873e-1 * t22986 * t23270 * t30622 * t2553;
    let t112873 = t2717 * t6662;
    let t112877 = 0.6579736267392905746e-1 * t1888 * t23270 * t112873 * t865;
    (t112855, t112863, t112868, t112872, t112877)
}
