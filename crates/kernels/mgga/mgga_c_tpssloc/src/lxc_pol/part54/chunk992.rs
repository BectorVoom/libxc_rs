//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 992/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk992<F: Float>(t26895: F, t26982: F, t27183: F, t27238: F, t3: F, t112: F, t7945: F, t1458: F, t7056: F, t2039: F, t4072: F, t671: F, t7801: F, t12524: F, t1401: F, t16521: F, t16524: F, t20173: F, t24462: F, t24465: F, t27170: F, t3938: F, t3941: F, t5371: F, t5376: F, t577: F, t7230: F, t7235: F, t7956: F) -> (F, F, F, F, F, F, F) {
    let t27240 = t26895 + t26982 + t27183 + t27238;
    let t27241 = t3 * t27240;
    let t27254 = t7945 * t112;
    let t27273 = t7056 * t1458;
    let t27276 = t2039 * t4072;
    let t27281 = t7801 * t671;
    let t27286 = 0.45e1 * t27240 * t577 + 0.135e2 * t27254 * t671 + 0.135e2 * t24462 * t1458 + 27.0 * t24465 * t5376 + 0.135e2 * t7230 * t4072 + 0.135e2 * t16521 * t2039 + 27.0 * t16524 * t7235 + 0.135e2 * t5371 * t7056 + 27.0 * t12524 * t7956 + 27.0 * t20173 * t7956 + 27.0 * t3941 * t27273 + 27.0 * t3941 * t27276 + 0.135e2 * t3938 * t7801 + 27.0 * t3941 * t27281 + 0.135e2 * t1401 * t27170;
    (t27240, t27241, t27254, t27273, t27276, t27281, t27286)
}
