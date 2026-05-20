//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1268;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta431<F: Float>(t15438: F, t19095: F, t19083: F, t4993: F, t18392: F, t5024: F, t1226: F, t22115: F, t1227: F, t21776: F, t248: F, t3521: F, t5005: F, t15737: F, t18356: F, t19040: F, t11738: F, t22299: F, t3570: F, t11728: F, t22312: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t72248, t72251, t72253, t72255, t72273) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267::<F>(t15438, t19095, t19083, t4993, t18392, t5024, t1226, t22115, t1227, t21776, t248, t3521);
        let (t72285, t72287, t72289, t72293, t72297) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1268::<F>(t18392, t5005, t15737, t18356, t19040, t5024, t11738, t22299, t248, t3570, t11728, t22312);
    (t72248, t72251, t72253, t72255, t72273, t72285, t72287, t72289, t72293, t72297)
}
