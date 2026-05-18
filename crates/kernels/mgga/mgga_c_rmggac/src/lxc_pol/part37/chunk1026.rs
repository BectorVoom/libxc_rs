//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1026/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1026<F: Float>(t78596: F, t1356: F, t13957: F, t43974: F, t7879: F, t884: F, t9530: F, t577: F, t703: F, t7933: F, t7934: F, t76547: F) -> (F, F, F, F, F) {
    let t78597 = F::new(0.36366215538993788971e-1) * t78596;
    let t78602 = F::new(0.11974241701863808564e0) * t1356 * t43974 * t13957;
    let t78605 = F::new(0.11974241701863808564e0) * t884 * t9530 * t7879;
    let t78608 = t7933 * t7934 * t577 * t703;
    let t78609 = F::new(0.36021158228745895953e-3) * t78608;
    let t78611 = F::new(0.20496175532535769483e-3) * t76547;
    (t78597, t78602, t78605, t78609, t78611)
}
