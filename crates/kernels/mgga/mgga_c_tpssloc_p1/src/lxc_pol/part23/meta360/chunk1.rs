//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1159/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1159<F: Float>(t340: F, t625: F, t221: F, t339: F, t344: F, t10277: F, t343: F, t42308: F, t974: F, t41666: F, t2978: F, t698: F) -> (F, F, F, F, F, F) {
    let t42813 = t625 * t340;
    let t42817 = F::cast_from(0.82304526748971193413e-3_f64) * t339 * t221 * t42813 * t344;
    let t42841 = t343 * t10277;
    let t42861 = t974 * t42308;
    let t42862 = t344 * t41666;
    let t42875 = t698 * t2978;
    (t42813, t42817, t42841, t42861, t42862, t42875)
}
