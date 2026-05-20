//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk877;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta228<F: Float>(t12289: F, t242: F, t1336: F, t3789: F, t5234: F, t3798: F, t3804: F, t820: F, t1824: F, t3792: F, t12345: F, t1831: F, t3865: F, t12189: F, t1811: F, t1815: F, t3862: F, t3802: F, t3788: F, t836: F, t1834: F, t3787: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16233, t16285, t16288, t16305, t16311, t16317) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk877::<F>(t12289, t242, t1336, t3789, t5234, t3798, t3804, t820, t1824, t3792, t12345, t1831);
        let (t16336, t16341, t16350, t16394, t16398, t16428) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk878::<F>(t3865, t5234, t12189, t1811, t1815, t3862, t3802, t3788, t836, t1336, t1834, t3787);
    (t16233, t16285, t16288, t16305, t16311, t16317, t16336, t16341, t16350, t16394, t16398, t16428)
}
