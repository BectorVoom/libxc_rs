//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2291/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2291<F: Float>(t29624: F, t491: F, t1760: F, t607: F, t27381: F, t8009: F, t103132: F, t1186: F, t1251: F, t17686: F, t2128: F, t24567: F, t24589: F, t24601: F, t24602: F, t27411: F, t27415: F, t27441: F, t27445: F, t27549: F, t27751: F, t27820: F, t29803: F, t4723: F, t4728: F, t4930: F, t5398: F, t7283: F, t7287: F, t8010: F, t85642: F, t85661: F, t94369: F, t94395: F, t94458: F, t94796: F, t95890: F) -> F {
    let t103175 = t29624 * t491;
    let t103179 = t1760 * t607;
    let t103188 = t8009 * t27381;
    let t103213 = F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24601 * t24602 * t5398 * t1251 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27441 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94458 * t27445 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t103175 * t7287 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94369 * t4728 * t103179 - F::cast_from(0.73108180748810063845e-2_f64) * t27549 * t94369 * t4723 * t103179 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t103188 + F::cast_from(0.18277045187202515961e-2_f64) * t85661 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t27751 * t27415 - F::cast_from(0.3289868133696452873e-1_f64) * t2128 * t27820 * t27411 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t24567 * t29803 - F::cast_from(0.8529287754027840782e-2_f64) * t94796 * t24601 * t95890 * t17686 - F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24601 * t85642 * t103132 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t8010;
    t103213
}
