//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 950/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk950<F: Float>(t74161: F, t74163: F, t70867: F, t68491: F, t74112: F, t74115: F, t74118: F, t74122: F, t74125: F, t74128: F, t74131: F, t74137: F, t74142: F, t74147: F, t74152: F, t74154: F, t74159: F) -> F {
    let t76878 = F::new(0.23268647941669485538e-4) * t74161;
    let t76879 = F::new(0.11634323970834742769e-3) * t74163;
    let t76880 = F::new(0.29795219925308487579e-4) * t70867;
    let t76881 = F::new(0.13139479569676025391e-5) * t74112 + F::new(0.58171619854173713846e-5) * t74115 + F::new(0.10511583655740820313e-5) * t74118 - F::new(0.10511583655740820313e-5) * t74122 + F::new(0.15767375483611230469e-5) * t74125 + F::new(0.52557918278704101564e-6) * t74128 - F::new(0.52557918278704101564e-6) * t74131 - F::new(0.81756761766873046872e-5) * t68491 + F::new(0.70077224371605468752e-6) * t74137 - F::new(0.10511583655740820313e-5) * t74142 + F::new(0.10511583655740820313e-5) * t74147 + F::new(0.35038612185802734376e-6) * t74152 + F::new(0.17451485956252114154e-4) * t74154 + F::new(0.17451485956252114154e-4) * t74159 - t76878 + t76879 - t76880;
    t76881
}
