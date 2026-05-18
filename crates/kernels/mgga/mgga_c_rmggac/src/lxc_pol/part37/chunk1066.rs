//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1066/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1066<F: Float>(t74919: F, t71196: F, t71207: F, t73309: F, t74927: F, t74929: F, t74930: F, t74932: F, t77256: F, t77258: F, t77260: F, t77265: F, t77271: F, t77275: F, t77279: F, t77280: F, t77281: F) -> F {
    let t80179 = F::new(0.24527028530061914062e-5) * t74919;
    let t80182 = -t77256 + t71196 + t80179 + t77258 + t73309 - t77260 + t77265 - t74927 + t74929 + F::new(0.93188427318671584242e-2) * t74930 - F::new(0.15531404553111930707e-1) * t74932 - t71207 - t77271 + t77275 + t77279 + t77280 - t77281;
    t80182
}
