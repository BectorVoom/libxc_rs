//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2002/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2002<F: Float>(t27982: F, t7032: F, t26959: F, t7435: F, t7432: F, t91957: F, t27966: F, t23963: F, t23975: F, t26055: F, t26090: F, t26911: F, t27961: F, t27972: F, t27976: F, t7026: F, t7782: F, t84190: F, t96403: F, t96502: F, t96506: F) -> F {
    let t102215 = t27982 * t7032;
    let t102217 = t7435 * t26959;
    let t102219 = t91957 * t7432;
    let t102221 = t27966 * t7032;
    let t102223 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26911 * t26090 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26055 * t7782 + F::cast_from(10.0_f64) * t84190 * t27961 + F::cast_from(10.0_f64) * t23963 * t96403 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t23975 * t27972 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7026 * t96502 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t7026 * t96506 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23975 * t27976 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t102215 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t102217 + F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t102219 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t102221;
    t102223
}
