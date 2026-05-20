//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1912;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1913;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1914;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta456<F: Float>(t1706: F, t3428: F, t1184: F, t460: F, t4928: F, t4934: F, t1714: F, t3469: F, t1178: F, t12606: F, t1177: F, t135: F, t457: F, t4936: F, t1174: F, t3431: F, t4912: F, t1090: F, t7319: F, t4919: F, t11531: F, t11534: F, t11537: F, t11541: F, t11591: F, t3447: F, t11583: F, t3961: F, t3449: F, t11529: F, t1709: F, t3475: F, t3432: F, t4889: F, t3450: F, t3966: F, t14749: F, t4908: F, t3448: F, t3451: F, t11579: F, t11584: F, t3443: F, t3457: F, t3461: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15265, t15268, t15269, t15273, t15274, t15277, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1912::<F>(t1706, t3428, t1184, t460, t4928, t4934, t1714, t3469, t1178, t12606, t1177, t135, t457);
        let (t15288, t15292) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1913::<F>(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let (t15293, t15294, t15300, t15303, t15304, t15307) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1914::<F>(t11583, t3961, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889);
        let (t15313, t15320, t15330) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1915::<F>(t3450, t3966, t3449, t14749, t4908, t3448, t4928, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t3443, t3447, t3457, t3461, t4889);
    (t15268, t15273, t15277, t15281, t15288, t15292, t15293, t15303, t15313, t15320, t15330)
}
