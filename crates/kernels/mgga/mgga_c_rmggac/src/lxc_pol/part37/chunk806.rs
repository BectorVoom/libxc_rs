//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 806/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk806<F: Float>(t14451: F, t1627: F, t26287: F, t8377: F, t30204: F, t1632: F, t1635: F, t26283: F, t5898: F, t26291: F, t74562: F, t74574: F, t74577: F, t74579: F, t74581: F, t74584: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77085 = t14451 * t1627;
    let t77086 = t26287 * t77085;
    let t77087 = 0.8980681276397856423e-1 * t77086;
    let t77088 = t14451 * t8377;
    let t77089 = t30204 * t77088;
    let t77090 = 0.5987120850931904282e-1 * t77089;
    let t77091 = t14451 * t1632;
    let t77092 = t26287 * t77091;
    let t77093 = 0.8980681276397856423e-1 * t77092;
    let t77094 = t14451 * t1635;
    let t77095 = t26283 * t77094;
    let t77096 = 0.17961362552795712846e0 * t77095;
    let t77097 = t14451 * t5898;
    let t77098 = t26291 * t77097;
    let t77099 = 0.8980681276397856423e-1 * t77098;
    let t77105 = 0.29085809927086856922e-4 * t74562;
    let t77107 = 0.23268647941669485538e-4 * t74574;
    let t77108 = 0.1276937996798935182e-4 * t74577;
    let t77109 = 0.85129199786595678799e-5 * t74579;
    let t77110 = 0.85129199786595678799e-5 * t74581;
    let t77111 = 0.85129199786595678799e-5 * t74584;
    (t77085, t77087, t77088, t77090, t77091, t77093, t77094, t77096, t77097, t77099, t77105, t77107, t77108, t77109, t77110, t77111)
}
