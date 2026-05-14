//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 818/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk818<F: Float>(t113123: F, t25927: F, t113069: F, t23788: F, t2240: F, t2244: F, t32: F, t641: F, t31: F, t607: F, t645: F, t79: F, t8306: F, t608: F, t22633: F, t22635: F, t31090: F, t90506: F) -> (F, F, F, F, F, F, F, F) {
    let t113764 = t25927 * t113123;
    let t113772 = t23788 * t113069;
    let t113824 = t2240 * t32 * t2244;
    let t113836 = t641 * t641;
    let t113864 = t645 * t31 * t607;
    let t113875 = t8306 * t79;
    let t113876 = t608 * t641;
    let t113931 = 0.13159472534785811492e0 * t22633 * t22635 * t31090 * t90506;
    (t113764, t113772, t113824, t113836, t113864, t113875, t113876, t113931)
}
