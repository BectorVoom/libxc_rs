//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 852/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk852<F: Float>(t75575: F, t75580: F, t75583: F, t75585: F, t75587: F, t14589: F, t8533: F, t75598: F, t14424: F, t4985: F, t14427: F, t5928: F, t15629: F, t504: F, t69870: F, t71516: F, t75572: F, t75590: F, t75593: F, t75596: F, t75602: F) -> (F,) {
    let t77664 = 0.10248087766267884741e-3 * t75575;
    let t77665 = 0.38430329123504567781e-4 * t75580;
    let t77666 = 0.72042316457491791901e-3 * t75583;
    let t77669 = 0.1276937996798935182e-3 * t75585;
    let t77670 = 0.1915406995198402773e-3 * t75587;
    let t77671 = t14589 * t8533;
    let t77672 = 0.18183107769496894486e-1 * t77671;
    let t77677 = 0.15961724959986689775e-4 * t75598;
    let t77679 = 0.11974241701863808564e0 * t4985 * t14424;
    let t77681 = 0.11974241701863808564e0 * t5928 * t14427;
    let t77682 = -0.2363e1 * t71516 - 0.15372131649401827111e-4 * t75572 - t77664 - t77665 + t77666 - 0.19957069503106347607e-1 * t504 * t15629 + t77669 - t77670 - t77672 + 0.17347588262831798123e-4 * t75590 + 0.17347588262831798123e-4 * t75593 + 0.12263514265030957031e-4 * t69870 - 0.81756761766873046877e-6 * t75596 + t77677 + t75602 - t77679 - t77681;
    (t77682,)
}
