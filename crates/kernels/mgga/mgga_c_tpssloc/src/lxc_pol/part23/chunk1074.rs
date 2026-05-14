//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1074/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1074<F: Float>(t10969: F, t121: F, t10213: F, t41687: F, t1043: F, t204: F, t340: F, t625: F, t221: F, t339: F, t344: F, t10277: F, t343: F, t42308: F, t974: F, t41666: F) -> (F, F, F, F, F, F, F, F) {
    let t42592 = t121 * t10969;
    let t42624 = t10213 * t41687;
    let t42749 = t204 * t1043;
    let t42813 = t625 * t340;
    let t42817 = 0.82304526748971193413e-3 * t339 * t221 * t42813 * t344;
    let t42841 = t343 * t10277;
    let t42861 = t974 * t42308;
    let t42862 = t344 * t41666;
    (t42592, t42624, t42749, t42813, t42817, t42841, t42861, t42862)
}
