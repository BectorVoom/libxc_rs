//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1196/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1196<F: Float>(t1268: F, t21011: F, t1338: F, t3490: F, t1321: F, t3537: F, t4674: F, t623: F, t13546: F, t93: F, t1976: F, t4573: F, t4570: F, t615: F, t77: F, t10289: F, t1290: F) -> (F, F, F, F, F, F, F, F) {
    let t68989 = t21011 * t1268;
    let t69023 = t3490 * t1338;
    let t69026 = t1321 * t3537;
    let t69069 = t623 * t4674;
    let t69072 = t93 * t13546;
    let t69087 = t1976 * t4573;
    let t69097 = t77 * t615 * t4570;
    let t69108 = t10289 * t1290;
    (t68989, t69023, t69026, t69069, t69072, t69087, t69097, t69108)
}
