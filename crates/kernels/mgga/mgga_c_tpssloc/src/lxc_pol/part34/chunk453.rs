//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 453/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk453<F: Float>(t2978: F, t974: F, t2770: F, t344: F, t337: F, t39: F, t1887: F, t60: F, t976: F, t343: F, t883: F, t2775: F) -> (F, F, F, F, F, F) {
    let t2979 = t974 * t2978;
    let t2980 = t344 * t2770;
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2989 = t343 * t883;
    let t2994 = t344 * t2775;
    (t2979, t2980, t2986, t2987, t2989, t2994)
}
