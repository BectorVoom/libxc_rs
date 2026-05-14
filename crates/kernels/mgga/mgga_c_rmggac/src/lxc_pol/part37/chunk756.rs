//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 756/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk756<F: Float>(t13989: F, t39705: F, t13993: F, t39570: F, t14004: F, t44788: F, t14170: F, t26857: F, t14368: F, t15350: F, t15411: F, t68891: F, t14077: F, t15290: F, t7282: F, t12200: F, t15313: F) -> (F, F, F, F, F, F, F, F) {
    let t75719 = t39705 * t13989;
    let t75721 = t39570 * t13993;
    let t75723 = t44788 * t14004;
    let t75725 = t26857 * t14170;
    let t75729 = t14368 * t15350;
    let t75733 = t68891 * t15411;
    let t75736 = t7282 * t14077 * t15290;
    let t75739 = t12200 * t14077 * t15313;
    (t75719, t75721, t75723, t75725, t75729, t75733, t75736, t75739)
}
