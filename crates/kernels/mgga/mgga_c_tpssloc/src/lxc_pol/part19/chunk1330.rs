//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1330/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1330<F: Float>(t11738: F, t11739: F, t248: F, t3570: F, t10471: F, t44690: F, t11727: F, t44722: F, t44833: F, t44834: F, t478: F, t11719: F, t11722: F, t3507: F, t486: F, t11655: F, t11731: F, t1174: F, t11825: F, t1214: F, t1227: F, t15615: F, t15654: F, t3490: F, t3494: F, t3555: F, t3587: F, t39097: F, t39103: F, t42468: F, t43764: F, t44699: F, t44725: F, t44803: F, t44805: F, t44811: F, t44817: F, t44828: F, t44836: F, t44847: F, t4582: F, t475: F, t974: F) -> (F, F, F) {
    let t44851 = t11738 * t248 * t3570 * t11739;
    let t44857 = t44690 * t10471;
    let t44858 = t44857 * t11727;
    let t44863 = t44833 * t44722 * t478 * t44834;
    let t44871 = t11719 * t248 * t3570 * t11722;
    let t44873 = t486 * t3507;
    let t44878 = -7.0 / 486.0 * t44803 + 35.0 / 972.0 * t1174 * t974 * t44805 * t39097 + t44811 / 216.0 + 5.0 / 384.0 * t1227 * t4582 * t15654 * t42468 - 7.0 / 54.0 * t1174 * t974 * t44817 * t39097 + 5.0 / 576.0 * t3490 * t11655 + 5.0 / 2304.0 * t11825 * t3587 + 55.0 / 15552.0 * t1227 * t248 * t44828 * t43764 - t44836 * t248 * t1214 * t44699 * t475 / 3072.0 - t1174 * t974 * t3555 * t39103 / 48.0 - t44847 / 162.0 + t44851 / 1152.0 - t1227 * t4582 * t15615 * t42468 / 128.0 - t44858 * t11731 / 128.0 + t44863 * t248 * t1214 * t44699 * t44725 / 128.0 + t44871 / 192.0 + t11738 * t4582 * t44873 * t3494 / 512.0;
    (t44857, t44873, t44878)
}
