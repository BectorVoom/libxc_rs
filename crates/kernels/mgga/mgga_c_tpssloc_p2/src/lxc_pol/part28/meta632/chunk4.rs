//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1993/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1993<F: Float>(t87835: F, t87873: F, t225: F, t26734: F, t13072: F, t13463: F, t1528: F, t2054: F, t218: F, t259: F, t26582: F, t26703: F, t2713: F, t47585: F, t7087: F, t7107: F, t85146: F, t85152: F, t866: F, t87893: F, t92722: F) -> (F, F) {
    let t92910 = F::cast_from(0.3289868133696452873e-1_f64) * t87835;
    let t92938 = F::cast_from(0.3289868133696452873e-1_f64) * t87873;
    let t92939 = t26734 * t225;
    let t92950 = -F::new(2.0) * t13463 * t7107 - t47585 * t2054 + F::new(4.0) * t2713 * t26703 + F::new(4.0) * t2713 * t26582 - t92938 - F::new(2.0) * t92939 * t866 - F::new(2.0) * t85146 * t1528 - t85152 * t1528 + F::new(4.0) * t7087 * t13072 + t218 * t92722 * t259 + F::cast_from(0.3289868133696452873e-1_f64) * t87893;
    (t92910, t92950)
}
