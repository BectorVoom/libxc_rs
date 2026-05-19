//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1055/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1055<F: Float>(t75393: F, t75395: F, t75397: F, t75400: F, t75402: F, t69303: F, t75405: F, t75407: F, t75409: F, t75412: F, t75414: F, t75417: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78165 = F::cast_from(0.26609426004141796809e-1_f64) * t75393;
    let t78166 = F::cast_from(0.5987120850931904282e-1_f64) * t75395;
    let t78167 = F::cast_from(0.53104616420242325357e-2_f64) * t75397;
    let t78168 = F::cast_from(0.14967802127329760705e-1_f64) * t75400;
    let t78169 = F::cast_from(0.79828278012425390427e-1_f64) * t75402;
    let t78170 = F::cast_from(0.53104616420242325357e-2_f64) * t69303;
    let t78171 = F::cast_from(0.99571155787954360044e-3_f64) * t75405;
    let t78172 = F::cast_from(0.66380770525302906696e-3_f64) * t75407;
    let t78173 = F::cast_from(0.35403077613494883571e-2_f64) * t75409;
    let t78174 = F::cast_from(0.14967802127329760705e-1_f64) * t75412;
    let t78175 = F::cast_from(0.14967802127329760705e-1_f64) * t75414;
    let t78176 = F::cast_from(0.5177134851037310236e-2_f64) * t75417;
    (t78165, t78166, t78167, t78168, t78169, t78170, t78171, t78172, t78173, t78174, t78175, t78176)
}
