//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1958/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1958<F: Float>(t45844: F, t7025: F, t12571: F, t23966: F, t6492: F, t7432: F, t84195: F, t1860: F, t2031: F, t2032: F, t22527: F, t22546: F, t23975: F, t26063: F, t26067: F, t26911: F, t26945: F, t6486: F, t7026: F, t84209: F, t90202: F, t90227: F, t90232: F, t90257: F) -> F {
    let t91954 = t45844 * t7025;
    let t91957 = t12571 * t23966;
    let t91959 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t91957 * t6492;
    let t91961 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t84195 * t7432;
    let t91966 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6486 * t26945 + t1860 * t2031 * t90257 / F::cast_from(3.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t84209 * t7432 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23975 * t26063 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23975 * t26067 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t90227 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7026 * t90232 + F::cast_from(10.0_f64) * t91954 * t22546 + t91959 + t91961 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t90202 * t2032 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26911 * t22527;
    t91966
}
