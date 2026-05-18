//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 555/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk555<F: Float>(t3204: F, t333: F, t13895: F, t13898: F, t13900: F, t13909: F, t13914: F, t13918: F, t13920: F, t13922: F, t13924: F, t13926: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14473 = t3204 * t333;
    let t14476 = F::new(0.49892673757765869017e-2) * t13895;
    let t14477 = F::new(0.14967802127329760705e-1) * t13898;
    let t14478 = F::new(0.26609426004141796809e-1) * t13900;
    let t14481 = F::new(0.18183107769496894486e-1) * t13909;
    let t14482 = F::new(0.31062809106223861416e-2) * t13914;
    let t14483 = F::new(0.5177134851037310236e-2) * t13918;
    let t14484 = F::new(0.55222771744397975851e-2) * t13920;
    let t14485 = F::new(0.66380770525302906696e-3) * t13922;
    let t14486 = F::new(0.99571155787954360044e-3) * t13924;
    let t14487 = F::new(0.88507694033737208928e-3) * t13926;
    (t14473, t14476, t14477, t14478, t14481, t14482, t14483, t14484, t14485, t14486, t14487)
}
