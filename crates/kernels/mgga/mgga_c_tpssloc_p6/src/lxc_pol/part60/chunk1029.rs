//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1029/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1029<F: Float>(t33363: F, t7756: F, t33623: F, t7685: F, t101138: F, t26161: F, t33221: F, t1983: F, t20085: F, t8640: F, t29377: F, t8643: F) -> (F, F, F, F, F) {
    let t128577 = F::cast_from(2.0_f64) * t33363 * t7756;
    let t128581 = F::cast_from(2.0_f64) * t7685 * t33623;
    let t128584 = F::cast_from(4.0_f64) * t26161 * t101138 * t33221;
    let t128588 = F::cast_from(2.0_f64) * t1983 * t8640 * t20085;
    let t128592 = t1983 * t29377 * t8643;
    (t128577, t128581, t128584, t128588, t128592)
}
