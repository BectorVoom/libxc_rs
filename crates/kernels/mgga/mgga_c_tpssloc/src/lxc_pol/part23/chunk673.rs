//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 673/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk673<F: Float>(t1021: F, t248: F, t5878: F, t3151: F, t5392: F, t974: F, t5398: F, t998: F, t3146: F, t1044: F, t5681: F, t225: F, t5848: F, t68: F, t369: F, t1539: F, t1616: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5880 = t248 * t1021 * t5878;
    let t5884 = t3151 * t5392;
    let t5885 = t974 * t5884;
    let t5889 = t998 * t5398;
    let t5890 = t974 * t5889;
    let t5893 = t3146 * t5392;
    let t5894 = t974 * t5893;
    let t5900 = t248 * t1044 * t5681;
    let t5903 = t5848 * t225;
    let t5904 = t5903 * t68;
    let t5905 = t5904 * t369;
    let t5908 = t1616 * t1539;
    (t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900, t5903, t5904, t5905, t5908)
}
