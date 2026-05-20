//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2234/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2234<F: Float>(t23384: F, t25718: F, t23665: F, t25541: F, t25545: F, t25503: F, t10216: F, t381: F, t1049: F, t14165: F, t14605: F, t23327: F, t23692: F, t23697: F, t25429: F, t25470: F, t25497: F, t25500: F, t25510: F, t25536: F, t2775: F, t3180: F, t3961: F, t6680: F, t6797: F, t6799: F, t6800: F, t7610: F, t82596: F, t88022: F) -> F {
    let t89151 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25718;
    let t89156 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25541;
    let t89158 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25545;
    let t89175 = F::cast_from(0.54831135561607547884e-2_f64) * t23665 * t25503;
    let t89176 = t381 * t10216;
    let t89181 = F::new(2.0) * t3180 * t25500 - F::cast_from(0.43864908449286038306e-1_f64) * t6680 * t25536 + F::new(2.0) * t3180 * t25497 + t89151 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t82596 * t7610 + t89156 + t89158 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t25510 * t1049 * t2775 * t3961 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25470 * t23692 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25470 * t23697 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t14605 * t6800 + t89175 + F::cast_from(0.8529287754027840782e-2_f64) * t88022 * t25510 * t89176 * t14165;
    t89181
}
