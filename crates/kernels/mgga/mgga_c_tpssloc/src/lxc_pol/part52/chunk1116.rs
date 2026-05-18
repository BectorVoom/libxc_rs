//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1116/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1116<F: Float>(t27561: F, t3961: F, t27550: F, t24589: F, t24845: F, t24849: F, t27533: F, t27537: F, t27540: F, t27543: F, t27546: F, t27549: F, t27553: F, t27556: F, t27558: F, t3604: F, t3610: F, t3624: F, t7373: F, t8083: F) -> F {
    let t27562 = t27561 * t3961;
    let t27563 = t27550 * t27562;
    let t27568 = -F::new(0.27415567780803773942e-2) * t24849 * t27533 - F::new(0.82246703342411321825e-2) * t7373 * t27537 - F::new(0.82246703342411321825e-2) * t7373 * t27540 + F::new(2.0) * t3610 * t27543 - t3624 * t27546 + F::new(0.36554090374405031923e-2) * t27549 * t27553 + F::new(0.27415567780803773942e-2) * t27556 + F::new(0.27415567780803773942e-2) * t24589 * t27558 - F::new(0.54831135561607547884e-2) * t24589 * t27563 + F::new(0.27415567780803773942e-2) * t24845 + t3604 * t8083;
    t27568
}
