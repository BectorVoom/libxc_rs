//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2295/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2295<F: Float>(t8015: F, t94490: F, t24574: F, t29682: F, t29691: F, t24589: F, t24880: F, t27392: F, t27406: F, t27437: F, t27761: F, t29536: F, t3487: F, t4945: F, t6146: F, t6268: F, t7283: F, t7295: F, t94475: F, t94476: F, t94492: F, t94494: F, t94514: F, t94525: F) -> F {
    let t103286 = t94490 * t8015;
    let t103291 = t24574 * t29682;
    let t103293 = t24574 * t29691;
    let t103303 = -t24880 * t6268 - t94475 + F::cast_from(0.36554090374405031923e-2_f64) * t94476 + t94492 + t94494 + F::new(4.0) * t4945 * t27761 + F::cast_from(0.14621636149762012769e-1_f64) * t103286 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6146 * t7295 - F::cast_from(0.27415567780803773942e-2_f64) * t103291 + F::cast_from(0.12184696791468343974e-2_f64) * t103293 - F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27392 - F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t94514 * t27437 + F::cast_from(0.12184696791468343974e-2_f64) * t94525 + F::new(2.0) * t3487 * t29536;
    t103303
}
