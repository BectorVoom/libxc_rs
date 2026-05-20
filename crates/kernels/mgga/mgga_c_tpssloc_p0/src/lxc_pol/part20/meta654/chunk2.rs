//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2421/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2421<F: Float>(t48762: F, t48765: F, t48768: F, t48770: F, t49068: F, t49071: F, t49075: F, t49080: F, t49496: F, t49499: F, t49502: F, t49506: F, t49508: F, t49510: F, t49512: F, t49517: F, t49520: F, t49522: F, t49525: F, t49529: F) -> F {
    let t49530 = t48762 - t48765 - t48768 - t48770 + t49496 - t49499 + t49502 + t49506 - t49508 - t49510 + t49512 - t49517 + t49520 + t49522 - t49525 - t49529 + t49068 + t49071 + t49075 + t49080;
    t49530
}
