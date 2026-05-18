//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1022/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1022<F: Float>(t381: F, t7577: F, t6691: F, t1052: F, t14545: F, t14552: F, t1956: F, t23327: F, t25400: F, t25403: F, t25407: F, t25410: F, t25413: F, t25416: F, t25420: F, t25425: F, t25429: F, t25432: F, t25436: F, t4660: F, t4694: F, t6687: F, t6771: F, t6776: F) -> F {
    let t25442 = t7577 * t381;
    let t25443 = t25442 * t6691;
    let t25446 = -t6771 * t4694 - F::new(0.82246703342411321825e-2) * t6687 * t25400 - F::new(0.82246703342411321825e-2) * t6687 * t25403 - F::new(0.82246703342411321825e-2) * t6687 * t25407 - F::new(0.82246703342411321825e-2) * t6687 * t25410 - F::new(0.82246703342411321825e-2) * t6687 * t25413 - F::new(0.27415567780803773942e-2) * t23327 * t25416 + F::new(2.0) * t1052 * t25420 - F::new(0.54831135561607547884e-2) * t23327 * t25425 + F::new(0.36554090374405031923e-2) * t25429 * t25432 - t14552 * t1956 + F::new(0.27415567780803773942e-2) * t6687 * t25436 - t14545 * t1956 + F::new(2.0) * t4660 * t6776 - F::new(0.27415567780803773942e-2) * t23327 * t25443;
    t25446
}
