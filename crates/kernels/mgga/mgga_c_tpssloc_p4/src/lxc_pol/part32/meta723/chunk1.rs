//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2307/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2307<F: Float>(t24574: F, t29702: F, t103515: F, t11907: F, t1216: F, t1716: F, t18525: F, t18946: F, t19203: F, t2148: F, t24812: F, t27489: F, t27490: F, t27492: F, t27496: F, t27507: F, t27510: F, t27536: F, t27540: F, t27732: F, t29709: F, t3610: F, t6140: F, t7283: F, t7373: F, t7381: F, t8082: F, t94858: F, t95033: F) -> F {
    let t103744 = t24574 * t29702;
    let t103766 = -F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t27536 * t27510 + F::new(4.0) * t3610 * t8082 * t19203 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t27732 - F::cast_from(0.27415567780803773942e-2_f64) * t103744 - t11907 * t29709 + F::cast_from(0.36554090374405031923e-2_f64) * t95033 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t18525 * t2148 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6140 * t7381 - F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t27496 * t103515 * t1216 + F::cast_from(0.3289868133696452873e-1_f64) * t24812 * t27489 * t27490 * t18946 + F::cast_from(0.43864908449286038306e-1_f64) * t27507 * t27540 - F::cast_from(0.87729816898572076612e-1_f64) * t94858 * t27492;
    t103766
}
