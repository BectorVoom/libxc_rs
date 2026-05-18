//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 942/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk942<F: Float>(t74533: F, t15624: F, t1971: F, t495: F, t515: F, t8517: F, t15617: F, t7508: F, t2145: F, t22: F, t656: F, t9486: F) -> (F, F, F, F) {
    let t77070 = F::new(0.18183107769496894487e-1) * t74533;
    let t77074 = t8517 * t1971 * t515 * t15624 * t495;
    let t77075 = F::new(0.11971293719990017331e-4) * t77074;
    let t77076 = t7508 * t15617;
    let t77077 = F::new(0.34093327067806677161e-2) * t77076;
    let t77080 = t2145 * t9486 * t22 * t656;
    (t77070, t77075, t77077, t77080)
}
