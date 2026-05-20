//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1738/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1738<F: Float>(t26198: F, t12020: F, t2091: F, t5325: F, t26200: F, t3887: F, t5353: F, t1375: F, t26184: F, t26187: F, t26191: F, t26195: F, t26204: F, t26207: F, t26212: F, t26224: F, t3758: F, t5326: F, t7194: F, t7925: F) -> (F, F, F, F) {
    let t26988 = F::cast_from(0.16449340668482264365e-1_f64) * t26198;
    let t26989 = t12020 * t2091;
    let t26990 = t26989 * t5325;
    let t26993 = F::cast_from(0.38381794893125283518e-1_f64) * t26200;
    let t26996 = t3887 * t2091 * t5353;
    let t27005 = F::cast_from(0.76763589786250567037e-1_f64) * t26184 - F::cast_from(0.3289868133696452873e-1_f64) * t26187 - F::cast_from(0.3289868133696452873e-1_f64) * t26191 - F::cast_from(0.3289868133696452873e-1_f64) * t26195 + t26988 - F::new(6.0) * t26224 * t26990 + t26993 - F::cast_from(0.16449340668482264365e-1_f64) * t26204 + F::new(2.0) * t1375 * t26996 + F::new(2.0) * t7194 * t5326 + F::new(2.0) * t3758 * t7925 - F::cast_from(0.16449340668482264365e-1_f64) * t26207 + F::cast_from(0.16449340668482264365e-1_f64) * t26212;
    (t26989, t26990, t26996, t27005)
}
