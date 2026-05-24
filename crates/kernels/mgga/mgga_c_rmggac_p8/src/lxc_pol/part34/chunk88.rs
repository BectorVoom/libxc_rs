//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 88/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk88<F: Float>(t84: F, t280: F, t90: F, t101: F, t266: F, t87: F, t91: F, t98: F, rho0: F, tau0: F) -> (F, F, F, F, F, F) {
    let t306 = rho0 * rho0;
    let t308 = F::new(1.0) / t84 / t306;
    let t309 = tau0 * t308;
    let t312 = t280 / F::new(2.0);
    let t313 = t90 * t312;
    let t316 = -t312;
    let t317 = t101 * t316;
    let t320 = F::new(2.0) / F::new(3.0) * t266;
    let t321 = -F::new(10.0) / F::new(3.0) * t309 * t91 + F::new(10.0) / F::new(3.0) * t87 * t313 + F::new(10.0) / F::new(3.0) * t98 * t317 + t320;
    (t309, t312, t316, t317, t320, t321)
}
