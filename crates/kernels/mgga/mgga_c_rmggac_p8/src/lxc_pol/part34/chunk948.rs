//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 948/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk948<F: Float>(t76840: F, t74050: F, t74052: F, t74056: F, t74041: F, t74049: F, t76828: F, t76829: F, t76830: F, t76831: F, t76832: F, t76834: F, t76835: F, t76836: F, t76837: F, t76838: F) -> F {
    let t76841 = F::cast_from(0.40650199722100037752e-3_f64) * t76840;
    let t76842 = F::cast_from(0.20455996240684006296e-1_f64) * t74050;
    let t76843 = F::cast_from(0.81823984962736025184e-1_f64) * t74052;
    let t76844 = F::cast_from(0.20455996240684006296e0_f64) * t74056;
    let t76845 = t76828 - t76829 - t76830 + t76831 + t76832 - F::cast_from(0.72714524817717142308e-5_f64) * t74041 + t76834 - t76835 + t74049 - t76836 + t76837 + t76838 - t76841 + t76842 + t76843 - t76844;
    t76845
}
