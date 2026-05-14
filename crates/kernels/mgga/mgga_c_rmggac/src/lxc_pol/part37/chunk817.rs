//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 817/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk817<F: Float>(t74994: F, t16503: F, t16504: F, t699: F, t9151: F, t3369: F, t9157: F, t74997: F, t69060: F, t2333: F, t71404: F, t14589: F, t8568: F, t2329: F, t71400: F, t14585: F, t8562: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77353 = 0.1276937996798935182e-4 * t74994;
    let t77356 = t16503 * t16504 * t699 * t9151;
    let t77357 = 0.12769379967989351819e-4 * t77356;
    let t77360 = t16503 * t3369 * t699 * t9157;
    let t77361 = 0.12769379967989351819e-4 * t77360;
    let t77362 = 0.14967802127329760705e-1 * t74997;
    let t77363 = 0.16263363996404810741e-4 * t69060;
    let t77364 = t71404 * t2333;
    let t77365 = 0.68186654135613354322e-2 * t77364;
    let t77366 = t14589 * t8568;
    let t77367 = 0.68186654135613354322e-2 * t77366;
    let t77369 = t71400 * t2329;
    let t77370 = 0.13637330827122670864e-1 * t77369;
    let t77371 = t14585 * t8562;
    (t77353, t77357, t77361, t77362, t77363, t77365, t77367, t77370, t77371)
}
