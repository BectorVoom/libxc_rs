//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 934/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk934<F: Float>(t2775: F, t387: F, t3961: F, t23329: F, t221: F, t4509: F, t1926: F, t2770: F, t23581: F, t7553: F, t381: F, t7577: F, t6691: F, t1052: F, t14545: F, t14552: F, t1956: F, t23327: F, t25400: F, t25403: F, t25407: F, t25410: F, t25413: F, t25416: F, t25420: F, t4660: F, t4694: F, t6687: F, t6771: F, t6776: F) -> (F, F) {
    let t25423 = t387 * t2775;
    let t25424 = t25423 * t3961;
    let t25425 = t23329 * t25424;
    let t25428 = t221 * t4509;
    let t25429 = t1926 * t25428;
    let t25430 = t387 * t2770;
    let t25431 = t25430 * t3961;
    let t25432 = t23329 * t25431;
    let t25436 = t23581 * t7553;
    let t25442 = t7577 * t381;
    let t25443 = t25442 * t6691;
    let t25446 = -t6771 * t4694 - 0.82246703342411321825e-2 * t6687 * t25400 - 0.82246703342411321825e-2 * t6687 * t25403 - 0.82246703342411321825e-2 * t6687 * t25407 - 0.82246703342411321825e-2 * t6687 * t25410 - 0.82246703342411321825e-2 * t6687 * t25413 - 0.27415567780803773942e-2 * t23327 * t25416 + 2.0 * t1052 * t25420 - 0.54831135561607547884e-2 * t23327 * t25425 + 0.36554090374405031923e-2 * t25429 * t25432 - t14552 * t1956 + 0.27415567780803773942e-2 * t6687 * t25436 - t14545 * t1956 + 2.0 * t4660 * t6776 - 0.27415567780803773942e-2 * t23327 * t25443;
    (t25429, t25446)
}
