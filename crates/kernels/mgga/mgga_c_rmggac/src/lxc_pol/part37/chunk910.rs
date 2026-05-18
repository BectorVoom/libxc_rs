//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 910/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk910<F: Float>(t5148: F, t74811: F, t15075: F, t30526: F, t3851: F, t75886: F, t75216: F, t793: F, t41400: F, t649: F, t8950: F, t40932: F, t8979: F) -> (F, F, F, F, F, F) {
    let t76326 = F::new(0.5987120850931904282e-1) * t5148 * t74811;
    let t76331 = t30526 * t15075;
    let t76333 = t3851 * t75886;
    let t76337 = t793 * t75216;
    let t76340 = t41400 * t649 * t8950;
    let t76343 = t40932 * t649 * t8979;
    (t76326, t76331, t76333, t76337, t76340, t76343)
}
