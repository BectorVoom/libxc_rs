//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1243/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1243<F: Float>(t102192: F, t102194: F, t102215: F, t102217: F, t102219: F, t102221: F, t102248: F, t106816: F, t2032: F, t26954: F, t27979: F, t7782: F, t91996: F, t96443: F) -> F {
    let t108743 = -F::cast_from(2.0_f64) * t106816 * t2032 - F::cast_from(2.0_f64) * t27979 * t7782 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t102192 + F::cast_from(40.0_f64) / F::cast_from(3.0_f64) * t102194 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t102215 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t102217 + F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t102219 + F::cast_from(32.0_f64) / F::cast_from(3.0_f64) * t102221 - F::cast_from(80.0_f64) * t102248 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t91996 + F::cast_from(20.0_f64) * t96443 * t26954;
    t108743
}
