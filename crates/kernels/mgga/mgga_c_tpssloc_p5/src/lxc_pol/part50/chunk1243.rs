//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1243/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1243<F: Float>(t120130: F, t26103: F, t7467: F, t26135: F, t6517: F, t33211: F, t6534: F, t120121: F, t120123: F, t120125: F, t120127: F, t120129: F, t31237: F, t31239: F, t33152: F, t33154: F, t8446: F) -> F {
    let t120131 = F::cast_from(2.0_f64) * t120130;
    let t120132 = t26103 * t7467;
    let t120134 = t6517 * t26135;
    let t120137 = F::cast_from(4.0_f64) * t33211 * t6534;
    let t120138 = t8446 + t33152 + t33154 + t31237 + t31239 + t120121 + t120123 + t120125 + t120127 + t120129 + t120131 + F::cast_from(4.0_f64) * t120132 + F::cast_from(4.0_f64) * t120134 + t120137;
    t120138
}
