//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1081;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta298<F: Float>(t3120: F, t3131: F, t10482: F, t3040: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F, t1022: F, t883: F, t607: F, t360: F, t10632: F, t2906: F, t11066: F, t6739: F, t135: F, t457: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t13980, t13985, t14164, t14172, t14187, t14212) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1081::<F>(t3120, t3131, t10482, t3040, t1043, t2770, t10277, t3061, t10216, t10969, t1022, t883);
        let (t14213, t14220, t14228, t14259, t14590, t14630, t15281) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1082::<F>(t14212, t607, t360, t883, t1022, t10632, t2906, t11066, t3040, t6739, t135, t457);
    (t13980, t13985, t14164, t14172, t14187, t14213, t14220, t14228, t14259, t14590, t14630, t15281)
}
