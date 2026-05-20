//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2409/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2409<F: Float>(t42087: F, t47787: F, t59700: F, t59702: F, t59704: F, t60274: F, t68619: F, t68626: F, t68628: F, t68630: F, t68633: F, t68635: F) -> F {
    let t68864 = t42087 - F::cast_from(0.3560484375e1_f64) * t68619 + F::cast_from(0.5477111111111111111e-1_f64) * t60274 - F::cast_from(0.11958666666666666667e1_f64) * t59700 + F::cast_from(0.39862222222222222222e0_f64) * t59702 + F::cast_from(0.33218518518518518518e0_f64) * t59704 + F::cast_from(0.93011851851851851854e0_f64) * t47787 + F::cast_from(0.427258125e1_f64) * t68626 - F::new(0.28483875e1) * t68628 - F::new(0.28483875e1) * t68630 + F::cast_from(0.1151859375e0_f64) * t68633 - F::cast_from(0.230371875e0_f64) * t68635;
    t68864
}
