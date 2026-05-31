//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 823/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk823<F: Float>(t533: F, t8492: F, t3701: F, t1983: F, t3941: F, t8319: F, t1401: F, t8326: F, t131: F, t8306: F) -> (F, F, F, F, F, F) {
    let t8493 = t533 * t8492;
    let t8494 = t8493 * t3701;
    let t8495 = t1983 * t8494;
    let t8506 = F::cast_from(27.0_f64) * t3941 * t8319;
    let t8508 = F::cast_from(0.135e2_f64) * t1401 * t8326;
    let t8513 = t131 * t8306;
    (t8493, t8494, t8495, t8506, t8508, t8513)
}
