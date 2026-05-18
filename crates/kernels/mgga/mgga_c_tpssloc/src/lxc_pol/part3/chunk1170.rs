//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1170/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1170<F: Float>(t3450: F, t3966: F, t3449: F, t14749: F, t4908: F, t3448: F, t4928: F, t3451: F, t11579: F, t4919: F, t11584: F, t1174: F, t15294: F, t15300: F, t15304: F, t15307: F, t3443: F, t3447: F, t3457: F, t3461: F, t4889: F) -> F {
    let t15313 = t3450 * t3966;
    let t15314 = t3449 * t15313;
    let t15317 = t4908 * t14749;
    let t15320 = t3448 * t4928;
    let t15321 = t15320 * t3451;
    let t15324 = t4919 * t11579;
    let t15327 = t4919 * t11584;
    let t15330 = F::new(0.11111111111111111111e-2) * t3447 * t15294 - F::new(0.98765432098765432097e-3) * t4889 * t3443 + F::new(0.6172839506172839506e-4) * t15300 - F::new(0.83333333333333333332e-3) * t1174 * t15304 + F::new(0.49382716049382716048e-3) * t15307 + F::new(0.74074074074074074073e-3) * t4889 * t3461 + F::new(0.14814814814814814815e-2) * t4889 * t3457 + F::new(0.55555555555555555554e-3) * t3447 * t15314 - F::new(0.11111111111111111111e-2) * t3447 * t15317 + F::new(0.55555555555555555554e-3) * t3447 * t15321 + F::new(0.27777777777777777777e-3) * t3447 * t15324 + F::new(0.55555555555555555554e-3) * t3447 * t15327;
    t15330
}
