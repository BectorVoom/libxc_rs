//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1347/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1347<F: Float>(t1256: F, t193: F, t336: F, t3640: F, t44161: F, t44164: F, t44167: F, t44358: F, t44375: F, t44377: F, t44378: F, t44384: F, t44388: F, t44392: F, t44396: F, t44400: F, t45344: F, t45382: F) -> (F,) {
    let t45387 = -t44161 - t44164 + t44167 - t44375 - t44377 - 3.0 * t193 * t336 * t44378 * t3640 + t44384 - t44388 + t44392 - t44358 + t44396 + t44400 + t193 * t336 * (t45344 + t45382) * t1256;
    (t45387,)
}
