//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2245/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2245<F: Float>(t13034: F, t225: F, t10104: F, t10116: F, t13029: F, t13042: F, t13050: F, t13072: F, t13460: F, t13461: F, t13463: F, t1528: F, t259: F, t2597: F, t2713: F, t2718: F, t2720: F, t2743: F, t40870: F, t4147: F, t4273: F, t852: F, t855: F, t865: F, t866: F, t9590: F) -> F {
    let t46452 = t13034 * t225;
    let t46481 = F::new(6.0) * t13460 * t2718 * t855 * t865 + F::new(3.0) * t13029 * t259 * t852 - t10104 * t4147 + F::new(6.0) * t10116 * t4147 + F::new(6.0) * t13042 * t2720 - F::new(3.0) * t13042 * t2743 - F::new(18.0) * t13050 * t2597 + F::new(12.0) * t13072 * t2713 - F::new(3.0) * t13461 * t2713 - F::new(3.0) * t13463 * t2743 - F::new(3.0) * t1528 * t40870 + F::new(6.0) * t4273 * t9590 - F::new(3.0) * t46452 * t866;
    t46481
}
