//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1231/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1231<F: Float>(t105437: F, t105441: F, t17090: F, t21034: F, t26700: F, t29056: F, t29060: F, t29091: F, t4147: F, t4268: F, t5658: F, t7087: F, t7830: F, t85060: F, t86991: F) -> F {
    let t108378 = -F::new(3.0) * t26700 * t5658 + F::new(6.0) * t17090 * t7830 - t7087 * t21034 - F::new(18.0) * t4147 * t29091 - F::new(3.0) * t4147 * t29056 + F::new(0.9869604401089358619e-1) * t105437 - F::new(0.38381794893125283518e0) * t86991 + F::new(6.0) * t4268 * t29060 - F::new(18.0) * t4268 * t29091 - F::new(0.49348022005446793095e-1) * t105441 - t85060;
    t108378
}
