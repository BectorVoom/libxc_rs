//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2459/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2459<F: Float>(t407: F, t43819: F, t3256: F, t3312: F, t1094: F, t11274: F, t3262: F, t3311: F, t409: F, t11285: F, t3395: F, t43776: F) -> (F, F, F, F, F, F, F, F) {
    let t43889 = F::powf(t407, -F::cast_from(0.25e1_f64));
    let t43895 = F::cast_from(0.31310740740740740741e1_f64) * t43819;
    let t43942 = F::cast_from(0.96141975308641975307e-1_f64) * t43819;
    let t43959 = t3256 * t3312;
    let t43964 = t1094 * t11274;
    let t43969 = t409 / t3311 / t3262;
    let t43984 = t11285 * t3395;
    let t44027 = F::cast_from(0.13388493827160493828e1_f64) * t43776;
    (t43889, t43895, t43942, t43959, t43964, t43969, t43984, t44027)
}
