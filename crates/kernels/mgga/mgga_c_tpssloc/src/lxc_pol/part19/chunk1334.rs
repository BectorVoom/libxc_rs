//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1334/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334<F: Float>(t11673: F, t11697: F, t3577: F, t11678: F, t11679: F, t11687: F, t11877: F, t3576: F, t11665: F, t11668: F, t11674: F, t11692: F, t11741: F, t11774: F, t1227: F, t15453: F, t3243: F, t3248: F, t3490: F, t3494: F, t3516: F, t3578: F, t3580: F, t42468: F, t44953: F, t44965: F, t44968: F, t44972: F, t44976: F, t44982: F, t44985: F, t4582: F) -> (F,) {
    let t44988 = t3577 * t11697 * t11673;
    let t44991 = t11678 * t11697 * t11679;
    let t44994 = t3577 * t11697 * t11687;
    let t44996 = t11877 * t3576;
    let t44999 = -t11665 * t11674 / 384.0 - 5.0 / 2304.0 * t11692 * t11668 * t3516 * t3243 + t44953 / 1728.0 + 5.0 / 2304.0 * t3577 * t11668 * t3494 * t3243 + 5.0 / 1152.0 * t3490 * t11774 - 5.0 / 864.0 * t1227 * t4582 * t15453 * t42468 + t44965 * t11741 / 768.0 + t44968 / 1728.0 + t44972 / 3456.0 + t44976 / 1728.0 + t11692 * t3578 * t3516 * t3248 / 384.0 - t44982 / 288.0 - t44985 / 576.0 - t44988 / 576.0 - t44991 / 288.0 - t44994 / 288.0 - t44996 * t3580 / 384.0;
    (t44999,)
}
