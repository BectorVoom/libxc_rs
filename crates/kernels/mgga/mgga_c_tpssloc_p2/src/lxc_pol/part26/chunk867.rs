//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 867/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk867<F: Float>(t10181: F, t3174: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t2990: F, t2986: F, t2770: F, t607: F) -> (F, F, F, F) {
    let t10182 = t3174 * t10181;
    let t10186 = t615 * t337 * t1887;
    let t10189 = t134 * t976;
    let t10190 = t10189 * t984;
    let t10191 = t10190 * t2990;
    let t10192 = t2986 * t10191;
    let t10194 = t2770 * t607;
    (t10182, t10186, t10192, t10194)
}
