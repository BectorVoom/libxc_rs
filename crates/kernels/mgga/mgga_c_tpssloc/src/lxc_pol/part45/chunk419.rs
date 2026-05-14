//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 419/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk419<F: Float>(t2244: F, t2980: F, t2979: F, t337: F, t39: F, t1887: F, t60: F, t976: F, t984: F, t343: F, t883: F, t607: F, t2775: F, t344: F, t977: F, t2250: F, t978: F) -> (F, F, F, F, F, F) {
    let t2981 = t2980 * t2244;
    let t2982 = t2979 * t2981;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2988 = t2987 * t984;
    let t2989 = t343 * t883;
    let t2990 = t2989 * t607;
    let t2991 = t2988 * t2990;
    let t2994 = t344 * t2775;
    let t2995 = t2994 * t2244;
    let t2996 = t977 * t2995;
    let t2999 = t978 * t2250;
    (t2982, t2986, t2987, t2991, t2996, t2999)
}
