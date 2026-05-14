//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 896/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk896<F: Float>(t71042: F, t71043: F, t71046: F, t73276: F, t74713: F, t74716: F, t77156: F, t77157: F, t77158: F, t77159: F, t77160: F, t77161: F, t77162: F, t77164: F, t77167: F, t77170: F, t77171: F) -> (F,) {
    let t80147 = t77156 - t77157 - t77158 + t77159 + t77160 + t77161 - t77162 + t77164 - t77167 - t77170 - t71042 + t71043 + t77171 + t71046 + t74713 + t74716 - t73276;
    (t80147,)
}
