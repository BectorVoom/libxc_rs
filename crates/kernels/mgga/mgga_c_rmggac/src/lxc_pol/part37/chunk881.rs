//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 881/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk881<F: Float>(t74015: F, t74041: F, t74049: F, t76821: F, t76823: F, t76828: F, t76829: F, t76830: F, t76831: F, t76832: F, t76834: F, t76835: F, t76836: F, t76837: F, t76838: F, t76841: F) -> (F,) {
    let t80028 = t76821 - 0.72714524817717142305e-5 * t74015 - t76823 + t76828 - t76829 - t76830 + t76831 + t76832 - 0.72714524817717142305e-5 * t74041 + t76834 - t76835 + t74049 - t76836 + t76837 + t76838 - t76841;
    (t80028,)
}
