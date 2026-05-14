//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1297/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1297<F: Float>(t11277: F, t43969: F, t43970: F, t11634: F, t3411: F, t3307: F, t11275: F, t3265: F, t11400: F, t11628: F, t1164: F, t11285: F, t3395: F, t11282: F, t3377: F, t11403: F, t11424: F) -> (F, F, F, F, F, F, F) {
    let t43973 = 0.62071215503128080361e4 * t43969 * t43970 * t11277;
    let t43975 = 0.20779030926817756511e3 * t3411 * t11634;
    let t43976 = t3307 * t11277;
    let t43979 = 0.3103560775156404018e4 * t11275 * t43976 * t3265;
    let t43982 = 0.46785788981077169656e1 * t1164 * t11628 * t11400;
    let t43984 = t11285 * t3395;
    let t43987 = 0.61524113149298439947e4 * t1164 * t11282 * t3377 * t43984;
    let t43989 = 24.0 * t11424 * t11403;
    (t43973, t43975, t43979, t43982, t43984, t43987, t43989)
}
