//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2174/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2174<F: Float>(t11923: F, t225: F, t10913: F, t11583: F, t11570: F, t1174: F, t3471: F, t698: F, t3477: F, t11504: F, t135: F, t43776: F) -> (F, F, F, F, F, F, F) {
    let t44412 = t11923 * t225;
    let t44415 = t11583 * t10913;
    let t44419 = t11570 * t10913;
    let t44424 = t1174 * t698 * t3471;
    let t44439 = t1174 * t698 * t3477;
    let t44445 = t1174 * t135 * t11504;
    let t44466 = F::new(220.0) / F::new(81.0) * t43776;
    (t44412, t44415, t44419, t44424, t44439, t44445, t44466)
}
