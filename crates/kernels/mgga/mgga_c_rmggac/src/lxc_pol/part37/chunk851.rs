//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 851/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk851<F: Float>(t69303: F, t75405: F, t75407: F, t75409: F, t75412: F, t75414: F, t75417: F, t75423: F, t69313: F, t15536: F, t40826: F, t72062: F, t14451: F, t1614: F, t4669: F, t72020: F, t8636: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78170 = 0.53104616420242325357e-2 * t69303;
    let t78171 = 0.99571155787954360044e-3 * t75405;
    let t78172 = 0.66380770525302906696e-3 * t75407;
    let t78173 = 0.35403077613494883571e-2 * t75409;
    let t78174 = 0.14967802127329760705e-1 * t75412;
    let t78175 = 0.14967802127329760705e-1 * t75414;
    let t78176 = 0.5177134851037310236e-2 * t75417;
    let t78179 = 0.14464861606874801909e-3 * t75423;
    let t78181 = 0.35403077613494883571e-2 * t69313;
    let t78188 = t40826 * t15536;
    let t78189 = 0.2993560425465952141e-1 * t78188;
    let t78194 = 0.90915538847484472429e-2 * t72062;
    let t78198 = t4669 * t14451 * t1614;
    let t78199 = 0.44903406381989282115e-1 * t78198;
    let t78200 = t72020 * t8636;
    (t78170, t78171, t78172, t78173, t78174, t78175, t78176, t78179, t78181, t78189, t78194, t78199, t78200)
}
