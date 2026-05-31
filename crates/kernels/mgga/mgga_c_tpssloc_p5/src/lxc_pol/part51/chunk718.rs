//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 718/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk718<F: Float>(t362: F, t381: F, t884: F, t6784: F, t1949: F, t986: F, t334: F, t371: F, t38: F, t131: F, t350: F) -> (F, F, F, F, F, F, F, F) {
    let t6785 = t362 * t381;
    let t6786 = t6785 * t884;
    let t6787 = t6784 * t6786;
    let t6790 = t986 * t1949;
    let t6793 = t371 * t334;
    let t6794 = F::cast_from(1.0_f64) / t6793;
    let t6795 = t38 * t6794;
    let t6796 = t6795 * t131;
    let t6797 = t6796 * t350;
    (t6785, t6786, t6787, t6790, t6794, t6795, t6796, t6797)
}
