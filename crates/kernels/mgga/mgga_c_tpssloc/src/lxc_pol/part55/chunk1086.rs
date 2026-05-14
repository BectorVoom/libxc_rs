//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1086/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1086<F: Float>(t26135: F, t8601: F, t12725: F, t8326: F, t33211: F, t6534: F, t31537: F, t7467: F, t120112: F, t114418: F, t1983: F, t7687: F, t15868: F, t8489: F, t22751: F, t32731: F) -> (F, F, F, F, F, F, F, F) {
    let t120129 = 4.0 * t8601 * t26135;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0 * t120130;
    let t120137 = 4.0 * t33211 * t6534;
    let t120140 = 4.0 * t31537 * t7467;
    let t120165 = 2.0 * t120112;
    let t120171 = 3.0 * t1983 * t114418 * t7687;
    let t120176 = t1983 * t8489 * t15868;
    let t120179 = t22751 * t32731;
    (t120129, t120131, t120137, t120140, t120165, t120171, t120176, t120179)
}
