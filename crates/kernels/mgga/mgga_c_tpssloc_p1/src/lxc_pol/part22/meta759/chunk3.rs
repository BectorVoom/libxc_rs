//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2552/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2552<F: Float>(t71183: F, t71187: F, t71446: F, t71449: F, t71452: F, t71454: F, t71456: F, t71458: F, t71461: F, t71463: F, t71465: F, t71191: F, t71195: F, t71199: F, t71468: F, t71470: F, t71472: F, t71474: F, t71477: F, t71480: F, t71483: F, t71486: F, t71489: F) -> (F, F) {
    let t71611 = -F::cast_from(0.59793333333333333333e0_f64) * t71183 - F::cast_from(0.59793333333333333333e0_f64) * t71187 + F::new(0.15358125e0) * t71446 - F::new(0.9494625e0) * t71449 - F::cast_from(0.3560484375e1_f64) * t71452 + F::cast_from(0.427258125e1_f64) * t71454 - F::new(0.28483875e1) * t71456 - F::new(0.28483875e1) * t71458 + F::cast_from(0.1151859375e0_f64) * t71461 - F::cast_from(0.230371875e0_f64) * t71463 + F::new(0.46074375e0) * t71465;
    let t71624 = F::new(0.46074375e0) * t71468 - F::cast_from(0.2434271604938271605e-1_f64) * t71470 + F::cast_from(0.10954222222222222222e0_f64) * t71472 - F::cast_from(0.32862666666666666666e0_f64) * t71474 + F::cast_from(0.16431333333333333333e0_f64) * t71477 - F::cast_from(0.82156666666666666668e-1_f64) * t71480 - F::cast_from(0.82156666666666666668e-1_f64) * t71483 + F::cast_from(0.49293999999999999999e0_f64) * t71486 + F::cast_from(0.49293999999999999999e0_f64) * t71489 + F::new(0.17938e1) * t71191 - F::cast_from(0.35876000000000000001e1_f64) * t71195 - F::cast_from(0.71752000000000000002e1_f64) * t71199;
    (t71611, t71624)
}
