//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1662;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1663;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1664;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta457<F: Float>(t25: F, t265: F, t394: F, t24379: F, t2064: F, t2250: F, t24355: F, t40: F, t607: F, t7131: F, t1081: F, t1877: F, t2057: F, t23781: F, t23789: F, t23792: F, t23796: F, t23807: F, t23810: F, t23813: F, t24191: F, t24335: F, t24339: F, t24344: F, t2522: F, t28: F, t3231: F, t4314: F, t6841: F, t6848: F, t7110: F, t7114: F, dens_threshold: F, rho0: F, zeta_threshold: F, t504: F, t2071: F, t52: F, t7150: F, t2094: F, t3701: F, rho1: F, t15904: F, t2075: F, t2363: F, t113: F, t12823: F, t1983: F, t2040: F, t2096: F, t22574: F, t22607: F, t2312: F, t2314: F, t2320: F, t23958: F, t24008: F, t24026: F, t24028: F, t24167: F, t24169: F, t24176: F, t4034: F, t510: F, t574: F, t650: F, t652: F, t6876: F, t7050: F, t7057: F, t7156: F, t7171: F, t7218: F, t7220: F) -> (F, F, F, F, F, F, F) {
        let (t24380, t24387, t24419) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1662::<F>(t25, t265, t394, t24379, t2064, t2250, t24355, t40, t607, t7131, t1081, t1877, t2057, t23781, t23789, t23792, t23796, t23807, t23810, t23813, t24191, t24335, t24339, t24344, t2522, t28, t3231, t4314, t6841, t6848, t7110, t7114, dens_threshold, rho0, zeta_threshold);
        let (t24420, t24428, t24432) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1663::<F>(t28, t265, t504, t24379, t2071, t2250, t24419, t52, t607, t7150, t24387, t2094, t3701, dens_threshold, rho1, zeta_threshold);
        let (t24433, t24442, t24446) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1664::<F>(t15904, t24432, t2075, t2363, t113, t12823, t1983, t2040, t2096, t22574, t22607, t2312, t2314, t2320, t23958, t24008, t24026, t24028, t24167, t24169, t24176, t24428, t4034, t510, t574, t650, t652, t6876, t7050, t7057, t7156, t7171, t7218, t7220);
    (t24380, t24420, t24428, t24432, t24433, t24442, t24446)
}
