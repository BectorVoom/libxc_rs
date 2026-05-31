//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2273/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2273<F: Float>(t2319: F, t7982: F, t12550: F, t1266: F, t12841: F, t1774: F, t24935: F, t27290: F, t27371: F, t3652: F, t4034: F, t510: F, t7266: F, t7983: F, t91564: F, t91568: F, t91570: F, t91573: F, t91578: F, t91580: F, t91582: F, t91585: F, t91587: F, t91589: F, t91591: F, t91593: F) -> (F, F) {
    let t94265 = t7982 * t2319;
    let t94272 = -F::cast_from(4.0_f64) * t12550 * t7266 - F::cast_from(2.0_f64) * t1266 * t27371 - F::cast_from(2.0_f64) * t12841 * t7266 - F::cast_from(2.0_f64) * t1774 * t24935 - F::cast_from(4.0_f64) * t27290 * t4034 - t3652 * t7983 - F::cast_from(2.0_f64) * t510 * t94265 + t91564 + t91568 - t91570 - t91573 - t91578 - t91580 + t91582 + t91585 - t91587 - t91589 - t91591 - t91593;
    (t94265, t94272)
}
