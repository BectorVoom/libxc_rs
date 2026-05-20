//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta360<F: Float>(t10277: F, t976: F, t11046: F, t42387: F, t10457: F, t820: F, t10969: F, t121: F, t10213: F, t41687: F, t1043: F, t204: F, t340: F, t625: F, t221: F, t339: F, t344: F, t343: F, t42308: F, t974: F, t41666: F, t2978: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42444, t42483, t42488, t42592, t42624, t42749) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158::<F>(t10277, t976, t11046, t42387, t10457, t820, t10969, t121, t10213, t41687, t1043, t204);
        let (t42813, t42817, t42841, t42861, t42862, t42875) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1159::<F>(t340, t625, t221, t339, t344, t10277, t343, t42308, t974, t41666, t2978, t698);
    (t42444, t42483, t42488, t42592, t42624, t42749, t42813, t42817, t42841, t42861, t42862, t42875)
}
