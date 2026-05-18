//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 944/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk944<F: Float>(t77095: F, t14451: F, t5898: F, t26291: F, t74562: F, t74574: F, t74577: F, t74579: F, t74581: F, t74584: F, t74590: F, t15616: F, t2106: F, t2145: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77096 = F::new(0.17961362552795712846e0) * t77095;
    let t77097 = t14451 * t5898;
    let t77098 = t26291 * t77097;
    let t77099 = F::new(0.8980681276397856423e-1) * t77098;
    let t77105 = F::new(0.29085809927086856922e-4) * t74562;
    let t77107 = F::new(0.23268647941669485538e-4) * t74574;
    let t77108 = F::new(0.1276937996798935182e-4) * t74577;
    let t77109 = F::new(0.85129199786595678799e-5) * t74579;
    let t77110 = F::new(0.85129199786595678799e-5) * t74581;
    let t77111 = F::new(0.85129199786595678799e-5) * t74584;
    let t77113 = F::new(0.2627895913935205078e-5) * t74590;
    let t77116 = t2145 * t15616 * t2106;
    (t77096, t77097, t77099, t77105, t77107, t77108, t77109, t77110, t77111, t77113, t77116)
}
