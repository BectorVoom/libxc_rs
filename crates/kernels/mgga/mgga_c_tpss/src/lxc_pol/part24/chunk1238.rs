//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1238/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1238<F: Float>(t65: F, t8491: F, t1980: F, t3416: F, t1286: F, t7689: F, t4566: F, t13296: F, t577: F, t116: F, t13451: F, t1232: F, t5407: F, t5380: F, t10089: F, t13943: F, t3205: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t36839 = t65 * t8491;
    let t42178 = t3416 * t1980;
    let t42181 = t1286 * t7689;
    let t42667 = t4566 * t1980;
    let t42690 = t13296 * t577;
    let t42710 = t13451 * t116;
    let t43101 = t5407 * t1232;
    let t43602 = t5380 * t1232;
    let t43710 = t5380 * t10089;
    let t44034 = t13943 * t3205;
    (t36839, t42178, t42181, t42667, t42690, t42710, t43101, t43602, t43710, t44034)
}
