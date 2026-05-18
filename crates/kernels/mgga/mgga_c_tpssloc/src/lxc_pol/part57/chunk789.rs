//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 789/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk789<F: Float>(t25810: F, t7553: F, t5685: F, t6690: F, t6689: F, t1922: F, t5844: F, t1052: F, t1635: F, t1920: F, t25450: F, t25736: F, t25755: F, t25778: F, t28470: F, t28475: F, t28481: F, t28485: F, t28488: F, t28492: F, t28496: F, t28500: F, t28505: F, t388: F, t4660: F, t6687: F, t7600: F, t7625: F) -> F {
    let t28510 = t25810 * t7553;
    let t28515 = t6690 * t5685;
    let t28516 = t6689 * t28515;
    let t28519 = t5844 * t1922;
    let t28523 = F::new(0.16449340668482264365e-1) * t6687 * t28470 + F::new(0.82246703342411321825e-2) * t1920 * t28475 + F::new(4.0) * t4660 * t7600 - F::new(0.82246703342411321825e-2) * t6687 * t28481 + F::new(4.0) * t1052 * t28485 + F::new(2.0) * t28488 * t388 + F::new(0.36554090374405031923e-2) * t6687 * t28492 + F::new(0.16449340668482264365e-1) * t6687 * t28496 - F::new(0.54831135561607547884e-2) * t6687 * t28500 - F::new(2.0) * t4660 * t7625 + t28505 * t388 + F::new(0.18277045187202515961e-2) * t25450 - F::new(2.0) * t25778 * t1635 + F::new(0.54831135561607547884e-2) * t6687 * t28510 - F::new(2.0) * t25755 * t1635 + F::new(0.27415567780803773942e-2) * t6687 * t28516 - F::new(0.82246703342411321825e-2) * t6687 * t28519 - F::new(0.54831135561607547884e-2) * t25736;
    t28523
}
