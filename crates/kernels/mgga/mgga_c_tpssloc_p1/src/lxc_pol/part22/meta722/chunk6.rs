//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2362/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2362<F: Float>(t16625: F, t1877: F, t25374: F, t4255: F, t4314: F, t59564: F, t67499: F, t67500: F, t67501: F, t67502: F, t67503: F, t67504: F, t67506: F, t67507: F, t67508: F) -> F {
    let t68414 = -F::new(18.0) * t16625 * t4255 * t4314 + F::new(6.0) * t1877 * t25374 * t59564 + t67499 + t67500 - t67501 + t67502 + t67503 + t67504 + t67506 - t67507 - t67508;
    t68414
}
