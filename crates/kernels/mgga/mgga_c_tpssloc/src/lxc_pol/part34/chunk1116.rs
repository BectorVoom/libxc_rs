//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1116/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1116<F: Float>(t81920: F, t81954: F, t2047: F, t9971: F, t81688: F, t81716: F, t82046: F, t82122: F, t82153: F, t82218: F, t1453: F, t81439: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t84921 = F::new(595.0) / F::new(2592.0) * t81920;
    let t84932 = F::new(0.67287926823567318088e-4) * t81954;
    let t84953 = t9971 * t2047;
    let t84995 = F::new(0.27415567780803773942e-2) * t81688;
    let t85003 = F::new(0.19739208802178717238e0) * t81716;
    let t85027 = F::new(0.55440370401180965083e0) * t82046;
    let t85060 = F::new(0.3244175520728446583e0) * t82122;
    let t85101 = F::new(0.27415567780803773942e-2) * t82153;
    let t85129 = F::new(0.55440370401180965083e0) * t82218;
    let t86586 = t81439 * t1453;
    (t84921, t84932, t84953, t84995, t85003, t85027, t85060, t85101, t85129, t86586)
}
