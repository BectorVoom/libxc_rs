//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1303/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1303<F: Float>(t18915: F, t6102: F, t6274: F, t3313: F, t5989: F, t6020: F, t1703: F, t71231: F, t14838: F, t21895: F, t14850: F, t21899: F, t11190: F, t6024: F, t1670: F, t21810: F, t3264: F) -> (F, F, F, F, F, F, F, F) {
    let t78344 = 0.35089341735807877242e1 * t18915 * t6102;
    let t78348 = t6274 * t6274;
    let t78355 = 36.0 * t3313 * t5989 * t6020;
    let t78357 = 0.23392894490538584828e1 * t71231 * t1703;
    let t78359 = 24.0 * t14838 * t21895;
    let t78361 = 0.1929837539843104208e3 * t14850 * t21899;
    let t78364 = 0.57895126195293126241e3 * t11190 * t6024 * t6020;
    let t78367 = 8.0 * t3264 * t21810 * t1670;
    (t78344, t78348, t78355, t78357, t78359, t78361, t78364, t78367)
}
