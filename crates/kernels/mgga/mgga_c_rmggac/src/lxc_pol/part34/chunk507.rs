//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 507/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk507<F: Float>(t3204: F, t333: F, t13895: F, t13898: F, t13900: F, t13909: F, t13914: F, t13918: F, t13920: F, t13922: F, t13924: F, t13926: F, t13935: F, t13938: F, t13943: F, t13903: F, t13906: F, t13929: F, t13932: F, t13941: F) -> (F, F, F, F, F, F, F, F) {
    let t14473 = t3204 * t333;
    let t14476 = 0.49892673757765869017e-2 * t13895;
    let t14477 = 0.14967802127329760705e-1 * t13898;
    let t14478 = 0.26609426004141796809e-1 * t13900;
    let t14481 = 0.18183107769496894486e-1 * t13909;
    let t14482 = 0.31062809106223861416e-2 * t13914;
    let t14483 = 0.5177134851037310236e-2 * t13918;
    let t14484 = 0.55222771744397975851e-2 * t13920;
    let t14485 = 0.66380770525302906696e-3 * t13922;
    let t14486 = 0.99571155787954360044e-3 * t13924;
    let t14487 = 0.88507694033737208928e-3 * t13926;
    let t14490 = 0.48384206071776340879e-3 * t13935;
    let t14491 = 0.14464861606874801909e-3 * t13938;
    let t14493 = 0.12857654761666490586e-3 * t13943;
    let t14494 = t14476 - t14477 - t14478 - 0.68186654135613354322e-2 * t13903 + 0.13637330827122670864e-1 * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - 0.45360193192290319574e-3 * t13929 + 0.63504270469206447404e-3 * t13932 + t14490 + t14491 - 0.19286482142499735878e-3 * t13941 - t14493;
    (t14473, t14478, t14481, t14484, t14487, t14490, t14493, t14494)
}
