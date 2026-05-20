//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2635/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2635<F: Float>(t72180: F, t72233: F, t72268: F, t72299: F, t72333: F, t72357: F, t72380: F, t72405: F, t72452: F, t72484: F, t72522: F, t72552: F, t72593: F, t72622: F, t72654: F, t72683: F, t72712: F, t72735: F, t72783: F, t72823: F, t72842: F, t72878: F, t72911: F, t72938: F, t72970: F, t72996: F, t73019: F, t73048: F, t73078: F, t73108: F, t73126: F, t73587: F) -> F {
    let t73592 = t72268 + t72233 + t72380 + t72878 + t72622 + t72484 + t72911 + t72735 + t72299 + t72522 + t72333 + t72683 + t72452 + t72552 + t72357 + t72405 + t73078 + t72996 + t72712 + t73019 + t73587 + t72593 + t72783 + t73108 + t72654 + t72180 + t72938 + t72823 + t73048 + t72842 + t72970 + t73126;
    t73592
}
