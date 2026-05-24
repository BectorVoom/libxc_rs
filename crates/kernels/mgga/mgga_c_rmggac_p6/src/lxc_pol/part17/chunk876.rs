//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 876/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk876<F: Float>(t1550: F, t2060: F, t44713: F, t558: F, t8708: F, t262: F, t7198: F, t570: F, t7204: F, t34884: F, t9971: F, t16503: F, t35039: F, t571: F, t8420: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44727 = t1550 * t2060 * t44713;
    let t44732 = t8708 * t558;
    let t44733 = t262 * t44732;
    let t44734 = t7198 * t44733;
    let t44736 = t8708 * t570;
    let t44737 = t262 * t44736;
    let t44738 = t7204 * t44737;
    let t44740 = t34884 * t9971;
    let t44744 = t16503 * t35039 * t571 * t8420;
    (t44727, t44732, t44733, t44734, t44736, t44737, t44738, t44740, t44744)
}
