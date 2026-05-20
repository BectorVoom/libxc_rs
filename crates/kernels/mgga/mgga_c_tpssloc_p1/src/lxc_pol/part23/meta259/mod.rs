//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk920;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta259<F: Float>(t118: F, t6330: F, t794: F, t12202: F, t6347: F, t3739: F, t12211: F, t6353: F, t213: F, t3726: F, t6358: F, t1814: F, t5343: F, t6378: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19767, t19768, t19775, t19776, t19779, t19781, t19791, t19810) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk920::<F>(t118, t6330, t794, t12202, t6347, t3739, t12211, t6353, t213, t3726, t6358, t1814, t5343);
        let t19815 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk921::<F>(t6378, t68);
    (t19767, t19768, t19775, t19776, t19779, t19781, t19791, t19810, t19815)
}
