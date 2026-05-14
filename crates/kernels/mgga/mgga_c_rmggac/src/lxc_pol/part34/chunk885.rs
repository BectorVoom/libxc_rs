//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 885/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk885<F: Float>(t75393: F, t75395: F, t75397: F, t75400: F, t75402: F, t69303: F, t75405: F, t75407: F, t75409: F, t75412: F, t75414: F, t75417: F, t75423: F, t69313: F, t75419: F, t75421: F, t75425: F) -> (F,) {
    let t78165 = 0.26609426004141796809e-1 * t75393;
    let t78166 = 0.5987120850931904282e-1 * t75395;
    let t78167 = 0.53104616420242325357e-2 * t75397;
    let t78168 = 0.14967802127329760705e-1 * t75400;
    let t78169 = 0.79828278012425390427e-1 * t75402;
    let t78170 = 0.53104616420242325357e-2 * t69303;
    let t78171 = 0.99571155787954360044e-3 * t75405;
    let t78172 = 0.66380770525302906696e-3 * t75407;
    let t78173 = 0.35403077613494883571e-2 * t75409;
    let t78174 = 0.14967802127329760705e-1 * t75412;
    let t78175 = 0.14967802127329760705e-1 * t75414;
    let t78176 = 0.5177134851037310236e-2 * t75417;
    let t78179 = 0.14464861606874801909e-3 * t75423;
    let t78181 = 0.35403077613494883571e-2 * t69313;
    let t78182 = -t78165 + t78166 + t78167 - t78168 + t78169 + t78170 - t78171 + t78172 - t78173 - t78174 - t78175 - t78176 - 0.45360193192290319574e-3 * t75419 + 0.63504270469206447404e-3 * t75421 + t78179 - 0.19286482142499735878e-3 * t75425 - t78181;
    (t78182,)
}
