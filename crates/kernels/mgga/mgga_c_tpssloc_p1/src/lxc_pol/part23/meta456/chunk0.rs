//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1319/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319<F: Float>(t1484: F, t5611: F, t13222: F, t13350: F, t1510: F, t16891: F, t20947: F, t20972: F, t20993: F, t210: F, t2571: F, t2643: F, t46876: F, t5544: F, t5567: F, t58723: F, t58744: F, t67880: F, t67882: F, t67884: F, t67920: F, t67937: F, t9559: F, t9646: F) -> (F, F) {
    let t76250 = t1484 * t5611;
    let t76259 = -F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9559 * t210 * t5567 * t5544 + t2571 * t210 * t20993 * t1484 / F::cast_from(4.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t67880 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t67882 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t67884 - F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t2643 * t13350 * t1510 * t20947 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t58723 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t67920 + F::cast_from(595.0_f64) / F::cast_from(2592.0_f64) * t46876 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t67937 + F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t58744 + t2643 * t13222 * t1510 * t76250 / F::cast_from(64.0_f64) - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t2643 * t9646 * t16891 * t20972;
    (t76250, t76259)
}
