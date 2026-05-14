//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 797/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk797<F: Float>(t76840: F, t74050: F, t74052: F, t74056: F, t74058: F, t74063: F, t74065: F, t74082: F, t74084: F, t74088: F, t74092: F, t74096: F, t74102: F, t74161: F, t74163: F, t70867: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t76841 = 0.40650199722100037752e-3 * t76840;
    let t76842 = 0.20455996240684006296e-1 * t74050;
    let t76843 = 0.81823984962736025184e-1 * t74052;
    let t76844 = 0.20455996240684006296e0 * t74056;
    let t76846 = 0.40911992481368012592e-1 * t74058;
    let t76848 = 0.2627895913935205078e-5 * t74063;
    let t76849 = 0.2627895913935205078e-5 * t74065;
    let t76854 = 0.35913881159970051994e-4 * t74082;
    let t76855 = 0.3830813990396805546e-4 * t74084;
    let t76856 = 0.2553875993597870364e-4 * t74088;
    let t76857 = 0.2553875993597870364e-4 * t74092;
    let t76858 = 0.1702583995731913576e-4 * t74096;
    let t76859 = 0.2553875993597870364e-4 * t74102;
    let t76878 = 0.23268647941669485538e-4 * t74161;
    let t76879 = 0.11634323970834742769e-3 * t74163;
    let t76880 = 0.29795219925308487579e-4 * t70867;
    (t76841, t76842, t76843, t76844, t76846, t76848, t76849, t76854, t76855, t76856, t76857, t76858, t76859, t76878, t76879, t76880)
}
