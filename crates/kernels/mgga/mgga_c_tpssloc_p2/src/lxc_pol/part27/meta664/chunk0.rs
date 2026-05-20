//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2330/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2330<F: Float>(t22844: F, t6976: F, t22828: F, t7708: F, t16391: F, t26309: F, t5259: F, t80820: F, t16265: F, t22833: F, t5293: F, t80816: F) -> (F, F, F, F, F) {
    let t91208 = t22844 * t6976;
    let t91210 = t91208 * t7708 * t22828;
    let t91212 = t26309 * t16391;
    let t91214 = t80820 * t5259;
    let t91215 = F::new(7.0) / F::new(288.0) * t91214;
    let t91216 = t22833 * t16265;
    let t91218 = t80816 * t5293;
    (t91210, t91212, t91215, t91216, t91218)
}
