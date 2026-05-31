//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1202/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1202<F: Float>(t225: F, t3023: F, t1053: F, t68: F, t3021: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t2990: F) -> (F, F, F, F, F, F, F) {
    let t10160 = t3023 * t225;
    let t10163 = t1053 * t1053;
    let t10164 = F::cast_from(1.0_f64) / t10163;
    let t10165 = t68 * t10164;
    let t10170 = t3021 * t225;
    let t10186 = t615 * t337 * t1887;
    let t10189 = t134 * t976;
    let t10190 = t10189 * t984;
    let t10191 = t10190 * t2990;
    (t10160, t10165, t10170, t10186, t10189, t10190, t10191)
}
