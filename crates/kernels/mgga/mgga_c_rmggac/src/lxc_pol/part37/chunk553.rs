//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 553/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk553<F: Float>(t265: F, t699: F, t305: F, t13895: F, t13898: F, t13900: F, t13909: F, t13914: F, t13918: F, t13920: F, t13922: F, t13924: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14469 = t699 * t265;
    let t14470 = t305 * t14469;
    let t14471 = F::cast_from(0.39914139006212695213e-1_f64) * t14470;
    let t14476 = F::cast_from(0.49892673757765869017e-2_f64) * t13895;
    let t14477 = F::cast_from(0.14967802127329760705e-1_f64) * t13898;
    let t14478 = F::cast_from(0.26609426004141796809e-1_f64) * t13900;
    let t14481 = F::cast_from(0.18183107769496894486e-1_f64) * t13909;
    let t14482 = F::cast_from(0.31062809106223861416e-2_f64) * t13914;
    let t14483 = F::cast_from(0.5177134851037310236e-2_f64) * t13918;
    let t14484 = F::cast_from(0.55222771744397975851e-2_f64) * t13920;
    let t14485 = F::cast_from(0.66380770525302906696e-3_f64) * t13922;
    let t14486 = F::cast_from(0.99571155787954360044e-3_f64) * t13924;
    (t14469, t14471, t14476, t14477, t14478, t14481, t14482, t14483, t14484, t14485, t14486)
}
