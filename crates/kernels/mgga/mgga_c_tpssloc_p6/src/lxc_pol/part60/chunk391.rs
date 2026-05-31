//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 391/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk391<F: Float>(t337: F, t39: F, t1887: F, t60: F, t976: F, t343: F, t883: F, t2775: F, t344: F, t2822: F, t1008: F, t191: F) -> (F, F, F, F, F, F) {
    let t2985 = t39 * t337;
    let t2986 = t2985 * t1887;
    let t2987 = t60 * t976;
    let t2989 = t343 * t883;
    let t2994 = t344 * t2775;
    let t3003 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t2822;
    let t3030 = F::cast_from(1.0_f64) / t1008 / t191;
    (t2986, t2987, t2989, t2994, t3003, t3030)
}
