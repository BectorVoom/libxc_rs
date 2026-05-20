//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta421 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1735;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta421<F: Float>(t1227: F, t18975: F, t4997: F, t5019: F, t4993: F, t5005: F, t1202: F, t6164: F, t5024: F, t11692: F, t11792: F, t11821: F, t15671: F, t15691: F, t15699: F, t15740: F, t18955: F, t18959: F, t18965: F, t18969: F, t18972: F, t3577: F, t488: F, t4950: F) -> (F, F, F, F, F, F) {
        let (t18976, t18978, t18980, t18982, t18987, t18989) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1735::<F>(t1227, t18975, t4997, t5019, t4993, t5005, t1202, t6164, t5024, t11692, t11792, t11821, t15671, t15691, t15699, t15740, t18955, t18959, t18965, t18969, t18972, t3577, t488, t4950);
    (t18976, t18978, t18980, t18982, t18987, t18989)
}
