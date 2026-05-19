//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 618/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk618<F: Float>(t15527: F, t15281: F, t2211: F, t2367: F, t1356: F, t14451: F, t570: F, t5148: F, t551: F, t5259: F, t558: F, t4669: F) -> (F, F, F, F, F, F, F, F) {
    let t15528 = F::cast_from(0.14967802127329760705e-1_f64) * t15527;
    let t15529 = F::cast_from(0.14967802127329760705e-1_f64) * t15281;
    let t15530 = t2211 * t2367;
    let t15531 = t1356 * t15530;
    let t15532 = F::cast_from(0.39914139006212695214e-1_f64) * t15531;
    let t15533 = t14451 * t570;
    let t15534 = t5148 * t15533;
    let t15535 = F::cast_from(0.2993560425465952141e-1_f64) * t15534;
    let t15536 = t14451 * t551;
    let t15537 = t5259 * t15536;
    let t15538 = F::cast_from(0.2993560425465952141e-1_f64) * t15537;
    let t15539 = t14451 * t558;
    let t15540 = t4669 * t15539;
    (t15528, t15529, t15530, t15532, t15535, t15536, t15538, t15540)
}
