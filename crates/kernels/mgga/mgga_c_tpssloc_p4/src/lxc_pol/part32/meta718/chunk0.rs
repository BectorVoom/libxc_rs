//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2283/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2283<F: Float>(t100674: F, t100716: F, t100763: F, t100803: F, t24987: F, t7754: F, t1983: F, t2019: F, t57806: F, t25971: F, t91655: F, t26161: F, t26162: F, t75210: F) -> (F, F, F, F, F) {
    let t100805 = t100674 + t100716 + t100763 + t100803;
    let t100828 = F::cast_from(2.0_f64) * t24987 * t7754;
    let t100833 = t1983 * t2019 * t57806;
    let t100835 = F::cast_from(6.0_f64) * t91655 * t25971;
    let t100838 = F::cast_from(2.0_f64) * t26161 * t26162 * t75210;
    (t100805, t100828, t100833, t100835, t100838)
}
