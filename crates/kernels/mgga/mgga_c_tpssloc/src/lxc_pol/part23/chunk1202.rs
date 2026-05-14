//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1202/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1202<F: Float>(t1484: F, t5611: F, t13222: F, t13350: F, t1510: F, t16891: F, t20947: F, t20972: F, t20993: F, t210: F, t2571: F, t2643: F, t46876: F, t5544: F, t5567: F, t58723: F, t58744: F, t67880: F, t67882: F, t67884: F, t67920: F, t67937: F, t9559: F, t9646: F) -> (F, F) {
    let t76250 = t1484 * t5611;
    let t76259 = -3.0 / 2.0 * t9559 * t210 * t5567 * t5544 + t2571 * t210 * t20993 * t1484 / 4.0 - 7.0 / 96.0 * t67880 - 7.0 / 1152.0 * t67882 + 7.0 / 1152.0 * t67884 - 5.0 / 64.0 * t2643 * t13350 * t1510 * t20947 - 119.0 / 2304.0 * t58723 + 7.0 / 36.0 * t67920 + 595.0 / 2592.0 * t46876 + 7.0 / 3.0 * t67937 + 35.0 / 12.0 * t58744 + t2643 * t13222 * t1510 * t76250 / 64.0 - 5.0 / 128.0 * t2643 * t9646 * t16891 * t20972;
    (t76250, t76259)
}
