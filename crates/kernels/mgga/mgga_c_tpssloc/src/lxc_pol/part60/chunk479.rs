//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 479/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk479<F: Float>(t360: F, t5866: F, t1021: F, t248: F, t1615: F, t3131: F, t3151: F, t5392: F, t974: F, t5398: F, t998: F, t3146: F) -> (F, F, F, F, F, F, F) {
    let t5867 = t5866 * t360;
    let t5869 = t248 * t1021 * t5867;
    let t5872 = t1615 * t1615;
    let t5873 = t5872 * t3131;
    let t5875 = t248 * t1021 * t5873;
    let t5878 = t5872 * t360;
    let t5880 = t248 * t1021 * t5878;
    let t5884 = t3151 * t5392;
    let t5885 = t974 * t5884;
    let t5889 = t998 * t5398;
    let t5890 = t974 * t5889;
    let t5893 = t3146 * t5392;
    (t5869, t5872, t5875, t5880, t5885, t5890, t5893)
}
