//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1188/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1188<F: Float>(t1873: F, t24932: F, t27888: F, t6534: F, t7266: F, t31227: F, t31229: F, t31231: F, t31233: F, t31235: F, t31237: F, t31239: F, t31877: F, t31880: F, t671: F, t8446: F) -> F {
    let t31883 = t24932 * t1873;
    let t31885 = t27888 * t1873;
    let t31887 = t7266 * t6534;
    let t31892 = F::new(2.0) * t31880 * t671 + F::new(2.0) * t31227 + F::new(2.0) * t31229 + F::new(2.0) * t31231 + t31233 + t31235 + t31237 + t31239 + t31877 + F::new(2.0) * t31883 + F::new(2.0) * t31885 + F::new(2.0) * t31887 + t8446;
    t31892
}
