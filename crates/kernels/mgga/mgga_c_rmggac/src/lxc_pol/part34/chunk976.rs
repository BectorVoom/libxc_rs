//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 976/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk976<F: Float>(t71112: F, t68910: F, t74753: F, t74772: F, t74775: F, t77183: F, t77184: F, t77185: F, t77186: F, t77187: F, t77189: F, t77190: F, t77191: F, t77192: F, t77193: F, t77195: F, t77196: F) -> F {
    let t77197 = F::new(0.29795219925308487579e-4) * t71112;
    let t77200 = t77183 - t77184 - t77185 + t77186 + t77187 + F::new(0.17451485956252114154e-4) * t74753 + t77189 - t77190 + t77191 - t77192 - t77193 - F::new(0.13139479569676025391e-5) * t74772 - t77195 - t77196 + t77197 - F::new(0.4379826523225341797e-6) * t74775 - F::new(0.16566831523319392755e-1) * t68910;
    t77200
}
