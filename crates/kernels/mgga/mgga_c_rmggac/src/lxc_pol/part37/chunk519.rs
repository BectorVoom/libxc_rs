//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 519/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk519<F: Float>(t14696: F, t2019: F, t2604: F, t3188: F, t14375: F, t2080: F, t2211: F, t739: F, t14108: F, t14364: F, t14369: F, t13935: F, t13903: F, t13906: F, t13929: F, t13932: F, t13941: F, t14476: F, t14477: F, t14478: F, t14481: F, t14482: F, t14483: F, t14484: F, t14485: F, t14486: F, t14487: F, t14491: F, t14493: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14697 = t2019 * t14696;
    let t14701 = t2604 * t3188;
    let t14702 = 0.14967802127329760705e-1 * t14701;
    let t14709 = 0.1276937996798935182e-4 * t14375;
    let t14710 = t2211 * t2080;
    let t14711 = t739 * t14710;
    let t14712 = 0.2993560425465952141e-1 * t14711;
    let t14865 = 0.15965655602485078085e0 * t14108;
    let t14918 = 0.1276937996798935182e-3 * t14364;
    let t14919 = 0.16351352353374609375e-5 * t14369;
    let t14933 = 0.4838420607177634088e-3 * t13935;
    let t14935 = t14476 - t14477 - t14478 - 0.68186654135613354324e-2 * t13903 + 0.13637330827122670865e-1 * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - 0.45360193192290319575e-3 * t13929 + 0.63504270469206447405e-3 * t13932 + t14933 + t14491 - 0.19286482142499735879e-3 * t13941 - t14493;
    (t14697, t14702, t14709, t14710, t14712, t14865, t14918, t14919, t14933, t14935)
}
