//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 976/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk976<F: Float>(t36103: F, t46083: F, t46086: F, t8750: F, t46089: F, t7603: F, t46092: F, t36110: F, t46095: F, t41329: F, t46098: F, t46101: F, t8761: F) -> (F, F, F, F, F, F, F) {
    let t46150 = t36103 * t46083;
    let t46152 = t8750 * t46086;
    let t46154 = t7603 * t46089;
    let t46156 = t7603 * t46092;
    let t46158 = t36110 * t46095;
    let t46160 = t41329 * t46098;
    let t46162 = t8761 * t46101;
    (t46150, t46152, t46154, t46156, t46158, t46160, t46162)
}
