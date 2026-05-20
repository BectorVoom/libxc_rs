//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2121/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2121<F: Float>(t849: F, t87340: F, t25068: F, t2707: F, t1516: F, t81763: F, t23083: F, t25094: F, t1510: F, t2379: F, t25119: F, t815: F) -> (F, F, F, F, F) {
    let t87341 = t87340 * t849;
    let t87342 = F::new(7.0) / F::new(288.0) * t87341;
    let t87343 = t25068 * t2707;
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87348 = F::cast_from(0.56521858531796547196e-2_f64) * t87347;
    let t87351 = t25119 * t815 * t1510 * t2379;
    (t87342, t87343, t87345, t87348, t87351)
}
