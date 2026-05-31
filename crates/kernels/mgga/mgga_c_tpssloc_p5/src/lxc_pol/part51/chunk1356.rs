//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1356/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1356<F: Float>(t119824: F, t119826: F, t119830: F, t120664: F, t120899: F, t120900: F, t120907: F, t120910: F, t120912: F, t22461: F, t24999: F, t26103: F, t26559: F, t27180: F, t27219: F, t6517: F, t7061: F, t7806: F) -> F {
    let t120921 = F::cast_from(2.0_f64) * t120664 * t26559 - F::cast_from(2.0_f64) * t22461 * t7806 - F::cast_from(2.0_f64) * t24999 * t7061 - F::cast_from(2.0_f64) * t26103 * t7806 - F::cast_from(2.0_f64) * t27180 * t6517 - F::cast_from(2.0_f64) * t27219 * t6517 - t119824 - t119826 - t119830 - t120899 - t120900 + t120907 - t120910 - t120912;
    t120921
}
