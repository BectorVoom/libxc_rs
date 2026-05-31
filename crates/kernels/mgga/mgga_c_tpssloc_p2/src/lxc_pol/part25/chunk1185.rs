//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1185/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1185<F: Float>(t1860: F, t22489: F, t7031: F, t1864: F, t67: F, t835: F, t22534: F, t7032: F, t23993: F, t6486: F, t2031: F, t2032: F, t22519: F, t7026: F, t7035: F, t83699: F, t83706: F, t83710: F, t83771: F, t83835: F, t83840: F, t83846: F) -> F {
    let t84270 = t1860 * t7031 * t22489;
    let t84280 = F::cast_from(1232.0_f64) / F::cast_from(81.0_f64) * t1860 * t835 * t67 * t1864;
    let t84283 = t22534 * t7032;
    let t84285 = t6486 * t23993;
    let t84287 = -F::cast_from(2.0_f64) * t83835 * t2032 - F::cast_from(4.0_f64) * t22519 * t7035 - F::cast_from(5.0_f64) * t7026 * t83771 - F::cast_from(5.0_f64) * t7026 * t83840 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t83846 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t84270 + t1860 * t2031 * t83706 / F::cast_from(3.0_f64) + t83710 * t2032 / F::cast_from(3.0_f64) - t84280 - F::cast_from(2.0_f64) * t83699 * t2032 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t84283 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t84285;
    t84287
}
