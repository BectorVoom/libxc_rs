//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta565 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1980;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1981;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1982;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta565<F: Float>(t1393: F, t1459: F, t1849: F, t24932: F, t26166: F, t26170: F, t26178: F, t26181: F, t26183: F, t26505: F, t27879: F, t27888: F, t27903: F, t4037: F, t4073: F, t4077: F, t574: F, t652: F, t7266: F, t7412: F, t8107: F, t27860: F, t27867: F, t27878: F, t3: F, t112: F, t8110: F, t1458: F, t24969: F, t24972: F, t26533: F, t26535: F, t26537: F, t26539: F, t26541: F, t26544: F, t26547: F, t26549: F, t26552: F, t26554: F, t4072: F, t5376: F, t577: F, t671: F, t7423: F, t3701: F, t6995: F, t7752: F, t1390: F, t22811: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t27905 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1980::<F>(t1393, t1459, t1849, t24932, t26166, t26170, t26178, t26181, t26183, t26505, t27879, t27888, t27903, t4037, t4073, t4077, t574, t652, t7266, t7412, t8107);
        let (t27907, t27908, t27921, t27930) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1981::<F>(t27860, t27867, t27878, t27905, t3, t112, t8110, t1458, t24969, t24972, t26533, t26535, t26537, t26539, t26541, t26544, t26547, t26549, t26552, t26554, t4072, t5376, t577, t671, t7423);
        let (t31035, t33136, t34475, t39041, t39049, t39054, t39063, t40590) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1982::<F>(t3701, t6995, t7752, t1390, t22811, t2233, t2239, t601, t9238, t85, t24, t12019, t566);
    (t27907, t27908, t27921, t27930, t31035, t33136, t34475, t39041, t39049, t39054, t39063, t40590)
}
