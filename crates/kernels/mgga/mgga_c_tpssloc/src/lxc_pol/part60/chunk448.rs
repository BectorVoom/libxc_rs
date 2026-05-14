//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 448/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk448<F: Float>(t4514: F, t4531: F, t2994: F, t5392: F, t977: F, t5398: F, t978: F, t3003: F, t4384: F, t5718: F, t5721: F, t5724: F, t340: F, t343: F, t974: F, t1597: F) -> (F, F, F, F, F) {
    let t5821 = t4531 * t4514;
    let t5824 = t2994 * t5392;
    let t5825 = t977 * t5824;
    let t5828 = t978 * t5398;
    let t5829 = t977 * t5828;
    let t5836 = -t3003 - 2.0 / 9.0 * t4384 + t5718 / 18.0 - t5721 / 3.0 + t5724 / 6.0;
    let t5837 = t340 * t5836;
    let t5838 = t5837 * t343;
    let t5839 = t974 * t5838;
    let t5842 = t1597 * t1597;
    (t5821, t5825, t5829, t5839, t5842)
}
