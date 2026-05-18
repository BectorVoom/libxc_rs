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
    let t84280 = F::new(1232.0) / F::new(81.0) * t1860 * t835 * t67 * t1864;
    let t84283 = t22534 * t7032;
    let t84285 = t6486 * t23993;
    let t84287 = -F::new(2.0) * t83835 * t2032 - F::new(4.0) * t22519 * t7035 - F::new(5.0) * t7026 * t83771 - F::new(5.0) * t7026 * t83840 - F::new(5.0) / F::new(3.0) * t7026 * t83846 - F::new(8.0) / F::new(3.0) * t84270 + t1860 * t2031 * t83706 / F::new(3.0) + t83710 * t2032 / F::new(3.0) - t84280 - F::new(2.0) * t83699 * t2032 + F::new(16.0) / F::new(3.0) * t84283 + F::new(88.0) / F::new(9.0) * t84285;
    t84287
}
