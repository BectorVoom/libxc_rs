//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 254/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk254<F: Float>(t974: F, t986: F, t346: F, t964: F, t971: F, t973: F, t980: F) -> F {
    let t987 = t974 * t986;
    let t990 = -F::new(0.22222222222222222222e-2) * t964 * t346 + t971 + F::new(0.27777777777777777777e-3) * t973 * t980 - F::new(0.83333333333333333332e-3) * t973 * t987;
    t990
}
