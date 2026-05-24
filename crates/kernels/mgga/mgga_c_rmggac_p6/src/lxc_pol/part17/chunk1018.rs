//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1018/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1018<F: Float>(t1756: F, t7778: F, t305: F, t45418: F, t5271: F, t46258: F, t5162: F, t46415: F, t4669: F, t1704: F, t2124: F, t27048: F, t46541: F) -> (F, F, F, F, F, F, F) {
    let t46764 = t7778 * t1756;
    let t46765 = t305 * t46764;
    let t46770 = t5271 * t45418;
    let t46772 = t5162 * t46258;
    let t46774 = t4669 * t46415;
    let t46779 = t2124 * t1704;
    let t46782 = t27048 * t46541;
    (t46764, t46765, t46770, t46772, t46774, t46779, t46782)
}
