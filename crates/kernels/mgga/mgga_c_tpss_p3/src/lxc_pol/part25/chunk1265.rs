//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1265/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1265<F: Float>(t1339: F, t1663: F, t1760: F, t1796: F, t1800: F, t1830: F, t18547: F, t20289: F, t21236: F, t21880: F, t21883: F, t21894: F, t21897: F, t21900: F, t21908: F, t21922: F, t21944: F, t3493: F, t4638: F, t485: F, t5314: F, t544: F, t6103: F, t6243: F, t626: F, t6318: F, t6324: F, t6328: F, t6409: F, t6413: F, t6439: F) -> F {
    let t21946 = -F::cast_from(4.0_f64) * t1339 * t20289 + F::cast_from(2.0_f64) * t1663 * t6409 + F::cast_from(3.0_f64) * t1760 * t21883 - t1796 * t5314 - F::cast_from(2.0_f64) * t1800 * t21236 - F::cast_from(2.0_f64) * t1830 * t4638 - F::cast_from(6.0_f64) * t18547 * t21900 - F::cast_from(4.0_f64) * t21880 * t626 - F::cast_from(2.0_f64) * t21894 * t626 - F::cast_from(2.0_f64) * t21897 * t626 - F::cast_from(2.0_f64) * t21908 * t626 - F::cast_from(2.0_f64) * t21922 * t485 + t21944 * t544 - F::cast_from(4.0_f64) * t3493 * t6318 - F::cast_from(4.0_f64) * t3493 * t6324 - F::cast_from(4.0_f64) * t3493 * t6328 - F::cast_from(4.0_f64) * t6103 * t6318 - F::cast_from(4.0_f64) * t6103 * t6324 + F::cast_from(6.0_f64) * t6243 * t6413 - F::cast_from(2.0_f64) * t6243 * t6439;
    t21946
}
