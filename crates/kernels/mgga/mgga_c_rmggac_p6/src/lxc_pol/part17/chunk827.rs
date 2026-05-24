//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 827/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk827<F: Float>(t275: F, t9064: F, t1679: F, t7197: F, t34760: F, t9221: F, t352: F, t8915: F, t5148: F, t333: F, t4669: F, t128: F, t30526: F) -> (F, F, F, F, F, F, F, F) {
    let t40750 = F::new(2.0) * t275 * t9064;
    let t40759 = t1679 * t7197;
    let t40771 = t9221 * t34760;
    let t40802 = t8915 * t352;
    let t40803 = t5148 * t40802;
    let t40804 = F::cast_from(0.15965655602485078085e0_f64) * t40803;
    let t40805 = t8915 * t333;
    let t40806 = t4669 * t40805;
    let t40807 = F::cast_from(0.23948483403727617128e0_f64) * t40806;
    let t40823 = t30526 * t128;
    (t40750, t40759, t40771, t40802, t40804, t40805, t40807, t40823)
}
