//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 949/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk949<F: Float>(t74058: F, t74063: F, t74065: F, t74082: F, t74084: F, t74088: F, t74092: F, t74096: F, t74102: F, t70819: F, t74060: F, t74069: F, t74072: F, t74075: F, t74078: F, t74105: F, t74107: F) -> F {
    let t76846 = F::new(0.40911992481368012592e-1) * t74058;
    let t76848 = F::new(0.2627895913935205078e-5) * t74063;
    let t76849 = F::new(0.2627895913935205078e-5) * t74065;
    let t76854 = F::new(0.35913881159970051994e-4) * t74082;
    let t76855 = F::new(0.3830813990396805546e-4) * t74084;
    let t76856 = F::new(0.2553875993597870364e-4) * t74088;
    let t76857 = F::new(0.2553875993597870364e-4) * t74092;
    let t76858 = F::new(0.1702583995731913576e-4) * t74096;
    let t76859 = F::new(0.2553875993597870364e-4) * t74102;
    let t76862 = -t76846 + t70819 + F::new(0.17451485956252114154e-4) * t74060 + t76848 - t76849 + F::new(0.17519306092901367187e-5) * t74069 + F::new(0.52557918278704101564e-6) * t74072 - F::new(0.52557918278704101564e-6) * t74075 - F::new(0.35038612185802734376e-6) * t74078 - t76854 + t76855 + t76856 - t76857 - t76858 + t76859 + F::new(0.58171619854173713846e-5) * t74105 + F::new(0.36357262408858571154e-4) * t74107;
    t76862
}
