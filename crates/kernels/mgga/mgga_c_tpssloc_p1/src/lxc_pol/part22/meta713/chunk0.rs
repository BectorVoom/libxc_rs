//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2312/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2312<F: Float>(t40804: F, t40806: F, t40790: F, t40793: F, t40797: F, t40799: F, t40801: F, t40803: F, t46311: F, t67214: F, t67215: F, t12939: F, t16716: F, t3966: F) -> (F, F, F, F) {
    let t67216 = F::cast_from(0.32530743900905219526e-1_f64) * t40804;
    let t67217 = F::cast_from(0.48159733137676571078e0_f64) * t40806;
    let t67218 = -t46311 + t67214 + t40790 + t40793 + t67215 + t40797 + t40799 + t40801 - t40803 - t67216 + t67217;
    let t67226 = F::cast_from(72.0_f64) * t12939 * t16716 * t3966;
    (t67216, t67217, t67218, t67226)
}
