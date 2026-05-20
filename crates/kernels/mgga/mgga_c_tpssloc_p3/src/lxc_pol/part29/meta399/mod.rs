//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1636;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1637;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1638;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta399<F: Float>(t3536: F, t4997: F, t248: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F, t3503: F, t3500: F, t1210: F, t11665: F, t1218: F, t1232: F, t15470: F, t15474: F, t15478: F, t15484: F, t15488: F, t3511: F, t3518: F, t3527: F, t3577: F, t3587: F, t4954: F, t5005: F, t5024: F, t11539: F, t4724: F, t1174: F, t15239: F, t475: F, t1214: F, t3494: F, t4977: F, t4582: F, t3516: F, t12652: F, t4987: F, t12648: F, t13969: F, t4983: F, t3515: F, t486: F, t5011: F, t4978: F, t11709: F, t11738: F, t11814: F, t11825: F, t1227: F, t1737: F, t1748: F, t3490: F, t3506: F, t3531: F, t4980: F, t4989: F, t5014: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15490, t15492, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1636::<F>(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let t15512 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1637::<F>(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
        let (t15524, t15527, t15531, t15535, t15540) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1638::<F>(t11539, t4724, t1174, t15239, t475, t1214, t248, t3494, t4977, t4582, t3516, t12652, t4987);
        let (t15541, t15545, t15548, t15553, t15555, t15558) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1639::<F>(t15540, t4582, t12648, t4987, t13969, t4983, t3515, t486, t5011, t4978, t11709, t11738, t11814, t11825, t1213, t1227, t15524, t15527, t15531, t15535, t1737, t1748, t3490, t3506, t3531, t3536, t4980, t4989, t5014, t5024);
    (t15492, t15501, t15512, t15527, t15531, t15535, t15541, t15545, t15548, t15553, t15555, t15558)
}
