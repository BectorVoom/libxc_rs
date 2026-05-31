//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 308/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk308<F: Float>(t220: F, t368: F, t975: F, t983: F, t984: F, t985: F, t981: F, t373: F, t976: F, t978: F, t375: F) -> (F, F, F, F) {
    let t990 = t220 * t368 * t975 + t983 * t984 * t985;
    let t991 = t981 * t990;
    let t993 = t373 * t976 - t978 * t991;
    let t995 = F::cast_from(1.0_f64) / t375;
    (t990, t991, t993, t995)
}
