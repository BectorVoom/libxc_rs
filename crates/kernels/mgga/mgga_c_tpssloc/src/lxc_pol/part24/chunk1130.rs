//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1130/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1130<F: Float>(t225: F, t9585: F, t10108: F, t257: F, t68: F, t252: F, t9957: F, t2678: F, t852: F, t9520: F, t1022: F, t2250: F, t11018: F, t11016: F, t11013: F, t10163: F, t386: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40875 = t9585 * t225;
    let t40889 = 1.0 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40909 = t252 * t9957;
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t43240 = t2250 * t1022;
    let t43431 = t11018 * t225;
    let t43440 = t11016 * t225;
    let t43599 = t11013 * t225;
    let t43603 = 1.0 / t10163 / t386;
    (t40875, t40890, t40909, t40955, t41554, t43240, t43431, t43440, t43599, t43603)
}
