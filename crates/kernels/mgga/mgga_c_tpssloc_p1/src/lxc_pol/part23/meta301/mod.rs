//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1030;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta301<F: Float>(t1539: F, t5878: F, t3071: F, t10930: F, t20234: F, t974: F, t20217: F, t998: F, t10942: F, t21510: F, t4583: F, t4582: F, t1041: F, t10413: F, t14117: F, t14160: F, t14203: F, t1618: F, t17885: F, t17907: F, t18005: F, t18008: F, t18030: F, t973: F) -> (F, F, F, F, F, F, F, F) {
        let (t21531, t21532, t21537, t21538, t21541, t21542, t21545, t21546, t21550, t21551) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1030::<F>(t1539, t5878, t3071, t10930, t20234, t974, t20217, t998, t10942, t21510, t4583, t4582);
        let t21560 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1031::<F>(t1041, t10413, t14117, t14160, t14203, t1618, t17885, t17907, t18005, t18008, t18030, t21532, t21538, t21542, t21546, t21551, t973);
    (t21531, t21532, t21537, t21541, t21545, t21550, t21551, t21560)
}
