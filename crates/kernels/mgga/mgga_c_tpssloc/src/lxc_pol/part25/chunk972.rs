//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 972/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk972<F: Float>(t10108: F, t257: F, t68: F, t252: F, t9957: F, t2678: F, t852: F, t225: F, t9520: F, t112: F, t12512: F, t111: F, t3931: F, t2311: F, t671: F, t2363: F, t649: F) -> (F, F, F, F, F, F, F, F) {
    let t40889 = 1.0 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40909 = t252 * t9957;
    let t40955 = t852 * t2678;
    let t41554 = t9520 * t225;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45602 = t2311 * t671;
    let t45637 = t649 * t2363;
    (t40890, t40909, t40955, t41554, t45557, t45560, t45602, t45637)
}
