//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 888/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk888<F: Float>(t10121: F, t193: F, t202: F, t2379: F, t2522: F, t2523: F, t2553: F, t262: F, t4314: F, t766: F, t776: F, t870: F, t9450: F, t9457: F, t9458: F, t9463: F, t9469: F, t9470: F, t9476: F, t9484: F, t9496: F, t9516: F) -> F {
    let t10125 = t10121 * t193 * t202 * t870 + F::cast_from(6.0_f64) * t193 * t262 * t9458 + F::cast_from(3.0_f64) * t193 * t766 * t9516 + F::cast_from(18.0_f64) * t2379 * t2523 * t4314 + F::cast_from(9.0_f64) * t2522 * t2523 * t2553 - F::cast_from(9.0_f64) * t2522 * t776 * t9470 + t9450 - t9457 + t9463 - t9469 + t9476 + t9484 - t9496;
    t10125
}
