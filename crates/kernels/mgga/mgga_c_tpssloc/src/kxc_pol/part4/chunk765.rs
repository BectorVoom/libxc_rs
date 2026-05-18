//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 765/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk765<F: Float>(t2349: F, t5480: F, t5396: F, t103: F, t100: F, t104: F, t1447: F, t1450: F, t5469: F, t5472: F, t5475: F, t92: F) -> (F, F) {
    let t5481 = t2349 * t5480;
    let t5484 = -t5396;
    let t5485 = t103 * t5484;
    let t5488 = F::new(10.0) / F::new(9.0) * t92 * t5469 + F::new(5.0) / F::new(3.0) * t92 * t5472 + F::new(40.0) / F::new(9.0) * t5475 * t104 - F::new(50.0) / F::new(9.0) * t1447 * t1450 + F::new(10.0) / F::new(9.0) * t100 * t5481 + F::new(5.0) / F::new(3.0) * t100 * t5485;
    (t5484, t5488)
}
