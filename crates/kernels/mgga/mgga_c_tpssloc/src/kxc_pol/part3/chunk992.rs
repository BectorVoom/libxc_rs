//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 992/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk992<F: Float>(t1569: F, t2880: F, t2862: F, t4437: F, t2888: F, t4433: F, t931: F, t10813: F, t1568: F, t4472: F, t950: F, t1581: F, t2924: F, t2906: F, t4475: F, t2932: F, t4471: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14429 = t1569 * t2880;
    let t14432 = t4437 * t2862;
    let t14435 = t4433 * t2888;
    let t14436 = t14435 * t931;
    let t14439 = t4437 * t2880;
    let t14442 = t1568 * t10813;
    let t14443 = t14442 * t2862;
    let t14450 = t4472 * t950;
    let t14453 = t1581 * t2924;
    let t14456 = t4475 * t2906;
    let t14459 = t4471 * t2932;
    (t14429, t14432, t14436, t14439, t14443, t14450, t14453, t14456, t14459)
}
