//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2294/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2294<F: Float>(t13396: F, t808: F, t1509: F, t2710: F, t4233: F, t852: F, t13170: F, t252: F, t10084: F, t10101: F, t13176: F, t13263: F, t13380: F, t13384: F, t13397: F, t13401: F, t13404: F, t13453: F, t2684: F, t2733: F, t4166: F, t4182: F, t4281: F, t4282: F, t4291: F, t829: F, t9661: F) -> (F, F, F, F, F) {
    let t47419 = t808 * t13396;
    let t47425 = t2710 * t1509;
    let t47439 = t852 * t4233;
    let t47448 = t252 * t13170;
    let t47452 = -F::new(18.0) * t13263 * t13380 * t13397 - F::new(3.0) * t13384 * t2684 * t4291 + F::new(6.0) * t4182 * t4281 * t47425 + F::new(12.0) * t4182 * t4281 * t47439 - t4282 * t4291 * t9661 - F::new(3.0) * t4291 * t47448 * t829 + F::new(6.0) * t10084 * t4166 - t10101 * t4166 - F::new(6.0) * t13176 * t2733 + F::new(18.0) * t13401 * t13453 + F::new(6.0) * t13404 * t13453;
    (t47419, t47425, t47439, t47448, t47452)
}
