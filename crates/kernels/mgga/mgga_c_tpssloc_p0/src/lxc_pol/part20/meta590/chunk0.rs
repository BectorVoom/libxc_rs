//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2169/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2169<F: Float>(t39267: F, t404: F, t410: F, t407: F, t43819: F, t1098: F, t11470: F, t3256: F, t3312: F, t1094: F, t11274: F, t3262: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F, F) {
    let t43880 = F::new(1.0) / t410 / t39267 / t404 / F::new(96.0);
    let t43889 = F::powf(t407, -F::new(0.25e1));
    let t43895 = F::cast_from(0.31310740740740740741e1_f64) * t43819;
    let t43942 = F::cast_from(0.96141975308641975307e-1_f64) * t43819;
    let t43954 = t11470 * t1098;
    let t43959 = t3256 * t3312;
    let t43964 = t1094 * t11274;
    let t43969 = t409 / t3311 / t3262;
    (t43880, t43889, t43895, t43942, t43954, t43959, t43964, t43969)
}
