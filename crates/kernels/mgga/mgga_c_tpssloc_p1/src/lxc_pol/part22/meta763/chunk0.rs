//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2570/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2570<F: Float>(t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F, t71461: F, t71463: F, t71465: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F) -> (F, F) {
    let t71955 = -F::new(0.103295e1) * t71183 - F::new(0.103295e1) * t71187 + F::new(0.31558125e0) * t71446 - F::new(0.17648625e1) * t71449 - F::cast_from(0.6618234375e1_f64) * t71452 + F::cast_from(0.794188125e1_f64) * t71454 - F::new(0.52945875e1) * t71456 - F::new(0.52945875e1) * t71458 + F::cast_from(0.2366859375e0_f64) * t71461 - F::cast_from(0.473371875e0_f64) * t71463 + F::new(0.94674375e0) * t71465;
    let t71968 = F::new(0.94674375e0) * t71468 - F::cast_from(0.30872592592592592593e-1_f64) * t71470 + F::cast_from(0.13892666666666666667e0_f64) * t71472 - F::new(0.41678e0) * t71474 + F::new(0.20839e0) * t71477 - F::new(0.104195e0) * t71480 - F::new(0.104195e0) * t71483 + F::new(0.62517e0) * t71486 + F::new(0.62517e0) * t71489 + F::new(0.309885e1) * t71191 - F::new(0.61977e1) * t71195 - F::new(0.123954e2) * t71199;
    (t71955, t71968)
}
