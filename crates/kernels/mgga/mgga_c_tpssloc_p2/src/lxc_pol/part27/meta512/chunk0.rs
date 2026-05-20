//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1916/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1916<F: Float>(t2770: F, t387: F, t3961: F, t23329: F, t23581: F, t7553: F, t381: F, t7577: F, t6691: F, t1052: F, t14545: F, t14552: F, t1956: F, t23327: F, t25400: F, t25403: F, t25407: F, t25410: F, t25413: F, t25416: F, t25420: F, t25425: F, t25429: F, t4660: F, t4694: F, t6687: F, t6771: F, t6776: F) -> (F, F, F, F, F, F, F) {
    let t25430 = t387 * t2770;
    let t25431 = t25430 * t3961;
    let t25432 = t23329 * t25431;
    let t25436 = t23581 * t7553;
    let t25442 = t7577 * t381;
    let t25443 = t25442 * t6691;
    let t25446 = -t6771 * t4694 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25400 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25403 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25407 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25410 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25413 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25416 + F::new(2.0) * t1052 * t25420 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25425 + F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t25432 - t14552 * t1956 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25436 - t14545 * t1956 + F::new(2.0) * t4660 * t6776 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t25443;
    (t25430, t25431, t25432, t25436, t25442, t25443, t25446)
}
