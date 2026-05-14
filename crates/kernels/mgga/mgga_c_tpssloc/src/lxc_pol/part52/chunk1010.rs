//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1010/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1010<F: Float>(t33: F, t7973: F, t2240: F, t12571: F, t7245: F, t1419: F, t55: F, t22510: F, t24498: F, t3961: F, t3966: F, t607: F, t7251: F, t67: F, t1864: F, t1860: F, t2110: F, t24520: F, t24526: F, t26055: F, t26063: F, t26067: F, t26090: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F, t7432: F, t7435: F, t7975: F, t7978: F) -> (F, F, F) {
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    let t27356 = t1419 * t55;
    let t27363 = 20.0 / 9.0 * t27356 * t607 + 5.0 / 18.0 * t24498 * t3961 - 5.0 / 6.0 * t7251 * t3966 - t22510;
    let t27364 = t27363 * t67;
    let t27365 = t27364 * t1864;
    let t27368 = t7435 * t7256 / 3.0 + t7435 * t7259 / 3.0 + 5.0 / 6.0 * t27332 * t6492 + t6495 * t7975 / 3.0 + 5.0 / 6.0 * t7246 * t26090 + t6495 * t7978 / 3.0 + 5.0 / 6.0 * t27341 * t6492 + t26055 * t2110 / 3.0 + 5.0 / 6.0 * t24520 * t7432 + 5.0 / 6.0 * t24526 * t7432 + 5.0 / 6.0 * t7246 * t26063 + 5.0 / 6.0 * t7246 * t26067 - t6486 * t7975 / 6.0 - t1860 * t27365 / 6.0;
    (t27331, t27363, t27368)
}
