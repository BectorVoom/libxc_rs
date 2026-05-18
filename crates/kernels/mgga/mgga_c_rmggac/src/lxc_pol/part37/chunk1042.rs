//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1042/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1042<F: Float>(t73857: F, t73862: F, t76713: F, t76718: F, t76723: F, t76728: F, t76733: F, t76738: F, t76743: F, t76744: F, t76745: F, t76748: F, t76749: F, t76750: F, t76751: F, t76752: F, t76753: F) -> F {
    let t79993 = -t76713 + t76718 - t76723 + t76728 - t76733 - t76738 + t76743 + t76744 - t76745 - F::new(0.87596530464506835932e-6) * t73857 + F::new(0.87596530464506835932e-6) * t73862 - t76748 - t76749 + t76750 - t76751 + t76752 + t76753;
    t79993
}
