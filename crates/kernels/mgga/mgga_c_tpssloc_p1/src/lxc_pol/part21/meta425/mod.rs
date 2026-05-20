//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1949;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta425<F: Float>(t1137: F, t15117: F, t1147: F, t4832: F, t1687: F, t3400: F, t1156: F, t14829: F, t3375: F, t1129: F, t11356: F, t1148: F, t1157: F, t14840: F, t14847: F, t14849: F, t14852: F, t1695: F, t3371: F, t3378: F, t3396: F, t3404: F, t4835: F, t4858: F, t1128: F, t4794: F, t1675: F, t3356: F, t1136: F, t4820: F, t1683: F, t3351: F, t3333: F, t4823: F, t1138: F, t11410: F, t11420: F, t14864: F, t14866: F, t14916: F, t14934: F, t14939: F, t3327: F, t3332: F, t3352: F, t3360: F, t4797: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15118, t15121, t15126, t15133, t15136, t15139) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1949::<F>(t1137, t15117, t1147, t4832, t1687, t3400, t1156, t14829, t3375, t1129, t11356, t1148, t1157, t14840, t14847, t14849, t14852, t1695, t3371, t3378, t3396, t3404, t4835, t4858);
        let (t15141, t15146, t15153, t15156, t15159, t15162) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1950::<F>(t1128, t4794, t1675, t3356, t1136, t4820, t1683, t3351, t3333, t4823, t1138, t11410, t11420, t14864, t14866, t14916, t14934, t14939, t3327, t3332, t3352, t3360, t4797);
    (t15118, t15121, t15126, t15133, t15136, t15139, t15141, t15146, t15153, t15156, t15159, t15162)
}
