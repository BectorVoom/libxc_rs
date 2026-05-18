//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 766/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk766<F: Float>(t14185: F, t17859: F, t14189: F, t14193: F, t14199: F, t3154: F, t38472: F, t1971: F, t2367: F, t495: F, t515: F, t7230: F) -> (F, F, F, F, F, F) {
    let t73873 = t17859 * t14185;
    let t73875 = t17859 * t14189;
    let t73877 = t17859 * t14193;
    let t73879 = t17859 * t14199;
    let t73881 = t38472 * t3154;
    let t73887 = F::new(0.1064114997332445985e-4) * t7230 * t1971 * t515 * t2367 * t495;
    (t73873, t73875, t73877, t73879, t73881, t73887)
}
