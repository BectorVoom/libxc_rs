//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1926;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta462<F: Float>(t3536: F, t4997: F, t248: F, t3570: F, t5012: F, t1213: F, t3535: F, t5018: F, t1202: F, t5023: F, t1742: F, t3036: F, t3503: F, t3500: F, t1210: F, t11665: F, t1218: F, t1232: F, t15470: F, t15474: F, t15478: F, t15484: F, t15488: F, t3511: F, t3518: F, t3527: F, t3577: F, t3587: F, t4954: F, t5005: F, t5024: F) -> (F, F, F, F, F, F, F, F) {
        let (t15490, t15492, t15494, t15495, t15498, t15501) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1926::<F>(t3536, t4997, t248, t3570, t5012, t1213, t3535, t5018, t1202, t5023, t1742, t3036);
        let (t15502, t15503, t15506, t15507, t15512) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1927::<F>(t15501, t3503, t3500, t1210, t11665, t1218, t1232, t15470, t15474, t15478, t15484, t15488, t15490, t15494, t15495, t15498, t3511, t3518, t3527, t3577, t3587, t4954, t5005, t5024);
    (t15492, t15495, t15498, t15502, t15503, t15506, t15507, t15512)
}
