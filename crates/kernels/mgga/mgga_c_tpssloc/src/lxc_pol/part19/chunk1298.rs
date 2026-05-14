//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1298/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1298<F: Float>(t43924: F, t43953: F, t43956: F, t43958: F, t43961: F, t43963: F, t43966: F, t43973: F, t43975: F, t43979: F, t43982: F, t43987: F, t43989: F, t3266: F, t3307: F, t3313: F) -> (F, F) {
    let t43990 = t43924 + t43953 + t43956 + t43958 + t43961 + t43963 + t43966 - t43973 - t43975 + t43979 + t43982 - t43987 - t43989;
    let t43994 = 36.0 * t3313 * t3266 * t3307;
    (t43990, t43994)
}
