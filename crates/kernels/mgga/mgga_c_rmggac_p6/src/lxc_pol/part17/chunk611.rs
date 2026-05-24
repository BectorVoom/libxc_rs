//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 611/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk611<F: Float>(t1616: F, t1986: F, t675: F, t2191: F, t2310: F, t1654: F, t446: F, t597: F, t201: F, t1979: F, t1982: F, t1451: F, t194: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8592 = t1986 * t1616;
    let t8593 = t675 * t8592;
    let t8595 = t2191 * t2310;
    let t8597 = t1986 * t1654;
    let t8598 = t675 * t8597;
    let t8601 = t446 * t597;
    let t8602 = t8601 * t201;
    let t8604 = t8602 * t1979 * t1982;
    let t8607 = t194 * t1451;
    (t8592, t8593, t8595, t8597, t8598, t8601, t8602, t8604, t8607)
}
