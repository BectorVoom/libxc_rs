//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2063;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta529<F: Float>(t12379: F, t3799: F, t12384: F, t3777: F, t3795: F, t12282: F, t3809: F, t12328: F, t1333: F, t1336: F, t2690: F, t3788: F, t67: F, t6924: F, t246: F, t12156: F, t550: F, t12012: F, t12371: F, t16398: F, t12283: F, t12426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40128, t40131, t40138, t40139, t40145, t40159) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2063::<F>(t12379, t3799, t12384, t3777, t3795, t12282, t3809, t12328, t1333, t1336, t2690, t3788);
        let (t40160, t40167, t40168, t40169, t40178, t40188, t40190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2064::<F>(t3795, t40159, t67, t6924, t246, t12156, t550, t12012, t12371, t16398, t12283, t12426);
    (t40128, t40131, t40138, t40139, t40145, t40160, t40167, t40168, t40169, t40178, t40188, t40190)
}
