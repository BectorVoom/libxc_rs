//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 569/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk569<F: Float>(t14364: F, t14369: F, t13935: F, t13903: F, t13906: F, t13929: F, t13932: F, t13941: F, t14476: F, t14477: F, t14478: F, t14481: F, t14482: F, t14483: F, t14484: F, t14485: F, t14486: F, t14487: F, t14491: F, t14493: F) -> (F, F, F, F) {
    let t14918 = F::cast_from(0.1276937996798935182e-3_f64) * t14364;
    let t14919 = F::cast_from(0.16351352353374609375e-5_f64) * t14369;
    let t14933 = F::cast_from(0.4838420607177634088e-3_f64) * t13935;
    let t14935 = t14476 - t14477 - t14478 - F::cast_from(0.68186654135613354324e-2_f64) * t13903 + F::cast_from(0.13637330827122670865e-1_f64) * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - F::cast_from(0.45360193192290319575e-3_f64) * t13929 + F::cast_from(0.63504270469206447405e-3_f64) * t13932 + t14933 + t14491 - F::cast_from(0.19286482142499735879e-3_f64) * t13941 - t14493;
    (t14918, t14919, t14933, t14935)
}
