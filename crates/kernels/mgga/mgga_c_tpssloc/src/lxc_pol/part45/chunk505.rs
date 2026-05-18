//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 505/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk505<F: Float>(t2586: F, t3749: F, t1315: F, t3725: F, t3727: F, t3731: F, t3733: F, t3736: F, t3742: F, t3745: F) -> F {
    let t3751 = F::new(0.83333333333333333332e-3) * t2586 * t3749;
    let t3752 = t3725 + F::new(0.77777777777777777775e-2) * t3727 + t3731 + F::new(0.49999999999999999998e-2) * t3733 * t3736 + F::new(0.16666666666666666666e-2) * t3742 - F::new(0.16666666666666666666e-2) * t1315 * t3745 - t3751;
    t3752
}
