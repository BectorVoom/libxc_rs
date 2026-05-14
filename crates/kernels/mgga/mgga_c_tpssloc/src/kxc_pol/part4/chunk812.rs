//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 812/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk812<F: Float>(t2693: F, t809: F, t597: F, t61: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F, t222: F, t805: F, t9541: F, t2627: F, t852: F, t856: F) -> (F, F, F, F, F, F, F, F) {
    let t10014 = t809 * t2693;
    let t10021 = 1.0 / t61 / t597;
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = 595.0 / 10368.0 * t238 * t10024;
    let t10027 = t9569 * t154;
    let t10029 = 455.0 / 1296.0 * t10027 * t222;
    let t10036 = t9541 * t805;
    let t10054 = t2627 * t852;
    let t10108 = t856 * t856;
    (t10014, t10022, t10026, t10027, t10029, t10036, t10054, t10108)
}
