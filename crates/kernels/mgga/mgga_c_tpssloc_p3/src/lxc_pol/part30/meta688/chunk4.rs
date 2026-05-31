//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2189/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2189<F: Float>(t2020: F, t97804: F, t15868: F, t1983: F, t7753: F, t22574: F, t74032: F, t8643: F, t24999: F, t4073: F, t5361: F, t7681: F, t96842: F, t96844: F, t96846: F, t97777: F, t97779: F, t97783: F, t97785: F, t97788: F, t97792: F, t97794: F, t97796: F, t97798: F, t97800: F, t97802: F) -> F {
    let t97805 = t97804 * t2020;
    let t97808 = F::cast_from(2.0_f64) * t1983 * t7753 * t15868;
    let t97811 = F::cast_from(3.0_f64) * t22574 * t8643 * t74032;
    let t97814 = -F::cast_from(4.0_f64) * t24999 * t4073 + F::cast_from(2.0_f64) * t5361 * t7681 - t96842 - t96844 - t96846 + t97777 - t97779 - t97783 - t97785 - t97788 - t97792 + t97794 - t97796 - t97798 - t97800 - t97802 + t97805 - t97808 - t97811;
    t97814
}
