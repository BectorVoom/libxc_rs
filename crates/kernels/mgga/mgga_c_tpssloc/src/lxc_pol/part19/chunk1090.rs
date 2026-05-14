//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1090/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1090<F: Float>(t39375: F, t39384: F, t39388: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t39456: F, t39463: F, t39468: F, t39472: F, t39476: F, t39479: F, t39483: F, t39490: F, t39492: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39511: F, t39513: F, t39515: F) -> (F, F) {
    let t40213 = -t39375 - t39384 - t39388 + t39393 - t39397 - t39400 + t39408 + t39411 + t39456 + t39463 - t39468 - t39472;
    let t40214 = -t39476 - t39479 + t39483 - t39490 + t39492 - t39496 + t39499 + t39502 - t39505 - t39508 + t39511 + t39513 - t39515;
    (t40213, t40214)
}
