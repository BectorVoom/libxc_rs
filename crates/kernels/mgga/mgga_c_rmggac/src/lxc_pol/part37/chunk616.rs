//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 616/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk616<F: Float>(t1356: F, t15530: F, t14451: F, t570: F, t5148: F, t551: F, t5259: F, t558: F, t4669: F, t14444: F, t8940: F, t15094: F) -> (F, F, F, F, F, F, F) {
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
    let t15541 = F::cast_from(0.44903406381989282115e-1_f64) * t15540;
    let t15542 = t14444 * t570;
    let t15544 = F::cast_from(0.11974241701863808564e0_f64) * t8940 * t15542;
    let t15545 = F::cast_from(0.14967802127329760705e-1_f64) * t15094;
    (t15532, t15535, t15536, t15538, t15541, t15544, t15545)
}
