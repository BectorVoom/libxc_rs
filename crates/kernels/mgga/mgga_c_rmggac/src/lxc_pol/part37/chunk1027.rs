//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1027/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1027<F: Float>(t70618: F, t76550: F, t14387: F, t14389: F, t14393: F, t14398: F, t14399: F, t14400: F, t15051: F, t15426: F, t15427: F, t15428: F, t15429: F, t15430: F, t15856: F, t15857: F, t70657: F) -> (F, F, F) {
    let t78612 = F::new(0.16263363996404810741e-4) * t70618;
    let t78613 = F::new(0.14967802127329760705e-1) * t76550;
    let t79943 = -t15856 - t15857 + t15426 + t15427 - t15428 - t15429 + t15051 + t14387 - t14389 + t14393 + t14398 - t14399 + t14400 + t70657 - t15430;
    (t78612, t78613, t79943)
}
