//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 974/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk974<F: Float>(t1509: F, t828: F, t2632: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t2642: F, t4166: F, t2628: F, t836: F) -> (F, F, F, F, F, F, F) {
    let t13223 = t1509 * t828;
    let t13228 = t1509 * t2632;
    let t13234 = t1500 * t2693;
    let t13237 = F::new(7.0) / F::new(2304.0) * t4163 * t838;
    let t13242 = t120 * t4233;
    let t13251 = t4166 * t2642;
    let t13257 = t2628 * t836;
    (t13223, t13228, t13234, t13237, t13242, t13251, t13257)
}
