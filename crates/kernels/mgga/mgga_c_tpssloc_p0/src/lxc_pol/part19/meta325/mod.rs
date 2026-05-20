//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta325<F: Float>(t12283: F, t12404: F, t12413: F, t12267: F, t3802: F, t3734: F, t3792: F, t12279: F, t16398: F, t12409: F, t3719: F, t12167: F, t1314: F, t9569: F, t1329: F, t12189: F, t3770: F, t12303: F, t12368: F, t12371: F, t12419: F, t1352: F, t16224: F, t16401: F, t3803: F, t3805: F, t3806: F, t3809: F, t5246: F, t5248: F, t12313: F, t3726: F, t2559: F, t3732: F, t3766: F, t12214: F, t782: F, t12320: F, t154: F, t1995: F, t205: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154::<F>(t12283, t12404, t12413, t12267, t3802, t3734, t3792, t12279, t16398, t12409, t3719, t12167);
        let (t40005, t40010) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155::<F>(t1314, t9569, t1329, t12189, t3770, t12279, t12303, t12368, t12371, t12419, t1352, t16224, t16401, t3803, t3805, t3806, t3809, t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000, t5246, t5248);
        let (t40012, t40018, t40019, t40021, t40022, t40025, t40026) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1156::<F>(t12313, t3726, t2559, t3732, t3766, t12214, t782, t12320, t154, t1995, t205, t3734);
    (t40005, t40010, t40012, t40018, t40019, t40021, t40022, t40025, t40026)
}
