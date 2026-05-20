//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3086/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3086<F: Float>(t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F) -> F {
    let t63994 = F::new(40.0) / F::new(9.0) * t63380 + F::new(8.0) / F::new(27.0) * t63382 + F::new(8.0) / F::new(9.0) * t63384 - F::new(4.0) / F::new(3.0) * t63388 - F::new(8.0) * t63392 - F::new(4.0) / F::new(9.0) * t63396 - F::new(8.0) / F::new(9.0) * t63398 - F::new(4.0) / F::new(3.0) * t63400 + F::new(2.0) * t63404 + F::new(8.0) * t63408 + F::new(4.0) / F::new(3.0) * t63412 + F::new(10.0) / F::new(27.0) * t63417 - F::new(80.0) / F::new(81.0) * t63422;
    t63994
}
