//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 978/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk978<F: Float>(t1704: F, t265: F, t262: F, t7648: F, t1737: F, t7653: F, t36094: F, t36096: F, t46150: F, t46152: F, t46154: F, t46156: F, t46158: F, t46160: F, t46162: F, t46165: F, t46168: F, t46170: F, t46172: F, t46178: F) -> (F, F, F, F, F) {
    let t46180 = t265 * t1704;
    let t46181 = t262 * t46180;
    let t46182 = t7648 * t46181;
    let t46184 = t265 * t1737;
    let t46185 = t262 * t46184;
    let t46186 = t7653 * t46185;
    let t46188 = -F::new(0.31752135234603223702e-2) * t46150 + F::new(0.9072038638458063915e-3) * t46152 - F::new(0.12700854093841289481e-2) * t46154 - F::new(0.12700854093841289481e-2) * t46156 + F::new(0.50803416375365157926e-2) * t46158 - F::new(0.7620512456304773689e-2) * t46160 + F::new(0.16934472125121719309e-2) * t46162 + F::new(0.45360193192290319575e-3) * t46165 - F::new(0.63504270469206447405e-3) * t46168 - F::new(0.63504270469206447408e-3) * t46170 + F::new(0.84672360625608596544e-3) * t46172 + F::new(0.33335697577410973224e-1) * t36094 - F::new(0.44447596769881297632e-1) * t36096 + F::new(0.12122071846331262991e-1) * t46178 - F::new(0.1209605151794408522e-2) * t46182 - F::new(0.22579296166828959078e-2) * t46186;
    (t46180, t46181, t46184, t46185, t46188)
}
